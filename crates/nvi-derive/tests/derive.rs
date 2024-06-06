use quote::quote;

use nvi_derive::{rpc_notify, rpc_request, rpc_service};

use nvi;

#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn it_derives_messages() {
    #[derive(Clone)]
    struct T {}

    #[rpc_service]
    impl T {
        #[rpc_request]
        async fn test_method(
            &self,
            client: &mut nvi::NviClient,
            a: i64,
            b: String,
        ) -> nvi::error::Result<String> {
            Ok(format!("{}:{}", a, b))
        }
    }

    // let s = quote! {
    //     struct TestService {
    //         fn test_method(&self, a: i64, b: String) -> Result<String> {
    //             Ok(format!("{}:{}", a, b))
    //         }
    //     }
    // };

    // println!("{}", crate::parse_struct(s));
    //
    //
    // struct Test {}
    //
    // impl Test {
    //     #[rpc_request]
    //     /// Some docs
    //     fn test_method(
    //         &self,
    //         client: nvi::NviClient,
    //         a: i32,
    //         b: String,
    //         c: &str,
    //     ) -> Result<String> {
    //         Ok(format!("{}:{}", a, b))
    //     }
    //     #[rpc_request]
    //     fn test_void(&self, client: nvi::NviClient) {}
    //     #[rpc_request]
    //     fn test_usize(&self, client: nvi::NviClient) -> usize {}
    //     #[rpc_request]
    //     fn test_resultvoid(&self, client: nvi::NviClient) -> Result<()> {}
    //     #[rpc_notification]
    //     fn test_notification(&self, client: nvi::NviClient) -> Result<()> {}
    //
    //     fn skip(&self) {
    //         println!("skipping");
    //     }
    // }
    //
    //
    //
}
