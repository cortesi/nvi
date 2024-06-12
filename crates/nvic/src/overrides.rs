use proc_macro2::TokenStream;
use quote::quote;

/// A map for rewriting identifiers in arguments to avoid built-ins
pub const IDENT_MAP: &[(&str, &str)] = &[("fn", "func"), ("type", "typ")];

/// Skip these, because they have LuaRef parameters, that don't seem to be supported on the client
/// yet.
pub const SKIP_FUNCTIONS: &[&str] = &["nvim_buf_call", "nvim_win_call"];

pub struct Arg {
    pub name: String,
    pub typ: TokenStream,
}

pub struct Override {
    pub ret: Option<TokenStream>,
    pub args: Vec<Arg>,
}

pub fn get_override(name: &str) -> Option<Override> {
    Some(match name {
        "nvim_create_autocmd" => Override {
            args: vec![
                Arg {
                    name: "opts".into(),
                    typ: quote! { CreateAutocmdOpts },
                },
                Arg {
                    name: "event".into(),
                    typ: quote! { &[Event] },
                },
            ],
            ret: None,
        },
        "nvim_exec_autocmds" => Override {
            args: vec![
                Arg {
                    name: "event".into(),
                    typ: quote! { &[Event] },
                },
                Arg {
                    name: "opts".into(),
                    typ: quote! { ExecAutocmdsOpts },
                },
            ],
            ret: None,
        },
        "nvim_get_api_info" => Override {
            args: vec![],
            ret: Some(quote! { (u64, ApiInfo) }),
        },
        "nvim_get_chan_info" => Override {
            args: vec![],
            ret: Some(quote! { ChanInfo }),
        },
        "nvim_notify" => Override {
            args: vec![Arg {
                name: "log_level".into(),
                typ: quote! { u64 },
            }],
            ret: Some(quote! { () }),
        },
        "nvim_win_get_config" => Override {
            args: vec![],
            ret: Some(quote! { WindowConf }),
        },
        "nvim_win_set_config" => Override {
            args: vec![Arg {
                name: "config".into(),
                typ: quote! { WindowConf },
            }],
            ret: None,
        },
        _ => return None,
    })
}

pub fn get_arg_type(name: &str, arg: &str) -> Option<TokenStream> {
    if let Some(override_) = get_override(name) {
        for a in override_.args.iter() {
            if a.name == arg {
                return Some(a.typ.clone());
            }
        }
    }
    None
}

pub fn get_return_override(name: &str) -> Option<TokenStream> {
    get_override(name).and_then(|o| o.ret)
}
