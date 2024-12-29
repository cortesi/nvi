use nvi::{
    nvim::{
        opts,
        types::{AutocmdEvent, Event},
    },
    test,
};

use nvi_macros::{nvi_service, request};

use tokio::sync::broadcast;
use tracing::trace;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn it_registers_pattern_autocmds() {
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

        async fn connected(&self, c: &mut nvi::Client) -> nvi::error::Result<()> {
            c.nvim
                .clear_autocmds(opts::ClearAutocmds::default())
                .await?;
            let id = c
                .autocmd_pattern(&[], "aucmd", &[Event::User], None, false, false)
                .await?;
            trace!("autocmd id: {:?}", id);
            c.nvim
                .exec_autocmds(&[Event::User], opts::ExecAutocmds::default())
                .await?;
            Ok(())
        }
    }
    trace!("starting test");
    let (tx, _) = broadcast::channel(16);
    test::run_plugin_with_shutdown(T {}, tx).await.unwrap();
    assert!(logs_contain("aucmd received"));
}

#[tokio::test]
#[traced_test]
async fn it_registers_buffer_autocmds() {
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

        async fn connected(&self, c: &mut nvi::Client) -> nvi::error::Result<()> {
            c.nvim
                .clear_autocmds(opts::ClearAutocmds::default())
                .await?;
            let id = c
                .autocmd_buffer(0.into(), "aucmd", &[Event::User], None, false, false)
                .await?;
            trace!("autocmd id: {:?}", id);
            c.nvim
                .exec_autocmds(&[Event::User], opts::ExecAutocmds::default())
                .await?;
            Ok(())
        }
    }
    trace!("starting test");
    let (tx, _) = broadcast::channel(16);
    test::run_plugin_with_shutdown(T {}, tx).await.unwrap();
    assert!(logs_contain("aucmd received"));
}

#[tokio::test]
async fn api_nvim_get_chan_info() {
    #[derive(Clone)]
    struct T {}

    #[nvi_service]
    impl T {
        async fn connected(&self, c: &mut nvi::Client) -> nvi::error::Result<()> {
            let chan = c.nvim.get_chan_info(0).await?;
            assert!(chan.id > 0);
            c.shutdown();
            Ok(())
        }
    }
    trace!("starting test");
    let (tx, _) = broadcast::channel(16);
    test::run_plugin_with_shutdown(T {}, tx).await.unwrap();
}
