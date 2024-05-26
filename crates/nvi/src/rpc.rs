use std::io;
use std::net::SocketAddr;
use std::pin::Pin;

use futures::Future;
use msgpack_rpc::{Endpoint, ServiceWithClient, Value};
use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};
use tokio_util::compat::TokioAsyncReadCompatExt;
use tracing::{debug, error};

use crate::client::Client;

// Service handles a single connection to neovim.
#[derive(Clone)]
pub struct Service<T>
where
    T: VimService,
{
    vimservice: T,
}

impl<T> Service<T>
where
    T: VimService,
{
    fn new(vimservice: T) -> Self {
        Service { vimservice }
    }
}

pub trait VimService {
    async fn handle_nvim_notification(
        &mut self,
        client: &mut Client,
        method: &str,
        params: &[Value],
    );

    fn handle_nvim_request(
        &mut self,
        client: &mut Client,
        method: &str,
        params: &[Value],
    ) -> Pin<Box<dyn Future<Output = Result<Value, Value>> + Send>>;
}

type ServiceMaker = dyn Fn(Client) -> Box<dyn VimService + Send>;

// Implement how the endpoint handles incoming requests and notifications.
// In this example, the endpoint does not handle notifications.
impl<T> ServiceWithClient for Service<T>
where
    T: VimService,
{
    type RequestFuture = Pin<Box<dyn Future<Output = Result<Value, Value>> + Send>>;

    fn handle_request(
        &mut self,
        client: &mut msgpack_rpc::Client,
        method: &str,
        params: &[Value],
    ) -> Self::RequestFuture {
        Box::pin(self.vimservice.handle_nvim_request(
            &mut Client { m_client: client },
            method,
            params,
        ))
    }

    fn handle_notification(
        &mut self,
        client: &mut msgpack_rpc::Client,
        method: &str,
        params: &[Value],
    ) {
        self.vimservice
            .handle_nvim_notification(&mut Client { m_client: client }, method, params);
    }
}

pub async fn connect_unix<T>(path: &str, service: T) -> io::Result<()>
where
    T: VimService + Unpin,
{
    let socket = UnixStream::connect(path).await?;
    let client = Service::new(service);
    let endpoint = Endpoint::new(socket.compat(), client);
    endpoint.await?;
    Ok(())
}

pub async fn connect_tcp<T>(addr: SocketAddr, service: T) -> io::Result<()>
where
    T: VimService + Unpin,
{
    let socket = TcpStream::connect(&addr).await?;
    let client = Service::new(service);
    let endpoint = Endpoint::new(socket.compat(), client);
    endpoint.await?;
    Ok(())
}

pub async fn listen_unix<T, F>(path: &str, service_maker: F) -> io::Result<()>
where
    F: Fn() -> T + Send + 'static,
    T: VimService + Unpin + Send + Clone + 'static,
{
    let listener = UnixListener::bind(path)?;
    let _ = tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((socket, _)) => {
                    tokio::spawn(Endpoint::new(
                        socket.compat(),
                        Service::new(service_maker()),
                    ));
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
    T: VimService + Unpin + Send + Clone + 'static,
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
