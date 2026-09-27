mod config;
mod routes;

use anyhow::Context;
use axum::Router;
use tokio::{net::TcpListener, signal};
use tracing::{info, warn};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let config = config::Config::from_env();
    let bind_address = config.bind_address();
    let listener = TcpListener::bind(&bind_address)
        .await
        .with_context(|| format!("failed to bind API listener at {bind_address}"))?;

    info!(address = %bind_address, "starting API server");

    let app = Router::new()
        .merge(routes::router())
        .fallback(routes::not_found);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("API server terminated unexpectedly")?;

    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer())
        .init();
}

async fn shutdown_signal() {
    if let Err(error) = signal::ctrl_c().await {
        warn!(%error, "failed to listen for shutdown signal");
    }

    info!("shutdown signal received");
}
