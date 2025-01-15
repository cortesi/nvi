use crate::error::Result;
use crate::highlights;
use crate::highlights::full_name;

use macro_types::{Method, MethodType};

pub enum Formats {
    Markdown,
}

fn render_text_markdown(
    name: &str,
    docs: &str,
    hl: highlights::Highlights,
    methods: Vec<Method>,
) -> Result<String> {
    let mut ret = format!("# {name}\n");
    if !docs.is_empty() {
        ret.push_str(&format!("\n{docs}\n"));
    }
    if !hl.is_empty() {
        ret.push_str("\n## Highlights\n\n");
        for (n, hl) in hl.highlights {
            ret.push_str(&format!("* {}: {}\n", full_name(name, &n)?, hl));
        }
        for (n, dst) in hl.links {
            ret.push_str(&format!("* {}: {}\n", full_name(name, &n)?, dst));
        }
    }
    if !methods.is_empty() {
        ret.push_str("\n## Methods\n\n");
        for m in methods {
            match m.method_type {
                MethodType::Request | MethodType::Notify => {
                    ret.push_str(&format!("### {name}.{}\n\n", m.name));
                    if !m.docs.is_empty() {
                        ret.push_str(&format!("{docs}\n\n", docs = m.docs));
                    }
                }
                _ => {}
            }
        }
    }

    Ok(textwrap::dedent(&ret))
}

pub fn render_docs(
    fmt: Formats,
    name: &str,
    docs: &str,
    hl: highlights::Highlights,
    methods: Vec<Method>,
) -> Result<String> {
    match fmt {
        Formats::Markdown => render_text_markdown(name, docs, hl, methods),
    }
}
