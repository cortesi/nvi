use std::{net::SocketAddr, path::Path};
use tokio::sync::broadcast;
use tracing::{error, trace};

use crate::error::Result;
use crate::service::{NviService, ServiceWrapper};
use mrpc::{Client, Server};

pub async fn listen_unix<T>(
    shutdown_tx: broadcast::Sender<()>,
    path: impl AsRef<Path>,
    service: T,
) -> Result<()>
where
    T: NviService + Clone + Send + Sync + 'static,
{
    let path = path.as_ref();
    let wrapped_service = ServiceWrapper::new(service, shutdown_tx.clone());
    let server = Server::new(wrapped_service).unix(path).await?;

    let mut shutdown_rx = shutdown_tx.subscribe();

    tokio::select! {
        result = server.run() => {
            if let Err(e) = result {
                error!("Server error: {}", e);
            }
        }
        _ = shutdown_rx.recv() => {
            trace!("Shutdown signal received, stopping listener.");
        }
    }

    let _ = std::fs::remove_file(path);
    Ok(())
}

pub async fn listen_tcp<T>(
    shutdown_tx: broadcast::Sender<()>,
    addr: SocketAddr,
    service: T,
) -> Result<()>
where
    T: NviService + Clone + Send + Sync + 'static,
{
    let wrapped_service = ServiceWrapper::new(service, shutdown_tx.clone());
    let server = Server::new(wrapped_service).tcp(&addr.to_string()).await?;

    let mut shutdown_rx = shutdown_tx.subscribe();

    tokio::select! {
        result = server.run() => {
            if let Err(e) = result {
                error!("Server error: {}", e);
            }
        }
        _ = shutdown_rx.recv() => {
            trace!("Shutdown signal received, stopping listener.");
        }
    }

    Ok(())
}

pub async fn connect_unix<T, P>(
    shutdown_tx: broadcast::Sender<()>,
    path: P,
    service: T,
) -> Result<()>
where
    P: AsRef<Path>,
    T: NviService + Clone + Send + Sync + 'static,
{
    let wrapped_service = ServiceWrapper::new(service, shutdown_tx.clone());
    let client = Client::connect_unix(path, wrapped_service).await?;
    handle_client(shutdown_tx, client).await
}

pub async fn connect_tcp<T>(
    shutdown_tx: broadcast::Sender<()>,
    addr: SocketAddr,
    service: T,
) -> Result<()>
where
    T: NviService + Clone + Send + Sync + 'static,
{
    let wrapped_service = ServiceWrapper::new(service, shutdown_tx.clone());
    let client = Client::connect_tcp(&addr.to_string(), wrapped_service).await?;
    handle_client(shutdown_tx, client).await
}

async fn handle_client<T: mrpc::RpcService + Clone + Send + Sync + 'static>(
    shutdown_tx: broadcast::Sender<()>,
    _client: Client<T>,
) -> Result<()> {
    let mut shutdown_rx = shutdown_tx.subscribe();

    tokio::select! {
        _ = shutdown_rx.recv() => {
            trace!("Shutdown signal received, closing connection.");
            Ok(())
        }
        _ = tokio::signal::ctrl_c() => {
            trace!("Ctrl-C received, closing connection.");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::Client as NviClient;
    use crate::Value;
    use std::sync::Arc;
    use tokio::sync::{broadcast, oneshot, Mutex};
    use tracing_test::traced_test;

    #[derive(Clone)]
    struct TestService {
        on_connect: Arc<Mutex<Option<oneshot::Sender<()>>>>,
    }

    impl TestService {
        fn new(sender: oneshot::Sender<()>) -> Self {
            Self {
                on_connect: Arc::new(Mutex::new(Some(sender))),
            }
        }
    }

    #[async_trait::async_trait]
    impl NviService for TestService {
        fn name(&self) -> String {
            "TestService".to_string()
        }

        async fn bootstrap(&mut self, _client: &mut NviClient) -> Result<()> {
            if let Some(sender) = self.on_connect.lock().await.take() {
                let _ = sender.send(());
            }
            Ok(())
        }

        async fn request(
            &mut self,
            _client: &mut NviClient,
            method: &str,
            params: &[Value],
        ) -> std::result::Result<Value, Value> {
            if method == "echo" {
                Ok(params.first().cloned().unwrap_or(Value::Nil))
            } else {
                Err(Value::String(format!("Unknown method: {}", method).into()))
            }
        }
    }

    #[tokio::test]
    #[traced_test]
    async fn it_listens() {
        let (shutdown_tx, _) = broadcast::channel(16);

        let tempdir = tempfile::tempdir().unwrap();
        let socket_path = tempdir.path().join("listen.socket");

        let (on_connect_tx, on_connect_rx) = oneshot::channel();
        let service = TestService::new(on_connect_tx);

        let listener = listen_unix(shutdown_tx.clone(), socket_path.clone(), service);
        let ls = tokio::spawn(listener);

        tokio::time::timeout(std::time::Duration::from_secs(5), async {
            while !socket_path.exists() {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        })
        .await
        .expect("Timeout waiting for socket to be created");

        let (client_connect_tx, _) = oneshot::channel();
        let client_service = TestService::new(client_connect_tx);
        let client = connect_unix(shutdown_tx.clone(), socket_path.clone(), client_service);
        let client_handle = tokio::spawn(client);

        on_connect_rx
            .await
            .expect("Failed to receive connection notification");

        shutdown_tx.send(()).unwrap();

        ls.await.unwrap().unwrap();
        client_handle.await.unwrap().unwrap();

        assert!(!socket_path.exists());
    }

    #[tokio::test]
    #[traced_test]
    async fn it_connects() {
        let (shutdown_tx, _) = broadcast::channel(16);
        let (server_ready_tx, server_ready_rx) = oneshot::channel();
        let (client_connected_tx, client_connected_rx) = oneshot::channel();

        let addr = "127.0.0.1:0".parse::<SocketAddr>().unwrap();
        let stx = shutdown_tx.clone();
        let server = tokio::spawn(async move {
            let service = TestService::new(client_connected_tx);
            let server_future = listen_tcp(stx, addr, service);
            let local_addr = addr;
            server_ready_tx.send(local_addr).unwrap();
            server_future.await.unwrap();
        });

        let server_addr = server_ready_rx.await.unwrap();

        let (on_connect_tx, on_connect_rx) = oneshot::channel();
        let client_service = TestService::new(on_connect_tx);
        let client = connect_tcp(shutdown_tx.clone(), server_addr, client_service);
        let client_handle = tokio::spawn(client);

        // Wait for both the server and client to establish the connection
        tokio::select! {
            _ = client_connected_rx => {
                trace!("Server received client connection");
            }
            _ = on_connect_rx => {
                trace!("Client connected to server");
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
                panic!("Timeout waiting for connection");
            }
        }

        // Shutdown everything
        shutdown_tx.send(()).unwrap();

        // Wait for client and server to shut down
        client_handle.await.unwrap().unwrap();
        server.await.unwrap();
    }
}
