use std::str::FromStr;
use std::sync::{Arc, Mutex};

use crate::Value;
use async_trait::async_trait;
use tokio::sync::broadcast;
use tracing::{debug, trace, warn};

use crate::{client::Client, error::Result, highlights, macro_types, nvim, nvim::types};

pub(crate) const STATUS_MESSAGE: &str = "__nvi_status";

#[derive(Debug, Clone, Copy, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum Status {
    Stopped,
    Connected,
    Running,
}

/// The `NviPlugin` trait is the way Nvi plugins are defined. Usually this is done with the
/// `nvi_plugin` attribute macro, which generates the required methods for the trait.
#[allow(unused_variables)]
#[async_trait]
pub trait NviPlugin: Sync + Send + 'static {
    fn name(&self) -> String;

    /// Return the highlight groups for this service. Highlight group names have the plugin name
    /// prepended (as in `Client::hl_name`) before creation.
    fn highlights(&self) -> Result<highlights::Highlights> {
        Ok(highlights::Highlights::default())
    }

    /// Inspect the service methods, as derived with the `nvi_plugin` attribute macro.
    fn inspect(&self) -> Vec<macro_types::Method> {
        vec![]
    }

    /// Return the plugin doc string. The string is empty if there are no docs.
    fn docs(&self) -> Result<String> {
        Ok("".into())
    }

    /// Bootstrapping that happens after connecting to the remote service, but before the run
    /// method is called. This method should execute and exit. Typically, this method will be
    /// derived with the `nvim_service` annotation, and should not be over-ridden by the user.
    async fn bootstrap(&self, client: &mut Client) -> Result<()> {
        let methods = self.inspect();
        for method in methods {
            let name = method.name.clone();

            match method.method_type {
                macro_types::MethodType::Notify => {
                    let args: Vec<String> = method.args.iter().map(|a| a.name.clone()).collect();
                    client
                        .register_rpcnotify(&self.name(), &name, &args)
                        .await?;
                }
                macro_types::MethodType::Request => {
                    let args: Vec<String> = method.args.iter().map(|a| a.name.clone()).collect();
                    client
                        .register_rpcrequest(&self.name(), &name, &args)
                        .await?;

                    // Handle autocmd registration if present
                    if let Some(autocmd) = method.autocmd {
                        let group = autocmd.group.map(types::Group::Name);
                        let events = autocmd
                            .events
                            .iter()
                            .map(|e| types::Event::from_str(e.as_str()).unwrap())
                            .collect::<Vec<_>>();

                        client
                            .autocmd_pattern(
                                &autocmd.patterns,
                                &name,
                                &events,
                                group,
                                false, // once (not supported yet)
                                autocmd.nested,
                            )
                            .await?;
                    }
                }
                macro_types::MethodType::Connected => (), // Nothing to register for connected methods
                macro_types::MethodType::Highlights => (), // Nothing to register for highlights methods
            }
        }
        client
            .register_rpcrequest::<String>(&self.name(), STATUS_MESSAGE, &[])
            .await?;
        let highlights = self.highlights()?;
        highlights.create(client).await?;
        Ok(())
    }

    /// Handle a generic notification from the remote service. Typcially, this method will be
    /// derived with the `nvim_service` annotation.
    async fn notify(&self, client: &mut Client, method: &str, params: &[Value]) -> Result<()> {
        warn!("unhandled notification: {:?}", method);
        Ok(())
    }

    /// Handle a generic request from the remote service. Typcially, this method will be
    /// derived with the `nvim_service` annotation.
    async fn request(
        &self,
        client: &mut Client,
        method: &str,
        params: &[Value],
    ) -> Result<Value, Value> {
        warn!("unhandled request: {:?}", method);
        Err(Value::Nil)
    }

    /// Handle a generic notification from the remote service, with a mutable receiver. Typcially,
    /// this method will be derived with the `nvim_service` annotation.
    async fn notify_mut(
        &mut self,
        client: &mut Client,
        method: &str,
        params: &[Value],
    ) -> Result<()> {
        warn!("unhandled notification: {:?}", method);
        Ok(())
    }

    /// Handle a generic request from the remote service, with a mutable receiver. Typcially, this
    /// method will be derived with the `nvim_service` annotation.
    async fn request_mut(
        &mut self,
        client: &mut Client,
        method: &str,
        params: &[Value],
    ) -> Result<Value, Value> {
        warn!("unhandled request: {:?}", method);
        Err(Value::Nil)
    }

    /// This method is run on first connecting to the remote service. A loop may be run here that
    /// persists for the life of the connection.
    async fn connected(&mut self, client: &mut Client) -> Result<()> {
        Ok(())
    }
}

// RpcConnection handles a single RPC connection
pub(crate) struct RpcConnection<T>
where
    T: NviPlugin,
{
    plugin: tokio::sync::RwLock<T>,
    shutdown_tx: broadcast::Sender<()>,
    channel_id: Arc<Mutex<Option<u64>>>,
    methods: std::collections::HashMap<String, bool>,
    status: Arc<Mutex<Status>>,
}

