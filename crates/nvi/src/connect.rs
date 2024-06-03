use std::{net::SocketAddr, path::Path};

use futures::io::{self, AsyncRead, AsyncWrite};

use msgpack_rpc::Endpoint;
use tokio::{
    net::{TcpListener, TcpStream, UnixListener, UnixStream},
    sync::broadcast,
    task::JoinSet,
};
use tokio_util::compat::TokioAsyncReadCompatExt;
use tracing::{debug, error, trace, warn};

use crate::{
    error::Result,
    service::{ServiceWrapper, BOOTSTRAP_NOTIFICATION},
    NviClient, NviService,
};

async fn bootstrap(c: msgpack_rpc::Client, shutdown_tx: broadcast::Sender<()>) -> Result<()> {
    let nc = &mut NviClient::new(&c, None, shutdown_tx);
    let (id, _v) = nc.api.nvim_get_api_info().await?;
    nc.api
        .nvim_exec_lua(
            &format!("vim.rpcnotify(..., '{}', ...)", BOOTSTRAP_NOTIFICATION),
            vec![id.into()],
        )
        .await?;
    Ok(())
}

/// A wrapper around connect_stream that doesn't fail.
async fn connect_listener<T, S>(shutdown_tx: broadcast::Sender<()>, stream: S, service: T)
where
    S: AsyncRead + AsyncWrite + Send + 'static,
    T: NviService + Unpin + 'static,
{
    trace!("connection started");
    let err = connect_stream(shutdown_tx, stream, service).await;
    match err {
        Ok(_) => trace!("Connection closed"),
        Err(e) => warn!("Error on connection: {}", e),
    }
}

/// Connect on a stream, and return a sender to shutdown the connection.
pub async fn connect_stream<T, S>(
    shutdown_tx: broadcast::Sender<()>,
    stream: S,
    service: T,
) -> Result<()>
where
    S: AsyncRead + AsyncWrite + Send + 'static,
    T: NviService + Unpin + 'static,
{
    let endpoint = Endpoint::new(stream, ServiceWrapper::new(service, shutdown_tx.clone()));

    let mut js = JoinSet::new();
    let epclient = endpoint.client();
    {
        let stx = shutdown_tx.clone();
        js.spawn(async move { bootstrap(epclient, stx).await });
    }
    {
        let stx = shutdown_tx.clone();
        js.spawn(async move {
            let ret = endpoint.await.map_err(|e| e.into());
            stx.send(()).unwrap();
            ret
        });
    }
    let _ = shutdown_tx.subscribe().recv().await;
    js.abort_all();
    while js.join_next().await.is_some() {}
    Ok(())
}

/// Connect on a Unix socket, and return a sender to shutdown the connection.
pub async fn connect_unix<T, P>(
    shutdown_tx: broadcast::Sender<()>,
    path: P,
    service: T,
) -> Result<()>
where
    P: AsRef<Path>,
    T: NviService + Unpin + 'static,
{
    connect_stream(
        shutdown_tx,
        UnixStream::connect(path).await?.compat(),
        service,
    )
    .await
}

/// Connect to a TCP address, and return a sender to shutdown the connection.
pub async fn connect_tcp<T>(
    shutdown_tx: broadcast::Sender<()>,
    addr: SocketAddr,
    service: T,
) -> Result<()>
where
    T: NviService + Unpin + 'static,
{
    connect_stream(
        shutdown_tx,
        TcpStream::connect(&addr).await?.compat(),
        service,
    )
    .await
}

pub async fn listen_unix<T, F, P>(
    shutdown_tx: broadcast::Sender<()>,
    path: P,
    nvi_service_maker: F,
) -> Result<()>
where
    F: Fn() -> T + Send + 'static,
    T: NviService + Unpin + 'static,
    P: AsRef<Path>,
{
    let listener = UnixListener::bind(path)?;
    let mut shutdown_rx = shutdown_tx.subscribe();
    let _ = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    trace!("shutdown signal received, stopping listener.");
                    break;
                }
                result = listener.accept() => {
                    match result {
                        Ok((socket, _)) => {
                            connect_listener(shutdown_tx.clone(), socket.compat(), nvi_service_maker())
                                .await
                        }
                        Err(e) => error!("Error accepting connection: {}", e),
                    }
                }
            }
        }
    })
    .await;
    Ok(())
}

pub async fn listen_tcp<T, F>(
    shutdown_tx: broadcast::Sender<()>,
    addr: SocketAddr,
    nvi_service_maker: F,
) -> io::Result<()>
where
    F: Fn() -> T + Send + 'static,
    T: NviService + Unpin + 'static,
{
    let listener = TcpListener::bind(&addr).await?;
    let _ = tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((socket, _)) => {
                    connect_listener(shutdown_tx.clone(), socket.compat(), nvi_service_maker())
                        .await
                }
                Err(e) => debug!("Error accepting connection: {}", e),
            }
        }
    })
    .await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use tokio::sync::broadcast;
    use tracing_test::traced_test;

    use crate::test;

    #[tokio::test]
    #[traced_test]
    async fn it_listens() {
        let (tx, _) = broadcast::channel(16);

        // Start a listener on a socket
        let tempdir = tempfile::tempdir().unwrap();
        let socket_path = tempdir.path().join("listen.socket");
        let itx = tx.clone();
        let listener = listen_unix(itx.clone(), socket_path.clone(), move || {
            let itx = itx.clone();
            crate::AsyncClosureService::new(move |client| {
                let itx = itx.clone();
                Box::pin({
                    let tx = itx.clone();
                    async move {
                        tx.send(()).unwrap();
                        client.shutdown();
                    }
                })
            })
        });
        let ls = tokio::spawn(listener);

        test::ensure_path(&socket_path).await.unwrap();

        // Now start a nvim instance, and connect to it with a client. Using the client, we
        // instruct nvim to connect back to the listener.
        let ts = tokio::spawn(test::test_service(
            crate::AsyncClosureService::new(move |c| {
                let socket_path = socket_path.clone();
                Box::pin({
                    async move {
                        trace!("client connected, sending sockconnect request");
                        c.api
                            .nvim_exec_lua(
                                &format!(
                                    "vim.fn.sockconnect('pipe', '{}',  {{rpc = true}});",
                                    socket_path.as_os_str().to_string_lossy()
                                ),
                                vec![],
                            )
                            .await
                            .unwrap();
                    }
                })
            }),
            tx.clone(),
        ));

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
        let s = crate::AsyncClosureService::new(move |client| {
            Box::pin({
                let tx = tx.clone();
                async move {
                    tx.send(()).unwrap();
                    client.shutdown();
                }
            })
        });
        test::test_service(s, rtx).await.unwrap();
    }
}
