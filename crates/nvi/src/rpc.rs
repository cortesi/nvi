use std::{io, net::SocketAddr, path::Path, pin::Pin};

use futures::{
    io::{AsyncRead, AsyncWrite},
    Future,
};

use async_trait::async_trait;
use msgpack_rpc::{Endpoint, Value};
use tokio::{
    net::{TcpListener, TcpStream, UnixListener, UnixStream},
    runtime::Handle,
    sync::{broadcast, mpsc},
    task::JoinSet,
};
use tokio_util::compat::TokioAsyncReadCompatExt;
use tracing::{debug, error, trace, warn};

use crate::{client::NviClient, error::Result};

const BOOTSTRAP_NOTIFICATION: &str = "nvi_bootstrap";

#[allow(unused_variables)]
#[async_trait]
pub trait NviService: Clone + Send {
    async fn connected(&mut self, client: &mut NviClient) {}

    async fn handle_nvim_notification(
        &mut self,
        client: &mut NviClient,
        method: &str,
        params: &[Value],
    ) {
        warn!("unhandled notification: {:?}", method);
    }

    async fn handle_nvim_request(
        &mut self,
        client: &mut NviClient,
        method: &str,
        params: &[Value],
    ) -> Result<Value, Value> {
        warn!("unhandled request: {:?}", method);
        Err(Value::Nil)
    }
}

#[derive(Clone)]
pub struct AsyncClosureService<F>
where
    F: for<'a> Fn(&'a mut NviClient) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>
        + Clone
        + Send
        + 'static,
{
    connected_closure: F,
}

impl<F> AsyncClosureService<F>
where
    F: for<'a> Fn(&'a mut NviClient) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>
        + Clone
        + Send
        + 'static,
{
    pub fn new(connected_closure: F) -> Self {
        AsyncClosureService { connected_closure }
    }
}

#[async_trait]
impl<F> NviService for AsyncClosureService<F>
where
    F: for<'a> Fn(&'a mut NviClient) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>
        + Clone
        + Send
        + 'static,
{
    async fn connected(&mut self, client: &mut NviClient) {
        (self.connected_closure)(client).await;
    }
}

// Service handles a single connection to neovim.
#[derive(Clone)]
struct ServiceWrapper<T>
where
    T: NviService,
{
    nvi_service: T,
    shutdown_tx: broadcast::Sender<()>,
    channel_id: Option<u64>,
}

