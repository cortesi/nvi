use std::path::Path;

use std::process::Stdio;
use tokio::process::Command;

use anyhow::Result;

pub async fn start_nvim(
    socket_path: std::path::PathBuf,
    headless: bool,
    lua: Option<String>,
) -> Result<()> {
    let mut cmd = Command::new("nvim");
    if headless {
        cmd.arg("--headless");
    }
    if let Some(lua) = lua {
        cmd.arg("-l").arg(lua);
    }
    cmd.kill_on_drop(true).arg("--clean").arg("--listen");
    let mut child = cmd
        .arg(format!("{}", socket_path.to_string_lossy()))
        .spawn()?;
    child.wait().await?;
    Ok(())
}

pub async fn build_plugin(path: String) -> Result<()> {
    if !Path::new(&path).join("Cargo.toml").exists() {
        return Err(anyhow::anyhow!("path does not exist"));
    }
    let mut child = Command::new("cargo")
        .current_dir(path)
        .kill_on_drop(true)
        .arg("build")
        .spawn()?;
    child.wait().await?;
    Ok(())
}

pub async fn start_plugin(
    path: String,
    socket_path: std::path::PathBuf,
    trace: bool,
    neovim_headless: bool,
) -> Result<()> {
    if !Path::new(&path).join("Cargo.toml").exists() {
        return Err(anyhow::anyhow!("path does not exist"));
    }

    let mut cmd = Command::new("cargo");
    cmd.current_dir(path)
        .kill_on_drop(true)
        .arg("run")
        .arg("--")
        .arg("connect");

    if !neovim_headless {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }

    if trace {
        cmd.arg("--trace");
    }
    let mut child = cmd
        .arg(format!("{}", socket_path.to_string_lossy()))
        .spawn()?;
    child.wait().await?;
    Ok(())
}

pub async fn run(path: &str, headless: bool, trace: bool, lua: Option<String>) -> Result<()> {
    let tempdir = tempfile::tempdir()?;
    let socket_path = tempdir.path().join("nvic.socket");

    build_plugin(path.to_string()).await?;

    let nvim = start_nvim(socket_path.clone(), headless, lua);

    let plugin = start_plugin(path.to_string(), socket_path, trace, headless);
    tokio::spawn(async move {
        match plugin.await {
            Ok(_) => println!("plugin exited"),
            Err(e) => eprintln!("error: {}", e),
        }
    });

    nvim.await?;

    Ok(())
}
