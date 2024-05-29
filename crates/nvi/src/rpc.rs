use std::{io, net::SocketAddr, path::Path, pin::Pin};

use futures::{
    io::{AsyncRead, AsyncWrite},
    Future,
};

use msgpack_rpc::{Endpoint, Value};
use tokio::{
    net::{TcpListener, TcpStream, UnixListener, UnixStream},
    runtime::Handle,
    sync::mpsc,
    task::JoinSet,
};
use tokio_util::compat::TokioAsyncReadCompatExt;
use tracing::{debug, error, trace, warn};

use crate::{client::NviClient, error::Result};

const BOOTSTRAP_NOTIFICATION: &str = "nvi_bootstrap";

/// NvService is the trait that must be implemented an addon that speaks to Neovim.
#[allow(unused_variables)]
pub trait NviService: Clone + Send {
    fn connected(
        &mut self,
        client: &mut NviClient,
    ) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }

    fn handle_nvim_notification(
        &mut self,
        client: &mut NviClient,
        method: &str,
        params: &[Value],
    ) -> impl std::future::Future<Output = ()> + Send {
        warn!("unhandled notification: {:?}", method);
        async {}
    }

    fn handle_nvim_request(
        &mut self,
        client: &mut NviClient,
        method: &str,
        params: &[Value],
    ) -> impl std::future::Future<Output = Result<Value, Value>> + Send {
        warn!("unhandled request: {:?}", method);
        async { Err(Value::Nil) }
    }
}

// Service handles a single connection to neovim.
#[derive(Clone)]
struct ServiceWrapper<T>
where
    T: NviService,
{
    nvi_service: T,
    shutdown_tx: mpsc::UnboundedSender<()>,
    channel_id: Option<u64>,
}

impl<T> ServiceWrapper<T>
where
    T: NviService,
{
    fn new(nvi_service: T, shutdown_tx: mpsc::UnboundedSender<()>) -> Self {
        ServiceWrapper {
            nvi_service,
            shutdown_tx,
            channel_id: None,
        }
    }
}

/// A wrapper service that translates from msgpack_rpc to NviService.
impl<T> msgpack_rpc::ServiceWithClient for ServiceWrapper<T>
where
    T: NviService + Send + 'static,
{
    type RequestFuture = Pin<Box<dyn Future<Output = Result<Value, Value>> + Send>>;

    fn handle_request(
        &mut self,
        client: &mut msgpack_rpc::Client,
        method: &str,
        params: &[Value],
    ) -> Self::RequestFuture {
        trace!("recv request: {:?} {:?}", method, params);
        let mut vimservice = self.nvi_service.clone();
        let mut client = NviClient::new(client, self.channel_id, self.shutdown_tx.clone());
        let method = method.to_string();
        let params = params.to_vec();
        Box::pin(async move {
            vimservice
                .handle_nvim_request(&mut client, &method, &params)
                .await
        })
    }

    fn handle_notification(
        &mut self,
        client: &mut msgpack_rpc::Client,
        method: &str,
        params: &[Value],
    ) {
        trace!("recv notifcation: {:?} {:?}", method, params);
        let mut vimservice = self.nvi_service.clone();
        let handle = Handle::current();
        let m_client = client.clone();
        if method == BOOTSTRAP_NOTIFICATION {
            let id = params[0].as_u64().unwrap();
            trace!("bootstrapped with channel id: {:?}", id);
            self.channel_id = Some(id);
            let channel_id = self.channel_id;
            let shutdown_tx = self.shutdown_tx.clone();
            handle.spawn(async move {
                vimservice
                    .connected(&mut NviClient::new(&m_client, channel_id, shutdown_tx))
                    .await;
            });
            return;
        }
        let method = method.to_string();
        let params = params.to_vec();
        let channel_id = self.channel_id;
        let shutdown_tx = self.shutdown_tx.clone();
        handle.spawn(async move {
            vimservice
                .handle_nvim_notification(
                    &mut NviClient::new(&m_client, channel_id, shutdown_tx),
                    &method,
                    &params,
                )
                .await;
        });
    }
}

async fn bootstrap(c: msgpack_rpc::Client, shutdown_tx: mpsc::UnboundedSender<()>) -> Result<()> {
    let nc = &mut NviClient::new(&c, None, shutdown_tx);
    let (id, _) = nc.api.nvim_get_api_info().await?;
    nc.api
        .nvim_exec_lua(
            &format!("vim.rpcnotify(..., '{}', ...)", BOOTSTRAP_NOTIFICATION),
            vec![id.into()],
        )
        .await?;
    Ok(())
}

