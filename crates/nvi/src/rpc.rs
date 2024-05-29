use std::{io, net::SocketAddr, path::Path, pin::Pin};

use futures::Future;
use msgpack_rpc::{Endpoint, ServiceWithClient, Value};
use tokio::{
    join,
    net::{TcpListener, TcpStream, UnixListener, UnixStream},
    runtime::Handle,
};
use tokio_util::compat::TokioAsyncReadCompatExt;
use tracing::{debug, error, trace, warn};

use crate::{client::NviClient, error::Result};

/// NvService is the trait that must be implemented an addon that speaks to Neovim.
pub trait NviService: Clone + Send {
    fn connected(
        &mut self,
        _client: &mut NviClient,
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
struct Service<T>
where
    T: NviService,
{
    vimservice: T,
}

impl<T> Service<T>
where
    T: NviService,
{
    fn new(vimservice: T) -> Self {
        Service { vimservice }
    }
}

/// A wrapper service that translates from msgpack_rpc to NviService.
impl<T> ServiceWithClient for Service<T>
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
        let mut vimservice = self.vimservice.clone();
        let mut client = NviClient {
            m_client: client.clone(),
        };
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
        let handle = Handle::current();
        let mut vimservice = self.vimservice.clone();
        let m_client = client.clone();
        let method = method.to_string();
        let params = params.to_vec();
        handle.spawn(async move {
            vimservice
                .handle_nvim_notification(&mut NviClient { m_client }, &method, &params)
                .await;
        });
    }
}

async fn bootstrap(c: &mut NviClient) -> Result<()> {
    let ret = c.nvim_get_api_info().await?;
    println!("API Info: {:#?}", ret);
    Ok(())
}

pub async fn connect_unix<T, P>(path: P, service: T) -> Result<()>
where
    P: AsRef<Path>,
    T: NviService + Unpin + 'static,
{
    let stream = UnixStream::connect(path).await?;
    let service = Service::new(service);
    let endpoint = Endpoint::new(stream.compat(), service);

    let epclient = endpoint.client();
    println!("pre");
    tokio::spawn(async move {
        bootstrap(&mut NviClient { m_client: epclient })
            .await
            .unwrap();
    });

    endpoint.await?;
    Ok(())
}

pub async fn connect_tcp<T>(addr: SocketAddr, service: T) -> Result<()>
where
    T: NviService + Unpin + 'static,
{
    let stream = TcpStream::connect(&addr).await?;
    let service = Service::new(service);
    let endpoint = Endpoint::new(stream.compat(), service);

    let epclient = endpoint.client();
    println!("pre");
    tokio::spawn(async move {
        bootstrap(&mut NviClient { m_client: epclient })
            .await
            .unwrap();
    });
    println!("awaiting endpoint");
    endpoint.await?;
    Ok(())
}

pub async fn listen_unix<T, F, P>(path: P, service_maker: F) -> Result<()>
where
    F: Fn() -> T + Send + 'static,
    T: NviService + Unpin + Send + Clone + 'static,
    P: AsRef<Path>,
{
    let listener = UnixListener::bind(path)?;

    let _ = tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((socket, _)) => {
                    let endpoint = Endpoint::new(socket.compat(), Service::new(service_maker()));

                    bootstrap(&mut NviClient {
                        m_client: endpoint.client(),
                    })
                    .await
                    .unwrap();

                    tokio::spawn(endpoint);
                }
                Err(e) => error!("Error accepting connection: {}", e),
            }
        }
    })
    .await;
    Ok(())
}

pub async fn listen_tcp<T, F>(addr: SocketAddr, service_maker: F) -> io::Result<()>
where
    F: Fn() -> T + Send + 'static,
    T: NviService + Unpin + Send + Clone + 'static,
{
    let listener = TcpListener::bind(&addr).await?;
    let _ = tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((socket, _)) => {
                    tokio::spawn(Endpoint::new(
                        socket.compat(),
                        Service::new(service_maker()),
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
    use tempfile;
    use tokio::process::Command;
    use tokio::sync::oneshot::{self, Sender};

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
    async fn it_connects() {
        let (tx, rx) = oneshot::channel::<()>();

        #[derive(Clone)]
        struct Service {
            tx: Arc<Mutex<Option<Sender<()>>>>,
        }

        impl NviService for Service {
            async fn connected(&mut self, _client: &mut NviClient) {
                let _ = self.tx.lock().unwrap().take().unwrap().send(());
            }
        }

        let s = Service {
            tx: Arc::new(Mutex::new(Some(tx))),
        };

        let _c = connect_service(s).await.unwrap();

        rx.await.unwrap();
    }
}
