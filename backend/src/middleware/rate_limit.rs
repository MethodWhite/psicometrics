//! Per-user & per-endpoint rate limiting using the `governor` token-bucket.
//!
//! | Endpoint                                  | Authenticated | Anonymous |
//! |-------------------------------------------|---------------|-----------|
//! | Everything except `/submit` and `/accounts` | 60 req/min    | 10 req/min |
//! | `/api/v1/tests/{type}/submit`              |  5 req/min    |  5 req/min |
//! | `/api/v1/accounts/*`                       | 10 req/min    | 10 req/min |
//!
//! The middleware extracts the user identity from either:
//!   - `x-session-token` header (authenticated)
//!   - `x-forwarded-for` or peer IP (anonymous)

use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use governor::{
    clock::DefaultClock,
    state::keyed::DefaultKeyedStateStore,
    Quota, RateLimiter as GovLimiter,
};
use std::num::NonZeroU32;
use std::sync::Arc;

// ── Type alias for our keyed rate limiter ────────────────────────────────

type KeyedRL = GovLimiter<String, DefaultKeyedStateStore<String>, DefaultClock>;

// ── Rate-limit buckets ──────────────────────────────────────────────────

pub struct RateLimitLayer {
    /// Authenticated users: 60 req/min
    user: Arc<KeyedRL>,
    /// Test-submit endpoint: 5 req/min
    submit: Arc<KeyedRL>,
    /// Anonymous (no session token): 10 req/min
    anon: Arc<KeyedRL>,
    /// Account endpoints: 10 req/min
    accounts: Arc<KeyedRL>,
}

impl RateLimitLayer {
    pub fn new() -> Self {
        Self {
            user: Arc::new(
                GovLimiter::keyed(Quota::per_minute(NonZeroU32::new(60).unwrap())),
            ),
            submit: Arc::new(
                GovLimiter::keyed(Quota::per_minute(NonZeroU32::new(5).unwrap())),
            ),
            anon: Arc::new(
                GovLimiter::keyed(Quota::per_minute(NonZeroU32::new(10).unwrap())),
            ),
            accounts: Arc::new(
                GovLimiter::keyed(Quota::per_minute(NonZeroU32::new(10).unwrap())),
            ),
        }
    }
}

impl Default for RateLimitLayer {
    fn default() -> Self {
        Self::new()
    }
}

// ── Key extraction helpers ──────────────────────────────────────────────

fn user_key(req: &Request) -> String {
    req.headers()
        .get("x-session-token")
        .and_then(|v| v.to_str().ok())
        .map(|s| format!("user:{s}"))
        .unwrap_or_else(|| anon_key(req))
}

fn anon_key(req: &Request) -> String {
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next().map(|s| s.trim().to_string()))
        .unwrap_or_else(|| "unknown".to_string());
    format!("anon:{ip}")
}

/// Pick the right limiter based on the request path.
fn select_limiter(layer: &RateLimitLayer, req: &Request) -> (Arc<KeyedRL>, String) {
    let path = req.uri().path();
    let key = user_key(req);

    if path.contains("/submit") {
        (layer.submit.clone(), key)
    } else if path.contains("/accounts") {
        (layer.accounts.clone(), key)
    } else if key.starts_with("user:") {
        (layer.user.clone(), key)
    } else {
        (layer.anon.clone(), key)
    }
}

// ── Middleware ──────────────────────────────────────────────────────────

pub async fn rate_limit_middleware(
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, &'static str)> {
    let layer = req
        .extensions()
        .get::<Arc<RateLimitLayer>>()
        .cloned()
        .unwrap_or_else(|| Arc::new(RateLimitLayer::new()));

    let (limiter, key) = select_limiter(&layer, &req);

    match limiter.check_key(&key) {
        Ok(()) => {
            tracing::debug!(rate_limit.key = %key, "rate_limit: allowed");
            Ok(next.run(req).await)
        }
        Err(_) => {
            tracing::warn!(rate_limit.key = %key, "rate_limit: denied");
            Err((StatusCode::TOO_MANY_REQUESTS, "rate limit exceeded"))
        }
    }
}
