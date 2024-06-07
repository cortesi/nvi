use nvi_derive::{nvi_service, rpc_request};

#[cfg(test)]
#[test]
fn it_derives_messages() {
    #[derive(Clone)]
    struct T {}

    #[nvi_service]
    impl T {
        #[rpc_request]
        async fn test_method(
            &self,
            client: &mut nvi::Client,
            a: i64,
            b: String,
        ) -> nvi::error::Result<String> {
            Ok(format!("{}:{}", a, b))
        }
    }

    let _ = T {};
}
