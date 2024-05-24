use std::{collections::HashMap, sync::Arc};

use crate::message;
use tokio::{
    io::{AsyncRead, AsyncWrite, AsyncWriteExt},
    sync::{mpsc, oneshot},
};
use tokio_util::io::SyncIoBridge;
use tracing::{debug, error, trace, warn};

use crate::error::*;

pub struct Subscription {
    pub chan: mpsc::Sender<Arc<message::Notification>>,
}

/// A request that is awaiting response from the editor. If the value is None,
/// the acknowledgement is dropped, else it is forwarded on the oneshot channel.
struct PendingRequest {
    // Keep track of the method for debugging
    method: String,
    channel: oneshot::Sender<Result<rmpv::Value>>,
}

/// Core runs the communication loop with the browser, ensures that each command
/// has a unique ID, and maintains our accounting datastructures, and posts to
/// the response oneshots when responses arrive.
pub struct Rpc<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    stream: T,
    pending_commands: HashMap<u64, PendingRequest>,
    subscriptions: Vec<Subscription>,
    id: u64,
}

impl<T> Rpc<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(stream: T) -> Self {
        Rpc {
            stream,
            pending_commands: HashMap::new(),
            subscriptions: Vec::new(),
            id: 0,
        }
    }

    fn get_id(&mut self) -> u64 {
        self.id += 1;
        self.id
    }

    pub async fn send_request(
        &mut self,
        method: String,
        params: Vec<rmpv::Value>,
    ) -> Result<rmpv::Value> {
        let id = self.get_id();
        let (tx, rx) = oneshot::channel();
        let m = message::Message::Request(message::Request {
            id,
            method: method.clone(),
            params,
        });
        // We write to a buffer, then flush it async to the stream. This should have very little
        // performance impact compared to writing directly.
        let mut buf = Vec::new();
        m.encode(&mut buf)?;
        self.stream.write_all(&buf).await?;
        self.pending_commands.insert(
            id,
            PendingRequest {
                method: method.clone(),
                channel: tx,
            },
        );
        rx.await.map_err(|_| Error::Timeout {
            method: method.clone(),
        })?
    }

    async fn run(&mut self) -> Result<()> {
        let b = SyncIoBridge::new(self.stream);
        Ok(())
    }

    // async fn handle_response(&mut self, v: Response) {
    //     if let Some(ret) = self.pending_commands.remove(&v.id) {
    //         match ret.channel.send(v.result) {
    //             Ok(_) => (),
    //             Err(e) => {
    //                 warn!("error sending command response: {:?}", e)
    //             }
    //         }
    //     } else {
    //         warn!("response for unregistered mesage: {:?}", &v.id)
    //     }
    // }
    //
    // async fn handle_notification(&mut self, v: Notification) -> Result<()> {
    //     self.subscriptions.retain(|s| !s.chan.is_closed());
    //     let n = Arc::new(v);
    //     for s in &self.subscriptions {
    //         _ = s.chan.send(n.clone()).await;
    //     }
    //     Ok(())
    // }
    //
    // /// Handle a command from the handler. Returns true if the core runloop should disconnect.
    // async fn handle_client_command(
    //     &mut self,
    //     browser_send: mpsc::UnboundedSender<MethodCall>,
    //     v: ClientMessage,
    // ) -> Result<bool> {
    //     Ok(match v {
    //         ClientMessage::Subscribe(s) => {
    //             self.subscriptions.push(s);
    //             false
    //         }
    //         ClientMessage::Exit => {
    //             debug!("received close event, exiting");
    //             true
    //         }
    //         ClientMessage::Command(v) => {
    //             let id = self.get_id();
    //             let m = MethodCall {
    //                 id,
    //                 method: v.method.clone().into(),
    //                 session_id: v.session_id,
    //                 params: v.params,
    //             };
    //             trace!(
    //                 proto_core = "",
    //                 proto_src = "core",
    //                 proto_dst = "browser",
    //                 "command: {:?}",
    //                 m
    //             );
    //             browser_send
    //                 .send(m)
    //                 .map_err(|_| Error::Disconnect("command sent to closed browser".into()))?;
    //             self.pending_commands.insert(
    //                 id,
    //                 PendingCommand {
    //                     command: v.method.clone(),
    //                     channel: v.response,
    //                 },
    //             );
    //             false
    //         }
    //     })
    // }
    //
    // pub async fn inner_run(
    //     &mut self,
    //     mut browser_recv: mpsc::UnboundedReceiver<BrowserMessage>,
    //     browser_send: mpsc::UnboundedSender<MethodCall>,
    //     mut client_recv: mpsc::UnboundedReceiver<ClientMessage>,
    //     mut bh: JoinHandle<Result<()>>,
    // ) -> Result<()> {
    //     let mut exited = false;
    //     loop {
    //         select! {
    //             _ = &mut bh => {
    //                 debug!("browser exited");
    //                 exited = true;
    //                 break
    //             }
    //             msg = browser_recv.recv() => {
    //                 match msg {
    //                     None => {
    //                         debug!("browser channel dropped, exiting");
    //                         break
    //                     }
    //                     Some(BrowserMessage::Response(v)) => {
    //                         if let Some(p) = self.pending_commands.remove(&v.id) {
    //                             self.handle_pending_response(v, p).await?;
    //                         } else {
    //                             warn!("response created by unknown sender: id={:?} resp={:?}", v.id, v.result);
    //                         }
    //                     }
    //                     Some(BrowserMessage::Event(v)) => {
    //                         self.handle_event(v).await?;
    //                     }
    //                     Some(BrowserMessage::Exit) => {
    //                         debug!("browser exited");
    //                         break
    //                     }
    //                 }
    //             }
    //             msg = client_recv.recv() => {
    //                 match msg {
    //                     None => {
    //                         debug!("all clients dropped, exiting");
    //                         break
    //                     },
    //                     Some(v) => {
    //                         if self.handle_client_command(browser_send.clone(), v).await? {
    //                             break
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     drop(browser_send);
    //     drop(browser_recv);
    //     if !exited {
    //         bh.await.map_err(Error::from_join)??;
    //     }
    //     Ok(())
    // }
    //
    // pub async fn run(
    //     mut self,
    //     browser_recv: mpsc::UnboundedReceiver<BrowserMessage>,
    //     browser_send: mpsc::UnboundedSender<MethodCall>,
    //     client_recv: mpsc::UnboundedReceiver<ClientMessage>,
    //     bh: JoinHandle<Result<()>>,
    // ) -> Result<()> {
    //     let res = self
    //         .inner_run(browser_recv, browser_send, client_recv, bh)
    //         .await;
    //
    //     if let Err(ref e) = res {
    //         warn!("core loop exited: {}", e);
    //     };
    //
    //     res
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize() {}
}
