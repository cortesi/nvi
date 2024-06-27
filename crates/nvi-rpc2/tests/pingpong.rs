use async_trait::async_trait;
use nvi_rpc2::{
    connect_unix, Client, ConnectionHandler, RpcError, RpcServer, RpcService, UnixListener,
};
use rmpv::Value;
use std::sync::Arc;
use tempfile::tempdir;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::Mutex;
use tracing_test::traced_test;

#[derive(Clone)]
struct PingService {
    pong_count: Arc<Mutex<i32>>,
}

#[async_trait]
impl RpcService for PingService {
    async fn handle_request<S>(
        &self,
        client: Client,
        method: &str,
        _params: Vec<Value>,
    ) -> Result<Value, RpcError>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        match method {
            "start_ping" => {
                let client_clone = client.clone();
                tokio::spawn(async move {
                    if let Err(e) = client_clone
                        .send_request("pong".to_string(), vec![Value::String("ping".into())])
                        .await
                    {
                        eprintln!("Error sending ping: {}", e);
                    }
                });
                Ok(Value::Boolean(true))
            }
            "get_pong_count" => {
                let count = *self.pong_count.lock().await;
                Ok(Value::Integer(count.into()))
            }
            _ => Err(RpcError::Protocol(format!(
                "PingService: Unknown method: {}",
                method
            ))),
        }
    }

    async fn handle_notification<S>(&self, _client: Client, method: &str, _params: Vec<Value>)
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        if method == "pong" {
            let mut count = self.pong_count.lock().await;
            *count += 1;
        }
    }
}

#[derive(Clone)]
struct PongService;

#[async_trait]
impl RpcService for PongService {
    async fn handle_request<S>(
        &self,
        client: Client,
        method: &str,
        _params: Vec<Value>,
    ) -> Result<Value, RpcError>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        match method {
            "pong" => {
                client
                    .send_notification("pong".to_string(), vec![Value::String("pong".into())])
                    .await?;
                Ok(Value::Boolean(true))
            }
            _ => Err(RpcError::Protocol(format!(
                "PongService: Unknown method: {}",
                method
            ))),
        }
    }

    async fn handle_notification<S>(&self, _client: Client, _method: &str, _params: Vec<Value>)
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        // PongService doesn't handle any notifications
    }
}

#[traced_test]
#[tokio::test]
async fn test_pingpong() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let ping_socket = temp_dir.path().join("ping.sock");
    let pong_socket = temp_dir.path().join("pong.sock");

    // Set up the PingService
    let ping_service = PingService {
        pong_count: Arc::new(Mutex::new(0)),
    };
    let ping_server = RpcServer::new(ping_service.clone());
    let ping_listener = UnixListener::bind(&ping_socket).await?;

    // Set up the PongService
    let pong_service = PongService;
    let pong_server = RpcServer::new(pong_service);
    let pong_listener = UnixListener::bind(&pong_socket).await?;

    // Start the servers
    let ping_server_task = tokio::spawn(async move {
        if let Err(e) = ping_server.run_unix(ping_listener).await {
            eprintln!("Ping server error: {}", e);
        }
    });

    let pong_server_task = tokio::spawn(async move {
        if let Err(e) = pong_server.run_unix(pong_listener).await {
            eprintln!("Pong server error: {}", e);
        }
    });

    // Give the servers some time to start
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Connect to the ping server
    let ping_connection = connect_unix(&ping_socket).await?;
    let mut ping_handler = ConnectionHandler::new(ping_connection, Arc::new(ping_service.clone()));
    let ping_client = ping_handler.client().clone();

    // Connect to the pong server
    let pong_connection = connect_unix(&pong_socket).await?;
    let mut pong_handler = ConnectionHandler::new(pong_connection, Arc::new(PongService));

    // Spawn the handlers
    let ping_handler_task = tokio::spawn(async move {
        if let Err(e) = ping_handler.run().await {
            eprintln!("Ping handler error: {}", e);
        }
    });

    let pong_handler_task = tokio::spawn(async move {
        if let Err(e) = pong_handler.run().await {
            eprintln!("Pong handler error: {}", e);
        }
    });

    // Start the ping-pong process
    let num_pings = 5;
    for _ in 0..num_pings {
        ping_client
            .send_request("start_ping".to_string(), vec![])
            .await?;
        // Give some time for the ping-pong to complete
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    // Give some extra time for all pongs to be processed
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Check the pong count
    let pong_count = ping_client
        .send_request("get_pong_count".to_string(), vec![])
        .await?;
    assert_eq!(pong_count, Value::Integer(num_pings.into()));

    // Clean up
    ping_handler_task.abort();
    pong_handler_task.abort();
    ping_server_task.abort();
    pong_server_task.abort();

    Ok(())
}
