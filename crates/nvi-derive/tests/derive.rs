use quote::quote;

use async_trait::async_trait;
use nvi_derive::{rpc_request, rpc_service};

use nvi;

#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn it_derives_messages() {
    #[derive(Clone)]
    struct T {}

    // #[rpc_service]
    // impl T {
    //     #[rpc_request]
    //     async fn test_method(&self, a: i32, b: String) -> nvi::error::Result<String> {
    //         Ok(format!("{}:{}", a, b))
    //     }
    // }
    //
    // let s = quote! {
    //     struct TestService {
    //         fn test_method(&self, a: i32, b: String) -> Result<String> {
    //             Ok(format!("{}:{}", a, b))
    //         }
    //     }
    // };
    //
    // println!("{}", crate::parse_struct(s));
}
