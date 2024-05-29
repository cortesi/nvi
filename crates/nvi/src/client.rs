use crate::nvim_api;
use tracing::trace;

/// A client to Neovim.
pub struct NviClient {
    pub(crate) m_client: msgpack_rpc::Client,
    pub api: nvim_api::NvimApi,
}

impl NviClient {
    pub fn new(client: &msgpack_rpc::Client) -> Self {
        NviClient {
            m_client: client.clone(),
            api: nvim_api::NvimApi {
                m_client: client.clone(),
            },
        }
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
