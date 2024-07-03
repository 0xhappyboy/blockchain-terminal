use app::App;
use std::io;
mod app;
mod event;
mod i18n;
mod router;
mod tui;
mod ui;
mod widget;
use anyhow::{Context, Result};
use aptos_sdk::{
    coin_client::CoinClient,
    rest_client::{Client, FaucetClient},
    types::{LocalAccount, PeerId},
};
use once_cell::sync::Lazy;
use std::{any::Any, fmt::Debug, str::FromStr};
use tokio;
use url::Url;

// :!:>section_1c
static NODE_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("APTOS_NODE_URL")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://fullnode.devnet.aptoslabs.com"),
    )
    .unwrap()
});

static FAUCET_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("APTOS_FAUCET_URL")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://faucet.devnet.aptoslabs.com"),
    )
    .unwrap()
});
// <:!:section_1c

#[tokio::main]
async fn main() -> io::Result<()> {
    // :!:>section_1a
    let rest_client = Client::new(NODE_URL.clone());
    let faucet_client = FaucetClient::new(FAUCET_URL.clone(), NODE_URL.clone()); // <:!:section_1a

    // :!:>section_1b
    let coin_client = CoinClient::new(&rest_client); // <:!:section_1b

    println!("ok");

    println!("v: {:?}", rest_client.get_aptos_version().await.unwrap());

    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
