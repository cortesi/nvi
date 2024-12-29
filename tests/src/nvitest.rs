use nvi::test::NviTest;
use nvi_macros::nvi_service;
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

    let test = NviTest::new(TestService {}).await.unwrap();
    test.await_log("service connected").await.unwrap();
    test.finish().await.unwrap();
}
