use std::path::Path;

use tokio::{process::Command, sync::broadcast};

use anyhow::Result;

pub async fn start_nvim(socket_path: std::path::PathBuf) -> Result<()> {
    let mut child = Command::new("nvim")
        .kill_on_drop(true)
        .arg("--clean")
        .arg("--listen")
        .arg(format!("{}", socket_path.to_string_lossy()))
        .spawn()?;
    child.wait().await?;
    Ok(())
}

pub async fn start_plugin(path: String, socket_path: std::path::PathBuf) -> Result<()> {
    if !Path::new(&path).join("Cargo.toml").exists() {
        return Err(anyhow::anyhow!("path does not exist"));
    }
    let mut child = Command::new("cargo")
        .current_dir(path)
        .kill_on_drop(true)
        .arg("run")
        .arg("--")
        .arg("connect")
        .arg(format!("{}", socket_path.to_string_lossy()))
        .spawn()?;
    child.wait().await?;
    Ok(())
}

pub async fn run(path: &str) -> Result<()> {
    let tempdir = tempfile::tempdir()?;
    let socket_path = tempdir.path().join("nvic.socket");

    let nvim = start_nvim(socket_path.clone());

    let plugin = start_plugin(path.to_string(), socket_path);
    tokio::spawn(async move {
        match plugin.await {
            Ok(_) => (),
            Err(e) => eprintln!("error: {}", e),
        }
    });

    nvim.await?;

    Ok(())
}
