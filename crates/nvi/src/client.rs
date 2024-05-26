pub struct Client {
    pub(crate) m_client: msgpack_rpc::Client,
}

impl Client {
    async fn raw_request(
        &mut self,
        method: &str,
        params: &[msgpack_rpc::Value],
    ) -> Result<msgpack_rpc::Value, msgpack_rpc::Value> {
        self.m_client.request(method, params).await
    }

    async fn raw_notify(&mut self, method: &str, params: &[msgpack_rpc::Value]) -> Result<(), ()> {
        self.m_client.notify(method, params).await
    }
}
