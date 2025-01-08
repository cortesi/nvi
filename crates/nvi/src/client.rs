//! A Neovim client. This is the primary interface for interacting with Neovim from a service.
use std::collections::HashMap;
use std::path::PathBuf;

use tokio::sync::broadcast;
use tracing::trace;

use crate::{
    error::{Error, Result},
    lua, lua_exec, nvim,
};

/// A client to Neovim. A `Client` object is passed to every method invocation in a `NviService`.
/// It exposes the full auto-generated API for Neovim on its `nvim` field, and provides a set of
/// higher-level methods directly on the `Client` object.
#[derive(Clone, Debug)]
pub struct Client {
    /// The name of the plugin.
    pub name: String,
    /// The compiled API for Neovim.
    pub nvim: nvim::api::NvimApi,
    /// The MessagePack-RPC channel ID for this client. Channel ID 0 is global.
    pub channel_id: u64,

    shutdown_tx: broadcast::Sender<()>,
}

impl Client {
    pub fn new(
        rpc_sender: mrpc::RpcSender,
        name: &str,
        channel_id: u64,
        shutdown_tx: broadcast::Sender<()>,
    ) -> Self {
        Client {
            name: name.into(),
            nvim: nvim::api::NvimApi { rpc_sender },
            shutdown_tx,
            channel_id,
        }
    }

    /// Get the current working directory from Neovim.
    pub async fn getcwd(&self) -> Result<PathBuf> {
        lua!(self, "return vim.fn.getcwd()",).await
    }

    /// Register an RPC method in Neovim. This creates a Lua function under the specified namespace
    /// that will send an RPC message back to this client when called. The `kind` parameter
    /// specifies whether this is a request or notification method.
    async fn register_method<P>(
        &mut self,
        kind: &str,
        namespace: &str,
        method: &str,
        params: &[P],
    ) -> Result<()>
    where
        P: std::string::ToString,
    {
        let arg_list = params
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let extra_sep = if !arg_list.is_empty() { ", " } else { "" };
        trace!(
            "nvi registering {}: {} {} {}",
            kind,
            namespace,
            method,
            arg_list
        );
        let channel_id = self.channel_id;

        lua_exec!(
            self,
            &format!(
                "
                    if not _G.{namespace} then
                        _G.{namespace} = {{}}
                    end
                    _G.{namespace}.{method} = function({arg_list})
                        return vim.{kind}({channel_id}, '{method}'{extra_sep} {arg_list})
                    end
                "
            ),
        )
        .await?;
        Ok(())
    }

    /// Register an RPC request method for use in Neovim. This sets a globally-avaialable Lua
    /// function under the specified namespace. When this function is called, an RPC message is
    /// sent back to the current addon.
    ///
    /// # Example
    ///
    /// client.register_rpcrequest("test_module", "test_fn", &["arg1", "arg2"]).await.unwrap();
    ///
    /// After this call, the following Lua function will be available in Neovim:
    ///
    /// test_module.test_fn(arg1, arg2)
    ///
    /// Which can be invoked from Lua like so:
    ///
    /// test_module.test_fn("value", 3)
    pub async fn register_rpcrequest<P>(
        &mut self,
        namespace: &str,
        method: &str,
        params: &[P],
    ) -> Result<()>
    where
        P: std::string::ToString,
    {
        self.register_method("rpcrequest", namespace, method, params)
            .await
    }

    /// Register an RPC notification method for use in Neovim. This sets a globally-avaialable Lua
    /// function under the specified namespace. When this function is called, an RPC message is
    /// sent back to the current addon.
    ///
    /// # Example
    ///
    /// client.register_rpcnotify("test_module", "test_fn", &["arg1", "arg2"]).await.unwrap();
    ///
    /// After this call, the following Lua function will be available in Neovim:
    ///
    /// test_module.test_fn(arg1, arg2)
    ///
    /// Which can be invoked from Lua like so:
    ///
    /// test_module.test_fn("value", 3)
    pub async fn register_rpcnotify<P>(
        &mut self,
        namespace: &str,
        method: &str,
        params: &[P],
    ) -> Result<()>
    where
        P: std::string::ToString,
    {
        self.register_method("rpcnotify", namespace, method, params)
            .await
    }

