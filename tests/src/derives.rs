use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

use nvi::{
    error::Result,
    nvim::{opts, types::Event},
    test, Client, NviPlugin,
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
    /// aaa bbb
    /// ccc
    impl TestPlugin {
        async fn connected(&self, _: &Client) -> Result<()> {
            trace!("connected");
            debug!("docs: {:#?}", self.docs().unwrap());
            self.tx.send(()).unwrap();
            Ok(())
        }
    }

    let (tx, _) = broadcast::channel(16);
    test::run_plugin_with_shutdown(TestPlugin { tx: tx.clone() }, tx)
        .await
        .unwrap();
    assert!(logs_contain("connected"));
    assert!(logs_contain(r"aaa bbb\nccc"));
}

#[tokio::test]
#[traced_test]
async fn it_derives_autocmd_handler() {
    #[derive(Clone)]
    struct TestPlugin {}

    #[nvi_plugin]
    impl TestPlugin {
        #[autocmd(["User"], patterns=["*.rs"])]
        async fn on_user_event(&self, _client: &Client) -> Result<()> {
            debug!("received");
            Ok(())
        }

        async fn connected(&self, _client: &Client) -> Result<()> {
            Ok(())
        }
    }

    let nvit = test::NviTest::builder()
        .show_logs()
        .log_level(tracing::Level::DEBUG)
        .with_plugin(TestPlugin {})
        .run()
        .await
        .unwrap();

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

#[tokio::test]
#[traced_test]
async fn it_derives_combined_handlers() {
    #[derive(Clone)]
    struct TestPlugin {
        count: Arc<AtomicU32>,
    }

    #[nvi_plugin]
    impl TestPlugin {
        #[request]
        async fn increment(&self, _: &Client) -> Result<u32> {
            let prev = self.count.fetch_add(1, Ordering::SeqCst);
            debug!("increment to {}", prev + 1);
            Ok(prev + 1)
        }

        #[notify]
        async fn reset(&self, _: &Client) -> Result<()> {
            self.count.store(0, Ordering::SeqCst);
            debug!("reset counter");
            Ok(())
        }

        #[autocmd(["BufNew"], patterns=["*.txt"])]
        async fn on_new_txt(&self, _: &Client) -> Result<()> {
            let count = self.count.fetch_add(10, Ordering::SeqCst);
            debug!("buffer event: adding 10, was {}", count);
            Ok(())
        }

        async fn connected(&self, _: &Client) -> Result<()> {
            Ok(())
        }
    }

    let nvit = test::NviTest::builder()
        .show_logs()
        .log_level(tracing::Level::DEBUG)
        .with_plugin(TestPlugin {
            count: Arc::new(AtomicU32::new(0)),
        })
        .run()
        .await
        .unwrap();

    nvit.finish().await.unwrap();
}
