use nvi_rpc2::{Client, Result, RpcService};
use rmpv::Value;
use std::error::Error;

// We need to define a dummy service even for a client
#[derive(Clone)]
struct DummyClientService;

#[async_trait::async_trait]
impl RpcService for DummyClientService {
    async fn handle_request<S>(
        &self,
        _: nvi_rpc2::RpcSender,
        _method: &str,
        _params: Vec<Value>,
    ) -> Result<Value> {
        // This won't be used for our simple client example
        Ok(Value::Nil)
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    // Connect to the server
    let client = Client::connect_unix("/tmp/example_socket", DummyClientService).await?;

    // Send a message
    let result = client
        .send_request(
            "echo".to_string(),
            vec![Value::String("Hello, RPC Server!".into())],
        )
        .await?;

    println!("Received response: {:?}", result);

    Ok(())
}
