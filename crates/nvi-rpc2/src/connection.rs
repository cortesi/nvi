use async_trait::async_trait;
use rmpv::Value;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::sync::{mpsc, oneshot};

use crate::error::{Result, RpcError, ServiceError};
use crate::message::*;

#[derive(Debug)]
enum ClientMessage {
    Request {
        method: String,
        params: Vec<Value>,
        response_sender: oneshot::Sender<Result<Value>>,
    },
    Notification {
        method: String,
        params: Vec<Value>,
    },
}

#[derive(Debug, Clone)]
pub struct Client {
    sender: mpsc::Sender<ClientMessage>,
}

impl Client {
    pub async fn send_request(&self, method: String, params: Vec<Value>) -> Result<Value> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send(ClientMessage::Request {
                method,
                params,
                response_sender,
            })
            .await
            .map_err(|_| RpcError::Protocol("Failed to send request".to_string()))?;
        response_receiver
            .await
            .map_err(|_| RpcError::Protocol("Failed to receive response".to_string()))?
    }

    pub async fn send_notification(&self, method: String, params: Vec<Value>) -> Result<()> {
        self.sender
            .send(ClientMessage::Notification { method, params })
            .await
            .map_err(|_| RpcError::Protocol("Failed to send notification".to_string()))
    }
}

pub struct ConnectionHandler<S, T: RpcService> {
    connection: RpcConnection<S>,
    service: Arc<T>,
    client_receiver: mpsc::Receiver<ClientMessage>,
    client: Client,
}

impl<S, T: RpcService> ConnectionHandler<S, T>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    pub fn new(connection: RpcConnection<S>, service: Arc<T>) -> Self {
        let (sender, receiver) = mpsc::channel(100);
        let client = Client { sender };
        Self {
            connection,
            service,
            client_receiver: receiver,
            client,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            tokio::select! {
                Some(client_message) = self.client_receiver.recv() => {
                    self.handle_client_message(client_message).await?;
                }
                message_result = self.connection.read_message() => {
                    let message = message_result?;
                    self.handle_incoming_message(message).await?;
                }
            }
        }
    }

    async fn handle_client_message(&mut self, message: ClientMessage) -> Result<()> {
        match message {
            ClientMessage::Request {
                method,
                params,
                response_sender,
            } => {
                let id = self.connection.next_request_id;
                self.connection.next_request_id += 1;
                self.connection.pending_requests.insert(id, response_sender);
                let request = Request { id, method, params };
                self.connection
                    .write_message(&Message::Request(request))
                    .await?;
            }
            ClientMessage::Notification { method, params } => {
                let notification = Notification { method, params };
                self.connection
                    .write_message(&Message::Notification(notification))
                    .await?;
            }
        }
        Ok(())
    }

    async fn handle_incoming_message(&mut self, message: Message) -> Result<()> {
        match message {
            Message::Request(request) => {
                let result = self
                    .service
                    .handle_request::<S>(self.client.clone(), &request.method, request.params)
                    .await;
                let response = match result {
                    Ok(value) => Response {
                        id: request.id,
                        result: Ok(value),
                    },
                    Err(RpcError::Service(service_error)) => Response {
                        id: request.id,
                        result: Err(service_error.into()),
                    },
                    Err(e) => return Err(e),
                };
                self.connection
                    .write_message(&Message::Response(response))
                    .await?;
            }
            Message::Notification(notification) => {
                self.service
                    .handle_notification::<S>(
                        self.client.clone(),
                        &notification.method,
                        notification.params,
                    )
                    .await;
            }
            Message::Response(response) => {
                if let Some(sender) = self.connection.pending_requests.remove(&response.id) {
                    let _ = sender.send(response.result.map_err(|e| {
                        RpcError::Service(ServiceError {
                            name: "RemoteError".to_string(),
                            value: e,
                        })
                    }));
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
pub trait RpcService: Send + Sync + Clone + 'static {
    async fn handle_request<S>(
        &self,
        client: Client,
        method: &str,
        params: Vec<Value>,
    ) -> Result<Value>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static;

    async fn handle_notification<S>(&self, client: Client, method: &str, params: Vec<Value>)
    where
        S: AsyncRead + AsyncWrite + Unpin + Send + 'static;
}

#[derive(Debug)]
pub struct RpcConnection<S> {
    stream: S,
    next_request_id: u32,
    pending_requests: std::collections::HashMap<u32, oneshot::Sender<Result<Value>>>,
}

impl<S> AsyncRead for RpcConnection<S>
where
    S: AsyncRead + Unpin,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.stream).poll_read(cx, buf)
    }
}

impl<S> AsyncWrite for RpcConnection<S>
where
    S: AsyncWrite + Unpin,
{
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::result::Result<usize, std::io::Error>> {
        Pin::new(&mut self.stream).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), std::io::Error>> {
        Pin::new(&mut self.stream).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), std::io::Error>> {
        Pin::new(&mut self.stream).poll_shutdown(cx)
    }
}

impl<S> RpcConnection<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            next_request_id: 1,
            pending_requests: std::collections::HashMap::new(),
        }
    }

    pub async fn read_message(&mut self) -> Result<Message> {
        // Implement message reading logic
        todo!("Implement message reading from the stream")
    }

    pub async fn write_message(&mut self, _message: &Message) -> Result<()> {
        // Implement message writing logic
        todo!("Implement message writing to the stream")
    }
}
