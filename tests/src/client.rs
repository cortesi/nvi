use nvi::test;
use nvi::types::{AutocmdEvent, Event, ExecAutocmdsOpts};

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
            c.autocmd("aucmd", &[Event::User], &[], None, false, false)
                .await?;
            c.nvim
                .nvim_exec_autocmds(&["User".to_string()], ExecAutocmdsOpts::default())
                .await?;
            Ok(())
        }
    }

    trace!("starting test");
    let (tx, _) = broadcast::channel(16);
    test::test_service(T {}, tx).await.unwrap();

    assert!(logs_contain("aucmd received"));
}
