use tokio::sync::broadcast;
use tracing::trace;

use crate::{
    error::{Error, Result},
    nvim_api,
};

/// A client to Neovim.
pub struct Client {
    pub(crate) m_client: msgpack_rpc::Client,
    /// The compiled API for Neovim.
    pub api: nvim_api::NvimApi,

    shutdown_tx: broadcast::Sender<()>,
    pub channel_id: Option<u64>,
}

impl Client {
    pub fn new(
        client: &msgpack_rpc::Client,
        channel_id: Option<u64>,
        shutdown_tx: broadcast::Sender<()>,
    ) -> Self {
        Client {
            m_client: client.clone(),
            api: nvim_api::NvimApi {
                m_client: client.clone(),
            },
            shutdown_tx,
            channel_id,
        }
    }

    async fn register_method<T>(
        &mut self,
        kind: &str,
        namespace: &str,
        method: &str,
        params: &[T],
    ) -> Result<()>
    where
        T: std::string::ToString,
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

        self.api
            .nvim_exec_lua(
                &format!(
                    "
                        if not _G.{namespace} then
                            _G.{namespace} = {{}}
                        end
                        if _G.{namespace}.{method} then
                            error('method already exists: {method}')
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
    /// function in the under the specified namespace. When this function is called, an RPC message
    /// is sent back to the current addon.
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
    ///
    /// If the method already exists, an error is returned.
    pub async fn register_rpcrequest<T>(
        &mut self,
        namespace: &str,
        method: &str,
        params: &[T],
    ) -> Result<()>
    where
        T: std::string::ToString,
    {
        self.register_method("rpcrequest", namespace, method, params)
            .await
    }

    /// Register an RPC notification method for use in Neovim. This sets a globally-avaialable Lua
    /// function in the under the specified namespace. When this function is called, an RPC message
    /// is sent back to the current addon.
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
    ///
    /// If the method already exists, an error is returned.
    pub async fn register_rpcnotify<T>(
        &mut self,
        namespace: &str,
        method: &str,
        params: &[T],
    ) -> Result<()>
    where
        T: std::string::ToString,
    {
        self.register_method("rpcnotify", namespace, method, params)
            .await
    }

    pub fn shutdown(&self) {
        trace!("shutdown request from client");
        let _ = self.shutdown_tx.send(());
    }

    /// Send a raw request to Neovim.
    pub async fn raw_request(
        &mut self,
        method: &str,
        params: &[msgpack_rpc::Value],
    ) -> Result<msgpack_rpc::Value, msgpack_rpc::Value> {
        trace!("send request: {:?} {:?}", method, params);
        self.m_client.request(method, params).await
    }

    /// Send a raw notification to Neovim.
    pub async fn raw_notify(
        &mut self,
        method: &str,
        params: &[msgpack_rpc::Value],
    ) -> Result<(), ()> {
        trace!("send notification: {:?} {:?}", method, params);
        self.m_client.notify(method, params).await
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use tokio::sync::broadcast;
    use tracing_test::traced_test;

    use crate::{error::Result, *};

    #[tokio::test]
    #[traced_test]
    async fn it_registers_request() {
        let (tx, _) = broadcast::channel(16);

        #[derive(Clone)]
        struct TestService {}

        #[async_trait]
        impl crate::NviService for TestService {
            async fn run(&mut self, client: &mut Client) -> Result<()> {
                client
                    .register_rpcrequest("test_module", "test_fn", &["foo"])
                    .await
                    .unwrap();
                // Second call should fail. We don't permit re-registering methods.
                client
                    .register_rpcrequest("test_module", "test_fn", &["foo"])
                    .await
                    .unwrap_err();

                let v = client
                    .api
                    .nvim_exec_lua("return test_module.test_fn(5)", vec![])
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
            async fn run(&mut self, client: &mut Client) -> Result<()> {
                client
                    .register_rpcnotify("test_module", "test_fn", &["foo"])
                    .await
                    .unwrap();
                // Second call should fail. We don't permit re-registering methods.
                client
                    .register_rpcnotify("test_module", "test_fn", &["foo"])
                    .await
                    .unwrap_err();

                client
                    .api
                    .nvim_exec_lua("return test_module.test_fn(5)", vec![])
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
