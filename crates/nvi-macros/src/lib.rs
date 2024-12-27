use std::vec;

use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, spanned::Spanned, Expr, ExprLit, Lit, Meta, Token};

use proc_macro2_diagnostics::{Diagnostic, SpanDiagnosticExt};

const CONNECTED: &str = "connected";

type Result<T> = std::result::Result<T, Diagnostic>;

const RPC: &str = "request";
const RPC_NOTIFICATION: &str = "notify";
const RPC_AUTOCMD: &str = "autocmd";
const RPC_AUTOCMD_EVENTS: &str = "events";
const RPC_AUTOCMD_PATTERNS: &str = "patterns";
const RPC_AUTOCMD_GROUP: &str = "group";
const RPC_AUTOCMD_NESTED: &str = "nested";

#[derive(Debug, Clone, PartialEq, Eq)]
enum MethodType {
    Request,
    Notify,
    Connected,
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
struct AutoCmd {
    events: Vec<String>,
    patterns: Vec<String>,
    group: Option<String>,
    nested: bool,
}

#[derive(Debug, Eq, PartialEq)]
struct Method {
    name: String,
    docs: String,
    ret: Return,
    message_type: MethodType,
    args: Vec<Arg>,
    autocmd: Option<AutoCmd>,
}

#[derive(Debug, Eq, PartialEq)]
struct ImplBlock {
    name: String,
    methods: Vec<Method>,
    run: Option<Method>,
}

impl Method {
    fn bootstrap_clause(&self, namespace: &str) -> proc_macro2::TokenStream {
        let method = self.name.clone();
        if self.args.is_empty() {
            // If we have no arguments, we must satisfy the compiler by specifying a generic type
            // for the empty array.
            if self.message_type == MethodType::Notify {
                quote! {
                    client.register_rpcnotify::<String>(#namespace, #method, &[]).await?;
                }
            } else {
                quote! {
                    client.register_rpcrequest::<String>(#namespace, #method, &[]).await?;
                }
            }
        } else {
            let args = self.args.clone().into_iter().map(|a| a.name.to_string());
            if self.message_type == MethodType::Notify {
                quote! {
                    client.register_rpcnotify(#namespace, #method, &[#(#args),*]).await?;
                }
            } else {
                quote! {
                    client.register_rpcrequest(#namespace, #method, &[#(#args),*]).await?;
                }
            }
        }
    }

    /// Output the invocation clause of a connecte function
    fn connected_invocation(&self, name: syn::Ident) -> proc_macro2::TokenStream {
        let method = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        match self.ret {
            Return::Void => {
                quote! {
                        #name::#method(self, client).await;
                }
            }
            Return::ResultVoid => {
                quote! {
                        #name::#method(self, client).await?;
                }
            }
            _ => panic!("unreachable"),
        }
    }

    /// Output the invocation clause of a notify function
    fn notify_invocation(&self) -> proc_macro2::TokenStream {
        let name = self.name.clone();
        let method = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let mut args = vec![];
        for (idx, _) in self.args.iter().enumerate() {
            let a = quote! {
                serde_rmpv::from_value(&params[#idx])?
            };
            args.push(a);
        }

        let inv = match self.ret {
            Return::Void => {
                quote! {
                        self.#method(client, #(#args),*).await;
                }
            }
            Return::ResultVoid => {
                quote! {
                        self.#method(client, #(#args),*).await?;
                }
            }
            _ => panic!("unreachable"),
        };

        quote! {
            #name => {
                #inv
            }
        }
    }

    /// Output the invocation clause of a request function
    fn request_invocation(&self) -> proc_macro2::TokenStream {
        let name = self.name.clone();
        let method = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
        let mut args = vec![];
        for (idx, _) in self.args.iter().enumerate() {
            let a = quote! {
                serde_rmpv::from_value(&params[#idx])
                    .map_err(|e| nvi::Value::from(format!("{}", e)))?
            };
            args.push(a);
        }

        let arg_len = self.args.len();

        let inv = match self.ret {
            Return::Void => {
                quote! {
                        self.#method(client, #(#args),*).await;
                        nvi::Value::Nil
                }
            }
            Return::ResultVoid => {
                quote! {
                        self.#method(client, #(#args),*).await.map_err(|e| nvi::Value::from(format!("{}", e)))?;
                        nvi::Value::Nil
                }
            }
            Return::Result(_) => {
                quote! {
                        serde_rmpv::to_value(
                            &self.#method(client, #(#args),*).await
                                .map_err(|e| nvi::Value::from(format!("{}", e)))?
                        ).map_err(|e| nvi::Value::from(format!("{}", e)))?
                }
            }
            Return::Type(_) => {
                quote! {
                        serde_rmpv::to_value(
                            &self.#method(client, #(#args),*).await
                        ).map_err(|e| nvi::Value::from(format!("{}", e)))?
                }
            }
        };

        quote! {
            #name => {
                if params.len() != #arg_len {
                    nvi::error::Result::Err(nvi::Value::from("invalid number of arguments"))?
                }
                #inv
            }
        }
    }
}

fn parse_method(method: &syn::ImplItemFn) -> Result<Option<Method>> {
    let mut message_type = None;
    let mut docs: Vec<String> = vec![];
    let mut autocmd = None;
    let name = method.sig.ident.to_string();

    for a in &method.attrs {
        if a.path().is_ident(RPC) {
            message_type = Some(MethodType::Request);
        } else if a.path().is_ident(RPC_NOTIFICATION) {
            message_type = Some(MethodType::Notify);
        } else if a.path().is_ident(RPC_AUTOCMD) {
            message_type = Some(MethodType::Request);
            if let Meta::List(list) = &a.meta {
                let mut events = vec![];
                let mut patterns = vec![];
                let mut group = None;
                let mut nested = false;

                for nested_meta in
                    list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?
                {
                    match nested_meta {
                        Meta::NameValue(meta) if meta.path.is_ident(RPC_AUTOCMD_EVENTS) => {
                            if let Expr::Array(array) = &meta.value {
                                for event in array.elems.iter() {
                                    if let Expr::Lit(ExprLit {
                                        lit: Lit::Str(lit), ..
                                    }) = event
                                    {
                                        events.push(lit.value());
                                    }
                                }
                            }
                        }
                        Meta::NameValue(meta) if meta.path.is_ident(RPC_AUTOCMD_PATTERNS) => {
                            if let Expr::Array(array) = &meta.value {
                                for pattern in array.elems.iter() {
                                    if let Expr::Lit(ExprLit {
                                        lit: Lit::Str(lit), ..
                                    }) = pattern
                                    {
                                        patterns.push(lit.value());
                                    }
                                }
                            }
                        }
                        Meta::NameValue(meta) if meta.path.is_ident(RPC_AUTOCMD_GROUP) => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(lit), ..
                            }) = &meta.value
                            {
                                group = Some(lit.value());
                            }
                        }
                        Meta::NameValue(meta) if meta.path.is_ident(RPC_AUTOCMD_NESTED) => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Bool(lit),
                                ..
                            }) = &meta.value
                            {
                                nested = lit.value();
                            }
                        }
                        _ => return Err(a.span().error("invalid autocmd attribute")),
                    }
                }

                if events.is_empty() {
                    return Err(a.span().error("autocmd must specify events"));
                }

                autocmd = Some(AutoCmd {
                    events,
                    patterns,
                    group,
                    nested,
                });
            }
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
                _ => Err(method.span().error("invalid doc attribute"))?,
            }
        }
    }

    if message_type.is_none() && name == CONNECTED {
        message_type = Some(MethodType::Connected);
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
                    syn::Pat::Wild(_) => {
                        arg.name = "_".to_string();
                    }
                    _ => return Err(i.span().error("unsupported argument type")),
                }
                match &*x.ty {
                    syn::Type::Path(p) => {
                        arg.typ = p.path.segments.last().unwrap().ident.to_string();
                    }
                    syn::Type::Reference(p) => {
                        arg.typ = p.to_token_stream().to_string();
                    }
                    _ => return Err(i.span().error("unsupported argument type")),
                }
                args.push(arg);
            }
        }
    }

    if let Some(first) = args.first() {
        if !first.typ.ends_with("Client") {
            return Err(method.span().error("first argument must be `Client`"));
        }
    } else {
        return Err(method.span().error("no arguments on command method"));
    }

    let args = args.into_iter().skip(1).collect::<Vec<_>>();

    let ret = match &method.sig.output {
        syn::ReturnType::Default => Return::Void,
        syn::ReturnType::Type(_, ty) => match &**ty {
            syn::Type::Path(p) => {
                if p.path.segments.last().unwrap().ident == "Result" {
                    match &p.path.segments.last().unwrap().arguments {
                        syn::PathArguments::AngleBracketed(a) => {
                            if a.args.len() != 1 {
                                return Err(method.span().error("invalid rpc method"));
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
                                    _ => return Err(method.span().error("invalid rpc method")),
                                }
                            }
                        }
                        _ => return Err(method.span().error("invalid rpc method")),
                    }
                } else {
                    Return::Type(p.to_token_stream().to_string())
                }
            }
            _ => return Err(method.span().error("invalid rpc method")),
        },
    };

    if message_type == MethodType::Notify && !(ret == Return::ResultVoid || ret == Return::Void) {
        return Err(method
            .span()
            .error("notification methods must return Result<()> or be void"));
    }

    if message_type == MethodType::Connected {
        if !(ret == Return::ResultVoid || ret == Return::Void) {
            return Err(method
                .span()
                .error("notification methods must return Result<()> or be void"));
        }
        if !args.is_empty() {
            return Err(method
                .span()
                .error("run methods must take only a Client argument"));
        }
    }

    Ok(Some(Method {
        name,
        message_type,
        ret,
        args,
        docs: docs.join("\n"),
        autocmd,
    }))
}

