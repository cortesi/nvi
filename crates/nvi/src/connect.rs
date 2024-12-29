use std::{net::SocketAddr, path::Path};
use tokio::sync::broadcast;
use tracing::{error, trace};

use crate::error::Result;
use crate::service::{NviPlugin, RpcConnection};
use mrpc::{Client, ConnectionMakerFn, Server};

pub async fn listen_unix<T, F>(
    shutdown_tx: broadcast::Sender<()>,
    path: impl AsRef<Path>,
    make_service: F,
) -> Result<()>
where
    T: NviPlugin + Clone + Send + Sync + 'static,
    F: Fn() -> T + Send + Sync + 'static,
{
    let path = path.as_ref();
    let itx = shutdown_tx.clone();
    let maker = ConnectionMakerFn::new(move || {
        let service = make_service();
        RpcConnection::new(itx.clone(), service)
    });
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

pub async fn listen_tcp<T, F>(
    shutdown_tx: broadcast::Sender<()>,
    addr: SocketAddr,
    make_service: F,
) -> Result<()>
where
    T: NviPlugin + Clone + Send + Sync + 'static,
    F: Fn() -> T + Send + Sync + 'static,
{
    let itx = shutdown_tx.clone();
    let maker = ConnectionMakerFn::new(move || {
        let service = make_service();
        RpcConnection::new(itx.clone(), service)
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

pub async fn connect_unix<T, P>(
    shutdown_tx: broadcast::Sender<()>,
    path: P,
    service: T,
) -> Result<()>
where
    P: AsRef<Path>,
    T: NviPlugin + Clone + Send + Sync + 'static,
{
    let wrapped_service = RpcConnection::new(shutdown_tx.clone(), service);
    let client = Client::connect_unix(path, wrapped_service).await?;
    handle_client(shutdown_tx.subscribe(), client).await
}

pub async fn connect_tcp<T>(
    shutdown_tx: broadcast::Sender<()>,
    addr: SocketAddr,
    service: T,
) -> Result<()>
where
    T: NviPlugin + Clone + Send + Sync + 'static,
{
    let wrapped_service = RpcConnection::new(shutdown_tx.clone(), service);
    let client = Client::connect_tcp(&addr.to_string(), wrapped_service).await?;
    handle_client(shutdown_tx.subscribe(), client).await
}

async fn handle_client<T: mrpc::Connection + Clone + Send + Sync + 'static>(
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
    use crate::NviPlugin;
    use std::path::PathBuf;
    use tokio::sync::broadcast;
    use tracing_test::traced_test;

    #[derive(Clone)]
    struct TestService {
        tx: broadcast::Sender<()>,
    }

    #[async_trait::async_trait]
    impl NviPlugin for TestService {
        fn name(&self) -> String {
            "TestService".into()
        }

        async fn connected(&self, client: &mut crate::Client) -> crate::error::Result<()> {
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
        let listener = listen_unix(itx.clone(), socket_path.clone(), move || TestService {
            tx: itx.clone(),
        });
        let ls = tokio::spawn(listener);

        crate::test::wait_for_path(&socket_path).await.unwrap();

        async fn tserv(
            socket_path: PathBuf,
            tx: broadcast::Sender<()>,
        ) -> crate::error::Result<()> {
            #[derive(Clone)]
            struct SockConnectService {
                socket_path: PathBuf,
            }

            #[async_trait::async_trait]
            impl NviPlugin for SockConnectService {
                fn name(&self) -> String {
                    "SockConnectService".into()
                }

                async fn connected(&self, client: &mut crate::Client) -> crate::error::Result<()> {
                    trace!("client connected, sending sockconnect request");
                    client
                        .nvim
                        .exec_lua(
                            &format!(
                                "vim.fn.sockconnect('pipe', '{}',  {{rpc = true}});",
                                self.socket_path.as_os_str().to_string_lossy()
                            ),
                            vec![],
                        )
                        .await
                        .unwrap();
                    Ok(())
                }
            }

            let _handle = crate::test::run_plugin_with_shutdown(
                SockConnectService { socket_path },
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
        let s = TestService { tx };
        crate::test::run_plugin_with_shutdown(s, rtx).await.unwrap();
    }
}
