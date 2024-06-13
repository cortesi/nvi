use nvi::{
    opts, test,
    types::{AutocmdEvent, Event},
};

use nvi_macros::{nvi_service, request};
use tokio::sync::broadcast;
use tracing::trace;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn it_registers_autocmds() {
    #[derive(Clone)]
    struct T {}

    #[nvi_service]
    impl T {
        #[request]
        async fn aucmd(&self, c: &mut nvi::Client, evt: AutocmdEvent) -> nvi::error::Result<bool> {
            trace!("aucmd received");
            trace!("aucmd event: {:?}", evt);
            c.shutdown();
            Ok(false)
        }

        async fn run(&self, c: &mut nvi::Client) -> nvi::error::Result<()> {
            let id = c
                .autocmd("aucmd", &[Event::User], &[], None, false, false)
                .await?;
            trace!("autocmd id: {:?}", id);
            c.nvim
                .exec_autocmds(&[Event::User], opts::ExecAutocmdsOpts::default())
                .await?;
            Ok(())
        }
    }
    trace!("starting test");
    let (tx, _) = broadcast::channel(16);
    test::test_service(T {}, tx).await.unwrap();
    assert!(logs_contain("aucmd received"));
}

#[tokio::test]
#[traced_test]
async fn api_nvim_get_chan_info() {
    #[derive(Clone)]
    struct T {}

    #[nvi_service]
    impl T {
        async fn run(&self, c: &mut nvi::Client) -> nvi::error::Result<()> {
            let chan = c.nvim.get_chan_info(0).await?;
            assert!(chan.id > 0);
            c.shutdown();
            Ok(())
        }
    }
    trace!("starting test");
    let (tx, _) = broadcast::channel(16);
    test::test_service(T {}, tx).await.unwrap();
}
