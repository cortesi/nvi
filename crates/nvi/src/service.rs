use std::pin::Pin;

use async_trait::async_trait;
use futures::Future;
use msgpack_rpc::Value;
use tokio::{runtime::Handle, sync::broadcast};
use tracing::{trace, warn};

use crate::{client::NviClient, error::Result};

pub(crate) const BOOTSTRAP_NOTIFICATION: &str = "nvi_bootstrap";

#[allow(unused_variables)]
#[async_trait]
pub trait NviService: Clone + Send {
    async fn run(&mut self, client: &mut NviClient) -> Result<()> {
        Ok(())
    }

    async fn notification(&mut self, client: &mut NviClient, method: &str, params: &[Value]) {
        warn!("unhandled notification: {:?}", method);
    }

    async fn request(
        &mut self,
        client: &mut NviClient,
        method: &str,
        params: &[Value],
    ) -> Result<Value, Value> {
        warn!("unhandled request: {:?}", method);
        Err(Value::Nil)
    }
}

#[derive(Clone)]
pub struct AsyncClosureService<F>
where
    F: for<'a> Fn(&'a mut NviClient) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>
        + Clone
        + Send
        + 'static,
{
    connected_closure: F,
}

impl<F> AsyncClosureService<F>
where
    F: for<'a> Fn(&'a mut NviClient) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>
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
    F: for<'a> Fn(&'a mut NviClient) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>
        + Clone
        + Send
        + 'static,
{
    async fn run(&mut self, client: &mut NviClient) -> Result<()> {
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

/// A wrapper service that translates from msgpack_rpc to NviService.
impl<T> msgpack_rpc::ServiceWithClient for ServiceWrapper<T>
where
    T: NviService + Send + 'static,
{
    type RequestFuture = Pin<Box<dyn Future<Output = Result<Value, Value>> + Send>>;

    fn handle_request(
        &mut self,
        client: &mut msgpack_rpc::Client,
        method: &str,
        params: &[Value],
    ) -> Self::RequestFuture {
        trace!("recv request: {:?} {:?}", method, params);
        let mut vimservice = self.nvi_service.clone();
        let mut client = NviClient::new(client, self.channel_id, self.shutdown_tx.clone());
        let method = method.to_string();
        let params = params.to_vec();
        Box::pin(async move { vimservice.request(&mut client, &method, &params).await })
    }

    fn handle_notification(
        &mut self,
        client: &mut msgpack_rpc::Client,
        method: &str,
        params: &[Value],
    ) {
        trace!("recv notifcation: {:?} {:?}", method, params);
        let mut vimservice = self.nvi_service.clone();
        let handle = Handle::current();
        let m_client = client.clone();
        if method == BOOTSTRAP_NOTIFICATION {
            let id = params[0].as_u64().unwrap();
            trace!("bootstrapped with channel id: {:?}", id);
            self.channel_id = Some(id);
            let channel_id = self.channel_id;
            let shutdown_tx = self.shutdown_tx.clone();
            handle.spawn(async move {
                vimservice
                    .run(&mut NviClient::new(&m_client, channel_id, shutdown_tx))
                    .await
            });
            return;
        }
        let method = method.to_string();
        let params = params.to_vec();
        let channel_id = self.channel_id;
        let shutdown_tx = self.shutdown_tx.clone();
        handle.spawn(async move {
            vimservice
                .notification(
                    &mut NviClient::new(&m_client, channel_id, shutdown_tx),
                    &method,
                    &params,
                )
                .await;
        });
    }
}
