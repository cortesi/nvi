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

#[tokio::test]
#[traced_test]
async fn it_derives_combined_handlers() {
    #[derive(Clone)]
    struct TestPlugin {
        count: std::sync::Arc<std::sync::atomic::AtomicU32>,
    }

    #[nvi_plugin]
    impl TestPlugin {
        #[request]
        async fn increment(&mut self, _: &mut nvi::Client) -> nvi::error::Result<u32> {
            let prev = self.count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            debug!("increment to {}", prev + 1);
            Ok(prev + 1)
        }

        #[notify]
        async fn reset(&mut self, _: &mut nvi::Client) -> nvi::error::Result<()> {
            self.count.store(0, std::sync::atomic::Ordering::SeqCst);
            debug!("reset counter");
            Ok(())
        }

        #[autocmd(["BufNew"], patterns=["*.txt"])]
        async fn on_new_txt(&mut self, _: &mut nvi::Client) -> nvi::error::Result<()> {
            let count = self
                .count
                .fetch_add(10, std::sync::atomic::Ordering::SeqCst);
            debug!("buffer event: adding 10, was {}", count);
            Ok(())
        }

        async fn connected(&self, _: &mut nvi::Client) -> nvi::error::Result<()> {
            debug!("connected");
            Ok(())
        }
    }

    let nvit = test::NviTest::builder()
        .show_logs()
        .log_level(tracing::Level::DEBUG)
        .run(TestPlugin {
            count: std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0)),
        })
        .await
        .unwrap();

    nvit.await_log("connected").await.unwrap();

    nvit.finish().await.unwrap();
}

