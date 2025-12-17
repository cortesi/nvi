use proc_macro2::TokenStream;
use quote::quote;

/// A map for rewriting identifiers in arguments to avoid built-ins
pub const IDENT_MAP: &[(&str, &str)] = &[("fn", "func"), ("type", "typ")];

/// Skip these, because they have LuaRef parameters, that don't seem to be supported on the client
/// yet.
pub const SKIP_FUNCTIONS: &[&str] = &["nvim_buf_call", "nvim_win_call"];

/// An argument override
pub struct Arg {
    /// The name of the argument
    pub name: String,
    /// The type of the argument
    pub typ: TokenStream,
}

/// An override definition
pub struct Override {
    /// The return type override
    pub ret: Option<TokenStream>,
    /// The argument overrides
    pub args: Vec<Arg>,
}

/// Get an override for a function
pub fn get_override(name: &str) -> Option<Override> {
    Some(match name {
        "nvim_buf_delete" => Override {
            args: vec![
                Arg {
                    name: "buf".into(),
                    typ: quote! { Buffer },
                },
                Arg {
                    name: "opts".into(),
                    typ: quote! { opts::BufDelete },
                },
            ],
            ret: None,
        },
        "nvim_clear_autocmds" => Override {
            args: vec![Arg {
                name: "opts".into(),
                typ: quote! { opts::ClearAutocmds },
            }],
            ret: None,
        },
        "nvim_create_autocmd" => Override {
            args: vec![
                Arg {
                    name: "opts".into(),
                    typ: quote! { opts::CreateAutocmd },
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
                    typ: quote! { opts::ExecAutocmds },
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
        "nvim_open_win" => Override {
            args: vec![Arg {
                name: "config".into(),
                typ: quote! { WindowConf },
            }],
            ret: None,
        },
        "nvim_set_hl" => Override {
            args: vec![Arg {
                name: "val".into(),
                typ: quote! { opts::SetHl },
            }],
            ret: None,
        },
        "nvim_set_option_value" => Override {
            args: vec![Arg {
                name: "opts".into(),
                typ: quote! { opts::SetOptionValue },
            }],
            ret: None,
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
        "nvim_win_get_position" => Override {
            args: vec![Arg {
                name: "window".into(),
                typ: quote! { &Window },
            }],
            ret: Some(quote! {(i64, i64)}),
        },
        _ => return None,
    })
}

/// Get the type of an argument
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

/// Get the return type override for a function
pub fn get_return_override(name: &str) -> Option<TokenStream> {
    get_override(name).and_then(|o| o.ret)
}
