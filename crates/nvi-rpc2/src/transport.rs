use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{
    TcpListener as TokioTcpListener, TcpStream, UnixListener as TokioUnixListener, UnixStream,
};

use crate::error::*;
use crate::{ConnectionHandler, RpcConnection, RpcService};

pub struct TcpListener {
    inner: TokioTcpListener,
}

impl TcpListener {
    pub async fn bind(addr: &str) -> Result<Self> {
        let listener = TokioTcpListener::bind(addr).await?;
        Ok(Self { inner: listener })
    }

    pub async fn accept(&self) -> Result<RpcConnection<TcpStream>> {
        let (stream, _) = self.inner.accept().await?;
        Ok(RpcConnection::new(stream))
    }
}

pub struct UnixListener {
    inner: TokioUnixListener,
}

impl UnixListener {
    pub async fn bind<P: AsRef<Path>>(path: P) -> Result<Self> {
        let listener = TokioUnixListener::bind(path)?;
        Ok(Self { inner: listener })
    }

    pub async fn accept(&self) -> Result<RpcConnection<UnixStream>> {
        let (stream, _) = self.inner.accept().await?;
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
    Ok(RpcConnection::new(stream))
}

pub async fn connect_unix<P: AsRef<Path>>(path: P) -> Result<RpcConnection<UnixStream>> {
    let stream = UnixStream::connect(path).await?;
    Ok(RpcConnection::new(stream))
}

pub struct RpcServer<T: RpcService> {
    service: Arc<T>,
}

impl<T: RpcService> RpcServer<T> {
    pub fn new(service: T) -> Self {
        Self {
            service: Arc::new(service),
        }
    }

    pub async fn run_tcp(&self, listener: TcpListener) -> Result<()> {
        self.run_internal(listener).await
    }

    pub async fn run_unix(&self, listener: UnixListener) -> Result<()> {
        self.run_internal(listener).await
    }

    async fn run_internal<L, S>(&self, listener: L) -> Result<()>
    where
        L: Accept<Stream = S>,
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        loop {
            let connection = RpcConnection::new(listener.accept().await?);
            let service = self.service.as_ref().clone();
            let mut handler = ConnectionHandler::new(connection, Arc::new(service));
            tokio::spawn(async move {
                if let Err(e) = handler.run().await {
                    tracing::error!("Connection error: {}", e);
                }
            });
        }
    }
}
