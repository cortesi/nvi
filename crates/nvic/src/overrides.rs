use proc_macro2::TokenStream;
use quote::quote;

/// A map for rewriting identifiers in arguments to avoid built-ins
pub const IDENT_MAP: &[(&str, &str)] = &[("fn", "func"), ("type", "typ")];

/// Skip these, because they have LuaRef parameters, that don't seem to be supported on the client
/// yet.
pub const SKIP_FUNCTIONS: &[&str] = &["nvim_buf_call", "nvim_win_call"];

pub struct Return {
    pub typ: TokenStream,
    pub conversion: TokenStream,
}

pub struct Override {
    pub ret: Option<Return>,
}

pub fn get_override(name: &str) -> Option<Override> {
    Some(match name {
        "nvim_get_api_info" => Override {
            ret: Some(Return {
                typ: quote! { (u64, ApiInfo) },
                conversion: quote! { Ok(from_value::<(u64, ApiInfo)>(&ret)?) },
            }),
        },
        _ => return None,
    })
}

pub fn get_return_override(name: &str) -> Option<Return> {
    get_override(name).and_then(|o| o.ret)
}
