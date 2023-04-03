use jsonrpsee::{core::client::Client, ws_client::WsClientBuilder};

const DEFAULT_PORT: u64 = 22766;

pub fn default_addr() -> String {
    format!("127.0.0.1:{}", DEFAULT_PORT)
}

pub async fn connect() -> anyhow::Result<Client> {
    let addr = default_addr();
    let client = WsClientBuilder::default()
        .build(format!("ws://{}", &addr))
        .await?;

    Ok(client)
}
