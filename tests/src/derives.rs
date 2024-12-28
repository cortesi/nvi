use nvi::test;
use nvi_macros::*;

use tokio::sync::broadcast;
use tracing::trace;
use tracing_test::traced_test;

#[tokio::test]
#[traced_test]
async fn it_derives_basic_service() {
    #[derive(Clone)]
    struct TestService {
        tx: broadcast::Sender<()>,
    }

    #[nvi_service]
    impl TestService {
        async fn connected(&self, _: &mut nvi::Client) -> nvi::error::Result<()> {
            trace!("connected");
            self.tx.send(()).unwrap();
            Ok(())
        }
    }

    let (tx, _) = broadcast::channel(16);
    test::run_plugin_with_shutdown(TestService { tx: tx.clone() }, tx)
        .await
        .unwrap();
    assert!(logs_contain("connected"));
}

#[tokio::test]
#[traced_test]
async fn it_derives_autocmd_handler() {
    #[derive(Clone)]
    struct TestService {}

    #[nvi_service]
    impl TestService {
        #[autocmd(["User"], patterns=["*.rs"])]
        async fn on_user_event(&self, client: &mut nvi::Client) -> nvi::error::Result<()> {
            trace!("user event received");
            client.shutdown();
            Ok(())
        }

        async fn connected(&self, client: &mut nvi::Client) -> nvi::error::Result<()> {
            use nvi::opts::ExecAutocmds;
            client
                .nvim
                .exec_autocmds(
                    &[nvi::types::Event::User],
                    ExecAutocmds {
                        pattern: Some(vec!["*.rs".to_string()]),
                        ..Default::default()
                    },
                )
                .await?;
            Ok(())
        }
    }

    let (tx, _) = broadcast::channel(16);
    test::run_plugin_with_shutdown(TestService {}, tx)
        .await
        .unwrap();
    assert!(logs_contain("user event received"));
}
