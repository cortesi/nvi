use std::collections::HashMap;

use tempfile::TempDir;
use tokio::process::Command;
use tokio::sync::broadcast;

use crate::{client::Client, connect_unix, error::Result, NviPlugin};
use mrpc;

/// A function that can be registered with the Demo struct.
pub type DemoFunction = Box<dyn Fn(&Client) -> Result<()>>;

/// Holds a collection of named demo functions that can be executed with a Client.
#[derive(Default)]
pub struct Demos {
    demos: HashMap<String, DemoFunction>,
}

impl Demos {
    fn spawn_nvim(
        &self,
        socket_path: std::path::PathBuf,
        mut shutdown_rx: broadcast::Receiver<()>,
    ) -> tokio::task::JoinHandle<Result<()>> {
        tokio::spawn(async move {
            let mut cmd = Command::new("nvim");
            cmd.kill_on_drop(true)
                .arg("--headless")
                .arg("--clean")
                .arg("--listen")
                .arg(format!("{}", socket_path.to_string_lossy()));
            let mut child = cmd.spawn()?;

            // Wait for socket to appear
            for _ in 0..10 {
                if socket_path.exists() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
            if !socket_path.exists() {
                return Err(crate::error::Error::IO {
                    msg: "socket never appeared".to_string(),
                });
            }

            // Wait for termination signal or process exit
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    let _ = child.kill().await;
                    Ok(())
                }
                status = child.wait() => {
                    if !status?.success() {
                        return Err(crate::error::Error::Internal {
                            msg: "nvim process failed".to_string(),
                        });
                    }
                    Ok(())
                }
            }
        })
    }
    /// Creates a new Demo instance.
    pub fn new() -> Self {
        Demos {
            demos: HashMap::new(),
        }
    }

    /// Adds a named function to the demo collection.
    pub fn add<F>(&mut self, name: impl Into<String>, f: F)
    where
        F: Fn(&Client) -> Result<()> + 'static,
    {
        self.demos.insert(name.into(), Box::new(f));
    }

    /// Run a named demo function with a plugin instance.
    ///
    /// This starts a new Neovim instance, connects the plugin to it, runs the demo,
    /// and then shuts everything down.
    pub async fn run<T>(&self, name: &str, plugin: T) -> Result<()>
    where
        T: NviPlugin + Send + Sync + 'static,
    {
        let tempdir = TempDir::new()?;
        let socket_path = tempdir.path().join("nvi.sock");

        // Get the demo function
        let demo = self
            .demos
            .get(name)
            .ok_or_else(|| crate::error::Error::User(format!("Demo '{}' not found", name)))?;

        // Setup the shutdown channel
        let (shutdown_tx, _) = broadcast::channel(1);

        let nvim_task = self.spawn_nvim(socket_path.clone(), shutdown_tx.subscribe());

        // Start plugin task
        let sp = socket_path.clone();
        let plugin_shutdown = shutdown_tx.clone();
        let plugin_task = tokio::spawn(connect_unix(plugin_shutdown, sp, plugin));

        // Connect demo client
        let rpc_client = mrpc::Client::connect_unix(&socket_path, ()).await?;
        let client = Client::new(rpc_client.sender, "demo", 0, shutdown_tx.clone());

        // Run the demo
        demo(&client)?;

        // Shutdown everything
        let _ = shutdown_tx.send(());

        // Wait for tasks to complete
        nvim_task
            .await
            .map_err(|e| crate::error::Error::Internal {
                msg: format!("nvim task failed: {}", e),
            })??;
        plugin_task
            .await
            .map_err(|e| crate::error::Error::Internal {
                msg: format!("plugin task failed: {}", e),
            })??;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test;
    use crate::NviPlugin;
    use async_trait::async_trait;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Clone)]
    struct TestPlugin;

    #[async_trait]
    impl NviPlugin for TestPlugin {
        fn name(&self) -> String {
            "test".into()
        }

        async fn connected(&mut self, _: &mut Client) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_add_function() {
        let nvit = test::NviTest::new(TestPlugin, false, tracing::Level::ERROR)
            .await
            .unwrap();

        let mut demo = Demos::new();
        let called = Rc::new(RefCell::new(false));
        let called_clone = called.clone();

        demo.add("test", move |_client| {
            *called_clone.borrow_mut() = true;
            Ok(())
        });

        if let Some(f) = demo.demos.get("test") {
            f(&nvit.client).unwrap();
            assert!(*called.borrow());
        } else {
            panic!("Function not found");
        }
    }
}
