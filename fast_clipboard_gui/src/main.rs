#![warn(clippy::all, rust_2018_idioms)]

mod components;
mod gui;
mod ws_client;

use gui::AppErr;
use jsonrpsee::core::client::Client;
use log::{debug, error, info};

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("logger initiated");

    info!("connecting to faclipdd");
    let client_res: anyhow::Result<Client, AppErr> = ws_client::connect().await.map_err(|e| {
        error!("connection to fastclipd failed: {}", e);
        AppErr::CantConnectToDaemon(e.to_string())
    });

    info!("starting app");
    gui::run_app(client_res).await;
    debug!("app exited");
}
