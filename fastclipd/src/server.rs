use fast_clipboard::{config::ConfigFile, store::ClipboardStorage};

use std::net::SocketAddr;

use jsonrpsee::{
    server::{RpcModule, ServerBuilder, ServerHandle},
    SubscriptionMessage,
};

use log::{debug, info};
use tokio::sync::broadcast::Sender;

const DEFAULT_PORT: u64 = 22766;

pub struct FastclipdContext {
    pub config: ConfigFile,
    pub store: ClipboardStorage,
    pub tx: Sender<Vec<u8>>,
}

pub async fn clip_module(
    config: ConfigFile,
    store: ClipboardStorage,
    tx: Sender<Vec<u8>>,
) -> RpcModule<FastclipdContext> {
    let ctx = FastclipdContext { config, store, tx };
    let mut module = RpcModule::new(ctx);

    module
        .register_method("ping", |_, _| {
            info!("SERVER: ping");
            Ok("pong".to_string())
        })
        .unwrap();

    module
        .register_method("get_entries", |_, ctx| {
            info!("SERVER: get_entries");
            let entries = ctx.store.list_entries();
            let s = serde_json::to_string(entries).unwrap();
            Ok(s)
        })
        .unwrap();

    module
        .register_subscription(
            "subscribe_entry",
            "s_entry",
            "unsubscribe_entry",
            |_, pending, ctx| async move {
                let mut rx = ctx.tx.subscribe();
                let sink = pending.accept().await.unwrap();
                let res = rx.recv().await.unwrap();
                let msg = SubscriptionMessage::from_json(&res).unwrap();
                sink.send(msg).await.unwrap();
                Ok(())
            },
        )
        .unwrap();

    module
}

pub async fn run_server(
    clip_mod: RpcModule<FastclipdContext>,
) -> anyhow::Result<(SocketAddr, ServerHandle)> {
    let ws_addr = default_addr();
    let server = ServerBuilder::new().build(ws_addr).await?;

    let addr = server.local_addr()?;
    let handle = server.start(clip_mod)?;

    Ok((addr, handle))
}

fn default_addr() -> String {
    format!("127.0.0.1:{}", DEFAULT_PORT)
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use futures::StreamExt;
    use jsonrpsee::{
        core::client::{ClientT, Subscription, SubscriptionClientT},
        rpc_params,
        ws_client::WsClientBuilder,
    };
    use log::debug;
    use tokio::sync::broadcast;

    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_server_can_run() {
        let (tx, _rx) = broadcast::channel::<Vec<u8>>(16);
        let config = ConfigFile::default();
        let store = ClipboardStorage::default();

        let clip_mod = clip_module(config, store, tx).await;
        let (addr, handle) = run_server(clip_mod).await.unwrap();
        let client = WsClientBuilder::default()
            .build(format!("ws://{}", &addr))
            .await
            .unwrap();
        let response: String = client.request("ping", rpc_params![]).await.unwrap();
        assert_eq!(response, "pong");
        handle.stop().unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_can_receive_clipboard_messages() {
        let (tx, _rx) = broadcast::channel::<Vec<u8>>(16);
        let config = ConfigFile::default();
        let store = ClipboardStorage::default();

        let clip_mod = clip_module(config, store, tx.clone()).await;
        let (addr, handle) = run_server(clip_mod).await.unwrap();
        let client = WsClientBuilder::default()
            .build(format!("ws://{}", &addr))
            .await
            .unwrap();
        let sub: Subscription<Vec<u8>> = client
            .subscribe("subscribe_entry", rpc_params![], "unsubscribe_entry")
            .await
            .unwrap();

        tx.send("Something copied".as_bytes().to_vec()).unwrap();
        sub.take(1)
            .for_each(|res| async move {
                debug!("res: {:?}", res);
                assert!(res.is_ok());
            })
            .await;
        handle.stop().unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_can_get_entries() {
        let (tx, _rx) = broadcast::channel::<Vec<u8>>(16);
        let config = ConfigFile::default();
        let store = ClipboardStorage::default();

        let clip_mod = clip_module(config, store, tx).await;
        let (addr, handle) = run_server(clip_mod).await.unwrap();
        let client = WsClientBuilder::default()
            .build(format!("ws://{}", &addr))
            .await
            .unwrap();
        let response: String = client.request("get_entries", rpc_params![]).await.unwrap();
        assert_eq!(response, "[]");
        handle.stop().unwrap();
    }
}
