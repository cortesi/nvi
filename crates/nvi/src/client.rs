use tracing::trace;

/// A client for speaking to Neovim.
pub struct NviClient {
    pub(crate) m_client: msgpack_rpc::Client,
}

impl NviClient {
    pub async fn raw_request(
        &mut self,
        method: &str,
        params: &[msgpack_rpc::Value],
    ) -> Result<msgpack_rpc::Value, msgpack_rpc::Value> {
        trace!("send request: {:?} {:?}", method, params);
        self.m_client.request(method, params).await
    }

    pub async fn raw_notify(
        &mut self,
        method: &str,
        params: &[msgpack_rpc::Value],
    ) -> Result<(), ()> {
        trace!("send notification: {:?} {:?}", method, params);
        self.m_client.notify(method, params).await
    }
}
