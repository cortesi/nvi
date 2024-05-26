use nvi;

#[derive(Clone)]
struct Client {}

impl nvi::VimService for Client {
    async fn handle_nvim_notification(
        &mut self,
        client: &mut nvi::Client,
        method: &str,
        params: &[nvi::Value],
    ) {
        println!("handle_nvim_notification");
    }

    async fn handle_nvim_request(
        &mut self,
        client: &mut nvi::Client,
        method: &str,
        params: &[nvi::Value],
    ) -> Result<nvi::Value, nvi::Value> {
        println!("handle_nvim_request");
        Ok(nvi::Value::Nil)
    }
}

#[tokio::main]
async fn main() {
    nvi::listen_tcp("127.0.0.1:54321".parse().unwrap(), || Client {})
        .await
        .unwrap();
    println!("Hello, world!");
}
