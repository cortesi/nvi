use nvi::{error::Result, highlights::*, nvi_macros::*};

#[derive(Default)]
struct Simple {
    n: usize,
}

// The `nvi_plugin` attribute macro on the impl block generates the `NviPlugin` trait. It inspects
// the the block for methods marked as `#[notify]`, `#[request]` or `#[autocmd]` and generates the
// required structure for them to be invoked from the editor. When the service connects to the
// editor, it makes a set of global namespace entries to expose the plugin API under the snake_case
// name of the plugin. In this case, after the editor connects, we can use it from Lua like so:
//
// ```lua
// simple.inc(5)
// print(simple.get())
// ```
#[nvi_plugin]
impl Simple {
    // The `#[notify]` attribute macro marks a method as an RPC notification handler. Notifications
    // are methods that don't provide a response, so they are fire-and-forget as far as the editor
    // is concerned. The first argument must be `&mut nvi::Client`, and all other arguments must be
    // serializable to a MessagePack Value. Notification methods can be void, or return a
    // `Result<()>`.
    #[notify]
    async fn inc(&mut self, _client: &mut nvi::Client, inc: usize) {
        self.n += inc;
    }

    // The `#[request]` attribute macro marks a method as an RPC request handler. Requests are
    // methods that provide a response, which is sent back to the editor. The first argument must
    // be `&mut nvi::Client`, and all other arguments and the response must be serializable to a
    // MessagePack Value. Requets may be void, return `T` or `Result<T>` where T is serializable to
    // a MesagePack value.
    //
    // Notice that this method doesn't have a mutable `self` argument. Methods that don't have a
    // mutable receiver can be called concurrently, while methods that have a mutable receiver are
    // guarded with a write lock, and only one mutable method can run at a time.
    #[request]
    async fn get(&self, _client: &mut nvi::Client) -> usize {
        self.n
    }

    /// The `#[autocmd]` attribute macro marks a method as an autocmd handler. Autocmds are methods
    /// that are called when an event occurs in the editor. The only argument apart from client
    /// must be an `AutocmdEvent`.
    #[autocmd(["BufEnter", "BufLeave"], patterns=["*.rs"], group="test", nested=true)]
    async fn on_buf_enter(
        &mut self,
        client: &mut nvi::Client,
        evt: nvi::AutocmdEvent,
    ) -> Result<()> {
        self.n += 1;
        client.info(&format!("bufenter: {:?}", evt)).await
    }

    // If the impl block has a method called `connected`, it will be called after connection to the
    // editor.
    async fn connected(&mut self, client: &mut nvi::Client) -> Result<()> {
        client.info("simple plugin connected").await
    }

    /// Return a `Highlights` struct that defines the highlight groups for this plugin. We break
    /// from convention somewhat because we use the `snake_case` name of our addon as a prefix. In
    /// this case, we're defining a group called simpleNormal with a red foreground.
    fn highlights(&self) -> nvi::error::Result<Highlights> {
        Ok(Highlights::default().hl("Normal", Hl::default().fg("red")?))
    }
}

#[tokio::main]
async fn main() {
    // Nvi has a built-in command-line interface to invoke plugins. This is not just convenient,
    // but standardized invocation lets us build tooling around plugins.
    nvi::cmd::run(Simple::default(), None).await.unwrap();
}