impl<T> ServiceWrapper<T>
where
    T: NviService,
{
    fn new(nvi_service: T, shutdown_tx: broadcast::Sender<()>) -> Self {
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

async fn bootstrap(c: msgpack_rpc::Client, shutdown_tx: broadcast::Sender<()>) -> Result<()> {
    let nc = &mut NviClient::new(&c, None, shutdown_tx);
    let (id, _v) = nc.api.nvim_get_api_info().await?;
    nc.api
        .nvim_exec_lua(
            &format!("vim.rpcnotify(..., '{}', ...)", BOOTSTRAP_NOTIFICATION),
            vec![id.into()],
        )
        .await?;
    Ok(())
}

/// Connect on a stream, and return a sender to shutdown the connection.
pub async fn connect_stream<T, S>(
    shutdown_tx: broadcast::Sender<()>,
    stream: S,
    service: T,
) -> Result<()>
where
    S: AsyncRead + AsyncWrite + Send + 'static,
    T: NviService + Unpin + 'static,
{
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
    let _ = shutdown_tx.subscribe().recv().await;
    js.abort_all();
    while js.join_next().await.is_some() {}
    Ok(())
}

/// Connect on a Unix socket, and return a sender to shutdown the connection.
pub async fn connect_unix<T, P>(
    shutdown_tx: broadcast::Sender<()>,
    path: P,
    service: T,
) -> Result<()>
where
    P: AsRef<Path>,
    T: NviService + Unpin + 'static,
{
    connect_stream(
        shutdown_tx,
        UnixStream::connect(path).await?.compat(),
        service,
    )
    .await
}

/// Connect to a TCP address, and return a sender to shutdown the connection.
pub async fn connect_tcp<T>(
    shutdown_tx: broadcast::Sender<()>,
    addr: SocketAddr,
    service: T,
) -> Result<()>
where
    T: NviService + Unpin + 'static,
{
    connect_stream(
        shutdown_tx,
        TcpStream::connect(&addr).await?.compat(),
        service,
    )
    .await
}

pub async fn listen_unix<T, F, P>(path: P, nvi_service_maker: F) -> Result<()>
where
    F: Fn() -> T + Send + 'static,
    T: NviService + Unpin + 'static,
    P: AsRef<Path>,
{
    // let (shutdown_tx, _) = mpsc::unbounded_channel();
    // let listener = UnixListener::bind(path)?;
    //
    // let _ = tokio::spawn(async move {
    //     loop {
    //         match listener.accept().await {
    //             Ok((socket, _)) => {
    //                 let endpoint = Endpoint::new(
    //                     socket.compat(),
    //                     ServiceWrapper::new(nvi_service_maker(), shutdown_tx.clone()),
    //                 );
    //
    //                 tokio::spawn(endpoint);
    //             }
    //             Err(e) => error!("Error accepting connection: {}", e),
    //         }
    //     }
    // })
    // .await;
    Ok(())
}

pub async fn listen_tcp<T, F>(addr: SocketAddr, nvi_service_maker: F) -> io::Result<()>
where
    F: Fn() -> T + Send + 'static,
    T: NviService + Unpin + 'static,
{
    // let (shutdown_tx, _) = mpsc::unbounded_channel();
    // let listener = TcpListener::bind(&addr).await?;
    // let _ = tokio::spawn(async move {
    //     loop {
    //         match listener.accept().await {
    //             Ok((socket, _)) => {
    //                 tokio::spawn(Endpoint::new(
    //                     socket.compat(),
    //                     ServiceWrapper::new(nvi_service_maker(), shutdown_tx.clone()),
    //                 ));
    //             }
    //             Err(e) => debug!("Error accepting connection: {}", e),
    //         }
    //     }
    // })
    // .await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use nix::sys::signal::{killpg, Signal};
    use nix::unistd::Pid;
    use std::os::unix::process::CommandExt;
    use tokio::{process::Command, sync::broadcast};
    use tracing_test::traced_test;

    async fn start_nvim(
        mut termrx: broadcast::Receiver<()>,
        socket_path: std::path::PathBuf,
    ) -> Result<()> {
        // This entire little dance requires explanation. First, neovim spawns a subprocess, so
        // in order to kill the process, we need to kill the entire process group. Second, tokio's
        // process group functionality is not stabilized yet, so we construct a
        // std::process::Command and convert it into a tokio::process::Command. Finally, we use nix
        // to kill the process group.
        let mut oscmd = std::process::Command::new("nvim");
        oscmd
            .arg("--headless")
            .arg("--clean")
            .arg("--listen")
            .process_group(0)
            .arg(format!("{}", socket_path.to_string_lossy()));
        let mut child = Command::from(oscmd).spawn()?;
        let pgid = Pid::from_raw(child.id().unwrap() as i32);

        let _ = termrx.recv().await;
        killpg(pgid, Signal::SIGTERM).map_err(|e| crate::error::Error::Internal {
            msg: format!("could not kill process group {}", e),
        })?;
        child.wait().await?;
        Ok(())
    }

    async fn test_service<T>(nvi: T, shutdown_tx: broadcast::Sender<()>) -> Result<()>
    where
        T: NviService + Unpin + 'static,
    {
        let tempdir = tempfile::tempdir()?;
        let socket_path = tempdir.path().join("nvim.socket");

        let sp = socket_path.clone();
        let srx = shutdown_tx.subscribe();
        let nv = tokio::spawn(async move { start_nvim(srx, sp).await });

        for _ in 0..10 {
            if socket_path.exists() {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }

        if !socket_path.exists() {
            return Err(crate::error::Error::IO {
                msg: "socket never appeared".to_string(),
            });
        }

        let serv = connect_unix(shutdown_tx, socket_path, nvi);
        serv.await?;
        nv.await.unwrap()?;
        Ok(())
    }

    #[tokio::test]
    #[traced_test]
    async fn it_connects() {
        let (tx, _) = broadcast::channel(16);

        let rtx = tx.clone();
        let s = AsyncClosureService::new(move |client| {
            Box::pin({
                let tx = tx.clone();
                async move {
                    tx.send(()).unwrap();
                    client.shutdown();
                }
            })
        });

        test_service(s, rtx).await.unwrap();
    }
}
