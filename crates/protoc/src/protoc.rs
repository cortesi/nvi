use anyhow::{Context, Result, bail};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::{api, docs, overrides};

/// Format code with prettyplease
fn format_with_prettyplease(code: TokenStream) -> Result<String> {
    let file = syn::parse2(code).context("generated code does not parse")?;
    Ok(prettyplease::unparse(&file))
}

/// Names for the generic type parameters of a generated function, in order.
const GENERIC_NAMES: [&str; 4] = ["T", "U", "V", "W"];

/// The identifier of the generic type parameter at `index`.
fn generic_name(index: usize) -> Result<Ident> {
    let name = GENERIC_NAMES
        .get(index)
        .context("too many generic parameters")?;
    Ok(Ident::new(name, Span::call_site()))
}

/// Clean a name to be a valid identifier
fn clean_name(name: &str) -> String {
    let mut name = name.to_string();
    for (a, b) in overrides::IDENT_MAP {
        if name == *a {
            name = b.to_string();
        }
    }
    name
}

/// Takes the function name, a parameter, and the current meta variable count.
/// Returns the argument and a boolean indicating whether the argument is a meta
/// variable.
fn generate_argument(
    func: &str,
    p: &api::Parameter,
    meta_count: usize,
) -> Result<(TokenStream, bool)> {
    let argname = clean_name(&p.1);
    let name = Ident::new(&argname, Span::call_site());

    if let Some(t) = overrides::get_arg_type(func, &argname) {
        Ok((
            quote! {
                #name: #t
            },
            false,
        ))
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
                let typ = mk_return_type(typ)?;
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
            api::Type::Float => (
                quote! {
                    f64
                },
                false,
            ),
            api::Type::Function => bail!("unsupported function type for argument {argname}"),
            api::Type::Integer => (
                quote! {
                    i64
                },
                false,
            ),
            api::Type::LuaRef => bail!("unsupported LuaRef type for argument {argname}"),
            api::Type::Dictionary | api::Type::Object => {
                let metavar = generic_name(meta_count)?;
                (
                    quote! {
                        #metavar
                    },
                    true,
                )
            }
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
            api::Type::Void => bail!("unsupported void type for argument {argname}"),
            api::Type::Window => (
                quote! {
                    &Window
                },
                false,
            ),
        };
        Ok((
            quote! {
                #name: #typ
            },
            meta,
        ))
    }
}

