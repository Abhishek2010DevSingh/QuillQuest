use anyhow::Context;
use axum::Router;
use dotenv::dotenv;
use quill_quest::config::{env_provider::EnvConfigProvider, interface::ConfigProvider};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().context("Failed to load .env file")?;
    let config_provider = EnvConfigProvider::new().context("Failed to create EnvConfigProvider")?;

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let port = config_provider
        .get_port()
        .context("Failed to retrieve port from configuration")?;

    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .context(format!("Failed to bind to port {port}"))?;

    let service_bulider = ServiceBuilder::new().layer(TraceLayer::new_for_http());

    let router = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(service_bulider);

    tracing::debug!("Server started at http://localhost:{port}/");
    axum::serve(listener, router)
        .await
        .context("Server encountered an error")
}
