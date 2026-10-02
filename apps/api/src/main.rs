mod auth;
mod config;
mod db;
mod error;
mod executor_client;
mod routes;

use std::sync::Arc;

use anyhow::Context;
use auth::Authenticator;
use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
};
use config::Config;
use reqwest::Client;
use sqlx::PgPool;
use tokio::{net::TcpListener, signal};
use tower_http::cors::{AllowOrigin, CorsLayer};
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
        allowed_origins = ?config.allowed_origins,
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

    let app = build_app(state).context("failed to configure API router")?;

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

fn build_app(state: AppState) -> anyhow::Result<Router> {
    let cors = cors_layer(&state.config)?;

    Ok(Router::new()
        .merge(routes::router(state))
        .layer(cors)
        .fallback(routes::not_found))
}

fn cors_layer(config: &Config) -> anyhow::Result<CorsLayer> {
    let origins = config
        .allowed_origins
        .iter()
        .map(|origin| {
            HeaderValue::from_str(origin)
                .with_context(|| format!("invalid API_ALLOWED_ORIGINS value: {origin}"))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]))
}

async fn shutdown_signal() {
    if let Err(error) = signal::ctrl_c().await {
        warn!(%error, "failed to listen for shutdown signal");
    }

    info!("shutdown signal received");
}

#[cfg(test)]
mod tests {
    use super::{Config, cors_layer};
    use axum::{
        Router,
        body::Body,
        http::{
            Method, Request, StatusCode,
            header::{
                ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_ORIGIN,
                ACCESS_CONTROL_REQUEST_HEADERS, ACCESS_CONTROL_REQUEST_METHOD, ORIGIN,
            },
        },
        routing::get,
    };
    use tower::ServiceExt;

    fn test_config(allowed_origins: Vec<&str>) -> Config {
        Config {
            host: "127.0.0.1".to_string(),
            port: 8080,
            allowed_origins: allowed_origins.into_iter().map(str::to_string).collect(),
            database_url: "postgres://localhost/test".to_string(),
            executor_base_url: "http://127.0.0.1:8082".to_string(),
            jwt_issuer: "issuer".to_string(),
            jwt_audience: "audience".to_string(),
            jwt_jwks_url: "http://127.0.0.1/jwks".to_string(),
            jwt_hs256_secret: None,
        }
    }

    async fn ok() -> &'static str {
        "ok"
    }

    #[tokio::test]
    async fn cors_preflight_allows_local_web_origin() {
        let app = Router::new()
            .route("/api/v1/health", get(ok))
            .layer(cors_layer(&test_config(vec!["http://localhost:5173"])).expect("cors layer"));

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::OPTIONS)
                    .uri("/api/v1/health")
                    .header(ORIGIN, "http://localhost:5173")
                    .header(ACCESS_CONTROL_REQUEST_METHOD, "GET")
                    .header(ACCESS_CONTROL_REQUEST_HEADERS, "authorization,content-type")
                    .body(Body::empty())
                    .expect("valid request"),
            )
            .await
            .expect("request handled");

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(ACCESS_CONTROL_ALLOW_ORIGIN),
            Some(&"http://localhost:5173".parse().expect("valid origin")),
        );
        assert_eq!(
            response.headers().get(ACCESS_CONTROL_ALLOW_HEADERS),
            Some(
                &"authorization,content-type"
                    .parse()
                    .expect("valid allow headers")
            ),
        );
    }

    #[tokio::test]
    async fn cors_simple_request_exposes_allow_origin_header() {
        let app = Router::new()
            .route("/api/v1/health", get(ok))
            .layer(cors_layer(&test_config(vec!["http://localhost:5173"])).expect("cors layer"));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/health")
                    .header(ORIGIN, "http://localhost:5173")
                    .body(Body::empty())
                    .expect("valid request"),
            )
            .await
            .expect("request handled");

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(ACCESS_CONTROL_ALLOW_ORIGIN),
            Some(&"http://localhost:5173".parse().expect("valid origin")),
        );
    }
}