/// Make a return type
fn mk_return_type(t: &api::Type) -> Result<TokenStream> {
    Ok(match t {
        api::Type::Array => quote! {
            Vec<Value>
        },
        api::Type::Dict => quote! {
            HashMap<String, Value>
        },
        api::Type::ArrayOf { typ, .. } => {
            let typ = mk_return_type(typ)?;
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
        api::Type::Function => bail!("unsupported function return type"),
        api::Type::Integer => quote! {
            i64
        },
        api::Type::LuaRef => bail!("unsupported LuaRef return type"),
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
    })
}

/// Retrieves and formats the documentation for a given function name.
/// Returns a vec of TokenStream, each representing a doc comment line.
fn get_docs(name: &str) -> Vec<TokenStream> {
    let Some((_, doc)) = docs::DOCS.iter().find(|(n, _)| *n == name) else {
        return Vec::new();
    };
    doc.trim()
        .lines()
        .map(|line| {
            let line = format!(" {}", line.trim());
            quote!(#[doc = #line])
        })
        .collect()
}

/// Generate a function definition
fn generate_function(f: &api::Function) -> Result<TokenStream> {
    let short = f
        .name
        .strip_prefix("nvim_")
        .context("function name lacks the nvim_ prefix")?;
    let id = Ident::new(short, Span::call_site());
    let name = &f.name;

    let mut args = vec![];
    let mut meta_count = 0;
    for a in &f.parameters {
        let (arg, meta) = generate_argument(name, a, meta_count)?;
        if meta {
            meta_count += 1;
        }
        args.push(arg);
    }

    // Start with the meta vars from arguments
    let mut generic_params = vec![];
    let mut where_bounds = vec![];

    // Add serialize bounds for meta variables from arguments
    for i in 0..meta_count {
        let param = generic_name(i)?;
        generic_params.push(param.clone());
        where_bounds.push(quote! { #param: Serialize });
    }

    let ret_type = if let Some(ret_type) = overrides::get_return_override(name) {
        ret_type
    } else if matches!(
        f.return_type,
        api::Type::Array | api::Type::Object | api::Type::Dictionary
    ) {
        let ret_param = generic_name(meta_count)?;
        generic_params.push(ret_param.clone());
        where_bounds.push(quote! { #ret_param: serde::de::DeserializeOwned });
        quote! { #ret_param }
    } else {
        mk_return_type(&f.return_type)?
    };

    let generics = if !generic_params.is_empty() {
        quote! { <#(#generic_params),*> }
    } else {
        quote! {}
    };

    let where_clause = if !where_bounds.is_empty() {
        quote! { where #(#where_bounds),* }
    } else {
        quote! {}
    };

    let arg_names: Vec<Ident> = f
        .parameters
        .iter()
        .map(|p| Ident::new(&clean_name(&p.1), Span::call_site()))
        .collect();
    let req_expr = match arg_names.as_slice() {
        [] => quote! { NO_PARAMS },
        [single] => quote! { #single },
        _ => quote! { (#(#arg_names),*) },
    };

    let doc_lines = get_docs(&f.name);
    let fn_def = quote! {
        #(#doc_lines)*
        pub async fn #id #generics(&self, #(#args),*) -> Result<#ret_type>
    };

    Ok(quote! {
        #fn_def
            #where_clause
        {
            #[allow(unused_variables)]
            let req = #req_expr;
            #[allow(clippy::needless_question_mark)]
            Ok(self.rpc_call(#name, req).await?)
        }
    })
}

/// Write the compiled protocol definition file to stdout.
pub fn protoc() -> Result<()> {
    let a = api::get_api()?;
    let funcs: Vec<TokenStream> = a
        .functions
        .into_iter()
        .filter(|f| !overrides::SKIP_FUNCTIONS.contains(&f.name.as_str()))
        .filter(|f| f.deprecated_since.is_none())
        .map(|f| generate_function(&f).with_context(|| format!("cannot generate {}", f.name)))
        .collect::<Result<_>>()?;
    let toks = quote!(
        #![allow(clippy::needless_question_mark)]
        #![allow(clippy::needless_borrow)]
        #![allow(clippy::doc_lazy_continuation)]
        use std::collections::HashMap;

        use mrpc::Value;
        use serde::{Serialize, de::DeserializeOwned};
        use tracing::trace;

        use crate::error::{Result};
        use super::types::*;
        use super::opts;

        const NO_PARAMS: [(); 0] = [];

        #[derive(Clone, Debug)]
        /// Generated bindings for Neovim's MessagePack-RPC API.
        pub struct NvimApi {
            pub(crate) rpc_sender: mrpc::RpcSender,
        }

        impl NvimApi {
            /// Make a typed request over the MessagePack-RPC protocol.
            pub async fn rpc_call<Req, Resp>(
                &self,
                method: &str,
                req: Req,
            ) -> Result<Resp, mrpc::RpcError>
            where
                Req: Serialize,
                Resp: DeserializeOwned,
            {
                let params = mrpc::serialize_params(&req)?;
                trace!("send request: {:?} {:?}", method, params);
                let ret = self.rpc_sender.send_request(method, &params).await?;
                trace!("got response for {:?}: {:?}", method, ret);
                mrpc::deserialize_response(&ret)
            }

            /// Send a typed notification over the MessagePack-RPC protocol.
            pub async fn rpc_notify<Req>(
                &self,
                method: &str,
                req: Req,
            ) -> Result<(), mrpc::RpcError>
            where
                Req: Serialize,
            {
                let params = mrpc::serialize_params(&req)?;
                trace!("send notification: {:?} {:?}", method, params);
                self.rpc_sender.send_notification(method, &params).await
            }

            /// Make a raw request over the MessagePack-RPC protocol.
            pub async fn raw_request(
                &self,
                method: &str,
                params: &[mrpc::Value],
            ) -> Result<mrpc::Value, mrpc::RpcError> {
                trace!("send request: {:?} {:?}", method, params);
                let ret = self.rpc_sender.send_request(method, params).await;
                trace!("got response for {:?}: {:?}", method, ret);
                ret
            }

            /// Send a raw notification over the MessagePack-RPC protocol.
            pub async fn raw_notify(
                &self,
                method: &str,
                params: &[mrpc::Value],
            ) -> Result<(), mrpc::RpcError> {
                trace!("send notification: {:?} {:?}", method, params);
                self.rpc_sender.send_notification(method, params).await
            }

            #(#funcs)*
        }
    );
    print!("{}", format_with_prettyplease(toks)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_rejects_unsupported_argument_types() {
        let f = api::Function {
            method: false,
            name: "nvim_test".into(),
            since: 1,
            parameters: vec![api::Parameter(api::Type::LuaRef, "callback".into())],
            return_type: api::Type::Void,
            deprecated_since: None,
        };
        let err = generate_function(&f).unwrap_err();
        assert_eq!(
            err.to_string(),
            "unsupported LuaRef type for argument callback"
        );
    }
}
