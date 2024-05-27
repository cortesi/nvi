use std::{io, net::SocketAddr, pin::Pin};

use futures::Future;
use msgpack_rpc::{Endpoint, ServiceWithClient, Value};
use tokio::{
    join,
    net::{TcpListener, TcpStream, UnixListener, UnixStream},
    runtime::Handle,
};
use tokio_util::compat::TokioAsyncReadCompatExt;
use tracing::{debug, error, trace};

use crate::{api::Api, client::NviClient};

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
    ) -> impl std::future::Future<Output = ()> + Send;

    fn handle_nvim_request(
        &mut self,
        client: &mut NviClient,
        method: &str,
        params: &[Value],
    ) -> impl std::future::Future<Output = Result<Value, Value>> + Send;
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
        let client = client.clone();
        let method = method.to_string();
        let params = params.to_vec();
        handle.spawn(async move {
            vimservice
                .handle_nvim_notification(
                    &mut NviClient {
                        m_client: client.clone(),
                    },
                    &method,
                    &params,
                )
                .await;
        });
    }
}

pub async fn connect_unix<T>(path: &str, service: T) -> io::Result<()>
where
    T: NviService + Unpin + 'static,
{
    let stream = UnixStream::connect(path).await?;
    let service = Service::new(service);
    let endpoint = Endpoint::new(stream.compat(), service);
    let c = endpoint.client();
    let req = c.request("nvim_command", &[Value::String("echo \"hello\"".into())]);
    let (e1, e2) = join!(req, endpoint);
    e1.unwrap();
    e2?;
    Ok(())
}

pub async fn connect_tcp<T>(addr: SocketAddr, service: T) -> io::Result<()>
where
    T: NviService + Unpin + 'static,
{
    let stream = TcpStream::connect(&addr).await?;
    let service = Service::new(service);
    let endpoint = Endpoint::new(stream.compat(), service);
    endpoint.await?;
    Ok(())
}

pub async fn listen_unix<T, F>(path: &str, service_maker: F) -> io::Result<()>
where
    F: Fn() -> T + Send + 'static,
    T: NviService + Unpin + Send + Clone + 'static,
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
