use nvi::{
    nvim::{
        opts,
        types::{AutocmdEvent, Event},
    },
    test,
};

use nvi_macros::{nvi_plugin, request};

use tokio::sync::broadcast;
use tracing::debug;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn it_registers_buffer_autocmds() {
    #[derive(Clone)]
    struct T {}

    #[nvi_plugin]
    impl T {
        #[request]
        async fn aucmd(&self, _: &mut nvi::Client, _: AutocmdEvent) -> nvi::error::Result<bool> {
            debug!("aucmd received");
            Ok(false)
        }

        async fn connected(&self, c: &mut nvi::Client) -> nvi::error::Result<()> {
            c.nvim
                .clear_autocmds(opts::ClearAutocmds::default())
                .await?;
            c.autocmd_buffer(0.into(), "aucmd", &[Event::User], None, false, false)
                .await?;
            Ok(())
        }
    }

    let nvit = test::NviTest::builder()
        .show_logs()
        .log_level(tracing::Level::DEBUG)
        .with_plugin(T {})
        .run()
        .await
        .unwrap();

    nvit.client
        .nvim
        .exec_autocmds(&[Event::User], opts::ExecAutocmds::default())
        .await
        .unwrap();

    nvit.assert_log("aucmd received");
    nvit.finish().await.unwrap();
}

#[tokio::test]
async fn api_nvim_get_chan_info() {
    #[derive(Clone)]
    struct T {}

    #[nvi_plugin]
    impl T {
        async fn connected(&self, c: &mut nvi::Client) -> nvi::error::Result<()> {
            let chan = c.nvim.get_chan_info(0).await?;
            assert!(chan.id > 0);
            c.shutdown();
            Ok(())
        }
    }
    debug!("starting test");
    let (tx, _) = broadcast::channel(16);
    test::run_plugin_with_shutdown(T {}, tx).await.unwrap();
}
