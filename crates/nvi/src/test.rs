use std::process::Stdio;

use nix::sys::signal::{killpg, Signal};
use nix::unistd::Pid;
use std::os::unix::process::CommandExt;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::select;
use tokio::{process::Command, sync::broadcast};

use crate::{connect_unix, error::Result, NviService};

/// Start a neovim process, and wait for a signal on the broadcast channel to trigger termination.
/// Start a neovim process, and wait for a signal on the broadcast channel to trigger termination.
pub async fn start_nvim(
    mut termrx: broadcast::Receiver<()>,
    socket_path: std::path::PathBuf,
) -> Result<()> {
    let mut oscmd = std::process::Command::new("nvim");
    oscmd
        .process_group(0)
        .arg("--headless")
        .arg("--clean")
        // Toggle this to enable verbose printing from nvim
        //.arg("-V3")
        .arg("--listen")
        .arg(format!("{}", socket_path.to_string_lossy()))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = Command::from(oscmd).spawn()?;
    let pgid = Pid::from_raw(child.id().unwrap() as i32);

    // Set up stdout and stderr readers
    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    loop {
        select! {
            _ = termrx.recv() => {
                // Termination signal received, kill the process group
                killpg(pgid, Signal::SIGTERM).map_err(|e| crate::error::Error::Internal {
                    msg: format!("could not kill process group {}", e),
                })?;
                child.wait().await?;
                return Ok(());
            }
            result = child.wait() => {
                // Child exited before receiving termination signal
                return match result {
                    Ok(status) => Err(crate::error::Error::Internal {
                        msg: format!("Neovim process exited unexpectedly with status: {}", status)
                    }),
                    Err(e) => Err(crate::error::Error::Internal {
                        msg: format!("Error waiting for Neovim process: {}", e)
                    }),
                };
            }
            line = stdout_reader.next_line() => {
                if let Ok(Some(line)) = line {
                    println!("stdout: {}", line);
                }
            }
            line = stderr_reader.next_line() => {
                if let Ok(Some(line)) = line {
                    eprintln!("stderr: {}", line);
                }
            }
        }
    }
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
