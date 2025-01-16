use crate::error::Result;
use crate::highlights;
use crate::highlights::full_name;

use macro_types::{Method, MethodType};

pub enum Formats {
    Markdown,
    Terminal,
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

fn render_text_terminal(
    name: &str,
    docs: &str,
    hl: highlights::Highlights,
    methods: Vec<Method>,
) -> Result<String> {
    use std::io::Write;
    use termcolor::{Buffer, ColorSpec, WriteColor};

    let mut buffer = Buffer::ansi();
    let mut heading_style = ColorSpec::new();
    heading_style
        .set_bold(true)
        .set_fg(Some(termcolor::Color::Blue));

    let mut method_style = ColorSpec::new();
    method_style
        .set_bold(true)
        .set_fg(Some(termcolor::Color::Cyan));

    let mut hl_style = ColorSpec::new();
    hl_style.set_fg(Some(termcolor::Color::Cyan));

    // Title
    buffer.set_color(&heading_style)?;
    writeln!(&mut buffer, "{name}")?;
    writeln!(&mut buffer, "{}", "=".repeat(name.len()))?;
    buffer.reset()?;

    // Docs
    if !docs.is_empty() {
        writeln!(&mut buffer, "\n{docs}")?;
    }

    // Highlights
    if !hl.is_empty() {
        buffer.set_color(&heading_style)?;
        writeln!(&mut buffer, "\nHighlights")?;
        writeln!(&mut buffer, "{}", "-".repeat(10))?;
        buffer.reset()?;

        for (n, hl) in hl.highlights {
            let full_name = full_name(name, &n)?;
            buffer.set_color(&hl_style)?;
            write!(&mut buffer, "\n{}", full_name)?;
            buffer.reset()?;
            write!(&mut buffer, ": {}", hl)?;
        }

        for (n, dst) in hl.links {
            let full_name = full_name(name, &n)?;
            buffer.set_color(&hl_style)?;
            write!(&mut buffer, "\n{}", full_name)?;
            buffer.reset()?;
            write!(&mut buffer, ": {}", dst)?;
        }
    }

    // Methods
    if !methods.is_empty() {
        buffer.set_color(&heading_style)?;
        writeln!(&mut buffer, "\n\nMethods")?;
        writeln!(&mut buffer, "{}", "-".repeat(7))?;
        buffer.reset()?;

        for m in methods {
            match m.method_type {
                MethodType::Request | MethodType::Notify => {
                    buffer.set_color(&method_style)?;
                    writeln!(&mut buffer, "\n{name}.{}", m.name)?;
                    buffer.reset()?;

                    if !m.docs.is_empty() {
                        writeln!(&mut buffer, "\n{}\n", m.docs)?;
                    }
                }
                _ => {}
            }
        }
    }

    String::from_utf8(buffer.into_inner()).map_err(|e| crate::error::Error::Internal {
        msg: format!("Invalid UTF-8 in terminal output: {}", e),
    })
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
        Formats::Terminal => render_text_terminal(name, docs, hl, methods),
    }
}
