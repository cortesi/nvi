use nvi_macros::*;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct Simple {
    n: Arc<Mutex<usize>>,
}

#[nvi_service]
impl Simple {
    fn new() -> Self {
        Simple {
            n: Arc::new(Mutex::new(0)),
        }
    }

    #[notify]
    async fn increment(&mut self, _client: &mut nvi::Client, inc: usize) {
        let mut n = self.n.lock().unwrap();
        *n += inc;
    }

    #[request]
    async fn retrieve(&mut self, _client: &mut nvi::Client) -> usize {
        *self.n.lock().unwrap()
    }

    async fn run(&self, client: &mut nvi::Client) -> nvi::error::Result<()> {
        client
            .notify(nvi::LogLevel::Info, "simple plugin connected")
            .await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    nvi::run(Simple::new()).await;
}
