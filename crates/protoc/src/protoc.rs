use std::{
    io::Write,
    process::{Command, Stdio},
};

use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::{api, overrides};

fn format_with_rustfmt(code: TokenStream) -> String {
    let mut rustfmt = Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start rustfmt");

    {
        let stdin = rustfmt.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(code.to_string().as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = rustfmt.wait_with_output().expect("Failed to read stdout");
    String::from_utf8(output.stdout).expect("Failed to convert output to string")
}

fn clean_name(name: &str) -> String {
    let mut name = name.to_string();
    for (a, b) in overrides::IDENT_MAP {
        if name == *a {
            name = b.to_string();
        }
    }
    name
}

/// Takes the function name, a parameter, and the current meta variable count. Returns the argument
/// and a boolean indicating whether the argument is a meta variable.
fn generate_argument(func: &str, p: &api::Parameter, meta_count: i32) -> (TokenStream, bool) {
    let argname = clean_name(&p.1);
    let name = Ident::new(&argname, Span::call_site());

    let metavar = Ident::new(vec!["T", "U", "V"][meta_count as usize], Span::call_site());
    if let Some(t) = overrides::get_arg_type(func, &argname) {
        (
            quote! {
                #name: #t
            },
            false,
        )
    } else {
        let (typ, meta) = match &p.0 {
            api::Type::Array => (
                quote! {
                    Vec<Value>
                },
                false,
            ),
            api::Type::Dict => (
                quote! {
                    HashMap<String, Value>
                },
                false,
            ),
            api::Type::ArrayOf { typ, .. } => {
                let typ = mk_return_type(typ);
                (
                    quote! {
                        Vec<#typ>
                    },
                    false,
                )
            }
            api::Type::Boolean => (
                quote! {
                    bool
                },
                false,
            ),
            api::Type::Buffer => (
                quote! {
                    &Buffer
                },
                false,
            ),
            api::Type::Dictionary => (
                quote! {
                    #metavar
                },
                true,
            ),
            api::Type::Float => (
                quote! {
                    f64
                },
                false,
            ),
            api::Type::Function => unreachable!("function type in arg position"),
            api::Type::Integer => (
                quote! {
                    i64
                },
                false,
            ),
            api::Type::LuaRef => unreachable!("luaref type in arg"),
            api::Type::Object => (
                quote! {
                    #metavar
                },
                true,
            ),
            api::Type::String => (
                quote! {
                    &str
                },
                false,
            ),
            api::Type::Tabpage => (
                quote! {
                    &TabPage
                },
                false,
            ),
            api::Type::Void => unreachable!("void type in arg position"),
            api::Type::Window => (
                quote! {
                    &Window
                },
                false,
            ),
        };
        (
            quote! {
                #name: #typ
            },
            meta,
        )
    }
}

fn mk_return_type(t: &api::Type) -> TokenStream {
    match t {
        api::Type::Array => quote! {
            Vec<Value>
        },
        api::Type::Dict => quote! {
            HashMap<String, Value>
        },
        api::Type::ArrayOf { typ, .. } => {
            let typ = mk_return_type(typ);
            quote! {
                Vec<#typ>
            }
        }
        api::Type::Boolean => quote! {
            bool
        },
        api::Type::Buffer => quote! {
            Buffer
        },
        api::Type::Dictionary => quote! {
            Value
        },
        api::Type::Float => quote! {
            f64
        },
        api::Type::Function => unreachable!("function type in return"),
        api::Type::Integer => quote! {
            i64
        },
        api::Type::LuaRef => unreachable!("luaref type in return"),
        api::Type::Object => quote! {
            Value
        },
        api::Type::String => quote! {
            String
        },
        api::Type::Tabpage => quote! {
            TabPage
        },
        api::Type::Void => quote! {
            ()
        },
        api::Type::Window => quote! {
            Window
        },
    }
}

fn mk_arg_value(p: &api::Parameter) -> TokenStream {
    let name = Ident::new(&clean_name(&p.1), Span::call_site());
    quote! {
        to_value(&#name)?
    }
}

/// Retrieves and formats the documentation for a given function name.
fn get_docs(name: &str) -> Option<String> {
    use std::io::{stderr, Write};

    let docs = crate::docs::DOCS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, doc)| {
            doc.lines()
                .map(|line| line.trim())
                .collect::<Vec<_>>()
                .join("\n")
        });

    if docs.is_none() {
        writeln!(stderr(), "Warning: no documentation found for {}", name).ok();
    }
    docs
}

fn generate_function(f: api::Function) -> TokenStream {
    let docs = get_docs(&f.name);
    // All functions have the nvim_ prefix, so we strip it.
    let id = Ident::new(&f.name[5..], Span::call_site());
    let name = &f.name;

    let mut args = vec![];
    let mut meta_count = 0;
    for a in &f.parameters {
        let (arg, meta) = generate_argument(name, a, meta_count);
        if meta {
            meta_count += 1;
        }
        args.push(arg);
    }

    let generics = match meta_count {
        0 => quote! {},
        1 => quote! { <T> },
        2 => quote! { <T, U> },
        3 => quote! { <T, U, V> },
        _ => panic!("unreachable"),
    };

    let where_clause = match meta_count {
        0 => quote! {},
        1 => quote! { where T: Serialize },
        2 => quote! { where T: Serialize, U: Serialize },
        3 => quote! { where T: Serialize, U: Serialize, V: Serialize },
        _ => panic!("unreachable"),
    };

    let arg_vals: Vec<TokenStream> = f.parameters.iter().map(mk_arg_value).collect();

    let ret_type =
        overrides::get_return_override(name).unwrap_or_else(|| mk_return_type(&f.return_type));

    let fn_def = if let Some(doc) = docs {
        quote! {
            #[doc = #doc]
            pub async fn #id #generics(&self, #(#args),*) -> Result<#ret_type>
        }
    } else {
        quote! {
            pub async fn #id #generics(&self, #(#args),*) -> Result<#ret_type>
        }
    };

    quote! {
        #fn_def
            #where_clause
        {
            #[allow(unused_variables)]
            let ret = self.raw_request(#name, &[#(#arg_vals),*]).await?;
            #[allow(clippy::needless_question_mark)]
            Ok(from_value(&ret)?)
        }
    }
}

/// Write the compiled protocol definition file to stdout.
pub fn protoc() -> Result<()> {
    let a = api::get_api()?;
    let funcs: Vec<TokenStream> = a
        .functions
        .into_iter()
        .filter(|f| !overrides::SKIP_FUNCTIONS.contains(&f.name.as_str()))
        .filter(|f| f.deprecated_since.is_none())
        .map(generate_function)
        .collect();
    let toks = quote!(
        #![allow(clippy::needless_question_mark)]
        #![allow(clippy::needless_borrow)]
        use std::collections::HashMap;

        use mrpc::Value;
        use tracing::{trace, debug};
        use serde_rmpv::{from_value, to_value};
        use serde::Serialize;

        use crate::error::{Result};
        use crate::types::*;
        use crate::opts;

        #[derive(Clone)]
        /// Generated bindings for Neovim's MessagePack-RPC API.
        pub struct NvimApi {
            pub(crate) rpc_sender: mrpc::RpcSender,
        }

        impl NvimApi {
            /// Make a raw request over the MessagePack-RPC protocol.
            pub async fn raw_request(
                &self,
                method: &str,
                params: &[mrpc::Value],
            ) -> Result<mrpc::Value, mrpc::RpcError> {
                trace!("send request: {:?} {:?}", method, params);
                let ret = self.rpc_sender.send_request(method, params).await;
                trace!("got response for {:?}: {:?}", method, ret);
                debug!("request: {:?}, ok", method);
                ret
            }

            /// Send a raw notification over the MessagePack-RPC protocol.
            pub async fn raw_notify(
                &self,
                method: &str,
                params: &[mrpc::Value],
            ) -> Result<(), mrpc::RpcError> {
                trace!("send notification: {:?} {:?}", method, params);
                debug!("notification: {:?}", method);
                self.rpc_sender.send_notification(method, params).await
            }

            #(#funcs)*
        }
    );
    print!("{}", format_with_rustfmt(toks));
    Ok(())
}
