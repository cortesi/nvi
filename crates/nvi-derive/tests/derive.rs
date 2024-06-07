use nvi_derive::{notify, nvi_service, request};

#[cfg(test)]
#[test]
fn it_derives_messages() {
    #[derive(Clone)]
    struct T {}

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

    let _ = T {};
}
