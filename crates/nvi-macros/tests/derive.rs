use nvi::test;
use nvi_macros::{notify, nvi_service, request};
use tokio::sync::broadcast;

#[cfg(test)]
#[tokio::test]
async fn it_derives_messages() {
    #[derive(Clone)]
    struct T {}
    let (tx, _) = broadcast::channel(16);

    #[nvi_service]
    impl T {
        #[request]
        async fn test_method(
            &self,
            _: &mut nvi::Client,
            a: i64,
            b: String,
        ) -> nvi::error::Result<String> {
            Ok(format!("{}:{}", a, b))
        }

        #[notify]
        async fn test_notify(
            &self,
            _: &mut nvi::Client,
            a: i64,
            b: String,
        ) -> nvi::error::Result<()> {
            println!("{}:{}", a, b);
            Ok(())
        }

        #[notify]
        async fn test_notify_void(&self, _: &mut nvi::Client, a: i64, b: String) {
            println!("{}:{}", a, b);
        }
    }

    let rtx = tx.clone();
    test::test_service(T {}, rtx).await.unwrap();
    let _ = T {};
}
