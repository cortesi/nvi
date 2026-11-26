use anyhow::Result;

use crate::api;

/// Dumps the Neovim API definition
pub fn dump(raw: bool) -> Result<()> {
    let v = api::get_api()?;
    if raw {
        println!("{v:#?}");
    } else {
        println!("API v{}.{}", v.version.major, v.version.minor);
        println!("Functions:");
        for f in v.functions {
            if f.deprecated_since.is_some() {
                continue;
            }
            println!(
                "\t{}({}) -> {}",
                f.name,
                format_params(&f.parameters),
                f.return_type
            );
        }
        println!("UI Events:");
        for e in v.ui_events {
            println!("\t{}({})", e.name, format_params(&e.parameters));
        }
    };
    Ok(())
}

/// Format parameters as a comma-separated list of `name: type` pairs.
fn format_params(params: &[api::Parameter]) -> String {
    params
        .iter()
        .map(|p| format!("{}: {}", p.1, p.0))
        .collect::<Vec<String>>()
        .join(", ")
}
