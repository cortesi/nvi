use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{
    TcpListener as TokioTcpListener, TcpStream, UnixListener as TokioUnixListener, UnixStream,
};
use tracing::trace;

use crate::error::*;
use crate::{ConnectionHandler, RpcConnection, RpcService};

pub struct TcpListener {
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

pub struct UnixListener {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{ConnectionHandler, RpcService};

    use std::sync::Arc;

    use async_trait::async_trait;
    use rmpv::Value;
    use tempfile::tempdir;
    use tracing_test::traced_test;

    #[derive(Clone)]
    struct MockService;

    #[async_trait]
    impl RpcService for MockService {
        async fn handle_request<S>(
            &self,
            _client: crate::Client,
            _method: &str,
            _params: Vec<Value>,
        ) -> Result<Value>
        where
            S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
        {
            Ok(Value::Nil)
        }

        async fn handle_notification<S>(
            &self,
            _client: crate::Client,
            _method: &str,
            _params: Vec<Value>,
        ) where
            S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
        {
        }
    }

    #[traced_test]
    #[tokio::test]
    async fn test_unix_socket_connection() -> Result<()> {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let socket_path = temp_dir.path().join("test.sock");

        let service = MockService;
        let server = RpcServer::new(service);

        let listener = UnixListener::bind(&socket_path).await?;

        let server_task = tokio::spawn(async move {
            if let Err(e) = server.run_unix(listener).await {
                eprintln!("Server error: {}", e);
            }
        });

        // Give the server some time to start
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        // Connect to the server
        let connection = connect_unix(&socket_path).await?;

        // Create a client from the connection
        let handler = ConnectionHandler::new(connection, Arc::new(MockService));

        // Disconnect without sending any messages
        drop(handler);

        // Stop the server
        server_task.abort();

        Ok(())
    }
}