/// Parse an impl block, and extract the methods marked with `request` or `notify`.
fn parse_impl(input: &proc_macro2::TokenStream) -> Result<(syn::ItemImpl, ImplBlock)> {
    let v = syn::parse2::<syn::ItemImpl>(input.clone())?;
    let tp = match *v.clone().self_ty {
        syn::Type::Path(p) => p,
        _ => panic!("unexpected input"),
    };

    let name = tp.path.segments[0].ident.to_string();

    let mut methods = vec![];
    for i in &v.items {
        if let syn::ImplItem::Fn(m) = i {
            if let Some(command) = parse_method(m)? {
                methods.push(command);
            }
        }
    }
    Ok((
        v,
        ImplBlock {
            name,
            methods,
            run: None,
        },
    ))
}

// Extract this to ease testing, since proc_macro::TokenStream can't cross proc-macro boundaries.
fn inner_nvi_service(
    _attr: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream> {
    let (output, imp) = parse_impl(&input)?;

    let bootstraps: Vec<proc_macro2::TokenStream> = imp
        .methods
        .iter()
        .filter(|x| x.message_type != MethodType::Connected)
        .map(|x| x.bootstrap_clause(&imp.name))
        .collect();

    let request_invocations: Vec<proc_macro2::TokenStream> = imp
        .methods
        .iter()
        .filter(|x| x.message_type == MethodType::Request)
        .map(|x| x.request_invocation())
        .collect();

    let notify_invocations: Vec<proc_macro2::TokenStream> = imp
        .methods
        .iter()
        .filter(|x| x.message_type == MethodType::Notify)
        .map(|x| x.notify_invocation())
        .collect();

    let name = syn::Ident::new(&imp.name, proc_macro2::Span::call_site());
    let namestr = imp.name.clone();

    let connected_invocations: Vec<proc_macro2::TokenStream> = imp
        .methods
        .iter()
        .filter(|x| x.message_type == MethodType::Connected)
        .map(|x| x.connected_invocation(name.clone()))
        .collect();

    if connected_invocations.len() > 1 {
        return Err(input.span().error("only one 'connected' method is allowed"));
    }

    let connected = connected_invocations.first().unwrap_or(&quote! {}).clone();

    Ok(quote! {
        #output

        #[async_trait::async_trait]
        impl nvi::NviService for #name {
            fn name(&self) -> String {
                #namestr.into()
            }

            async fn bootstrap(&self, client: &mut nvi::Client) -> nvi::error::Result<()> {
                #(#bootstraps)*
                Ok(())
            }

            async fn connected(&self, client: &mut nvi::Client) -> nvi::error::Result<()> {
                #connected
                Ok(())
            }

            async fn request(
                &self,
                client: &mut nvi::Client,
                method: &str,
                params: &[nvi::Value],
            ) -> nvi::error::Result<nvi::Value, nvi::Value> {
                Ok(
                    match method {
                        #(#request_invocations),*
                        _ => {
                            nvi::error::Result::Err(nvi::Value::from("unknown method"))?
                        }
                    }
                )
            }

            async fn notify(
                &self,
                client: &mut nvi::Client,
                method: &str,
                params: &[nvi::Value],
            ) -> nvi::error::Result<()> {
                match method {
                    #(#notify_invocations),*
                    _ => {
                        Err(nvi::error::Error::Internal{ msg: "unknown method".to_string() })?
                    }
                }
                Ok(())
            }
        }
    }
    .to_token_stream())
}

