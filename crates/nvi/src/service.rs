use std::str::FromStr;
use std::sync::{Arc, Mutex};

use crate::Value;
use async_trait::async_trait;
use tokio::sync::broadcast;
use tracing::{debug, trace, warn};

use crate::{client::Client, error::Result, macro_types, nvim, nvim::types};

pub(crate) const PING_MESSAGE: &str = "__nvi_ping";

/// The `NviService` trait is the way Nvi plugins are defined. Usually this is done with the
/// `nvi_plugin` attribute macro, which generates the required methods for the trait.
#[allow(unused_variables)]
#[async_trait]
pub trait NviPlugin: Clone + Sync + Send + 'static {
    fn name(&self) -> String;

    /// Introspect the service methods, as derived with the `nvi_plugin` attribute macro.
    fn introspect(&self) -> Vec<macro_types::Method> {
        vec![]
    }

    /// Bootstrapping that happens after connecting to the remote service, but before the run
    /// method is called. This method should execute and exit. Typically, this method will be
    /// derived with the `nvim_service` annotation, and should not be over-ridden by the user.
    async fn bootstrap(&self, client: &mut Client) -> Result<()> {
        let methods = self.introspect();
        for method in methods {
            let name = method.name.clone();

            match method.message_type {
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
            }
        }
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

    /// This method is run on first connecting to the remote service. A loop may be run here that
    /// persists for the life of the connection.
    async fn connected(&self, client: &mut Client) -> Result<()> {
        Ok(())
    }
}

// Service handles a single RPC connection
#[derive(Clone)]
pub(crate) struct RpcConnection<T>
where
    T: NviPlugin,
{
    plugin: T,
    shutdown_tx: broadcast::Sender<()>,
    channel_id: Arc<Mutex<Option<u64>>>,
}

impl<T> RpcConnection<T>
where
    T: NviPlugin,
{
    pub fn new(shutdown_tx: broadcast::Sender<()>, plugin: T) -> Self {
        RpcConnection {
            plugin,
            shutdown_tx,
            channel_id: Arc::new(Mutex::new(None)),
        }
    }
}

/// A wrapper service that translates from mrpc to NviService.
#[async_trait::async_trait]
impl<T> mrpc::Connection for RpcConnection<T>
where
    T: NviPlugin,
{
    async fn connected(&self, sender: mrpc::RpcSender) -> mrpc::Result<()> {
        let shutdown_tx = self.shutdown_tx.clone();

        let nv = nvim::api::NvimApi {
            rpc_sender: sender.clone(),
        };
        let ci = nv.get_chan_info(0).await.map_err(|e| {
            warn!("error getting channel info: {:?}", e);
            mrpc::RpcError::Service(mrpc::ServiceError {
                name: "NviServiceError".to_string(),
                value: Value::String(format!("{e:?}").into()),
            })
        })?;
        self.channel_id.lock().unwrap().replace(ci.id);

        let mut c = Client::new(
            sender.clone(),
            &self.plugin.name(),
            self.channel_id.lock().unwrap().expect("channel id not set"),
            shutdown_tx.clone(),
        );
        match self.plugin.bootstrap(&mut c).await {
            Ok(_) => trace!("bootstrap complete"),
            Err(e) => {
                warn!("bootstrap failed: {:?}", e);
                c.shutdown();
                return Ok(());
            }
        }
        let ret = self.plugin.connected(&mut c).await;
        match ret {
            Ok(_) => trace!("connected() completed"),
            Err(e) => {
                warn!("connected() failed: {:?}", e);
                c.shutdown();
            }
        };
        Ok(())
    }

    async fn handle_request(
        &self,
        sender: mrpc::RpcSender,
        method: &str,
        params: Vec<Value>,
    ) -> mrpc::Result<Value> {
        let nvi_plugin = self.plugin.clone();
        let mut client = Client::new(
            sender,
            &nvi_plugin.name(),
            self.channel_id.lock().unwrap().expect("channel id not set"),
            self.shutdown_tx.clone(),
        );

        if method == PING_MESSAGE {
            return Ok(Value::Boolean(true));
        }

        debug!("recv request: {:?}", method);
        trace!("recv request data: {:?} {:?}", method, params);
        match nvi_plugin.request(&mut client, method, &params).await {
            Ok(v) => Ok(v),
            Err(e) => {
                warn!("nvi request error: {:?}", e);
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
                    name: "NviServiceError".to_string(),
                    value: Value::String(format!("{e:?}").into()),
                }))
            }
        }
    }

    async fn handle_notification(
        &self,
        client: mrpc::RpcSender,
        method: &str,
        params: Vec<Value>,
    ) -> mrpc::Result<()> {
        debug!("recv notification: {:?}", method);
        trace!("recv notification data: {:?} {:?}", method, params);
        let plugin = self.plugin.clone();
        let mut client = Client::new(
            client,
            &plugin.name(),
            self.channel_id.lock().unwrap().expect("channel id not set"),
            self.shutdown_tx.clone(),
        );

        if let Err(e) = plugin.notify(&mut client, method, &params).await {
            warn!("error handling notification: {:?}", e);
            if let Err(notify_err) = client
                .notify(
                    types::LogLevel::Warn,
                    &format!("nvi notify error: {method} - {e:?}"),
                )
                .await
            {
                warn!("error sending notify error notification: {:?}", notify_err);
            }
            return Err(mrpc::RpcError::Service(mrpc::ServiceError {
                name: "NviNotifyError".to_string(),
                value: Value::String(format!("{e:?}").into()),
            }));
        }

        Ok(())
    }
}
