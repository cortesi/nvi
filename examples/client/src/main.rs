use async_trait::async_trait;
use tokio::sync::broadcast;

#[derive(Clone)]
struct Client {}

#[async_trait]
impl nvi::NviService for Client {
    async fn notification(
        &mut self,
        _client: &mut nvi::NviClient,
        _method: &str,
        _params: &[nvi::Value],
    ) -> nvi::error::Result<()> {
        println!("handle_nvim_notification");
        Ok(())
    }

    async fn request(
        &mut self,
        client: &mut nvi::NviClient,
        _method: &str,
        _params: &[nvi::Value],
    ) -> Result<nvi::Value, nvi::Value> {
        let _ = client.raw_request("foo", &[]).await;
        println!("handle_nvim_request");
        Ok(nvi::Value::Nil)
    }
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(16);

    nvi::connect_unix(tx, "/tmp/sock", Client {}).await.unwrap();
    println!("Hello, world!");
}
