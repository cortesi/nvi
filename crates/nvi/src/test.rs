//! Utilities for writing tests for Nvi plugins.
use std::{os::unix::process::CommandExt, process::Stdio};

use mrpc;
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

use crate::{connect_unix, error::Result, NviService};

/// A handle to a running test instance.
pub struct TestHandle {
    pub client: crate::Client,
    pub shutdown_tx: broadcast::Sender<()>,
    pub nvim_task: tokio::task::JoinHandle<Result<()>>,
    pub plugin_task: tokio::task::JoinHandle<Result<()>>,
}

impl TestHandle {
    /// Start a neovim instance and plugin in separate tasks. Returns a handle that can be used to control
    /// and monitor the test instance.
    pub async fn new<T>(nvi: T) -> Result<Self>
    where
        T: NviService + Unpin + Sync + 'static,
    {
        let tempdir = tempfile::tempdir()?;
        let socket_path = tempdir.path().join("nvim.socket");

        // Create shutdown channel
        let (shutdown_tx, _) = broadcast::channel(1);

        // Start neovim task
        let sp = socket_path.clone();
        let srx = shutdown_tx.subscribe();
        let nvim_task = tokio::spawn(async move { start_nvim(srx, sp).await });

        // Wait for the socket to appear
        wait_for_path(&socket_path).await?;

        // Start plugin task
        let sp = socket_path.clone();
        let service_shutdown = shutdown_tx.clone();
        let plugin_task = tokio::spawn(connect_unix(service_shutdown, sp, nvi));

        // Connect to nvim and create a client
        let client = mrpc::Client::connect_unix(&socket_path, ()).await?;
        let client = crate::Client::new(client.sender, "test", None, shutdown_tx.clone());

        Ok(Self {
            client,
            shutdown_tx,
            nvim_task,
            plugin_task,
        })
    }

    /// Send termination signal and await all tasks.
    pub async fn finish(self) -> Result<()> {
        let _ = self.shutdown_tx.send(()).unwrap();
        self.nvim_task
            .await
            .map_err(|e| crate::error::Error::Internal {
                msg: format!("nvim task failed: {}", e),
            })??;
        self.plugin_task
            .await
            .map_err(|e| crate::error::Error::Internal {
                msg: format!("plugin task failed: {}", e),
            })??;
        Ok(())
    }
}

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
/// received on the broadcast channel, all tasks are stopped. This variant takes the shutdown
/// signal as an argument, for cases where the caller wants to pass the signal into the plugin
/// itself. This is mostly useful for Nvi's internal tests.
pub async fn run_plugin_with_shutdown<T>(nvi: T, shutdown_tx: broadcast::Sender<()>) -> Result<()>
where
    T: NviService + Unpin + Sync + 'static,
{
    let tempdir = tempfile::tempdir()?;
    let socket_path = tempdir.path().join("nvim.socket");

    let sp = socket_path.clone();
    let srx = shutdown_tx.subscribe();
    let nv = tokio::spawn(async move { start_nvim(srx, sp).await });

    wait_for_path(&socket_path).await?;

    // Start the service
    let sp = socket_path.clone();
    let service_shutdown = shutdown_tx.clone();
    tokio::spawn(async move {
        connect_unix(service_shutdown, sp, nvi).await.unwrap();
    });

    nv.await.unwrap()?;
    Ok(())
}
