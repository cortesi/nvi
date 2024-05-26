pub struct Client<'a> {
    pub(crate) m_client: &'a msgpack_rpc::Client,
}

impl<'a> Client<'a> {
    async fn raw_request(
        &mut self,
        method: &str,
        params: &[msgpack_rpc::Value],
    ) -> Result<msgpack_rpc::Value, msgpack_rpc::Value> {
        self.m_client.request(method, params).await
    }

    async fn raw_notify(&mut self, method: &str, params: &[msgpack_rpc::Value]) -> Result<(), ()> {
        self.m_client.notify(method, params).await;
        Ok(())
    }
}
