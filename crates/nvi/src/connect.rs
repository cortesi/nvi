//! Functions to listen for and establish connections between Nvi plugins and Neovim instances.
//!
//! Provides two main connection patterns:
//! - Listen for incoming connections on Unix domain sockets and TCP ports
//! - Connect to a running Neovim instance through Unix domain sockets and TCP
//!
//! Each function takes a shutdown broadcast channel that can be used to gracefully terminate
//! the connection.

use std::{net::SocketAddr, path::Path};
use tokio::sync::broadcast;
use tracing::{error, trace};

use crate::error::Result;
use crate::service::{NviPlugin, RpcConnection};
use mrpc::{Client, ConnectionMakerFn, Server};

/// Listen for incoming connections on a Unix domain socket.
///
/// Creates a Unix domain socket at the specified path and listens for incoming connections.
/// For each connection, creates a new plugin instance using the provided factory function.
///
/// * `shutdown_tx` - Broadcast channel for shutdown signals
/// * `path` - Path where the Unix domain socket will be created
/// * `make_plugin` - Factory function to create plugin instances
pub async fn listen_unix<T, F>(
    shutdown_tx: broadcast::Sender<()>,
    path: impl AsRef<Path>,
    make_plugin: F,
) -> Result<()>
where
    T: NviPlugin + Send + Sync + 'static,
    F: Fn() -> T + Send + Sync + 'static,
{
    let path = path.as_ref();
    let itx = shutdown_tx.clone();
    let maker = ConnectionMakerFn::new(move || RpcConnection::new(itx.clone(), make_plugin()));
    let server = Server::from_maker(maker).unix(path).await?;
    let mut shutdown_rx = shutdown_tx.subscribe();
    tokio::select! {
        result = server.run() => {
            if let Err(e) = result {
                error!("Server error: {}", e);
            }
        }
        _ = shutdown_rx.recv() => {
            trace!("Shutdown signal received, stopping listener.");
        }
    }
    let _ = std::fs::remove_file(path);
    Ok(())
}

/// Listen for incoming connections on a TCP socket.
///
/// Binds to the specified address and listens for incoming TCP connections.
/// For each connection, creates a new plugin instance using the provided factory function.
///
/// * `shutdown_tx` - Broadcast channel for shutdown signals
/// * `addr` - TCP socket address to bind to
/// * `make_plugin` - Factory function to create plugin instances
pub async fn listen_tcp<T, F>(
    shutdown_tx: broadcast::Sender<()>,
    addr: SocketAddr,
    make_plugin: F,
) -> Result<()>
where
    T: NviPlugin + Send + Sync + 'static,
    F: Fn() -> T + Send + Sync + 'static,
{
    let itx = shutdown_tx.clone();
    let maker = ConnectionMakerFn::new(move || {
        let plugin = make_plugin();
        RpcConnection::new(itx.clone(), plugin)
    });
    let server = Server::from_maker(maker).tcp(&addr.to_string()).await?;

    let mut shutdown_rx = shutdown_tx.subscribe();
    tokio::select! {
        result = server.run() => {
            if let Err(e) = result {
                error!("Server error: {}", e);
            }
        }
        _ = shutdown_rx.recv() => {
            trace!("Shutdown signal received, stopping listener.");
        }
    }
    Ok(())
}

/// Connect to a Neovim instance through a Unix domain socket.
///
/// Establishes a connection to an existing Neovim instance via a Unix domain socket
/// and starts the plugin.
///
/// * `shutdown_tx` - Broadcast channel for shutdown signals
/// * `path` - Path to the Unix domain socket to connect to
/// * `plugin` - Plugin instance to run
pub async fn connect_unix<T, P>(
    shutdown_tx: broadcast::Sender<()>,
    path: P,
    plugin: T,
) -> Result<()>
where
    P: AsRef<Path>,
    T: NviPlugin + Send + Sync + 'static,
{
    let rpc_conn = RpcConnection::new(shutdown_tx.clone(), plugin);
    let client = Client::connect_unix(path, rpc_conn).await?;
    handle_client(shutdown_tx.subscribe(), client).await
}

