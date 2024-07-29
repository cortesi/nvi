use nix::sys::signal::{killpg, Signal};
use nix::unistd::Pid;
use std::os::unix::process::CommandExt;
use tokio::{process::Command, sync::broadcast};

use crate::{connect_unix, error::Result, NviService};

/// Start a neovim process, and wait for a signal on the broadcast channel to trigger termination.
pub async fn start_nvim(
    mut termrx: broadcast::Receiver<()>,
    socket_path: std::path::PathBuf,
) -> Result<()> {
    // This entire little dance requires explanation. First, neovim spawns a subprocess, so
    // in order to kill the process, we need to kill the entire process group. Second, tokio's
    // process group functionality is not stabilized yet, so we construct a
    // std::process::Command and convert it into a tokio::process::Command. Finally, we use nix
    // to kill the process group.
    let mut oscmd = std::process::Command::new("nvim");
    oscmd
        .process_group(0)
        .arg("--headless")
        .arg("--clean")
        .arg("--listen")
        .arg(format!("{}", socket_path.to_string_lossy()));
    let mut child = Command::from(oscmd).spawn()?;
    let pgid = Pid::from_raw(child.id().unwrap() as i32);

    let _ = termrx.recv().await;
    killpg(pgid, Signal::SIGTERM).map_err(|e| crate::error::Error::Internal {
        msg: format!("could not kill process group {}", e),
    })?;
    child.wait().await?;
    Ok(())
}

/// Wait a short while for a path to exist. Returns an error after 500ms if the path has not
/// appeared.
pub async fn wait_for_path(path: &std::path::Path) -> Result<()> {
    for _ in 0..10 {
        if path.exists() {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    Err(crate::error::Error::IO {
        msg: "socket never appeared".to_string(),
    })
}

/// Run a test service, starting a neovim headless instance and connecting to it. When a signal is
/// received on the broadcast channel, all tasks are stopped.
pub async fn test_service<T>(nvi: T, shutdown_tx: broadcast::Sender<()>) -> Result<()>
where
    T: NviService + Unpin + Sync + 'static,
{
    let tempdir = tempfile::tempdir()?;
    let socket_path = tempdir.path().join("nvim.socket");

    let sp = socket_path.clone();
    let srx = shutdown_tx.subscribe();
    let nv = tokio::spawn(async move { start_nvim(srx, sp).await });

    wait_for_path(&socket_path).await?;

    let serv = connect_unix(shutdown_tx, socket_path, nvi);
    serv.await?;
    nv.await.unwrap()?;
    Ok(())
}
