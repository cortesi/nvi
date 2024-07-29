use crate::Value;
use async_trait::async_trait;
use tokio::sync::broadcast;
use tracing::{debug, trace, warn};

use crate::{client::Client, error::Result, types};

pub(crate) const PING_MESSAGE: &str = "__nvi_ping";

#[allow(unused_variables)]
#[async_trait]
pub trait NviService: Clone + Send {
    fn name(&self) -> String;

    /// Bootstrapping that happens after connecting to the remote service, but before the run
    /// method is called. This method should execute and exit. Typically, this method will be
    /// derived with the `nvim_service` annotation.
    async fn bootstrap(&mut self, client: &mut Client) -> Result<()> {
        Ok(())
    }

    /// Handle a generic notification from the remote service. Typcially, this method will be
    /// derived with the `nvim_service` annotation.
    async fn notify(&mut self, client: &mut Client, method: &str, params: &[Value]) -> Result<()> {
        warn!("unhandled notification: {:?}", method);
        Ok(())
    }

    /// Handle a generic request from the remote service. Typcially, this method will be
    /// derived with the `nvim_service` annotation.
    async fn request(
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
    async fn run(&mut self, client: &mut Client) -> Result<()> {
        Ok(())
    }
}

// Service handles a single connection to neovim.
#[derive(Clone)]
pub(crate) struct ServiceWrapper<T>
where
    T: NviService,
{
    nvi_service: T,
    shutdown_tx: broadcast::Sender<()>,
    channel_id: Option<u64>,
}

impl<T> ServiceWrapper<T>
where
    T: NviService,
{
    pub fn new(nvi_service: T, shutdown_tx: broadcast::Sender<()>) -> Self {
        ServiceWrapper {
            nvi_service,
            shutdown_tx,
            channel_id: None,
        }
    }
}

/// A wrapper service that translates from mrpc to NviService.
#[async_trait::async_trait]
impl<T> mrpc::RpcService for ServiceWrapper<T>
where
    T: NviService + Send + Sync + 'static,
{
    async fn connected(&self, client: mrpc::RpcSender) {
        let mut vimservice = self.nvi_service.clone();
        let channel_id = self.channel_id;
        let shutdown_tx = self.shutdown_tx.clone();
        let mut c = Client::new(
            client.clone(),
            &vimservice.name(),
            channel_id,
            shutdown_tx.clone(),
        );

        match vimservice.bootstrap(&mut c).await {
            Ok(_) => trace!("bootstrap complete"),
            Err(e) => {
                warn!("bootstrap failed: {:?}", e);
                c.shutdown();
                return;
            }
        }
        let mut c = Client::new(client, &vimservice.name(), channel_id, shutdown_tx.clone());
        let ret = vimservice.run(&mut c).await;
        match ret {
            Ok(_) => trace!("run() completed"),
            Err(e) => {
                warn!("run() failed: {:?}", e);
                c.shutdown();
            }
        }
    }

    async fn handle_request<S>(
        &self,
        client: mrpc::RpcSender,
        method: &str,
        params: Vec<Value>,
    ) -> mrpc::Result<Value> {
        debug!("recv request: {:?}", method);
        trace!("recv request data: {:?} {:?}", method, params);
        let mut vimservice = self.nvi_service.clone();
        let mut client = Client::new(
            client,
            &vimservice.name(),
            self.channel_id,
            self.shutdown_tx.clone(),
        );

        if method == PING_MESSAGE {
            trace!("ping received");
            return Ok(Value::Boolean(true));
        }

        match vimservice.request(&mut client, method, &params).await {
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

    async fn handle_notification<S>(
        &self,
        client: mrpc::RpcSender,
        method: &str,
        params: Vec<Value>,
    ) -> mrpc::Result<()> {
        debug!("recv notification: {:?}", method);
        trace!("recv notification data: {:?} {:?}", method, params);
        let mut vimservice = self.nvi_service.clone();
        let mut client = Client::new(
            client,
            &vimservice.name(),
            self.channel_id,
            self.shutdown_tx.clone(),
        );

        if let Err(e) = vimservice.notify(&mut client, method, &params).await {
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
