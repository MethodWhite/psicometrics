use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

/// Newtype for injecting the authenticated account ID into request extensions.
#[derive(Clone, Debug)]
pub struct AuthAccountId(pub String);

// ─── JWT Claims ───────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String, // account_id
    pub exp: usize,  // expiry timestamp (UTC epoch seconds)
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
    #[error("Missing authorization header")]
    MissingHeader,
    #[error("Invalid authorization scheme, expected Bearer")]
    InvalidScheme,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, detail) = match &self {
            AuthError::InvalidToken | AuthError::TokenExpired
            | AuthError::InvalidScheme => (StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::MissingHeader => (StatusCode::UNAUTHORIZED, self.to_string()),
        };
        (status, Json(serde_json::json!({"detail": detail}))).into_response()
    }
}

// ─── Token creation ──────────────────────────────────────────────────────────

pub fn create_token(account_id: &str, secret: &[u8]) -> AppResult<String> {
    let now = chrono::Utc::now();
    let exp = (now + chrono::Duration::days(7)).timestamp() as usize;

    let claims = JwtClaims {
        sub: account_id.to_string(),
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
    .map_err(|e| AppError::Internal(format!("Failed to create token: {e}")))
}

// ─── Token verification ──────────────────────────────────────────────────────

pub fn verify_token(token: &str, secret: &[u8]) -> AppResult<String> {
    let token_data = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            AppError::Unauthorized("Token has expired".to_string())
        }
        _ => AppError::Unauthorized("Invalid token".to_string()),
    })?;

    Ok(token_data.claims.sub)
}

// ─── Auth middleware ─────────────────────────────────────────────────────────

/// Middleware that extracts a Bearer token from the Authorization header,
/// verifies the JWT, and injects the account_id into request extensions.
pub async fn auth_middleware(mut req: Request, next: Next) -> Response {
    let header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let token = match header {
        Some(h) if h.starts_with("Bearer ") => h[7..].to_string(),
        _ => {
            return AuthError::MissingHeader.into_response();
        }
    };

    let secret = match req.extensions().get::<Vec<u8>>().map(|v| v.clone()) {
        Some(s) => s,
        None => {
            return AuthError::InvalidToken.into_response();
        }
    };

    let account_id = match verify_token(&token, &secret) {
        Ok(id) => id,
        Err(e) => {
            return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"detail": e.to_string()}))).into_response();
        }
    };

    req.extensions_mut().insert(AuthAccountId(account_id));
    next.run(req).await
}

// ─── Password hashing ───────────────────────────────────────────────────────

pub fn hash_password(password: &str) -> AppResult<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(format!("Failed to hash password: {e}")))
}

pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    bcrypt::verify(password, hash)
        .map_err(|e| AppError::Internal(format!("Failed to verify password: {e}")))
}
