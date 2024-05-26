use nvi;

#[derive(Clone)]
struct Client {}

impl nvi::VimService for Client {
    fn handle_nvim_notification(
        &mut self,
        client: &mut nvi::Client,
        method: &str,
        params: &[nvi::Value],
    ) {
        println!("handle_nvim_notification");
    }

    fn handle_nvim_request(
        &mut self,
        client: &mut nvi::Client,
        method: &str,
        params: &[nvi::Value],
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<nvi::Value, nvi::Value>> + Send>>
    {
        println!("handle_nvim_request");
        Box::pin(async { Ok(nvi::Value::Nil) })
    }
}

#[tokio::main]
async fn main() {
    nvi::listen_tcp("127.0.0.1:54321".parse().unwrap(), || Client {})
        .await
        .unwrap();
    println!("Hello, world!");
}
