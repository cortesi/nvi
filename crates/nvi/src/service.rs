use std::pin::Pin;

use crate::Value;
use async_trait::async_trait;
use futures::Future;
use tokio::{runtime::Handle, sync::broadcast};
use tracing::{debug, trace, warn};

use crate::{client::Client, error::Result, types};

pub(crate) const BOOTSTRAP_NOTIFICATION: &str = "nvi_bootstrap";

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

#[derive(Clone)]
pub struct AsyncClosureService<F>
where
    F: for<'a> Fn(&'a mut Client) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>
        + Clone
        + Send
        + 'static,
{
    connected_closure: F,
}

impl<F> AsyncClosureService<F>
where
    F: for<'a> Fn(&'a mut Client) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>
        + Clone
        + Send
        + 'static,
{
    pub fn new(connected_closure: F) -> Self {
        AsyncClosureService { connected_closure }
    }
}

#[async_trait]
impl<F> NviService for AsyncClosureService<F>
where
    F: for<'a> Fn(&'a mut Client) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>
        + Clone
        + Send
        + 'static,
{
    fn name(&self) -> String {
        "NvimClosure".to_string()
    }

    async fn run(&mut self, client: &mut Client) -> Result<()> {
        (self.connected_closure)(client).await;
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

/// A wrapper service that translates from nvi_rpc to NviService.
impl<T> nvi_rpc::ServiceWithClient for ServiceWrapper<T>
where
    T: NviService + Send + 'static,
{
    type RequestFuture = Pin<Box<dyn Future<Output = Result<Value, Value>> + Send>>;

    fn handle_request(
        &mut self,
        client: &mut nvi_rpc::Client,
        method: &str,
        params: &[Value],
    ) -> Self::RequestFuture {
        debug!("recv request: {:?}", method);
        trace!("recv request data: {:?} {:?}", method, params);
        let mut vimservice = self.nvi_service.clone();
        let mut client = Client::new(
            client,
            &vimservice.name(),
            self.channel_id,
            self.shutdown_tx.clone(),
        );
        let method = method.to_string();
        let params = params.to_vec();
        Box::pin(async move {
            let c = client.clone();
            match vimservice.request(&mut client, &method, &params).await {
                Ok(v) => Ok(v),
                Err(e) => {
                    warn!("nvi request error: {:?}", e);
                    _ = c
                        .notify(
                            types::LogLevel::Warn,
                            &format!("nvi request error: {method} - {e}"),
                        )
                        .await
                        .map_err(|e| warn!("error sending request error notification: {:?}", e));
                    Err(e)
                }
            }
        })
    }

    fn handle_notification(
        &mut self,
        client: &mut nvi_rpc::Client,
        method: &str,
        params: &[Value],
    ) {
        debug!("recv notification: {:?}", method);
        trace!("recv notification data: {:?} {:?}", method, params);
        let mut vimservice = self.nvi_service.clone();
        let handle = Handle::current();
        let m_client = client.clone();
        if method == BOOTSTRAP_NOTIFICATION {
            let id = params[0].as_u64().unwrap();
            debug!("connected to nvim with channel id: {:?}", id);
            self.channel_id = Some(id);
            let channel_id = self.channel_id;
            let shutdown_tx = self.shutdown_tx.clone();
            handle.spawn(async move {
                let mut c = Client::new(
                    &m_client,
                    &vimservice.name(),
                    channel_id,
                    shutdown_tx.clone(),
                );
                let ret = vimservice.bootstrap(&mut c).await;
                match ret {
                    Ok(_) => trace!("bootstrap complete"),
                    Err(e) => {
                        warn!("bootstrap failed: {:?}", e);
                        c.shutdown();
                        return;
                    }
                }
                let ret = vimservice
                    .run(&mut Client::new(
                        &m_client,
                        &vimservice.name(),
                        channel_id,
                        shutdown_tx,
                    ))
                    .await;
                match ret {
                    Ok(_) => trace!("run() completed"),
                    Err(e) => {
                        warn!("run() failed: {:?}", e);
                        c.shutdown();
                    }
                }
            });
            return;
        }
        let method = method.to_string();
        let params = params.to_vec();
        let channel_id = self.channel_id;
        let shutdown_tx = self.shutdown_tx.clone();
        handle.spawn(async move {
            let c = &mut Client::new(&m_client, &vimservice.name(), channel_id, shutdown_tx);

            match vimservice.notify(&mut c.clone(), &method, &params).await {
                Ok(_) => {}
                Err(e) => {
                    warn!("error handling request: {:?}", e);
                    _ = c
                        .notify(
                            types::LogLevel::Warn,
                            &format!("nvi notify error: {method} - {e}"),
                        )
                        .await
                        .map_err(|e| warn!("error sending notify error notification: {:?}", e));
                }
            };
        });
    }
}
