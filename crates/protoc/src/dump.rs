use crate::api;
use anyhow::Result;

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
            print!("\t{}(", f.name);
            let args = f
                .parameters
                .iter()
                .map(|p| format!("{}: {}", p.1, p.0))
                .collect::<Vec<String>>()
                .join(", ");
            print!("{args})");
            print!(") -> {}", f.return_type);
            println!();
        }
        println!("UI Events:");
        for e in v.ui_events {
            let params = e
                .parameters
                .iter()
                .map(|p| format!("{}: {}", p.1, p.0))
                .collect::<Vec<String>>()
                .join(", ");
            println!("\t{}({})", e.name, params);
        }
    };
    Ok(())
}
