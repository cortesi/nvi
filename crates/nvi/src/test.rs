//! Utilities for writing tests for Nvi plugins.
use futures_util::future::BoxFuture;
use std::{os::unix::process::CommandExt, process::Stdio, sync::Mutex, time::Duration};
use tracing::subscriber::DefaultGuard;
use tracing_subscriber::util::SubscriberInitExt;

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

use crate::{
    connect_unix,
    error::Result,
    service::{Status, STATUS_MESSAGE},
    NviPlugin,
};

/// Default timeout for log assertions
const DEFAULT_TEST_TIMEOUT: Duration = Duration::from_secs(5);

/// Builder for NviTest configuration
pub struct NviTestBuilder {
    show_logs: bool,
    log_level: tracing::Level,
}

impl Default for NviTestBuilder {
    fn default() -> Self {
        Self {
            show_logs: false,
            log_level: tracing::Level::TRACE,
        }
    }
}

impl NviTestBuilder {
    /// Enable or disable log output to stdout
    pub fn show_logs(mut self) -> Self {
        self.show_logs = true;
        self
    }

    /// Set the log level for the test
    pub fn log_level(mut self, level: tracing::Level) -> Self {
        self.log_level = level;
        self
    }

    /// Run the test with the configured options. After this method returns, the plugin is
    /// guaranteed to have completed it's connected() method and be in a running state.
    pub async fn run<T>(self, plugin: T) -> Result<NviTest>
    where
        T: NviPlugin + Unpin + Sync + 'static,
    {
        NviTest::new(plugin, self.show_logs, self.log_level).await
    }
}

/// A handle to a running test instance.
pub struct NviTest {
    pub client: crate::Client,
    shutdown_tx: broadcast::Sender<()>,
    nvim_task: tokio::task::JoinHandle<Result<()>>,
    plugin_task: tokio::task::JoinHandle<Result<()>>,
    logs: std::sync::Arc<Mutex<Vec<String>>>,
    _guard: Option<DefaultGuard>,
}

impl NviTest {
    /// Start a neovim instance and plugin in separate tasks. Returns a handle that can be used to control
    /// and monitor the test instance. After this method returns, the plugin is guaranteed to have
    /// completed it's connected() method and be in a running state.
    pub(crate) async fn new<T>(
        plugin: T,
        show_logs: bool,
        log_level: tracing::Level,
    ) -> Result<Self>
    where
        T: NviPlugin + Unpin + Sync + 'static,
    {
        let logs = std::sync::Arc::new(Mutex::new(Vec::new()));
        let logs_clone = (logs.clone(), show_logs);
        let name = plugin.name();

        let guard = tracing_subscriber::fmt()
            .with_max_level(log_level)
            .with_writer(move || {
                let logs = logs_clone.clone();
                struct Writer((std::sync::Arc<Mutex<Vec<String>>>, bool));
                impl std::io::Write for Writer {
                    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                        if let Ok(s) = String::from_utf8(buf.to_vec()) {
                            let mut logs = self.0 .0.lock().unwrap();
                            logs.push(s.clone());
                            if self.0 .1 {
                                print!("{}", s);
                            }
                        }
                        Ok(buf.len())
                    }
                    fn flush(&mut self) -> std::io::Result<()> {
                        Ok(())
                    }
                }
                Writer(logs)
            })
            .with_ansi(true)
            .without_time()
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_target(false)
            .compact()
            .set_default();

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
        let plugin_task = tokio::spawn(connect_unix(service_shutdown, sp, plugin));

        // Connect to nvim and create a client
        let rpc_client = mrpc::Client::connect_unix(&socket_path, ()).await?;
        // Channel ID 0 is the global channel
        let client = crate::Client::new(rpc_client.sender, "test", 0, shutdown_tx.clone());

        let start = std::time::Instant::now();
        loop {
            if start.elapsed() > DEFAULT_TEST_TIMEOUT {
                return Err(crate::error::Error::Internal {
                    msg: format!(
                        "Plugin failed to reach running state after {:?}",
                        DEFAULT_TEST_TIMEOUT
                    ),
                });
            }
            let val = client
                .lua(&format!("return {}.{}()", name, STATUS_MESSAGE))
                .await;
            if let Ok(val) = val {
                if let Some(val) = val.as_str() {
                    if val == Status::Running.to_string() {
                        break;
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }

        Ok(Self {
            client,
            shutdown_tx,
            nvim_task,
            plugin_task,
            logs,
            _guard: Some(guard),
        })
    }

    /// Assert that a log message containing the given string exists
    pub fn assert_log(&self, contains: &str) {
        let logs = self.logs.lock().unwrap();
        assert!(
            logs.iter().any(|log| log.contains(contains)),
            "Log containing '{}' not found in logs: {:?}",
            contains,
            logs
        );
    }

    /// Create a new NviTest builder - the recommended way to create a test instance
    pub fn builder() -> NviTestBuilder {
        NviTestBuilder::default()
    }

    /// Get a copy of the current logs
    pub fn logs(&self) -> Vec<String> {
        self.logs.lock().unwrap().clone()
    }

    /// Execute two closures concurrently, returning the result of the first when it completes and
    /// abandoning the second.
    pub async fn concurrent<T, A, B>(&self, a: A, b: B) -> Result<T>
    where
        T: Send + 'static,
        A: FnOnce(crate::Client) -> BoxFuture<'static, Result<T>> + Send + 'static,
        B: FnOnce(crate::Client) -> BoxFuture<'static, Result<()>> + Send + 'static,
    {
        let client_a = self.client.clone();
        let client_b = self.client.clone();

        let handle_a = tokio::spawn(a(client_a));
        let handle_b = tokio::spawn(b(client_b));

        let result = handle_a.await.map_err(|e| crate::error::Error::Internal {
            msg: format!("task a failed: {}", e),
        })??;

        handle_b.abort();
        Ok(result)
    }

    /// Wait for a log message containing the given string to appear, with a default timeout of 5 seconds
    pub async fn await_log(&self, contains: &str) -> Result<()> {
        self.await_log_timeout(contains, DEFAULT_TEST_TIMEOUT).await
    }

    /// Wait for a log message containing the given string to appear, with a timeout
    pub async fn await_log_timeout(
        &self,
        contains: &str,
        timeout: std::time::Duration,
    ) -> Result<()> {
        let start = std::time::Instant::now();
        while start.elapsed() < timeout {
            if self
                .logs
                .lock()
                .unwrap()
                .iter()
                .any(|log| log.contains(contains))
            {
                return Ok(());
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
        Err(crate::error::Error::Internal {
            msg: format!(
                "Timeout waiting for log containing '{}' after {:?}",
                contains, timeout
            ),
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
    T: NviPlugin + Unpin + Sync + 'static,
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
