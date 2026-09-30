mod auth;
mod config;
mod db;
mod error;
mod executor_client;
mod routes;

use std::sync::Arc;

use anyhow::Context;
use auth::Authenticator;
use axum::Router;
use config::Config;
use reqwest::Client;
use sqlx::PgPool;
use tokio::{net::TcpListener, signal};
use tracing::{debug, info, warn};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub auth: Authenticator,
    pub http_client: Client,
    pub config: Arc<Config>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let config = Arc::new(Config::from_env());
    debug!(
        host = %config.host,
        port = config.port,
        executor_base_url = %config.executor_base_url,
        jwt_issuer = %config.jwt_issuer,
        jwt_audience = %config.jwt_audience,
        jwt_jwks_url = %config.jwt_jwks_url,
        hs256_enabled = config.jwt_hs256_secret.is_some(),
        "loaded API configuration"
    );

    let pool = db::connect(&config.database_url)
        .await
        .context("failed to connect to postgres")?;
    debug!("running database migrations");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("failed to run migrations")?;
    debug!("database migrations applied successfully");

    let auth = if let Some(secret) = config.jwt_hs256_secret.clone() {
        debug!(
            issuer = %config.jwt_issuer,
            audience = %config.jwt_audience,
            "initializing HS256 authenticator"
        );
        Authenticator::hs256(
            secret,
            config.jwt_issuer.clone(),
            config.jwt_audience.clone(),
        )
    } else {
        debug!(
            issuer = %config.jwt_issuer,
            audience = %config.jwt_audience,
            jwks_url = %config.jwt_jwks_url,
            "initializing JWKS authenticator"
        );
        Authenticator::jwks(
            config.jwt_issuer.clone(),
            config.jwt_audience.clone(),
            config.jwt_jwks_url.clone(),
            Client::new(),
        )
    };

    let state = AppState {
        db: pool,
        auth,
        http_client: Client::new(),
        config: Arc::clone(&config),
    };

    let bind_address = config.bind_address();
    let listener = TcpListener::bind(&bind_address)
        .await
        .with_context(|| format!("failed to bind API listener at {bind_address}"))?;

    info!(address = %bind_address, "starting API server");

    let app = Router::new()
        .merge(routes::router(state))
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