/// Connect on a stream, and return a sender to shutdown the connection.
pub async fn connect_stream<T, S>(stream: S, service: T) -> Result<mpsc::UnboundedSender<()>>
where
    S: AsyncRead + AsyncWrite + Send + 'static,
    T: NviService + Unpin + 'static,
{
    let (shutdown_tx, mut shutdown_rx) = mpsc::unbounded_channel();
    let endpoint = Endpoint::new(stream, ServiceWrapper::new(service, shutdown_tx.clone()));

    let mut js = JoinSet::new();
    let epclient = endpoint.client();
    {
        let stx = shutdown_tx.clone();
        js.spawn(async move { bootstrap(epclient, stx).await });
    }
    {
        let stx = shutdown_tx.clone();
        js.spawn(async move {
            let ret = endpoint.await.map_err(|e| e.into());
            stx.send(()).unwrap();
            ret
        });
    }
    let _ = shutdown_rx.recv().await;
    js.abort_all();
    while js.join_next().await.is_some() {}
    Ok(shutdown_tx.clone())
}

/// Connect on a Unix socket, and return a sender to shutdown the connection.
pub async fn connect_unix<T, P>(path: P, service: T) -> Result<mpsc::UnboundedSender<()>>
where
    P: AsRef<Path>,
    T: NviService + Unpin + 'static,
{
    connect_stream(UnixStream::connect(path).await?.compat(), service).await
}

/// Connect to a TCP address, and return a sender to shutdown the connection.
pub async fn connect_tcp<T>(addr: SocketAddr, service: T) -> Result<mpsc::UnboundedSender<()>>
where
    T: NviService + Unpin + 'static,
{
    connect_stream(TcpStream::connect(&addr).await?.compat(), service).await
}

pub async fn listen_unix<T, F, P>(path: P, nvi_service_maker: F) -> Result<()>
where
    F: Fn() -> T + Send + 'static,
    T: NviService + Unpin + 'static,
    P: AsRef<Path>,
{
    let (mut shutdown_tx, mut shutdown_rx) = mpsc::unbounded_channel();
    let listener = UnixListener::bind(path)?;

    let _ = tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((socket, _)) => {
                    let endpoint = Endpoint::new(
                        socket.compat(),
                        ServiceWrapper::new(nvi_service_maker(), shutdown_tx.clone()),
                    );

                    tokio::spawn(endpoint);
                }
                Err(e) => error!("Error accepting connection: {}", e),
            }
        }
    })
    .await;
    Ok(())
}

pub async fn listen_tcp<T, F>(addr: SocketAddr, nvi_service_maker: F) -> io::Result<()>
where
    F: Fn() -> T + Send + 'static,
    T: NviService + Unpin + 'static,
{
    let (mut shutdown_tx, mut shutdown_rx) = mpsc::unbounded_channel();
    let listener = TcpListener::bind(&addr).await?;
    let _ = tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((socket, _)) => {
                    tokio::spawn(Endpoint::new(
                        socket.compat(),
                        ServiceWrapper::new(nvi_service_maker(), shutdown_tx.clone()),
                    ));
                }
                Err(e) => debug!("Error accepting connection: {}", e),
            }
        }
    })
    .await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::{Arc, Mutex};
    use tokio::process::Command;
    use tokio::sync::oneshot::{self, Sender};
    use tracing_test::traced_test;

    async fn start_nvim() -> Result<(Sender<()>, std::path::PathBuf), Box<dyn std::error::Error>> {
        let tempdir = tempfile::tempdir()?;
        let socket_path = tempdir.path().join("nvim.socket");

        let mut child = Command::new("nvim")
            .arg("--headless")
            .arg("--clean")
            .arg("--listen")
            .arg(format!("{}", socket_path.to_string_lossy()))
            .spawn()?;

        let (tx, rx) = oneshot::channel::<()>();

        tokio::spawn(async move {
            let _ = rx.await;
            let _ = child.kill().await;
            tempdir.close().unwrap();
        });

        Ok((tx, socket_path))
    }

    async fn connect_service<T>(nvi: T) -> Result<Sender<()>, Box<dyn std::error::Error>>
    where
        T: NviService + Unpin + 'static,
    {
        let (tx, socket_path) = start_nvim().await?;
        for _ in 0..10 {
            if socket_path.exists() {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        connect_unix(socket_path, nvi).await?;
        Ok(tx)
    }

    #[tokio::test]
    #[traced_test]
    async fn it_connects() {
        let (tx, rx) = oneshot::channel::<()>();

        #[derive(Clone)]
        struct Service {
            tx: Arc<Mutex<Option<Sender<()>>>>,
        }

        impl NviService for Service {
            async fn connected(&mut self, client: &mut NviClient) {
                println!("CONNECTED");
                let _ = self.tx.lock().unwrap().take().unwrap().send(());
                client.shutdown();
            }
        }

        let s = Service {
            tx: Arc::new(Mutex::new(Some(tx))),
        };

        let _c = connect_service(s).await.unwrap();

        rx.await.unwrap();
    }
}
