use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::{StatusCode, request::Parts},
};
use jsonwebtoken::{
    Algorithm, DecodingKey, Validation, decode, decode_header,
    jwk::{Jwk, JwkSet},
};
use reqwest::Client;
use serde::Deserialize;
use tokio::sync::RwLock;
use tracing::debug;

use crate::{AppState, error::AppError};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub subject: String,
    pub roles: Vec<String>,
}

#[derive(Clone)]
pub struct Authenticator {
    mode: AuthMode,
}

#[derive(Clone)]
enum AuthMode {
    Hs256 {
        secret: String,
        issuer: String,
        audience: String,
    },
    Jwks {
        issuer: String,
        audience: String,
        jwks_url: String,
        client: Client,
        cache: Arc<RwLock<Option<JwkSet>>>,
    },
}

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    #[serde(default)]
    realm_access: RealmAccess,
}

#[derive(Debug, Default, Deserialize)]
struct RealmAccess {
    #[serde(default)]
    roles: Vec<String>,
}

impl Authenticator {
    pub fn hs256(secret: String, issuer: String, audience: String) -> Self {
        Self {
            mode: AuthMode::Hs256 {
                secret,
                issuer,
                audience,
            },
        }
    }

    pub fn jwks(issuer: String, audience: String, jwks_url: String, client: Client) -> Self {
        Self {
            mode: AuthMode::Jwks {
                issuer,
                audience,
                jwks_url,
                client,
                cache: Arc::new(RwLock::new(None)),
            },
        }
    }

    pub async fn authenticate(&self, token: &str) -> Result<AuthenticatedUser, AppError> {
        let claims = match &self.mode {
            AuthMode::Hs256 {
                secret,
                issuer,
                audience,
            } => {
                debug!(issuer = %issuer, audience = %audience, "validating HS256 token");
                let mut validation = Validation::new(Algorithm::HS256);
                validation.set_issuer(&[issuer]);
                validation.set_audience(&[audience]);

                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret.as_bytes()),
                    &validation,
                )
                .map_err(|error| {
                    debug!(%error, "HS256 token validation failed");
                    AppError::new(
                        StatusCode::UNAUTHORIZED,
                        "AUTH_INVALID_TOKEN",
                        "invalid token",
                    )
                })?;

                debug!(
                    subject = %token_data.claims.sub,
                    roles = ?token_data.claims.realm_access.roles,
                    "HS256 token validation succeeded"
                );
                token_data.claims
            }
            AuthMode::Jwks {
                issuer,
                audience,
                jwks_url,
                client,
                cache,
            } => {
                debug!("decoding token header for JWKS authentication");
                let header = decode_header(token).map_err(|error| {
                    debug!(%error, "failed to decode token header");
                    AppError::new(
                        StatusCode::UNAUTHORIZED,
                        "AUTH_INVALID_TOKEN",
                        "invalid token header",
                    )
                })?;

                let kid = header.kid.ok_or_else(|| {
                    debug!("token header is missing key ID (kid)");
                    AppError::new(
                        StatusCode::UNAUTHORIZED,
                        "AUTH_INVALID_TOKEN",
                        "missing key id",
                    )
                })?;

                let jwk_set = {
                    let cached = cache.read().await;
                    if let Some(existing) = cached.clone() {
                        debug!(kid = %kid, "using cached JWKS keys");
                        existing
                    } else {
                        drop(cached);
                        debug!(jwks_url = %jwks_url, "fetching fresh JWKS key set");
                        let fetched = fetch_jwks(client, jwks_url).await?;
                        let mut cache_guard = cache.write().await;
                        *cache_guard = Some(fetched.clone());
                        debug!("JWKS key set cached successfully");
                        fetched
                    }
                };

                let jwk = find_key(&jwk_set, &kid).ok_or_else(|| {
                    debug!(kid = %kid, "key ID not found in JWKS key set");
                    AppError::new(
                        StatusCode::UNAUTHORIZED,
                        "AUTH_INVALID_TOKEN",
                        "unknown key id",
                    )
                })?;

                let mut validation = Validation::new(Algorithm::RS256);
                validation.set_issuer(&[issuer]);
                validation.set_audience(&[audience]);

                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_jwk(jwk).map_err(|error| {
                        debug!(%error, "failed to create decoding key from JWK");
                        AppError::new(
                            StatusCode::UNAUTHORIZED,
                            "AUTH_INVALID_TOKEN",
                            "invalid jwk",
                        )
                    })?,
                    &validation,
                )
                .map_err(|error| {
                    debug!(%error, "RS256 token validation failed");
                    AppError::new(
                        StatusCode::UNAUTHORIZED,
                        "AUTH_INVALID_TOKEN",
                        "invalid token",
                    )
                })?;

                debug!(
                    subject = %token_data.claims.sub,
                    roles = ?token_data.claims.realm_access.roles,
                    "JWKS token validation succeeded"
                );
                token_data.claims
            }
        };

        Ok(AuthenticatedUser {
            subject: claims.sub,
            roles: claims.realm_access.roles,
        })
    }
}

fn find_key<'a>(jwk_set: &'a JwkSet, kid: &str) -> Option<&'a Jwk> {
    jwk_set
        .keys
        .iter()
        .find(|key| key.common.key_id.as_deref() == Some(kid))
}

async fn fetch_jwks(client: &Client, url: &str) -> Result<JwkSet, AppError> {
    debug!(url = %url, "sending request to fetch JWKS");
    let response = client.get(url).send().await.map_err(|error| {
        debug!(%error, url = %url, "failed to reach JWKS endpoint");
        AppError::new(
            StatusCode::UNAUTHORIZED,
            "AUTH_JWKS_UNAVAILABLE",
            "jwks unavailable",
        )
    })?;

    response.json::<JwkSet>().await.map_err(|error| {
        debug!(%error, "failed to parse JWKS JSON response");
        AppError::new(
            StatusCode::UNAUTHORIZED,
            "AUTH_JWKS_INVALID",
            "invalid jwks response",
        )
    })
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let State(state) = State::<AppState>::from_request_parts(parts, state)
            .await
            .map_err(|_| {
                debug!("AppState extraction failed in AuthenticatedUser extractor");
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "STATE_MISSING",
                    "application state missing",
                )
            })?;

        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| {
                debug!("missing or non-utf8 Authorization header");
                AppError::new(
                    StatusCode::UNAUTHORIZED,
                    "AUTH_MISSING",
                    "authorization header is required",
                )
            })?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| {
                debug!("Authorization header does not start with 'Bearer '");
                AppError::new(
                    StatusCode::UNAUTHORIZED,
                    "AUTH_INVALID",
                    "bearer token is required",
                )
            })?
            .trim();

        state.auth.authenticate(token).await
    }
}
