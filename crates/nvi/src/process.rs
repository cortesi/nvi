use std::{os::unix::process::CommandExt, process::Stdio};

use nix::{
    sys::signal::{killpg, Signal},
    unistd::Pid,
};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    select,
    sync::broadcast,
};

use crate::error::Result;

/// Start a headless neovim process, redirecting stdout/stderr and waiting for termination signal.
pub async fn start_nvim_headless(
    mut termrx: broadcast::Receiver<()>,
    socket_path: std::path::PathBuf,
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
    let pgid = Pid::from_raw(child.id().unwrap() as i32);

    // Set up stdout and stderr readers
    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    loop {
        select! {
            _ = termrx.recv() => {
                killpg(pgid, Signal::SIGTERM).map_err(|e| crate::error::Error::Internal {
                    msg: format!("could not kill process group {}", e),
                })?;
                child.wait().await?;
                return Ok(());
            }
            result = child.wait() => {
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

/// Start an interactive neovim process by executing nvim directly.
pub fn start_nvim_cmdline(socket_path: std::path::PathBuf, clean: bool) -> Result<()> {
    let mut oscmd = std::process::Command::new("nvim");
    oscmd.process_group(0);

    if clean {
        oscmd.arg("--clean");
    }

    oscmd
        .arg("--listen")
        .arg(format!("{}", socket_path.to_string_lossy()));

    Err(crate::error::Error::Internal {
        msg: format!("Failed to exec nvim: {}", oscmd.exec()),
    })
}
