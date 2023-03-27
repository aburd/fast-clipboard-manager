use crate::tracker::TrackerSender;

use std::net::SocketAddr;

use jsonrpsee::{
    server::{RpcModule, ServerBuilder, ServerHandle},
    SubscriptionMessage,
};

use log::info;
use tokio::sync::broadcast::Sender;

const DEFAULT_PORT: u64 = 22766;

pub async fn clip_module(tx: Sender<Vec<u8>>) -> RpcModule<TrackerSender> {
    let mut module = RpcModule::new(tx.clone());
    module
        .register_method("ping", |a, b| {
            info!("SERVER: ping");
            Ok("pong".to_string())
        })
        .unwrap();
    module
        .register_method("get_entries", |a, b| {
            info!("SERVER: get_entries");
            Ok("Hello world")
        })
        .unwrap();
    module
        .register_subscription(
            "subscribe_entry",
            "s_entry",
            "unsubscribe_entry",
            |_, pending, tx| async move {
                let mut rx = tx.subscribe();
                let res = rx.recv().await?;
                let sink = pending.accept().await?;
                let msg = SubscriptionMessage::from_json(&res).unwrap();
                sink.send(msg).await.unwrap();
                Ok(())
            },
        )
        .unwrap();

    module
}

pub async fn run_server(
    clip_mod: RpcModule<TrackerSender>,
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
    use jsonrpsee::{core::client::ClientT, rpc_params, ws_client::WsClientBuilder};
    use tokio::sync::broadcast;

    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_server_can_run() {
        let (tx, _rx) = broadcast::channel::<Vec<u8>>(16);
        let clip_mod = clip_module(tx).await;
        let (addr, handle) = run_server(clip_mod).await.unwrap();
        let client = WsClientBuilder::default()
            .build(format!("ws://{}", &addr))
            .await
            .unwrap();
        let response: String = client.request("ping", rpc_params![]).await.unwrap();
        assert_eq!(response, "pong");
        handle.stop().unwrap();
    }
}