    /// Shutdown the service, causing it to exit cleanly and disconnect from Neovim.
    pub fn shutdown(&self) {
        trace!("shutdown request from client");
        let _ = self.shutdown_tx.send(());
    }

    /// Send an nvim_notify notification, with a specified log level.
    pub async fn notify(&self, level: nvim::types::LogLevel, msg: &str) -> Result<()> {
        self.nvim.notify(msg, level.to_u64(), HashMap::new()).await
    }

    /// Send an nvim_notify notification with a log level of `LogLevel::Trace`.
    pub async fn trace(&self, msg: &str) -> Result<()> {
        self.notify(nvim::types::LogLevel::Trace, msg).await
    }

    /// Send an nvim_notify notification with a log level of `LogLevel::Debug`.
    pub async fn debug(&self, msg: &str) -> Result<()> {
        self.notify(nvim::types::LogLevel::Debug, msg).await
    }

    /// Send an nvim_notify notification with a log level of `LogLevel::Info`.
    pub async fn info(&self, msg: &str) -> Result<()> {
        self.notify(nvim::types::LogLevel::Info, msg).await
    }

    /// Send an nvim_notify notification with a log level of `LogLevel::Warn`.
    pub async fn warn(&self, msg: &str) -> Result<()> {
        self.notify(nvim::types::LogLevel::Warn, msg).await
    }

    /// Send an nvim_notify notification with a log level of `LogLevel::Error`.
    pub async fn error(&self, msg: &str) -> Result<()> {
        self.notify(nvim::types::LogLevel::Error, msg).await
    }

    /// Set an autocmd for a buffer.
    pub async fn autocmd_buffer(
        &self,
        buffer: nvim::types::Buffer,
        rpc_request: &str,
        events: &[nvim::types::Event],
        group: Option<nvim::types::Group>,
        once: bool,
        nested: bool,
    ) -> Result<u64> {
        // Vim autocommands can return values through several mechanisms:
        //
        // 1. Direct callback returns in Lua (nvim_buf_attach, nvim_create_autocmd) to control
        //    autocommand lifecycle
        // 2. Special variables in the input object that get modified (v:swapchoice, v:fcs_choice,
        //    v:event.abort) to control Vim behavior
        // 3. Buffer modifications and mark changes in *Cmd events (BufReadCmd, FileWriteCmd, etc.)
        // 4. Event data modifications (CompleteChanged event)
        //
        // Modifying variables in the input object specifically is not covered by the current API.

        if events.is_empty() {
            return Err(Error::Internal {
                msg: "events must not be empty".into(),
            });
        }
        let events = events
            .iter()
            .map(|e| format!("\"{}\"", e))
            .collect::<Vec<String>>()
            .join(", ");

        let group = if let Some(g) = group {
            let arg = g.to_lua_arg();
            format!("group = {arg},")
        } else {
            "".to_string()
        };
        let bufno: u64 = buffer.into();

        let namespace = &self.name;
        // We execute a Lua function here because we need to specify a callback function for the
        // rpcrequest. At the moment, we can't specify callbacks through the msgpack-rpc API.
        let ret: u64 = self
            .nvim
            .exec_lua(
                &format!(
                    r#"
                        return vim.api.nvim_create_autocmd(
                            {{ {events} }},
                            {{
                                buffer = {bufno},
                                once = {once},
                                nested = {nested},
                                {group}
                                callback = function(ev)
                                  {namespace}.{rpc_request}(ev)
                                end
                            }}
                        )
                    "#,
                ),
                vec![],
            )
            .await?;
        Ok(ret)
    }