impl<T> RpcConnection<T>
where
    T: NviPlugin,
{
    pub fn new(shutdown_tx: broadcast::Sender<()>, plugin: T) -> Self {
        let method_mutability = plugin
            .inspect()
            .into_iter()
            .map(|m| (m.name, m.is_mut))
            .collect();

        RpcConnection {
            plugin: tokio::sync::RwLock::new(plugin),
            shutdown_tx,
            channel_id: Arc::new(Mutex::new(None)),
            methods: method_mutability,
            status: Arc::new(Mutex::new(Status::Stopped)),
        }
    }

    fn make_client(&self, plugin_name: &str, sender: mrpc::RpcSender) -> Client {
        Client::new(
            sender,
            plugin_name,
            self.channel_id.lock().unwrap().expect("channel id not set"),
            self.shutdown_tx.clone(),
        )
    }

    async fn handle_notification_error(
        &self,
        method: &str,
        e: crate::error::Error,
        sender: mrpc::RpcSender,
    ) -> mrpc::Result<()> {
        warn!("error handling notification: {:?}", e);
        let client = self.make_client(&self.plugin.read().await.name(), sender);
        if let Err(notify_err) = client
            .notify(
                types::LogLevel::Warn,
                &format!("nvi notify error: {method} - {e:?}"),
            )
            .await
        {
            warn!("error sending notify error notification: {:?}", notify_err);
        }
        Err(mrpc::RpcError::Service(mrpc::ServiceError {
            name: "NviNotifyError".to_string(),
            value: Value::String(format!("{e:?}").into()),
        }))
    }

    async fn handle_request_error(
        &self,
        method: &str,
        e: Value,
        sender: mrpc::RpcSender,
    ) -> mrpc::Result<Value> {
        warn!("error handling request: {:?}", e);
        let client = self.make_client(&self.plugin.read().await.name(), sender);
        if let Err(notify_err) = client
            .notify(
                types::LogLevel::Warn,
                &format!("nvi request error: {method} - {e:?}"),
            )
            .await
        {
            warn!("error sending request error notification: {:?}", notify_err);
        }
        Err(mrpc::RpcError::Service(mrpc::ServiceError {
            name: "NviRequestError".to_string(),
            value: Value::String(format!("{e:?}").into()),
        }))
    }
}

/// A wrapper service that translates from mrpc to NviPlugin.
#[async_trait::async_trait]
impl<T> mrpc::Connection for RpcConnection<T>
where
    T: NviPlugin,
{
    async fn connected(&self, sender: mrpc::RpcSender) -> mrpc::Result<()> {
        *self.status.lock().unwrap() = Status::Connected;
        let nv = nvim::NvimApi {
            rpc_sender: sender.clone(),
        };
        let mut plugin = self.plugin.write().await;
        let ci = nv.get_chan_info(0).await.map_err(|e| {
            warn!("error getting channel info: {:?}", e);
            mrpc::RpcError::Service(mrpc::ServiceError {
                name: "NviServiceError".to_string(),
                value: Value::String(format!("{e:?}").into()),
            })
        })?;
        self.channel_id.lock().unwrap().replace(ci.id);

        let mut client = self.make_client(&plugin.name(), sender);
        match plugin.bootstrap(&mut client).await {
            Ok(_) => trace!("bootstrap complete"),
            Err(e) => {
                warn!("bootstrap failed: {:?}", e);
                client.shutdown();
                return Ok(());
            }
        }
        let ret = plugin.connected(&mut client).await;
        match ret {
            Ok(_) => trace!("connected() completed"),
            Err(e) => {
                warn!("connected() failed: {:?}", e);
                client.shutdown();
            }
        };
        *self.status.lock().unwrap() = Status::Running;
        Ok(())
    }

    async fn handle_request(
        &self,
        sender: mrpc::RpcSender,
        method: &str,
        params: Vec<Value>,
    ) -> mrpc::Result<Value> {
        if method == STATUS_MESSAGE {
            return Ok(self.status.lock().unwrap().to_string().into());
        }

        debug!("recv request: {:?}", method);
        trace!("recv request data: {:?} {:?}", method, params);

        let is_mut = self.methods.get(method).copied().unwrap_or(false);
        let result = if is_mut {
            let mut plugin = self.plugin.write().await;
            let mut client = self.make_client(&plugin.name(), sender.clone());
            plugin.request_mut(&mut client, method, &params).await
        } else {
            let plugin = self.plugin.read().await;
            let mut client = self.make_client(&plugin.name(), sender.clone());
            plugin.request(&mut client, method, &params).await
        };

        match result {
            Ok(v) => Ok(v),
            Err(e) => self.handle_request_error(method, e, sender).await,
        }
    }

    async fn handle_notification(
        &self,
        sender: mrpc::RpcSender,
        method: &str,
        params: Vec<Value>,
    ) -> mrpc::Result<()> {
        debug!("recv notification: {:?}", method);
        trace!("recv notification data: {:?} {:?}", method, params);

        let is_mut = self.methods.get(method).copied().unwrap_or(false);
        let result = if is_mut {
            let mut plugin = self.plugin.write().await;
            let mut client = self.make_client(&plugin.name(), sender.clone());
            plugin.notify_mut(&mut client, method, &params).await
        } else {
            let plugin = self.plugin.read().await;
            let mut client = self.make_client(&plugin.name(), sender.clone());
            plugin.notify(&mut client, method, &params).await
        };

        match result {
            Ok(()) => Ok(()),
            Err(e) => self.handle_notification_error(method, e, sender).await,
        }
    }
}
