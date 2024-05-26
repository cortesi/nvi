use nvi;

#[derive(Clone)]
struct Client {}

impl nvi::NviService for Client {
    async fn handle_nvim_notification(
        &mut self,
        client: &mut nvi::NviClient,
        method: &str,
        params: &[nvi::Value],
    ) {
        println!("handle_nvim_notification");
    }

    async fn handle_nvim_request(
        &mut self,
        client: &mut nvi::NviClient,
        method: &str,
        params: &[nvi::Value],
    ) -> Result<nvi::Value, nvi::Value> {
        let _ = client.raw_request("foo", &[]).await;
        println!("handle_nvim_request");
        Ok(nvi::Value::Nil)
    }
}

#[tokio::main]
async fn main() {
    nvi::connect_unix("/tmp/sock", Client {}).await.unwrap();
    println!("Hello, world!");
}
