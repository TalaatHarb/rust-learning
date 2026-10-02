use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub allowed_origins: Vec<String>,
    pub database_url: String,
    pub executor_base_url: String,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub jwt_jwks_url: String,
    pub jwt_hs256_secret: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        let host = env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("API_PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(8080);
        let allowed_origins = env::var("API_ALLOWED_ORIGINS")
            .ok()
            .map(|value| {
                value
                    .split(',')
                    .map(str::trim)
                    .filter(|origin| !origin.is_empty())
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
            .filter(|origins| !origins.is_empty())
            .unwrap_or_else(|| vec!["http://localhost:5173".to_string()]);

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            format!(
                "postgres://{}:{}@localhost:5432/rust_learning",
                "rust_learning", "rust_learning"
            )
        });
        let executor_base_url =
            env::var("EXECUTOR_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8082".to_string());

        let jwt_issuer = env::var("JWT_ISSUER")
            .unwrap_or_else(|_| "http://localhost:8081/realms/nextechincubator".to_string());
        let jwt_audience =
            env::var("JWT_AUDIENCE").unwrap_or_else(|_| "rust-learning-web".to_string());
        let jwt_jwks_url = env::var("JWT_JWKS_URL").unwrap_or_else(|_| {
            "http://localhost:8081/realms/nextechincubator/protocol/openid-connect/certs"
                .to_string()
        });
        let jwt_hs256_secret = env::var("JWT_HS256_SECRET").ok();

        Self {
            host,
            port,
            allowed_origins,
            database_url,
            executor_base_url,
            jwt_issuer,
            jwt_audience,
            jwt_jwks_url,
            jwt_hs256_secret,
        }
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
