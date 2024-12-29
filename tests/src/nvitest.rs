use nvi::test::NviTest;
use nvi_macros::nvi_service;
use std::time::Duration;
use tracing::info;

#[tokio::test]
async fn test_nvi_test() {
    #[derive(Clone)]
    struct TestService {}

    #[nvi_service]
    impl TestService {
        async fn connected(&self, _: &mut nvi::Client) -> nvi::error::Result<()> {
            info!("service connected");
            Ok(())
        }
    }

    let test = NviTest::builder().run(TestService {}).await.unwrap();
    test.await_log("service connected").await.unwrap();
    test.finish().await.unwrap();
}

#[tokio::test]
async fn test_concurrent() {
    #[derive(Clone)]
    struct TestService {}

    #[nvi_service]
    impl TestService {
        async fn connected(&self, _: &mut nvi::Client) -> nvi::error::Result<()> {
            Ok(())
        }
    }

    let test = NviTest::builder().run(TestService {}).await.unwrap();

    let result = test
        .concurrent(
            |_client| Box::pin(async { Ok(42) }),
            |_client| {
                Box::pin(async {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    Ok(())
                })
            },
        )
        .await
        .unwrap();

    assert_eq!(result, 42);
    test.finish().await.unwrap();
}
