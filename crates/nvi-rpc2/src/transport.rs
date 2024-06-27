use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use rmpv::Value;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{
    TcpListener as TokioTcpListener, TcpStream, UnixListener as TokioUnixListener, UnixStream,
};
use tokio::sync::mpsc;
use tracing::trace;

use crate::error::*;
use crate::{ConnectionHandler, RpcConnection, RpcSender, RpcService};

struct TcpListener {
    inner: TokioTcpListener,
}

impl TcpListener {
    pub async fn bind(addr: &str) -> Result<Self> {
        trace!("Binding TCP listener to address: {}", addr);
        let listener = TokioTcpListener::bind(addr).await?;
        Ok(Self { inner: listener })
    }

    pub async fn accept(&self) -> Result<RpcConnection<TcpStream>> {
        let (stream, addr) = self.inner.accept().await?;
        trace!("Accepted TCP connection from: {}", addr);
        Ok(RpcConnection::new(stream))
    }
}

struct UnixListener {
    inner: TokioUnixListener,
}

impl UnixListener {
    pub async fn bind<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_string_lossy();
        trace!("Binding Unix listener to path: {}", path_str);
        let listener = TokioUnixListener::bind(path)?;
        Ok(Self { inner: listener })
    }

    pub async fn accept(&self) -> Result<RpcConnection<UnixStream>> {
        let (stream, _) = self.inner.accept().await?;
        trace!("Accepted Unix connection");
        Ok(RpcConnection::new(stream))
    }
}

#[async_trait]
pub trait Accept {
    type Stream: AsyncRead + AsyncWrite + Unpin;
    async fn accept(&self) -> Result<RpcConnection<Self::Stream>>;
}

#[async_trait]
impl Accept for TcpListener {
    type Stream = TcpStream;
    async fn accept(&self) -> Result<RpcConnection<Self::Stream>> {
        self.accept().await
    }
}

#[async_trait]
impl Accept for UnixListener {
    type Stream = UnixStream;
    async fn accept(&self) -> Result<RpcConnection<Self::Stream>> {
        self.accept().await
    }
}

pub async fn connect_tcp(addr: &str) -> Result<RpcConnection<TcpStream>> {
    let stream = TcpStream::connect(addr).await?;
    trace!("TCP connection established to: {}", addr);
    Ok(RpcConnection::new(stream))
}

pub async fn connect_unix<P: AsRef<Path>>(path: P) -> Result<RpcConnection<UnixStream>> {
    let path_str = path.as_ref().to_string_lossy().to_string();
    let stream = UnixStream::connect(path).await?;
    trace!("Unix connection established to: {:?}", path_str);
    Ok(RpcConnection::new(stream))
}

pub struct Server<T: RpcService> {
    service: Arc<T>,
    listener: Option<Listener>,
}

enum Listener {
    Tcp(TcpListener),
    Unix(UnixListener),
}

impl<T: RpcService> Server<T> {
    pub fn new(service: T) -> Self {
        Self {
            service: Arc::new(service),
            listener: None,
        }
    }

    pub async fn tcp(mut self, addr: &str) -> Result<Self> {
        self.listener = Some(Listener::Tcp(TcpListener::bind(addr).await?));
        Ok(self)
    }

    pub async fn unix<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        self.listener = Some(Listener::Unix(UnixListener::bind(path).await?));
        Ok(self)
    }

    pub async fn run(self) -> Result<()> {
        let listener = self
            .listener
            .ok_or_else(|| RpcError::Protocol("No listener configured".into()))?;
        match listener {
            Listener::Tcp(tcp_listener) => Self::run_internal(self.service, tcp_listener).await,
            Listener::Unix(unix_listener) => Self::run_internal(self.service, unix_listener).await,
        }
    }

    async fn run_internal<L>(service: Arc<T>, listener: L) -> Result<()>
    where
        L: Accept,
        L::Stream: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        loop {
            let connection = RpcConnection::new(listener.accept().await?);
            let service_clone = service.clone();

            tokio::spawn(async move {
                let (sender, receiver) = mpsc::channel(100);
                let mut handler =
                    ConnectionHandler::new(connection, service_clone, receiver, sender.clone());

                if let Err(e) = handler.run().await {
                    tracing::error!("Connection error: {}", e);
                }
            });
        }
    }
}

pub struct Client<T: RpcService> {
    sender: RpcSender,
    _handler: tokio::task::JoinHandle<()>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: RpcService> Client<T> {
    pub async fn connect_unix<P: AsRef<Path>>(path: P, service: T) -> Result<Self> {
        Self::new(connect_unix(path).await?, service).await
    }

    pub async fn connect_tcp(addr: &str, service: T) -> Result<Self> {
        Self::new(connect_tcp(addr).await?, service).await
    }

    async fn new<S>(connection: RpcConnection<S>, service: T) -> Result<Self>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        let (sender, receiver) = mpsc::channel(100);
        let rpc_sender = RpcSender {
            sender: sender.clone(),
        };
        let mut handler = ConnectionHandler::new(connection, Arc::new(service), receiver, sender);

        let handler_task = tokio::spawn(async move {
            if let Err(e) = handler.run().await {
                tracing::error!("Handler error: {}", e);
            }
        });

        Ok(Self {
            sender: rpc_sender,
            _handler: handler_task,
            _phantom: std::marker::PhantomData,
        })
    }

    pub async fn send_request(&self, method: String, params: Vec<Value>) -> Result<Value> {
        self.sender.send_request(method, params).await
    }

    pub async fn send_notification(&self, method: String, params: Vec<Value>) -> Result<()> {
        self.sender.send_notification(method, params).await
    }
}
