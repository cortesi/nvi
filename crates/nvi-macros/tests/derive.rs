use nvi::test;
use nvi_macros::{notify, nvi_service, request};
use tokio::sync::broadcast;
use tracing_test::traced_test;

#[cfg(test)]
#[tokio::test]
#[traced_test]
async fn it_derives_messages() {
    #[derive(Clone)]
    struct T {
        tx: broadcast::Sender<()>,
    }

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

        async fn connected(&self, _: &mut nvi::Client) -> nvi::error::Result<()> {
            self.tx.send(()).unwrap();
            Ok(())
        }
    }

    let (tx, _) = broadcast::channel(16);
    test::test_service(T { tx: tx.clone() }, tx).await.unwrap();
}
