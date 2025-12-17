//! Tests for nvi-macros.
#[cfg(test)]
mod tests {
    use nvi::{error::Result, test, Client};
    use nvi_macros::{notify, nvi_plugin, request};
    use tokio::sync::broadcast;
    use tracing_test::traced_test;

    #[tokio::test]
    #[traced_test]
    async fn it_derives_messages() {
        #[derive(Clone)]
        struct T {
            tx: broadcast::Sender<()>,
        }

        #[nvi_plugin]
        impl T {
            #[request]
            async fn test_method(&self, _: &Client, a: i64, b: String) -> Result<String> {
                Ok(format!("{a}:{b}"))
            }

            #[notify]
            async fn test_notify(&self, _: &Client, a: i64, b: String) -> Result<()> {
                println!("{a}:{b}");
                Ok(())
            }

            #[notify]
            async fn test_notify_void(&self, _: &Client, a: i64, b: String) {
                println!("{a}:{b}");
            }

            async fn connected(&self, _: &Client) -> Result<()> {
                self.tx.send(()).unwrap();
                Ok(())
            }
        }

        let (tx, _) = broadcast::channel(16);
        test::run_plugin_with_shutdown(T { tx: tx.clone() }, tx)
            .await
            .unwrap();
    }
}
