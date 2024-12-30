use nvi::{
    nvim::{opts, types::Event},
    test,
};

use nvi_macros::*;

use tokio::sync::broadcast;
use tracing::{debug, trace};
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn it_derives_basic_service() {
    #[derive(Clone)]
    struct TestPlugin {
        tx: broadcast::Sender<()>,
    }

    #[nvi_plugin]
    impl TestPlugin {
        async fn connected(&mut self, _: &mut nvi::Client) -> nvi::error::Result<()> {
            trace!("connected");
            self.tx.send(()).unwrap();
            Ok(())
        }
    }

    let (tx, _) = broadcast::channel(16);
    test::run_plugin_with_shutdown(TestPlugin { tx: tx.clone() }, tx)
        .await
        .unwrap();
    assert!(logs_contain("connected"));
}

#[tokio::test]
#[traced_test]
async fn it_derives_autocmd_handler() {
    #[derive(Clone)]
    struct TestPlugin {}

    #[nvi_plugin]
    impl TestPlugin {
        #[autocmd(["User"], patterns=["*.rs"])]
        async fn on_user_event(&self, _client: &mut nvi::Client) -> nvi::error::Result<()> {
            debug!("received");
            Ok(())
        }

        async fn connected(&self, _client: &mut nvi::Client) -> nvi::error::Result<()> {
            debug!("started");
            Ok(())
        }
    }

    let nvit = test::NviTest::builder()
        .show_logs()
        .log_level(tracing::Level::DEBUG)
        .run(TestPlugin {})
        .await
        .unwrap();

    nvit.await_log("started").await.unwrap();

    nvit.client
        .nvim
        .exec_autocmds(
            &[Event::User],
            opts::ExecAutocmds::default().pattern(vec!["*.rs".into()]),
        )
        .await
        .unwrap();

    nvit.assert_log("received");
    nvit.finish().await.unwrap();
}
