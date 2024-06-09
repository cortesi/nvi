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

fn generate_argument(func: &str, p: &api::Parameter) -> TokenStream {
    let argname = clean_name(&p.1);
    let name = Ident::new(&argname, Span::call_site());
    if let Some(t) = overrides::get_arg_type(func, &argname) {
        quote! {
            #name: #t
        }
    } else {
        let typ = mk_arg_type(&p.0);
        quote! {
            #name: #typ
        }
    }
}

fn mk_arg_type(t: &api::Type) -> TokenStream {
    match t {
        api::Type::Array => quote! {
            Vec<Value>
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
            &Buffer
        },
        api::Type::Dictionary => quote! {
            Value
        },
        api::Type::Float => quote! {
            f64
        },
        api::Type::Function => unreachable!("function type in arg position"),
        api::Type::Integer => quote! {
            i64
        },
        api::Type::LuaRef => unreachable!("luaref type in arg"),
        api::Type::Object => quote! {
            Value
        },
        api::Type::String => quote! {
            &str
        },
        api::Type::Tabpage => quote! {
            &TabPage
        },
        api::Type::Void => unreachable!("void type in arg position"),
        api::Type::Window => quote! {
            &Window
        },
    }
}

fn mk_return_type(t: &api::Type) -> TokenStream {
    match t {
        api::Type::Array => quote! {
            Vec<Value>
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

fn generate_function(f: api::Function) -> TokenStream {
    let id = Ident::new(&f.name, Span::call_site());
    let name = &f.name;

    let args: Vec<TokenStream> = f
        .parameters
        .iter()
        .map(|a| generate_argument(name, a))
        .collect();
    let arg_vals: Vec<TokenStream> = f.parameters.iter().map(mk_arg_value).collect();

    let ret_type =
        overrides::get_return_override(name).unwrap_or_else(|| mk_return_type(&f.return_type));

    quote! {
        pub async fn #id(&self, #(#args),*) -> Result<#ret_type> {
            #[allow(unused_variables)]
            let ret = self.raw_request(#name, &[#(#arg_vals),*]).await
            .map_err(Error::RemoteError)?;
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
        .map(generate_function)
        .collect();
    let toks = quote!(
        #![allow(clippy::needless_question_mark)]
        #![allow(clippy::needless_borrow)]

        use msgpack_rpc::Value;
        use tracing::trace;
        use serde_rmpv::{from_value, to_value};

        use crate::error::{Result, Error};
        use crate::types::*;

        #[derive(Clone)]
        pub struct NvimApi {
            pub(crate) m_client: msgpack_rpc::Client,
        }

        impl NvimApi {
            pub async fn raw_request(
                &self,
                method: &str,
                params: &[msgpack_rpc::Value],
            ) -> Result<msgpack_rpc::Value, msgpack_rpc::Value> {
                trace!("send request: {:?} {:?}", method, params);
                self.m_client.request(method, params).await
            }

            pub async fn raw_notify(
                &self,
                method: &str,
                params: &[msgpack_rpc::Value],
            ) -> Result<(), ()> {
                trace!("send notification: {:?} {:?}", method, params);
                self.m_client.notify(method, params).await
            }

            #(#funcs)*
        }
    );
    print!("{}", format_with_rustfmt(toks));
    Ok(())
}
