use nvi_macros::*;
use std::sync::{Arc, Mutex};

// The only requirement for an Nvi service is that it be `Clone`, so that we can share it between
// inbound connections (in server mode) and async tasks (everywhere else).
#[derive(Clone)]
struct Simple {
    n: Arc<Mutex<usize>>,
}

// The `nvi_service` attribute macro on the impl block generates the `NviService` trait. It
// inspects the the block for methods marked as `#[notify]` or `#[request]` and generates the
// required structure for them to be invoked from the editor. When the service connects to the
// editor, it makes a set of global namespace entries to expose the plugin API. In this case, after
// the editor connects, we can use it from Lua like so:
//
// ```lua
// Simple.increment(5)
// print(Simple.retrieve())
// ```
#[nvi_service]
impl Simple {
    fn new() -> Self {
        Simple {
            n: Arc::new(Mutex::new(0)),
        }
    }

    // The `#[notify]` attribute macro marks a method as an RPC notification handler. Notifications
    // are methods that don't provide a response, so they are fire-and-forget as far as the editor
    // is concerned. The first argument must be `&mut nvi::Client`, and all other arguments must be
    // serializable to a MessagePack Value. Notification methods can be void, or return a
    // `Result<()>`.
    #[notify]
    async fn increment(&mut self, _client: &mut nvi::Client, inc: usize) {
        let mut n = self.n.lock().unwrap();
        *n += inc;
    }

    // The `#[request]` attribute macro marks a method as an RPC request handler. Requests are
    // methods that provide a response, which is sent back to the editor. The first argument must
    // be `&mut nvi::Client`, and all other arguments and the response must be serializable to a
    // MessagePack Value. Requets may be void, return `T` or `Result<T>` where T is serializable to
    // a MesagePack value.
    #[request]
    async fn retrieve(&mut self, _client: &mut nvi::Client) -> usize {
        *self.n.lock().unwrap()
    }

    // If the impl block has a method called `run`, it will be called after connection to the
    // editor.
    async fn run(&self, client: &mut nvi::Client) -> nvi::error::Result<()> {
        client.info("simple plugin connected").await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // Nvi provides a standard way to invoke plugins. This is not just convenient, but lets us
    // standardize tooling so we can build tooling around plugins.
    nvi::run(Simple::new()).await;
}
