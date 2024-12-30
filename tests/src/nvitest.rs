use futures_util::future::FutureExt;
use nvi::test::NviTest;
use nvi_macros::nvi_plugin;
use std::time::Duration;
use tracing::info;

#[tokio::test]
async fn test_nvi_test() {
    #[derive(Clone)]
    struct TestPlugin {}

    #[nvi_plugin]
    impl TestPlugin {
        async fn connected(&mut self, _: &mut nvi::Client) -> nvi::error::Result<()> {
            info!("plugin connected");
            Ok(())
        }
    }

    let test = NviTest::builder().run(TestPlugin {}).await.unwrap();
    test.await_log("plugin connected").await.unwrap();
    test.finish().await.unwrap();
}

#[tokio::test]
async fn test_concurrent() {
    #[derive(Clone)]
    struct TestPlugin {}

    #[nvi_plugin]
    impl TestPlugin {
        async fn connected(&self, _: &mut nvi::Client) -> nvi::error::Result<()> {
            Ok(())
        }
    }

    let test = NviTest::builder().run(TestPlugin {}).await.unwrap();

    let result = test
        .concurrent(
            |_| async { Ok(42) }.boxed(),
            |_| {
                async {
                    tokio::time::sleep(Duration::from_secs(1)).await;
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