    /// Set an autocmd for a set of patterns.
    pub async fn autocmd_pattern(
        &self,
        patterns: &[String],
        rpc_request: &str,
        events: &[nvim::types::Event],
        group: Option<nvim::types::Group>,
        once: bool,
        nested: bool,
    ) -> Result<u64> {
        // Vim autocommands can return values through several mechanisms:
        //
        // 1. Direct callback returns in Lua (nvim_buf_attach, nvim_create_autocmd) to control
        //    autocommand lifecycle
        // 2. Special variables in the input object that get modified (v:swapchoice, v:fcs_choice,
        //    v:event.abort) to control Vim behavior
        // 3. Buffer modifications and mark changes in *Cmd events (BufReadCmd, FileWriteCmd, etc.)
        // 4. Event data modifications (CompleteChanged event)
        //
        // Modifying variables in the input object specifically is not covered by the current API.

        if events.is_empty() {
            return Err(Error::Internal {
                msg: "events must not be empty".into(),
            });
        }
        let events = events
            .iter()
            .map(|e| format!("\"{}\"", e))
            .collect::<Vec<String>>()
            .join(", ");

        let patterns = patterns
            .iter()
            .map(|p| format!("\"{}\"", p))
            .collect::<Vec<String>>()
            .join(", ");

        let group = if let Some(g) = group {
            let arg = g.to_lua_arg();
            format!("group = {arg},")
        } else {
            "".to_string()
        };

        let namespace = &self.name;
        // We execute a Lua function here because we need to specify a callback function for the
        // rpcrequest. At the moment, we can't specify callbacks through the msgpack-rpc API.
        let ret: u64 = self
            .nvim
            .exec_lua(
                &format!(
                    r#"
                        return vim.api.nvim_create_autocmd(
                            {{ {events} }},
                            {{
                                pattern = {{ {patterns} }},
                                once = {once},
                                nested = {nested},
                                {group}
                                callback = function(ev)
                                  {namespace}.{rpc_request}(ev)
                                end
                            }}
                        )
                    "#,
                ),
                vec![],
            )
            .await?;
        Ok(ret)
    }

