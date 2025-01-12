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

/// Start a neovim process, and wait for a signal on the broadcast channel to trigger termination.
///
/// headless: Run neovim in headless mode (--headless)
/// clean: Run neovim in clean mode (--clean)
pub async fn start_nvim(
    mut termrx: broadcast::Receiver<()>,
    socket_path: std::path::PathBuf,
    headless: bool,
    clean: bool,
) -> Result<()> {
    let mut oscmd = std::process::Command::new("nvim");
    oscmd.process_group(0);

    if headless {
        oscmd.arg("--headless");
    }
    if clean {
        oscmd.arg("--clean");
    }

    // Toggle this to enable verbose printing from nvim
    //.arg("-V3")
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
