use std::vec;

use proc_macro_error::*;
use quote::{quote, ToTokens};
use structmeta::StructMeta;
use syn::{parse_macro_input, DeriveInput, Meta};

type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq, thiserror::Error, Debug, Clone)]
enum Error {
    Parse(String),
    Unsupported(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<syn::Error> for Error {
    fn from(e: syn::Error) -> Error {
        Error::Parse(e.to_string())
    }
}

impl From<Error> for Diagnostic {
    fn from(e: Error) -> Diagnostic {
        Diagnostic::spanned(
            proc_macro2::Span::call_site(),
            Level::Error,
            format!("{}", e),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MessageType {
    Request,
    Notification,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Arg {
    name: String,
    typ: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Return {
    /// A void return
    Void,
    /// A Result<()> return
    ResultVoid,
    /// A Result with an inner type
    Result(String),
    /// A naked non-Result return
    Type(String),
}

#[derive(Debug, Eq, PartialEq)]
struct Method {
    name: String,
    docs: String,
    ret: Return,
    message_type: MessageType,
    args: Vec<Arg>,
}

#[derive(Debug, Eq, PartialEq)]
struct Struct {
    name: String,
    methods: Vec<Method>,
}

impl Method {
    /// Output the invocation clause of a match macro
    fn invocation_clause(&self) -> proc_macro2::TokenStream {
        quote! {}
        // let ident = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        //
        // let mut args = vec![];
        // for (i, a) in self.args.iter().enumerate() {
        //     args.push(quote! {core});
        // }
        //
        // let mut inv = if self.ret.result {
        //     quote! {let s = self.#ident (#(#args),*) ?;}
        // } else {
        //     quote! {let s = self.#ident (#(#args),*) ;}
        // };
        //
        // // match self.ret.typ {
        // //     ReturnTypes::Void => inv.extend(quote! {Ok(canopy::commands::ReturnValue::Void)}),
        // //     ReturnTypes::String => {
        // //         inv.extend(quote! {Ok(canopy::commands::ReturnValue::String(s))})
        // //     }
        // // };
        //
        // let command = &self.name;
        // quote! { #command => { #inv } }
    }
}

impl quote::ToTokens for Method {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // let command = &self.name;
        // let docs = &self.docs;
        // let ret = &self.ret;
        // let args = &self.args;

        tokens.extend(quote! {canopy::commands::CommandSpec {
            // node: canopy::NodeName::convert(#node_name),
            // command: #command.to_string(),
            // docs: #docs.to_string(),
            // ret: #ret,
            // args: vec![#(#args),*]
        }})
    }
}

fn parse_command_method(method: &syn::ImplItemFn) -> Result<Option<Method>> {
    let mut docs: Vec<String> = vec![];
    let mut message_type = None;

    for a in &method.attrs {
        if a.path().is_ident(RPC_REQUEST) {
            message_type = Some(MessageType::Request);
        } else if a.path().is_ident(RPC_NOTIFICATION) {
            message_type = Some(MessageType::Notification);
        } else if a.path().is_ident("doc") {
            match &a.meta {
                Meta::NameValue(syn::MetaNameValue {
                    value:
                        syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit),
                            ..
                        }),
                    ..
                }) => {
                    docs.push(lit.value().trim().to_string());
                }
                _ => Err(Error::Parse("invalid doc attribute".into()))?,
            }
        }
    }
    let message_type = if let Some(a) = message_type {
        a
    } else {
        // This is not a command method
        return Ok(None);
    };

    let mut args = vec![];
    for i in &method.sig.inputs {
        match i {
            syn::FnArg::Receiver(_) => {}
            syn::FnArg::Typed(x) => {
                let mut arg = Arg::default();

                match &*x.pat {
                    syn::Pat::Ident(i) => {
                        arg.name = i.ident.to_string();
                    }
                    _ => {
                        return Err(Error::Unsupported(format!(
                            "unsupported argument type {:?} on command: {}",
                            quote! {#x.pat},
                            method.sig.ident
                        )))
                    }
                }
                match &*x.ty {
                    syn::Type::Path(p) => {
                        arg.typ = p.path.segments.last().unwrap().ident.to_string();
                    }
                    syn::Type::Reference(p) => {
                        arg.typ = p.to_token_stream().to_string();
                    }
                    _ => {
                        return Err(Error::Unsupported(format!(
                            "unsupported argument type {:?} on command: {}",
                            x.ty, method.sig.ident
                        )))
                    }
                }
                args.push(arg);
            }
        }
    }

    let ret = match &method.sig.output {
        syn::ReturnType::Default => Return::Void,
        syn::ReturnType::Type(_, ty) => match &**ty {
            syn::Type::Path(p) => {
                if p.path.segments.last().unwrap().ident == "Result" {
                    match &p.path.segments.last().unwrap().arguments {
                        syn::PathArguments::AngleBracketed(a) => {
                            if a.args.len() != 1 {
                                panic!("invalid");
                            } else {
                                match a.args.first().unwrap() {
                                    syn::GenericArgument::Type(syn::Type::Path(t)) => {
                                        Return::Result(t.to_token_stream().to_string())
                                    }
                                    syn::GenericArgument::Type(syn::Type::Tuple(e)) => {
                                        if e.elems.is_empty() {
                                            Return::ResultVoid
                                        } else {
                                            Return::Result(e.to_token_stream().to_string())
                                        }
                                    }
                                    _ => panic!("invalid"),
                                }
                            }
                        }
                        _ => panic!("invalid"),
                    }
                } else {
                    Return::Type(p.to_token_stream().to_string())
                }
            }
            _ => panic!("invalid"),
        },
    };

    Ok(Some(Method {
        name: method.sig.ident.to_string(),
        docs: docs.join("\n"),
        message_type,
        ret,
        args,
    }))
}

fn parse_struct(input: proc_macro2::TokenStream) -> Result<Struct> {
    let v = syn::parse2::<syn::ItemImpl>(input)?;

    let tp = match *v.clone().self_ty {
        syn::Type::Path(p) => p,
        _ => panic!("unexpected input"),
    };

    let name = tp.path.segments[0].ident.to_string();
    let (impl_generics, _, where_clause) = v.generics.split_for_impl();

    let mut methods = vec![];
    for i in v.items {
        if let syn::ImplItem::Fn(m) = i {
            if let Some(command) = parse_command_method(&m)? {
                methods.push(command);
            }
        }
    }
    Ok(Struct { name, methods })
}

/// Derive an implementation of the `CommandNode` trait. This macro should be added
/// to the impl block of a struct. All methods that are annotated with `command`
/// are added as commands, with their doc comments as the command documentation.
#[proc_macro_error]
#[proc_macro_attribute]
pub fn rpc_service(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::ItemImpl);

    let tp = match *input.clone().self_ty {
        syn::Type::Path(p) => p,
        _ => panic!("unexpected input"),
    };

    // The default node name
    let node_name = tp.path.segments[0].ident.to_string();

    let orig = input.clone();
    let name = input.self_ty;
    let (impl_generics, _, where_clause) = input.generics.split_for_impl();

    let mut commands = vec![];
    for i in input.items {
        if let syn::ImplItem::Fn(m) = i {
            if let Some(command) = parse_command_method(&m).unwrap_or_abort() {
                commands.push(command);
            }
        }
    }

    let invocations: Vec<proc_macro2::TokenStream> =
        commands.iter().map(|x| x.invocation_clause()).collect();

    let expanded = quote! {
        impl #impl_generics canopy::commands::CommandNode for #name #where_clause {
            fn commands() -> Vec<canopy::commands::CommandSpec> {
                vec![#(#commands),*]
            }
            fn dispatch(&mut self, core: &mut dyn canopy::Context, cmd: &canopy::commands::CommandInvocation) -> canopy::Result<canopy::commands::ReturnValue> {
                if cmd.node != self.name() {
                    return Err(canopy::Error::UnknownCommand(cmd.command.to_string()));
                }
                match cmd.command.as_str() {
                    #(#invocations),*
                    _ => Err(canopy::Error::UnknownCommand(cmd.command.to_string()))
                }
            }
        }
    };
    let out = quote! {
        #orig
        #expanded
    };
    out.into()
}

const RPC_REQUEST: &str = "rpc_request";

/// Mark a method as a command. This macro should be used to decorate methods in
/// an `impl` block that uses the `derive_commands` macro. A number of optional
/// arguments can be passed:
///
/// - `ignore_result` tells Canopy that the return value of the method should
///   not be exposed through the command mechanism. This is useful for dual-use
///   methods that may return values when called from Rust.
#[proc_macro_attribute]
pub fn rpc_request(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input
}

const RPC_NOTIFICATION: &str = "rpc_notification";

#[proc_macro_attribute]
pub fn rpc_notification(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input
}

/// Derive an implementation of the StatefulNode trait for a struct. The struct
/// should have a `self.state` attribute of type `NodeState`.
#[proc_macro_derive(StatefulNode)]
pub fn derive_statefulnode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let rname = name.to_string();
    let expanded = quote! {
        impl #impl_generics canopy::StatefulNode for #name #ty_generics #where_clause {
            fn state_mut(&mut self) -> &mut canopy::NodeState {
                &mut self.state
            }
            fn state(&self) -> &canopy::NodeState {
                &self.state
            }
            fn name(&self) -> canopy::NodeName {
                canopy::NodeName::convert(#rname)
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_parses_struct() {
        let s = quote! {
            impl TestService {
                #[rpc_request]
                /// Some docs
                fn test_method(&self, a: i32, b: String, c: &str, d: foo::bar::Voing) -> Result<String> {
                    Ok(format!("{}:{}", a, b))
                }
                #[rpc_request]
                fn test_void(&self) {}
                #[rpc_request]
                fn test_usize(&self) -> usize {}
                #[rpc_request]
                fn test_resultvoid(&self) -> Result<()> {}
                #[rpc_notification]
                fn test_notification(&self) -> Result<()> {}
            }
        };

        let expected = Struct {
            name: "TestService".into(),
            methods: vec![
                Method {
                    name: "test_method".into(),
                    docs: "Some docs".into(),
                    ret: Return::Result("String".into()),
                    message_type: MessageType::Request,
                    args: vec![
                        Arg {
                            name: "a".into(),
                            typ: "i32".into(),
                        },
                        Arg {
                            name: "b".into(),
                            typ: "String".into(),
                        },
                        Arg {
                            name: "c".into(),
                            typ: "& str".into(),
                        },
                        Arg {
                            name: "d".into(),
                            typ: "Voing".into(),
                        },
                    ],
                },
                Method {
                    name: "test_void".into(),
                    docs: "".into(),
                    ret: Return::Void,
                    message_type: MessageType::Request,
                    args: vec![],
                },
                Method {
                    name: "test_usize".into(),
                    docs: "".into(),
                    ret: Return::Type("usize".into()),
                    message_type: MessageType::Request,
                    args: vec![],
                },
                Method {
                    name: "test_resultvoid".into(),
                    docs: "".into(),
                    ret: Return::ResultVoid,
                    message_type: MessageType::Request,
                    args: vec![],
                },
                Method {
                    name: "test_notification".into(),
                    docs: "".into(),
                    ret: Return::ResultVoid,
                    message_type: MessageType::Notification,
                    args: vec![],
                },
            ],
        };
        assert_eq!(parse_struct(s).unwrap(), expected);
    }
}