    /// Wait for a plugin to reach running state.
    pub async fn await_plugin(&self, name: &str, timeout: std::time::Duration) -> Result<()> {
        let start = std::time::Instant::now();
        loop {
            if start.elapsed() > timeout {
                return Err(crate::error::Error::Internal {
                    msg: format!("Plugin failed to reach running state after {:?}", timeout),
                });
            }
            let val = lua_exec!(
                self,
                &format!("return {}.{}()", name, crate::service::STATUS_MESSAGE)
            )
            .await;
            if let Ok(val) = val {
                if let Some(val) = val.as_str() {
                    if val == crate::service::Status::Running.to_string() {
                        break;
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
        Ok(())
    }

    /// Redraw pending screen updates.
    ///
    /// Equivalent to the :redraw command in Neovim.
    pub async fn redraw(&self) -> Result<()> {
        lua!(self, "vim.cmd('redraw')").await
    }

    /// Redraw the entire screen.
    ///
    /// Equivalent to the :redraw! command in Neovim.
    pub async fn redraw_screen(&self) -> Result<()> {
        lua!(self, "vim.cmd('redraw!')").await
    }

    /// Redraw the status line and window bar of the current window.
    ///
    /// Equivalent to the :redrawstatus command in Neovim.
    pub async fn redraw_status(&self) -> Result<()> {
        lua!(self, "vim.cmd('redrawstatus')").await
    }

    ///  Redraw the status line and window bar of all windows.
    ///
    /// Equivalent to the :redrawstatus! command in Neovim.
    pub async fn redraw_status_all(&self) -> Result<()> {
        lua!(self, "vim.cmd('redrawstatus!')").await
    }

    /// Equivalent to the :redrawtabline command in Neovim.
    pub async fn redraw_tabline(&self) -> Result<()> {
        lua!(self, "vim.cmd('redrawtabline')").await
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use tokio::sync::broadcast;
    use tracing::warn;
    use tracing_test::traced_test;

    use crate::{error::Result, *};

    macro_rules! qtest {
        (@inner $s:stmt) => { $s };
        ( $($s:stmt)+ ) => {
                {
                   let (tx, _) = broadcast::channel(16);

                   #[derive(Clone)]
                   struct TestPlugin {}

                   #[async_trait]
                   impl crate::NviPlugin for TestPlugin {
                       fn name(&self) -> String {
                           "TestPlugin".into()
                       }

                         async fn connected(&mut self, client: &mut Client) -> Result<()> {
                           $(qtest!{@inner $s})+
                           match ret(client).await {
                               Ok(_) => (),
                               Err(e) => {
                                   warn!("error: {:?}", e);
                                }
                            };
                           client.shutdown();
                           Ok(())
                       }
                   }
                   let rtx = tx.clone();
                   test::run_plugin_with_shutdown(TestPlugin {}, rtx)
               }
        }
    }

    #[tokio::test]
    #[traced_test]
    async fn it_gets_cwd() {
        qtest! {
            async fn ret(c: &mut Client) -> Result<()> {
                c.getcwd().await?;
                Ok(())
            }
        }
        .await
        .unwrap();
    }

    #[tokio::test]
    #[traced_test]
    async fn it_registers_request() {
        #[derive(Clone)]
        struct TestPlugin {}

        #[async_trait]
        impl crate::NviPlugin for TestPlugin {
            fn name(&self) -> String {
                "TestPlugin".into()
            }

            async fn connected(&mut self, client: &mut Client) -> Result<()> {
                client
                    .register_rpcrequest("test_module", "test_fn", &["foo"])
                    .await
                    .unwrap();
                Ok(())
            }

            async fn request(
                &self,
                client: &mut Client,
                method: &str,
                params: &[Value],
            ) -> Result<Value, Value> {
                if method == "test_fn" {
                    assert_eq!(params.len(), 1);
                    assert_eq!(params[0], Value::from(5));
                    Ok(params[0].clone())
                } else {
                    client.shutdown();
                    Err(Value::Nil)
                }
            }
        }

        let nvit = test::NviTest::builder()
            .show_logs()
            .log_level(tracing::Level::DEBUG)
            .with_plugin(TestPlugin {})
            .run()
            .await
            .unwrap();

        let v: u64 = lua!(nvit.client, "return test_module.test_fn(5)")
            .await
            .unwrap();
        assert_eq!(v, 5);
        nvit.finish().await.unwrap();
    }

    #[tokio::test]
    #[traced_test]
    async fn it_registers_notification() {
        let (tx, _) = broadcast::channel(16);

        #[derive(Clone)]
        struct TestPlugin {}

        #[async_trait]
        impl crate::NviPlugin for TestPlugin {
            fn name(&self) -> String {
                "TestPlugin".into()
            }

            async fn connected(&mut self, client: &mut Client) -> Result<()> {
                client
                    .register_rpcnotify("test_module", "test_fn", &["foo"])
                    .await
                    .unwrap();

                let _: Value = client
                    .nvim
                    .exec_lua("return test_module.test_fn(5)", vec![])
                    .await
                    .unwrap();
                client.shutdown();
                Ok(())
            }

            async fn notify(
                &self,
                client: &mut Client,
                method: &str,
                params: &[Value],
            ) -> Result<()> {
                if method == "test_fn" {
                    assert_eq!(params.len(), 1);
                    assert_eq!(params[0], Value::from(5));
                } else {
                    client.shutdown();
                }
                Ok(())
            }
        }

        let rtx = tx.clone();
        test::run_plugin_with_shutdown(TestPlugin {}, rtx)
            .await
            .unwrap();
    }
}
