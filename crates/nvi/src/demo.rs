//! Functions for managing plugin demo functionality.

use std::process::Command as StdCommand;
use std::time::Duration;

use crate::error::Result;
use crate::test::wait_for_path;
use crate::{client::Client, connect_unix, NviPlugin};
use tokio::process::Command;
use tokio::sync::broadcast;

const TIMEOUT: Duration = Duration::from_secs(5);

/// A function that can be registered with the Demo struct.
pub type DemoFunction = Box<
    dyn Fn(&crate::client::Client) -> futures::future::BoxFuture<'static, crate::error::Result<()>>,
>;

/// Holds a collection of named demo functions that can be executed with a Client.
#[derive(Default)]
pub struct Demos {
    functions: std::collections::HashMap<String, DemoFunction>,
}

/// Start an interactive nvim instance listening on the given socket path
pub async fn start_nvim(
    socket_path: &std::path::Path,
    clean: bool,
) -> Result<tokio::process::Child> {
    let mut oscmd = StdCommand::new("nvim");
    if clean {
        oscmd.arg("--clean");
    }
    oscmd
        .arg("--listen")
        .arg(socket_path.to_string_lossy().to_string());

    let child = Command::from(oscmd).spawn()?;
    wait_for_path(socket_path).await?;
    Ok(child)
}

impl Demos {
    /// Creates a new Demo instance.
    pub fn new() -> Self {
        Self {
            functions: std::collections::HashMap::new(),
        }
    }

    /// Returns an alphabetically sorted list of available demos.
    pub fn list(&self) -> Vec<String> {
        let mut names: Vec<String> = self.functions.keys().cloned().collect();
        names.sort();
        names
    }

    /// Helper to create a demo function that automatically handles client cloning.
    pub fn demo_fn<F, Fut>(
        f: F,
    ) -> impl Fn(&Client) -> futures::future::BoxFuture<'static, Result<()>>
    where
        F: Fn(Client) -> Fut + 'static,
        Fut: futures::Future<Output = Result<()>> + Send + 'static,
    {
        move |client| {
            let client = client.clone();
            Box::pin(f(client))
        }
    }

    /// Adds a named function to the demo collection.
    ///
    /// The function receives a cloned Client instance, so it can be moved into an async block.
    pub fn add<F, Fut>(&mut self, name: impl Into<String>, f: F)
    where
        F: Fn(Client) -> Fut + 'static,
        Fut: futures::Future<Output = Result<()>> + Send + 'static,
    {
        self.functions
            .insert(name.into(), Box::new(Self::demo_fn(f)));
    }

    /// Run a named demo function with a plugin instance.
    ///
    /// This starts an interactive Neovim instance, connects the plugin to it, runs the demo,
    /// and then shuts everything down.
    pub async fn run<T>(&self, demo_name: &str, plugin: T) -> Result<()>
    where
        T: NviPlugin + Send + Sync + Unpin + 'static,
    {
        let tempdir = tempfile::tempdir()?;
        let socket_path = tempdir.path().join("nvim.socket");

        let (shutdown_tx, _) = broadcast::channel(1);
        let neovim_task = start_nvim(&socket_path, true).await?;
        let neovim_handle = tokio::spawn(async move { neovim_task.wait_with_output().await });

        let rpc_client = mrpc::Client::connect_unix(&socket_path, ()).await?;
        let client = crate::Client::new(rpc_client.sender, "demo", 0, shutdown_tx.clone());

        let plugin_shutdown = shutdown_tx.clone();
        let plugin_name = plugin.name();
        let plugin_task = tokio::spawn(connect_unix(plugin_shutdown, socket_path.clone(), plugin));

        let mut demo_result = client.await_plugin(&plugin_name, TIMEOUT).await;

        if demo_result.is_ok() {
            let f = self
                .functions
                .get(demo_name)
                .ok_or_else(|| crate::error::Error::User(format!("no such demo: {}", demo_name)))?;
            demo_result = f(&client).await;
        }

        let (plugin_result, neovim_result) = tokio::join!(plugin_task, neovim_handle);

        let plugin_result = plugin_result
            .map_err(|e| crate::error::Error::Internal {
                msg: format!("plugin task failed: {}", e),
            })
            .and_then(|r| r);

        let _ = neovim_result.map_err(|e| crate::error::Error::Internal {
            msg: format!("neovim task failed: {}", e),
        });

        if let Err(e) = demo_result {
            eprintln!("Demo error: {}", e);
        }
        if let Err(e) = plugin_result {
            eprintln!("Plugin error: {}", e);
        }

        // Reset terminal state
        let _ = StdCommand::new("reset").status();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lua_exec;

    #[tokio::test]
    async fn test_demos() {
        let mut d = Demos::new();
        assert!(d.list().is_empty());

        d.add("two", |c| async move {
            lua_exec!(c, "print('demo two')").await?;
            Ok(())
        });
        d.add("one", |c| async move {
            lua_exec!(c, "print('demo one')").await?;
            Ok(())
        });

        let lst = d.list();
        assert_eq!(lst, vec!["one", "two"]);
    }
}
