//! Utilities for managing Neovim processes.
use std::{os::unix::process::CommandExt, path::PathBuf, process::Stdio};

use nix::{
    sys::signal::{Signal, killpg},
    unistd::Pid,
};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, Command},
    select,
    sync::broadcast,
};

use crate::{
    error::{Error, Result},
    test::wait_for_path,
};

/// Start a headless neovim process, redirecting stdout/stderr and waiting for termination signal.
pub async fn start_nvim_headless(
    mut termrx: broadcast::Receiver<()>,
    socket_path: PathBuf,
    clean: bool,
) -> Result<()> {
    let mut oscmd = std::process::Command::new("nvim");
    oscmd.process_group(0);
    oscmd.arg("--headless");

    if clean {
        oscmd.arg("--clean");
    }

    oscmd
        .arg("--listen")
        .arg(format!("{}", socket_path.to_string_lossy()))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = Command::from(oscmd).spawn()?;
    let pgid = child
        .id()
        .map(|id| Pid::from_raw(id as i32))
        .ok_or_else(|| Error::Internal {
            msg: "Neovim process exited before its process ID was available".to_string(),
        })?;

    let stdout = child.stdout.take().ok_or_else(|| Error::Internal {
        msg: "Neovim process stdout was not captured".to_string(),
    })?;
    let stderr = child.stderr.take().ok_or_else(|| Error::Internal {
        msg: "Neovim process stderr was not captured".to_string(),
    })?;

    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    loop {
        select! {
            _ = termrx.recv() => {
                killpg(pgid, Signal::SIGTERM).map_err(|e| Error::Internal {
                    msg: format!("could not kill process group {e}"),
                })?;
                child.wait().await?;
                return Ok(());
            }
            result = child.wait() => {
                return match result {
                    Ok(status) => Err(Error::Internal {
                        msg: format!("Neovim process exited unexpectedly with status: {status}")
                    }),
                    Err(e) => Err(Error::Internal {
                        msg: format!("Error waiting for Neovim process: {e}")
                    }),
                };
            }
            line = stdout_reader.next_line() => {
                if let Ok(Some(line)) = line {
                    println!("stdout: {line}");
                }
            }
            line = stderr_reader.next_line() => {
                if let Ok(Some(line)) = line {
                    eprintln!("stderr: {line}");
                }
            }
        }
    }
}

/// Start an interactive nvim instance listening on the given socket path
pub async fn start_nvim_cmdline<P>(socket_path: P, clean: bool) -> Result<Child>
where
    P: Into<PathBuf>,
{
    let path = socket_path.into();
    let mut oscmd = std::process::Command::new("nvim");
    if clean {
        oscmd.arg("--clean");
    }
    oscmd
        .arg("--listen")
        .arg(path.to_string_lossy().to_string());

    let child = Command::from(oscmd).spawn()?;
    wait_for_path(&path).await?;
    Ok(child)
}
