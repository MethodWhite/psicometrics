//! CORS configuration — permissive in dev, locked-down in production.

use axum::http::{header, HeaderName, Method};
use std::sync::Arc;
use tower_http::cors::{AllowOrigin, CorsLayer};

/// Build the appropriate CORS layer based on the environment.
///
/// # Production (`DEBUG` not set)
///   - Only origins listed in `ALLOWED_ORIGINS` env var (comma-separated)
///   - Methods: GET, POST, OPTIONS
///   - Headers: Content-Type, Authorization, X-Request-Id, X-Session-Token
///   - Expose headers: X-Request-Id
///   - Preflight max-age: 3600s
///   - Vary: Origin header set automatically by tower-http
///
/// # Development (`DEBUG` is set)
///   - `very_permissive()` — allows all origins, methods, headers
pub fn build() -> CorsLayer {
    if std::env::var("DEBUG").is_ok() {
        tracing::info!("CORS: development mode — very_permissive()");
        return CorsLayer::very_permissive();
    }

    let origins_str = std::env::var("ALLOWED_ORIGINS").unwrap_or_default();
    let origins: Vec<String> = origins_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if origins.is_empty() {
        tracing::warn!(
            "CORS: ALLOWED_ORIGINS not set or empty — falling back to very_permissive()"
        );
        return CorsLayer::very_permissive();
    }

    let origins = Arc::new(origins);

    CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(move |origin, _parts| {
            let origin_bytes = origin.as_bytes();
            origins
                .iter()
                .any(|allowed| origin_bytes == allowed.as_bytes())
        }))
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_credentials(true)
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            HeaderName::from_static("x-request-id"),
            HeaderName::from_static("x-session-token"),
        ])
        .expose_headers([HeaderName::from_static("x-request-id")])
        .max_age(std::time::Duration::from_secs(3600))
}
