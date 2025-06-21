use std::vec;

use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, spanned::Spanned, Expr, ExprLit, Lit, Meta, Token};

use macro_types::*;
use proc_macro2_diagnostics::SpanDiagnosticExt;

type Result<T> = std::result::Result<T, syn::Error>;

const CONNECTED: &str = "connected";
const HIGHLIGHTS: &str = "highlights";

const RPC: &str = "request";
const RPC_NOTIFICATION: &str = "notify";
const RPC_AUTOCMD: &str = "autocmd";
const RPC_AUTOCMD_PATTERNS: &str = "patterns";
const RPC_AUTOCMD_GROUP: &str = "group";
const RPC_AUTOCMD_NESTED: &str = "nested";

#[derive(Debug, Eq, PartialEq)]
struct ImplBlock {
    name: String,
    methods: Vec<Method>,
    run: Option<Method>,
}

/// Output the invocation clause of a connect function
/// Generate the invocation for the connected method implementation
fn connected_invocation(m: &Method, type_name: syn::Ident) -> proc_macro2::TokenStream {
    let method = syn::Ident::new(&m.name, proc_macro2::Span::call_site());
    match m.ret {
        Return::Void => quote! { #type_name::#method(self, client).await; },
        Return::ResultVoid => quote! { #type_name::#method(self, client).await?; },
        _ => panic!("connected method must return Result<()> or be void"),
    }
}

/// Output the invocation clause of a notify function
fn notify_invocation(m: &Method) -> proc_macro2::TokenStream {
    let name = m.name.clone();
    let method = syn::Ident::new(&m.name, proc_macro2::Span::call_site());
    let mut args = vec![];
    for (idx, _) in m.args.iter().enumerate() {
        let a = quote! {
            nvi::serde_rmpv::from_value(&params[#idx])?
        };
        args.push(a);
    }

    let inv = match m.ret {
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
fn request_invocation(m: &Method) -> proc_macro2::TokenStream {
    let name = m.name.clone();
    let method = syn::Ident::new(&m.name, proc_macro2::Span::call_site());
    let mut args = vec![];
    for (idx, _) in m.args.iter().enumerate() {
        let a = quote! {
            nvi::serde_rmpv::from_value(&params[#idx])
                .map_err(|e| nvi::Value::from(format!("{e}")))?
        };
        args.push(a);
    }

    let arg_len = m.args.len();

    let inv = match m.ret {
        Return::Void => {
            quote! {
                    self.#method(client, #(#args),*).await;
                    nvi::Value::Nil
            }
        }
        Return::ResultVoid => {
            quote! {
                    self.#method(client, #(#args),*).await.map_err(|e| nvi::Value::from(format!("{e}")))?;
                    nvi::Value::Nil
            }
        }
        Return::Result(_) => {
            quote! {
                    nvi::serde_rmpv::to_value(
                        &self.#method(client, #(#args),*).await
                            .map_err(|e| nvi::Value::from(format!("{e}")))?
                    ).map_err(|e| nvi::Value::from(format!("{e}")))?
            }
        }
        Return::Type(_) => {
            quote! {
                    nvi::serde_rmpv::to_value(
                        &self.#method(client, #(#args),*).await
                    ).map_err(|e| nvi::Value::from(format!("{e}")))?
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

fn parse_autocmd(a: &syn::Attribute) -> Result<Option<AutoCmd>> {
    if let Meta::List(list) = &a.meta {
        let mut patterns = vec![];
        let mut group = None;
        let mut nested = false;
        let mut events = vec![];

        let nested_metas = list.parse_args_with(Punctuated::<Expr, Token![,]>::parse_terminated)?;
        let mut iter = nested_metas.iter();

        // First argument must be an array of events
        if let Some(Expr::Array(array)) = iter.next() {
            for event in array.elems.iter() {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(lit), ..
                }) = event
                {
                    events.push(lit.value());
                } else {
                    return Err(syn::Error::new(
                        event.span(),
                        "event must be a string literal",
                    ));
                }
            }
        } else {
            return Err(syn::Error::new(
                a.span(),
                "first argument must be an array of event strings",
            ));
        }

        if events.is_empty() {
            return Err(syn::Error::new(
                a.span(),
                "autocmd must specify at least one event",
            ));
        }

        // Parse optional named arguments
        for meta in iter {
            if let Expr::Assign(assign) = meta {
                if let Expr::Path(path) = &*assign.left {
                    let ident = path.path.get_ident().unwrap().to_string();
                    match ident.as_str() {
                        RPC_AUTOCMD_PATTERNS => {
                            if let Expr::Array(array) = &*assign.right {
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
                        RPC_AUTOCMD_GROUP => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(lit), ..
                            }) = &*assign.right
                            {
                                group = Some(lit.value());
                            }
                        }
                        RPC_AUTOCMD_NESTED => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Bool(lit),
                                ..
                            }) = &*assign.right
                            {
                                nested = lit.value();
                            }
                        }
                        _ => return Err(syn::Error::new(meta.span(), "invalid autocmd attribute")),
                    }
                }
            }
        }

        Ok(Some(AutoCmd {
            events,
            patterns,
            group,
            nested,
        }))
    } else {
        Ok(None)
    }
}

fn parse_method(method: &syn::ImplItemFn) -> Result<Option<Method>> {
    let mut method_type = None;
    let mut docs: Vec<String> = vec![];
    let mut autocmd = None;
    let name = method.sig.ident.to_string();

    for a in &method.attrs {
        if a.path().is_ident(RPC) {
            method_type = Some(MethodType::Request);
        } else if a.path().is_ident(RPC_NOTIFICATION) {
            method_type = Some(MethodType::Notify);
        } else if a.path().is_ident(RPC_AUTOCMD) {
            method_type = Some(MethodType::Request);
            autocmd = parse_autocmd(a)?;
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
                _ => return Err(syn::Error::new(method.span(), "invalid doc attribute")),
            }
        }
    }

    if method_type.is_none() {
        if name == CONNECTED {
            method_type = Some(MethodType::Connected);
        } else if name == HIGHLIGHTS {
            method_type = Some(MethodType::Highlights);
        }
    }

    let method_type = if let Some(a) = method_type {
        a
    } else {
        // This is not a command method
        return Ok(None);
    };

    let mut args = vec![];
    let mut is_mut = false;
    for i in &method.sig.inputs {
        match i {
            syn::FnArg::Receiver(r) => {
                is_mut = r.mutability.is_some();
            }
            syn::FnArg::Typed(x) => {
                let mut arg = Arg::default();

                match &*x.pat {
                    syn::Pat::Ident(i) => {
                        arg.name = i.ident.to_string();
                    }
                    syn::Pat::Wild(_) => {
                        arg.name = "_".to_string();
                    }
                    _ => return Err(syn::Error::new(i.span(), "unsupported argument type")),
                }
                match &*x.ty {
                    syn::Type::Path(p) => {
                        arg.typ = p.path.segments.last().unwrap().ident.to_string();
                    }
                    syn::Type::Reference(p) => {
                        arg.typ = p.to_token_stream().to_string();
                    }
                    _ => return Err(syn::Error::new(i.span(), "unsupported argument type")),
                }
                args.push(arg);
            }
        }
    }

    // The highlights method is special and takes no Client argument
    if name != HIGHLIGHTS {
        if let Some(first) = args.first() {
            if !first.typ.ends_with("Client") {
                return Err(syn::Error::new(
                    method.span(),
                    "first argument must be `Client`",
                ));
            }
        } else {
            return Err(syn::Error::new(
                method.span(),
                "no arguments on command method",
            ));
        }
        args = args.into_iter().skip(1).collect::<Vec<_>>();
    }

    let ret = match &method.sig.output {
        syn::ReturnType::Default => Return::Void,
        syn::ReturnType::Type(_, ty) => match &**ty {
            syn::Type::Path(p) => {
                if p.path.segments.last().unwrap().ident == "Result" {
                    match &p.path.segments.last().unwrap().arguments {
                        syn::PathArguments::AngleBracketed(a) => {
                            if a.args.len() != 1 {
                                return Err(syn::Error::new(method.span(), "invalid rpc method"));
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
                                    _ => {
                                        return Err(syn::Error::new(
                                            method.span(),
                                            "invalid rpc method",
                                        ))
                                    }
                                }
                            }
                        }
                        _ => return Err(syn::Error::new(method.span(), "invalid rpc method")),
                    }
                } else {
                    Return::Type(p.to_token_stream().to_string())
                }
            }
            _ => return Err(syn::Error::new(method.span(), "invalid rpc method")),
        },
    };

    if method_type == MethodType::Notify && !(ret == Return::ResultVoid || ret == Return::Void) {
        return Err(syn::Error::new(
            method.span(),
            "notification methods must return Result<()> or be void",
        ));
    }

    // Validate autocmd arguments
    if autocmd.is_some() && !args.is_empty() {
        if args.len() != 1 {
            return Err(syn::Error::new(
                method.span(),
                "autocmd must take exactly one argument of type AutocmdEvent",
            ));
        }
        let arg = &args[0];
        if arg.typ != "AutocmdEvent" && arg.typ != "nvi::AutocmdEvent" {
            return Err(syn::Error::new(
                method.span(),
                "autocmd argument must be of type AutocmdEvent",
            ));
        }
    }

    match method_type {
        MethodType::Connected => {
            if !args.is_empty() {
                return Err(syn::Error::new(
                    method.span(),
                    "connected method must take only a Client argument",
                ));
            }
            if !(ret == Return::ResultVoid || ret == Return::Void) {
                return Err(syn::Error::new(
                    method.span(),
                    "connected method must return Result<()> or be void",
                ));
            }
        }
        MethodType::Highlights => {
            if !args.is_empty() {
                return Err(syn::Error::new(
                    method.span(),
                    "highlights method must not take any arguments",
                ));
            }
            match &ret {
                Return::Result(s) if s.ends_with("Highlights") => (),
                Return::Type(s) if s.ends_with("Highlights") => (),
                _ => {
                    return Err(syn::Error::new(
                        method.span(),
                        "highlights method must return either Highlights or Result<Highlights>",
                    ))
                }
            }
        }
        _ => {}
    }

    Ok(Some(Method {
        name,
        method_type,
        ret,
        args,
        docs: docs.join("\n"),
        autocmd,
        is_mut,
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
fn generate_methods(imp: &ImplBlock) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    imp.methods.iter().map(|m| {
        let name = &m.name;
        let docs = &m.docs;
        let is_mut = m.is_mut;
        let method_type = match m.method_type {
            MethodType::Request => quote! { nvi::macro_types::MethodType::Request },
            MethodType::Notify => quote! { nvi::macro_types::MethodType::Notify },
            MethodType::Connected => quote! { nvi::macro_types::MethodType::Connected },
            MethodType::Highlights => quote! { nvi::macro_types::MethodType::Highlights },
        };

        let ret = match &m.ret {
            Return::Void => quote! { nvi::macro_types::Return::Void },
            Return::ResultVoid => quote! { nvi::macro_types::Return::ResultVoid },
            Return::Result(typ) => quote! { nvi::macro_types::Return::Result(#typ.to_string()) },
            Return::Type(typ) => quote! { nvi::macro_types::Return::Type(#typ.to_string()) },
        };

        let args = m.args.iter().map(|a| {
            let name = &a.name;
            let typ = &a.typ;
            quote! {
                nvi::macro_types::Arg {
                    name: #name.to_string(),
                    typ: #typ.to_string(),
                }
            }
        });

        let autocmd = if let Some(a) = &m.autocmd {
            let events = &a.events;
            let patterns = &a.patterns;
            let group = match &a.group {
                Some(g) => quote! { Some(#g.to_string()) },
                None => quote! { None },
            };
            let nested = a.nested;
            quote! {
                Some(nvi::macro_types::AutoCmd {
                    events: vec![#(#events.to_string()),*],
                    patterns: vec![#(#patterns.to_string()),*],
                    group: #group,
                    nested: #nested,
                })
            }
        } else {
            quote! { None }
        };

        quote! {
            nvi::macro_types::Method {
                name: #name.to_string(),
                docs: #docs.to_string(),
                ret: #ret,
                method_type: #method_type,
                args: vec![#(#args),*],
                autocmd: #autocmd,
                is_mut: #is_mut,
            }
        }
    })
}

fn inner_nvi_plugin(
    _attr: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream> {
    // First parse the input as an impl block to verify basic syntax
    let impl_block = syn::parse2::<syn::ItemImpl>(input.clone())
        .map_err(|e| input.span().error(format!("Invalid impl block: {e}")))?;

    // Verify this is implementing a struct or enum
    let impl_type = match &*impl_block.self_ty {
        syn::Type::Path(p) => p,
        _ => {
            return Err(syn::Error::new(
                impl_block.self_ty.span(),
                "Expected a struct or enum name",
            ))
        }
    };

    // Get the name of the type being implemented
    let type_name = impl_type
        .path
        .segments
        .last()
        .ok_or_else(|| impl_block.self_ty.span().error("Invalid type name"))?
        .ident
        .to_string();

    let (impl_block, imp) = parse_impl(&input)?;

    // Collect impl block doc comments
    let mut docs = String::new();
    for attr in &impl_block.attrs {
        if attr.path().is_ident("doc") {
            if let Meta::NameValue(meta) = &attr.meta {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(s), ..
                }) = &meta.value
                {
                    if !docs.is_empty() {
                        docs.push('\n');
                    }
                    docs.push_str(s.value().trim());
                }
            }
        }
    }
    let docs = docs; // Make docs immutable

    let request_invocations: Vec<proc_macro2::TokenStream> = imp
        .methods
        .iter()
        .filter(|x| x.method_type == MethodType::Request)
        .filter(|x| !x.is_mut)
        .map(request_invocation)
        .collect();

    let request_invocations_mut: Vec<proc_macro2::TokenStream> = imp
        .methods
        .iter()
        .filter(|x| x.method_type == MethodType::Request)
        .filter(|x| x.is_mut)
        .map(request_invocation)
        .collect();

    let notify_invocations: Vec<proc_macro2::TokenStream> = imp
        .methods
        .iter()
        .filter(|x| x.method_type == MethodType::Notify)
        .filter(|x| !x.is_mut)
        .map(notify_invocation)
        .collect();

    let notify_invocations_mut: Vec<proc_macro2::TokenStream> = imp
        .methods
        .iter()
        .filter(|x| x.method_type == MethodType::Notify)
        .filter(|x| x.is_mut)
        .map(notify_invocation)
        .collect();

    let name = syn::Ident::new(&type_name, proc_macro2::Span::call_site());
    let namestr = heck::ToSnakeCase::to_snake_case(type_name.as_str());
    let docs = &docs;

    let methods = generate_methods(&imp);
    // Handle the connected method
    let connected = imp
        .methods
        .iter()
        .find(|x| x.method_type == MethodType::Connected)
        .map(|x| connected_invocation(x, name.clone()))
        .unwrap_or_else(|| quote! {});

    // Handle the highlights method
    let highlights = if let Some(m) = imp
        .methods
        .iter()
        .find(|x| x.method_type == MethodType::Highlights)
    {
        // User can return either Highlights or Result<Highlights>
        match &m.ret {
            Return::Result(_) => quote! { self.highlights() },
            Return::Type(_) => quote! { Ok(self.highlights()) },
            _ => unreachable!("highlights validation should prevent this"),
        }
    } else {
        // Default implementation
        quote! { Ok(nvi::highlights::Highlights::default()) }
    };

    // Verify we have at least one method
    if imp.methods.is_empty() {
        return Err(syn::Error::new(
            input.span(),
            "No RPC methods found in the implementation. Use #[request], #[notify], or #[autocmd] to mark RPC methods"
        ));
    }

    Ok(quote! {
        #impl_block

        #[nvi::async_trait::async_trait]
        impl nvi::NviPlugin for #name {
            fn name(&self) -> String {
                #namestr.into()
            }

            fn highlights(&self) -> nvi::error::Result<nvi::highlights::Highlights> {
                #highlights
            }

            async fn connected(&mut self, client: &mut nvi::Client) -> nvi::error::Result<()> {
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
                            nvi::error::Result::Err(nvi::Value::from(format!("Unknown method: {method}")))?
                        }
                    }
                )
            }

            async fn request_mut(
                &mut self,
                client: &mut nvi::Client,
                method: &str,
                params: &[nvi::Value],
            ) -> nvi::error::Result<nvi::Value, nvi::Value> {
                Ok(
                    match method {
                        #(#request_invocations_mut),*
                        _ => {
                            nvi::error::Result::Err(nvi::Value::from(format!("Unknown method: {method}")))?
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
                        Err(nvi::error::Error::Internal{ msg: format!("Unknown notification: {method}") })?
                    }
                }
                Ok(())
            }

            async fn notify_mut(
                &mut self,
                client: &mut nvi::Client,
                method: &str,
                params: &[nvi::Value],
            ) -> nvi::error::Result<()> {
                match method {
                    #(#notify_invocations_mut),*
                    _ => {
                        Err(nvi::error::Error::Internal{ msg: format!("Unknown notification: {method}") })?
                    }
                }
                Ok(())
            }

            #[inline]
            fn inspect(&self) -> Vec<nvi::macro_types::Method> {
                vec![#(#methods),*]
            }

            fn docs(&self) -> nvi::error::Result<String> {
                Ok(#docs.into())
            }
        }
    }
    .to_token_stream())
}

/// Add this attribute to the *impl* block for the `NviPlugin` trait to derive implementations for
/// the `message` and `notification` methods.
#[proc_macro_attribute]
pub fn nvi_plugin(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input_span = proc_macro2::TokenStream::from(input.clone());
    match inner_nvi_plugin(_attr.into(), input_span.clone()) {
        Ok(x) => x.into(),
        Err(e) => e.into_compile_error().into(),
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

/// Mark a method as an AutoCommand.
#[proc_macro_attribute]
pub fn autocmd(
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
    fn test_doc_collection() {
        let s = quote! {
            /// First line of doc
            /// Second line of doc
            impl Test {
                #[request]
                async fn test(&self, client: &mut nvi::Client) -> nvi::error::Result<()> {
                    Ok(())
                }
            }
        };

        let result = inner_nvi_plugin(quote! {}, s).unwrap();
        let result_str = result.to_string();
        assert!(result_str.contains("First line of doc\\nSecond line of doc"));
    }

    #[test]
    fn it_handles_special_methods() {
        // Test struct with highlights and connected methods
        let s = quote! {
            impl Test {
                fn connected(&mut self, client: &mut nvi::Client) -> Result<()> {
                    Ok(())
                }

                fn highlights(&self) -> Result<nvi::highlights::Highlights> {
                    Ok(nvi::highlights::Highlights::default())
                }
            }
        };

        let (_, ret) = parse_impl(&s).unwrap();
        let connected = ret
            .methods
            .iter()
            .filter(|m| m.method_type == MethodType::Connected)
            .count();
        let highlights = ret
            .methods
            .iter()
            .filter(|m| m.method_type == MethodType::Highlights)
            .count();

        assert_eq!(connected, 1, "Should have one connected method");
        assert_eq!(highlights, 1, "Should have one highlights method");
    }

    #[test]
    fn it_parses_struct() {
        let s = quote! {
            impl <T>TestPlugin for Test<T> {
                #[request]
                /// Some docs
                fn test_method(&self, client: &mut nvi::Client, a: i32, b: String, c: &str, d: foo::bar::Voing) -> Result<String> {
                    Ok(format!("{a}:{b}"))
                }
                #[request]
                fn test_void(&mut self, client: &mut nvi::Client) {}
                #[request]
                fn test_usize(&self, client: &mut nvi::Client) -> usize {}
                #[request]
                fn test_resultvoid(&self, client: &mut nvi::Client) -> Result<()> {}
                #[notify]
                fn test_notification(&mut self, client: &mut nvi::Client) -> Result<()> {}

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
                    method_type: MethodType::Request,
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
                    is_mut: false,
                },
                Method {
                    name: "test_void".into(),
                    docs: "".into(),
                    ret: Return::Void,
                    method_type: MethodType::Request,
                    args: vec![],
                    autocmd: None,
                    is_mut: true,
                },
                Method {
                    name: "test_usize".into(),
                    docs: "".into(),
                    ret: Return::Type("usize".into()),
                    method_type: MethodType::Request,
                    args: vec![],
                    autocmd: None,
                    is_mut: false,
                },
                Method {
                    name: "test_resultvoid".into(),
                    docs: "".into(),
                    ret: Return::ResultVoid,
                    method_type: MethodType::Request,
                    args: vec![],
                    autocmd: None,
                    is_mut: false,
                },
                Method {
                    name: "test_notification".into(),
                    docs: "".into(),
                    ret: Return::ResultVoid,
                    method_type: MethodType::Notify,
                    args: vec![],
                    autocmd: None,
                    is_mut: true,
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
        assert!(inner_nvi_plugin(quote! {}, s).is_err());
    }

    #[test]
    fn it_validates_highlights_method() {
        // Valid direct return type
        let s = quote! {
            impl Test {
                fn highlights(&self) -> nvi::highlights::Highlights {
                    nvi::highlights::Highlights::default()
                }
            }
        };
        assert!(
            parse_impl(&s).is_ok(),
            "Should accept direct Highlights return type"
        );

        // Valid Result return type
        let s = quote! {
            impl Test {
                fn highlights(&self) -> Result<nvi::highlights::Highlights> {
                    Ok(nvi::highlights::Highlights::default())
                }
            }
        };
        assert!(
            parse_impl(&s).is_ok(),
            "Should accept Result<Highlights> return type"
        );

        // Invalid direct return type
        let s = quote! {
            impl Test {
                fn highlights(&self) -> String {
                    "invalid".into()
                }
            }
        };
        assert!(
            parse_impl(&s).is_err(),
            "Should reject invalid direct return type"
        );

        // Invalid Result type
        let s = quote! {
            impl Test {
                fn highlights(&self) -> Result<String> {
                    Ok("invalid".into())
                }
            }
        };
        assert!(parse_impl(&s).is_err(), "Should reject invalid Result type");

        // Invalid arguments
        let s = quote! {
            impl Test {
                fn highlights(&self, extra: String) -> Result<nvi::highlights::Highlights> {
                    Ok(nvi::highlights::Highlights::default())
                }
            }
        };
        assert!(parse_impl(&s).is_err(), "Should reject arguments");
    }

    #[test]
    fn it_renders_service() {
        let s = quote! {
            impl <T>TestPlugin for Test<T> {
                #[request]
                /// Some docs
                async fn test_method(&self, client: &mut nvi::Client, a: i32, b: String, c: &str, d: foo::bar::Voing) -> Result<String> {
                    Ok(format!("{a}:{b}"))
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
                .format_tokens(inner_nvi_plugin(quote! {}, s).unwrap())
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
                    Ok(format!("{a}:{b}"))
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
                .format_tokens(inner_nvi_plugin(quote! {}, s).unwrap())
                .unwrap()
        );
    }

    #[test]
    fn test_autocmd_validation() {
        // Test valid autocmd with AutocmdEvent
        let s = quote! {
            impl Test {
                #[autocmd(["BufEnter", "BufLeave"])]
                async fn test_autocmd(&self, client: &mut nvi::Client, event: AutocmdEvent) -> nvi::error::Result<()> {
                    Ok(())
                }
            }
        };
        assert!(parse_impl(&s).is_ok());

        // Test valid autocmd with nvi::AutocmdEvent
        let s = quote! {
            impl Test {
                #[autocmd(["BufEnter", "BufLeave"])]
                async fn test_autocmd(&self, client: &mut nvi::Client, event: nvi::AutocmdEvent) -> nvi::error::Result<()> {
                    Ok(())
                }
            }
        };
        assert!(parse_impl(&s).is_ok());

        // Test invalid autocmd with wrong type
        let s = quote! {
            impl Test {
                #[autocmd(["BufEnter", "BufLeave"])]
                async fn test_autocmd(&self, client: &mut nvi::Client, event: String) -> nvi::error::Result<()> {
                    Ok(())
                }
            }
        };
        assert!(parse_impl(&s).is_err());

        // Test invalid autocmd with multiple args
        let s = quote! {
            impl Test {
                #[autocmd(["BufEnter", "BufLeave"])]
                async fn test_autocmd(&self, client: &mut nvi::Client, event: AutocmdEvent, other: String) -> nvi::error::Result<()> {
                    Ok(())
                }
            }
        };
        assert!(parse_impl(&s).is_err());
    }

    #[test]
    fn it_parses_autocmd() {
        let s = quote! {
            impl Test {
                #[autocmd(["BufEnter", "BufLeave"], patterns=["*.rs"], group="test", nested=true)]
                async fn test_autocmd(&self, client: &mut nvi::Client, event: AutocmdEvent) -> nvi::error::Result<()> {
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

    #[test]
    fn it_parses_autocmd_without_options() {
        let s = quote! {
            impl Test {
                #[autocmd(["BufEnter", "BufLeave"])]
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
                patterns: vec![],
                group: None,
                nested: false,
            })
        );
    }

    #[test]
    fn it_parses_autocmd_with_patterns() {
        let s = quote! {
            impl Test {
                #[autocmd(["BufEnter", "BufLeave"], patterns=["*.rs"])]
                async fn test_autocmd(&self, client: &mut nvi::Client, event: AutocmdEvent) -> nvi::error::Result<()> {
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
                group: None,
                nested: false,
            })
        );
    }

    #[test]
    fn it_parses_autocmd_with_group() {
        let s = quote! {
            impl Test {
                #[autocmd(["BufEnter", "BufLeave"], group="test")]
                async fn test_autocmd(&self, client: &mut nvi::Client, event: AutocmdEvent) -> nvi::error::Result<()> {
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
                patterns: vec![],
                group: Some("test".into()),
                nested: false,
            })
        );
    }

    #[test]
    fn it_parses_autocmd_with_nested() {
        let s = quote! {
            impl Test {
                #[autocmd(["BufEnter", "BufLeave"], nested=true)]
                async fn test_autocmd(&self, client: &mut nvi::Client, event: AutocmdEvent) -> nvi::error::Result<()> {
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
                patterns: vec![],
                group: None,
                nested: true,
            })
        );
    }

    #[test]
    fn it_handles_mutability() {
        let s = quote! {
            impl Test {
                #[request]
                async fn immut_method(&self, client: &mut nvi::Client) -> nvi::error::Result<()> {
                    Ok(())
                }

                #[request]
                async fn mut_method(&mut self, client: &mut nvi::Client) -> nvi::error::Result<()> {
                    Ok(())
                }
            }
        };

        let (_, ret) = parse_impl(&s).unwrap();
        assert_eq!(
            ret.methods[0].is_mut, false,
            "immut_method should not be mut"
        );
        assert_eq!(ret.methods[1].is_mut, true, "mut_method should be mut");
    }
}
