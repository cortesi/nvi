use tokio::sync::mpsc;
use tracing::trace;

use crate::{error::Result, nvim_api};

/// A client to Neovim.
pub struct NviClient {
    pub(crate) m_client: msgpack_rpc::Client,
    /// The compiled API for Neovim.
    pub api: nvim_api::NvimApi,

    shutdown_tx: mpsc::UnboundedSender<()>,
    pub channel_id: Option<u64>,
}

impl NviClient {
    pub fn new(
        client: &msgpack_rpc::Client,
        channel_id: Option<u64>,
        shutdown_tx: mpsc::UnboundedSender<()>,
    ) -> Self {
        NviClient {
            m_client: client.clone(),
            api: nvim_api::NvimApi {
                m_client: client.clone(),
            },
            shutdown_tx,
            channel_id,
        }
    }

    pub fn shutdown(&self) {
        trace!("shutting down client");
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