/// Connect to a Neovim instance through a TCP socket.
///
/// Establishes a connection to an existing Neovim instance via TCP
/// and starts the plugin.
///
/// * `shutdown_tx` - Broadcast channel for shutdown signals
/// * `addr` - TCP address to connect to
/// * `plugin` - Plugin instance to run
pub async fn connect_tcp<T>(
    shutdown_tx: broadcast::Sender<()>,
    addr: SocketAddr,
    plugin: T,
) -> Result<()>
where
    T: NviPlugin + Send + Sync + 'static,
{
    let rpc_conn = RpcConnection::new(shutdown_tx.clone(), plugin);
    let client = Client::connect_tcp(&addr.to_string(), rpc_conn).await?;
    handle_client(shutdown_tx.subscribe(), client).await
}

async fn handle_client<T: mrpc::Connection + Send + Sync + 'static>(
    mut shutdown_rx: broadcast::Receiver<()>,
    client: Client<T>,
) -> Result<()> {
    tokio::select! {
        _ = client.join()  => {
            trace!("Client connection closed.");
            Ok(())
        }
        _ = shutdown_rx.recv() => {
            trace!("Shutdown signal received, closing connection.");
            Ok(())
        }
        _ = tokio::signal::ctrl_c() => {
            trace!("Ctrl-C received, closing connection.");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lua_exec;
    use crate::NviPlugin;
    use std::path::PathBuf;
    use tokio::sync::broadcast;
    use tracing_test::traced_test;

    #[derive(Clone)]
    struct TestPlugin {
        tx: broadcast::Sender<()>,
    }

    #[async_trait::async_trait]
    impl NviPlugin for TestPlugin {
        fn name(&self) -> String {
            "TestPlugin".into()
        }

        async fn connected(&mut self, client: &mut crate::Client) -> crate::error::Result<()> {
            self.tx.send(()).unwrap();
            client.shutdown();
            Ok(())
        }
    }

    #[tokio::test]
    #[traced_test]
    async fn it_listens() {
        let (tx, _) = broadcast::channel(16);

        // Start a listener on a socket
        let tempdir = tempfile::tempdir().unwrap();
        let socket_path = tempdir.path().join("listen.socket");
        let itx = tx.clone();
        let listener = listen_unix(itx.clone(), socket_path.clone(), move || TestPlugin {
            tx: itx.clone(),
        });
        let ls = tokio::spawn(listener);

        crate::test::wait_for_path(&socket_path).await.unwrap();

        async fn tserv(
            socket_path: PathBuf,
            tx: broadcast::Sender<()>,
        ) -> crate::error::Result<()> {
            #[derive(Clone)]
            struct SockConnectPlugin {
                socket_path: PathBuf,
            }

            #[async_trait::async_trait]
            impl NviPlugin for SockConnectPlugin {
                fn name(&self) -> String {
                    "SockConnectPlugin".into()
                }

                async fn connected(
                    &mut self,
                    client: &mut crate::Client,
                ) -> crate::error::Result<()> {
                    trace!("client connected, sending sockconnect request");
                    let _ = lua_exec!(
                        client,
                        "vim.fn.sockconnect('pipe', ...,  {{rpc = true}});",
                        self.socket_path.as_os_str().to_string_lossy()
                    )
                    .await?;
                    Ok(())
                }
            }

            let _handle = crate::test::run_plugin_with_shutdown(
                SockConnectPlugin { socket_path },
                tx.clone(),
            )
            .await;
            Ok(())
        }

        // Now start a nvim instance, and connect to it with a client. Using the client, we
        // instruct nvim to connect back to the listener.
        let ts = tokio::spawn(tserv(socket_path.clone(), tx.clone()));
        ts.await.unwrap().unwrap();

        ls.await.unwrap().unwrap();

        // We only get here if the listener has been connected to, and has sent the termination
        // signal.
    }

    #[tokio::test]
    #[traced_test]
    async fn it_connects() {
        let (tx, _) = broadcast::channel(16);
        let rtx = tx.clone();
        let s = TestPlugin { tx };
        crate::test::run_plugin_with_shutdown(s, rtx).await.unwrap();
    }
}
