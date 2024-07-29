use std::path::PathBuf;

use tokio::sync::broadcast;
use tracing::trace;

use crate::Value;

use crate::{
    error::{Error, Result},
    nvim_api, types,
};

/// A client to Neovim. A `Client` object is passed to every method invocation in a `NviService`.
/// It exposes the full auto-generated API for Neovim on its `nvim` field, and provides a set of
/// higher-level methods directly on the `Client` object.
#[derive(Clone)]
pub struct Client {
    /// The name of the plugin.
    pub name: String,
    /// The compiled API for Neovim.
    pub nvim: nvim_api::NvimApi,
    /// The MessagePack-RPC channel ID for this client.
    pub channel_id: Option<u64>,

    shutdown_tx: broadcast::Sender<()>,
}

impl Client {
    pub fn new(
        rpc_sender: mrpc::RpcSender,
        name: &str,
        channel_id: Option<u64>,
        shutdown_tx: broadcast::Sender<()>,
    ) -> Self {
        Client {
            name: name.into(),
            nvim: nvim_api::NvimApi { rpc_sender },
            shutdown_tx,
            channel_id,
        }
    }

    pub async fn getcwd(&self) -> Result<PathBuf> {
        let r = self.nvim.exec_lua("return vim.fn.getcwd()", vec![]).await?;
        Ok(r.as_str().unwrap().into())
    }

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
        let channel_id = self.channel_id.ok_or_else(|| Error::Internal {
            msg: "channel_id not set".into(),
        })?;

        let arg_list = params
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let extra_sep = if !arg_list.is_empty() { ", " } else { "" };

        self.nvim
            .exec_lua(
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
                vec![],
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
    pub async fn notify(&self, level: types::LogLevel, msg: &str) -> Result<()> {
        self.nvim
            .notify(msg, level.to_u64(), Value::Map(vec![]))
            .await
    }

    /// Send an nvim_notify notification with a log level of `LogLevel::Trace`.
    pub async fn trace(&self, msg: &str) -> Result<()> {
        self.notify(types::LogLevel::Trace, msg).await
    }

    /// Send an nvim_notify notification with a log level of `LogLevel::Debug`.
    pub async fn debug(&self, msg: &str) -> Result<()> {
        self.notify(types::LogLevel::Debug, msg).await
    }

    /// Send an nvim_notify notification with a log level of `LogLevel::Info`.
    pub async fn info(&self, msg: &str) -> Result<()> {
        self.notify(types::LogLevel::Info, msg).await
    }

    /// Send an nvim_notify notification with a log level of `LogLevel::Warn`.
    pub async fn warn(&self, msg: &str) -> Result<()> {
        self.notify(types::LogLevel::Warn, msg).await
    }

    /// Send an nvim_notify notification with a log level of `LogLevel::Error`.
    pub async fn error(&self, msg: &str) -> Result<()> {
        self.notify(types::LogLevel::Error, msg).await
    }

    // pub async fn autocmd_buflocal(
    //     &self,
    //     rpc_request: &str,
    //     events: &[types::Event],
    //     buffer: Option<u64>,
    //     group: Option<u64>,
    //     once: bool,
    //     nested: bool,
    // ) -> Result<()> {
    //     Ok(())
    // }

    pub async fn autocmd(
        &self,
        rpc_request: &str,
        events: &[types::Event],
        patterns: &[String],
        group: Option<u64>,
        once: bool,
        nested: bool,
    ) -> Result<u64> {
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
            format!("group = {g},")
        } else {
            "".to_string()
        };

        let namespace = &self.name;
        let ret = self
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
        Ok(ret.as_u64().unwrap())
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
                   struct TestService {}

                   #[async_trait]
                   impl crate::NviService for TestService {
                       fn name(&self) -> String {
                           "TestService".into()
                       }

                       async fn run(&mut self, client: &mut Client) -> Result<()> {
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
                   test::test_service(TestService {}, rtx)
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
        let (tx, _) = broadcast::channel(16);

        #[derive(Clone)]
        struct TestService {}

        #[async_trait]
        impl crate::NviService for TestService {
            fn name(&self) -> String {
                "TestService".into()
            }

            async fn run(&mut self, client: &mut Client) -> Result<()> {
                client
                    .register_rpcrequest("test_module", "test_fn", &["foo"])
                    .await
                    .unwrap();

                let v = client
                    .nvim
                    .exec_lua("return test_module.test_fn(5)", vec![])
                    .await
                    .unwrap();
                assert_eq!(v, Value::from(5));
                client.shutdown();
                Ok(())
            }

            async fn request(
                &mut self,
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

        let rtx = tx.clone();
        test::test_service(TestService {}, rtx).await.unwrap();
    }

    #[tokio::test]
    #[traced_test]
    async fn it_registers_notification() {
        let (tx, _) = broadcast::channel(16);

        #[derive(Clone)]
        struct TestService {}

        #[async_trait]
        impl crate::NviService for TestService {
            fn name(&self) -> String {
                "TestService".into()
            }

            async fn run(&mut self, client: &mut Client) -> Result<()> {
                client
                    .register_rpcnotify("test_module", "test_fn", &["foo"])
                    .await
                    .unwrap();

                client
                    .nvim
                    .exec_lua("return test_module.test_fn(5)", vec![])
                    .await
                    .unwrap();
                client.shutdown();
                Ok(())
            }

            async fn notify(
                &mut self,
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
        test::test_service(TestService {}, rtx).await.unwrap();
    }
}