/// Add this attribute to the *impl* block for the `NviService` trait to derive implementations for
/// the `message` and `notification` methods.
#[proc_macro_attribute]
pub fn nvi_service(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match inner_nvi_service(_attr.into(), input.into()) {
        Ok(x) => x.into(),
        Err(e) => e.emit_as_expr_tokens().into(),
    }
}

/// Mark a method as an RPC request.
#[proc_macro_attribute]
pub fn request(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input
}

/// Mark a method as an RPC notification. Notification methods do not return a value,
/// so must return `Result<()>` or be void.
#[proc_macro_attribute]
pub fn notify(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rust_format::{Formatter, RustFmt};

    #[test]
    fn it_parses_struct() {
        let s = quote! {
            impl <T>TestService for Test<T> {
                #[request]
                /// Some docs
                fn test_method(&self, client: &mut nvi::Client, a: i32, b: String, c: &str, d: foo::bar::Voing) -> Result<String> {
                    Ok(format!("{}:{}", a, b))
                }
                #[request]
                fn test_void(&self, client: &mut nvi::Client) {}
                #[request]
                fn test_usize(&self, client: &mut nvi::Client) -> usize {}
                #[request]
                fn test_resultvoid(&self, client: &mut nvi::Client) -> Result<()> {}
                #[notify]
                fn test_notification(&self, client: &mut nvi::Client) -> Result<()> {}

                fn skip(&self, client: &mut nvi::Client) {
                    println!("skipping");
                }
            }
        };

        let expected = ImplBlock {
            name: "Test".into(),
            run: None,
            methods: vec![
                Method {
                    name: "test_method".into(),
                    docs: "Some docs".into(),
                    ret: Return::Result("String".into()),
                    message_type: MethodType::Request,
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
                    autocmd: None,
                },
                Method {
                    name: "test_void".into(),
                    docs: "".into(),
                    ret: Return::Void,
                    message_type: MethodType::Request,
                    args: vec![],
                    autocmd: None,
                },
                Method {
                    name: "test_usize".into(),
                    docs: "".into(),
                    ret: Return::Type("usize".into()),
                    message_type: MethodType::Request,
                    args: vec![],
                    autocmd: None,
                },
                Method {
                    name: "test_resultvoid".into(),
                    docs: "".into(),
                    ret: Return::ResultVoid,
                    message_type: MethodType::Request,
                    args: vec![],
                    autocmd: None,
                },
                Method {
                    name: "test_notification".into(),
                    docs: "".into(),
                    ret: Return::ResultVoid,
                    message_type: MethodType::Notify,
                    args: vec![],
                    autocmd: None,
                },
            ],
        };
        let (_, ret) = parse_impl(&s).unwrap();
        assert_eq!(ret, expected);
    }

    #[test]
    fn it_constrains_signatures() {
        let s = quote! {
            impl Test {
                #[request]
                async fn test_void(&self) {}
            }
        };
        assert!(inner_nvi_service(quote! {}, s).is_err());
    }

    #[test]
    fn it_renders_service() {
        let s = quote! {
            impl <T>TestService for Test<T> {
                #[request]
                /// Some docs
                async fn test_method(&self, client: &mut nvi::Client, a: i32, b: String, c: &str, d: foo::bar::Voing) -> Result<String> {
                    Ok(format!("{}:{}", a, b))
                }

                #[request]
                async fn test_void(&self, client: &mut nvi::Client) {}

                #[request]
                async fn test_usize(&self, client: &mut nvi::Client) -> usize {}

                #[request]
                async fn test_resultvoid(&self, client: &mut nvi::Client) -> nvi::error::Result<()> {}

                #[notify]
                async fn test_notification(&self, client: &mut nvi::Client) -> nvi::error::Result<()> {}

                #[notify]
                async fn test_notification_void(&self, client: &mut nvi::Client)  {}

                fn skip(&self) {
                    println!("skipping");
                }
            }
        };
        println!(
            "{}",
            RustFmt::default()
                .format_tokens(inner_nvi_service(quote! {}, s).unwrap())
                .unwrap()
        );
    }

    #[test]
    fn it_renders_simple_service() {
        let s = quote! {
            impl Test {
                #[request]
                /// Some docs
                async fn test_method(&self, client: &mut nvi::Client, a: i32, b: String, c: &str) -> nvi::error::Result<String> {
                    Ok(format!("{}:{}", a, b))
                }
                #[request]
                async fn test_void(&self, client: &mut nvi::Client) {}

                #[request]
                async fn test_usize(&self, client: &mut nvi::Client) -> usize {
                    0
                }

                #[request]
                async fn test_resultvoid(&self, client: &mut nvi::Client) -> nvi::error::Result<()> {
                    Ok(())
                }

                #[notify]
                async fn test_notification(&self, client: &mut nvi::Client, a: i32) -> nvi::error::Result<()> {
                    Ok(())
                }

                fn skip(&self) {
                    println!("skipping");
                }
            }
        };
        println!(
            "{}",
            RustFmt::default()
                .format_tokens(inner_nvi_service(quote! {}, s).unwrap())
                .unwrap()
        );
    }

    #[test]
    fn it_parses_autocmd() {
        let s = quote! {
            impl Test {
                #[autocmd(events=["BufEnter", "BufLeave"], patterns=["*.rs"], group="test", nested=true)]
                async fn test_autocmd(&self, client: &mut nvi::Client) -> nvi::error::Result<()> {
                    Ok(())
                }
            }
        };

        let result = parse_impl(&s);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

        let (_, ret) = result.unwrap();
        assert_eq!(
            ret.methods[0].autocmd,
            Some(AutoCmd {
                events: vec!["BufEnter".into(), "BufLeave".into()],
                patterns: vec!["*.rs".into()],
                group: Some("test".into()),
                nested: true,
            })
        );
    }
}
