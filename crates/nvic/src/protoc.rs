use std::{
    io::Write,
    process::{Command, Stdio},
};

use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::api;

/// A map for rewriting identifiers in arguments to avoid built-ins
const IDENT_MAP: &'static [(&str, &str)] = &[("fn", "func"), ("type", "typ")];

/// Skip these, because they have LuaRef parameters, that don't seem to be supported on the client
/// yet.
const SKIP_FUNCTIONS: &'static [&str] = &["nvim_buf_call", "nvim_win_call"];

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
    for (a, b) in IDENT_MAP {
        if name == *a {
            name = b.to_string();
        }
    }
    name
}

fn generate_argument(p: &api::Parameter) -> TokenStream {
    let argname = clean_name(&p.1);
    let name = Ident::new(&argname, Span::call_site());
    let typ = to_type_arg(&p.0);
    quote! {
        #name: #typ
    }
}

fn to_type_arg(t: &api::Type) -> TokenStream {
    match t {
        api::Type::Array => quote! {
            Vec<Value>
        },
        api::Type::ArrayOf { typ, .. } => {
            let typ = to_type_result(typ);
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
        api::Type::Function => quote! {
            unreachable!("function type in arg position")
        },
        api::Type::Integer => quote! {
            i64
        },
        api::Type::LuaRef => quote! {
            unreachable!("luaref type in arg")
        },
        api::Type::Object => quote! {
            Value
        },
        api::Type::String => quote! {
            &str
        },
        api::Type::Tabpage => quote! {
            &TabPage
        },
        api::Type::Void => quote! {
            unreachable!("void type in arg position")
        },
        api::Type::Window => quote! {
            &Window
        },
    }
}

fn to_type_result(t: &api::Type) -> TokenStream {
    match t {
        api::Type::Array => quote! {
            Vec<Value>
        },
        api::Type::ArrayOf { typ, .. } => {
            let typ = to_type_result(typ);
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
        api::Type::Function => quote! {
            unreachable!("function type in return")
        },
        api::Type::Integer => quote! {
            i64
        },
        api::Type::LuaRef => quote! {
            unreachable!("luaref type in return")
        },
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

fn to_value(p: &api::Parameter) -> TokenStream {
    let name = Ident::new(&clean_name(&p.1), Span::call_site());
    match p.0 {
        api::Type::Array => quote! {
            Value::Array(#name.to_vec())
        },
        api::Type::ArrayOf { .. } => {
            // Allow clone on copy here, so we can treat values uniformly
            quote! {
                #[allow(clippy::clone_on_copy)]
                Value::Array(#name.iter().map(|x| Value::from(x.clone())).collect())
            }
        }
        api::Type::Boolean => quote! {
            Value::Boolean(#name)
        },
        api::Type::Buffer => quote! {
            Value::Ext(BUFFER_EXT_TYPE, #name.data.clone())
        },
        api::Type::Dictionary => quote! {
            #name.clone()
        },
        api::Type::Float => quote! {
            Value::F64(#name)
        },
        api::Type::Function => quote! {
            unreachable!("function type in return")
        },
        api::Type::Integer => quote! {
            Value::Integer(#name.into())
        },
        api::Type::LuaRef => quote! {
            unreachable!("luaref type in return")
        },
        api::Type::Object => quote! {
            #name.clone()
        },
        api::Type::String => quote! {
            Value::String(#name.into())
        },
        api::Type::Tabpage => quote! {
            Value::Ext(TABPAGE_EXT_TYPE, #name.data.clone())
        },
        api::Type::Void => quote! {
            unreachable!("void in value position")
        },
        api::Type::Window => quote! {
            Value::Ext(WINDOW_EXT_TYPE, #name.data.clone())
        },
    }
}

fn generate_function(f: api::Function) -> TokenStream {
    let id = Ident::new(&f.name, Span::call_site());
    let name = &f.name;
    let args: Vec<TokenStream> = f.parameters.iter().map(generate_argument).collect();
    let ret = to_type_result(&f.return_type);
    let arg_vals: Vec<TokenStream> = f.parameters.iter().map(to_value).collect();
    quote! {
        pub async fn #id(&self, #(#args),*) -> Result<#ret> {
            self.m_client.request(#name, &[#(#arg_vals),*]).await
            .map_err(Error::RemoteError)?;

            Err(Error::Unimplemented)
        }
    }
}

/// Write the compiled protocol definition file to stdout.
pub fn protoc() -> Result<()> {
    let a = api::get_api()?;
    let funcs: Vec<TokenStream> = a
        .functions
        .into_iter()
        .filter(|f| !SKIP_FUNCTIONS.contains(&f.name.as_str()))
        .map(generate_function)
        .collect();
    let toks = quote!(
        use msgpack_rpc::Value;

        use crate::error::{Result, Error};
        use crate::types::*;

        pub struct Api {
            m_client: msgpack_rpc::Client,
        }

        impl Api {
            #(#funcs)*
        }
    );
    print!("{}", format_with_rustfmt(toks));
    Ok(())
}
