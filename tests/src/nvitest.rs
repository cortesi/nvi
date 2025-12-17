use std::time::Duration;

use futures_util::future::FutureExt;
use nvi::{error::Result, test::NviTest, Client};
use nvi_macros::nvi_plugin;
use tokio::time::sleep;
use tracing::info;

#[tokio::test]
async fn test_nvi_test() {
    #[derive(Clone)]
    struct TestPlugin {}

    #[nvi_plugin]
    impl TestPlugin {
        async fn connected(&self, _: &Client) -> Result<()> {
            info!("plugin connected");
            Ok(())
        }
    }

    let test = NviTest::builder()
        .with_plugin(TestPlugin {})
        .run()
        .await
        .unwrap();
    test.await_log("plugin connected").await.unwrap();
    test.finish().await.unwrap();
}

#[tokio::test]
async fn test_without_plugin() {
    let test = NviTest::builder()
        .log_level(tracing::Level::DEBUG)
        .run()
        .await
        .unwrap();

    // Verify we can still interact with nvim
    test.client.nvim.command("echo 'test'").await.unwrap();
    test.finish().await.unwrap();
}

#[tokio::test]
async fn test_concurrent() {
    #[derive(Clone)]
    struct TestPlugin {}

    #[nvi_plugin]
    impl TestPlugin {
        async fn connected(&self, _: &Client) -> Result<()> {
            Ok(())
        }
    }

    let test = NviTest::builder()
        .with_plugin(TestPlugin {})
        .run()
        .await
        .unwrap();

    let result = test
        .concurrent(
            |_| async { Ok(42) }.boxed(),
            |_| {
                async {
                    sleep(Duration::from_secs(1)).await;
                    Ok(())
                }
                .boxed()
            },
        )
        .await
        .unwrap();

    assert_eq!(result, 42);
    test.finish().await.unwrap();
}
