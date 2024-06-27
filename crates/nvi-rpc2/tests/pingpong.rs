use async_trait::async_trait;
use nvi_rpc2::{connect_unix, Client, ConnectionHandler, RpcError, RpcService, Server};
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
        _client: Client,
        method: &str,
        _params: Vec<Value>,
    ) -> Result<Value, RpcError>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    {
        Err(RpcError::Protocol(format!(
            "PingService: Unknown method: {}",
            method
        )))
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
            "ping" => {
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
    let socket_path = temp_dir.path().join("pong.sock");

    // Set up the Pong server
    let server = Server::new(PongService).unix(&socket_path).await?;
    let pong_server_task = tokio::spawn(async move {
        let e = server.run().await;
        if let Err(e) = e {
            eprintln!("Server error: {}", e);
        }
    });

    // Set up the Ping client
    let pong_count = Arc::new(Mutex::new(0));
    let mut handler = ConnectionHandler::new(
        connect_unix(&socket_path).await?,
        Arc::new(PingService {
            pong_count: pong_count.clone(),
        }),
    );
    let client = handler.client();
    let handler_task = tokio::spawn(async move {
        if let Err(e) = handler.run().await {
            eprintln!("Ping handler error: {}", e);
        }
    });

    // Start the ping-pong process
    let num_pings = 5;
    for _ in 0..num_pings {
        client.send_request("ping".to_string(), vec![]).await?;
    }

    assert_eq!(*pong_count.lock().await, num_pings);

    handler_task.abort();
    pong_server_task.abort();
    Ok(())
}
