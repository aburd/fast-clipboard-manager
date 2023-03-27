mod server;
mod tracker;

use log::{error, info};
use tokio::sync::broadcast;
use tracker::Tracker;

#[tokio::main]
async fn main() {
    env_logger::init();

    let (tx, _rx) = broadcast::channel::<Vec<u8>>(16);
    let tracker_tx = tx.clone();
    info!("Starting fastclipd");
    tokio::spawn(async move {
        loop {
            info!("Starting tracker");
            let tracker = Tracker::new();
            let s = tracker.await;
            tracker_tx.send(s).unwrap();
        }
    });

    let clip_mod = server::clip_module(tx).await;

    info!("Fastclipd server starting");
    let (_addr, handle) = server::run_server(clip_mod).await.unwrap();
    info!("Fastclipd server ready for connections");

    handle.stopped().await;
    info!("Server stopped");
}
