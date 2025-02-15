use anyhow::Context;
use axum::Router;
use dotenv::dotenv;
use quill_quest::config::{env_provider::EnvConfigProvider, interface::ConfigProvider};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().context("Failed to load .env file")?;
    let config_provider = EnvConfigProvider::new().context("Failed to create EnvConfigProvider")?;

    let port = config_provider
        .get_port()
        .context("Failed to retrieve port from configuration")?;

    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .context(format!("Failed to bind to port {port}"))?;

    let router = Router::new();

    axum::serve(listener, router)
        .await
        .context("Server encountered an error")
}
