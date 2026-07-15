use std::sync::Arc;

use axum::routing::{delete, get, post};
use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use ztf::challenge::{ChallengeResponse, HmacVerifier};

mod accounts;
mod auth;
mod compatibility;
mod config;
mod data;
mod database;
mod error;
mod i18n;
mod interpretation;
mod metrics;
mod middleware;
mod models;
mod report;
mod routes;
mod scoring;
mod validation;

use middleware as mw;

/// Newtype for injecting the authenticated account ID into request extensions.
#[derive(Clone, Debug)]
pub struct AuthAccountId(pub String);

async fn security_headers_mw(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let mut res = next.run(req).await;
    mw::headers::add_security_headers(&mut res);
    res
}

#[tokio::main]
async fn main() {
    // ── Structured logging ──────────────────────────────────────────────
    let is_debug = std::env::var("DEBUG").is_ok();
    if is_debug {
        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| EnvFilter::new("info")),
            )
            .with_target(true)
            .init();
        tracing::info!("structured_logging: human-readable (DEBUG mode)");
    } else {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| EnvFilter::new("info")),
            )
            .flatten_event(true)
            .with_current_span(true)
            .with_span_list(true)
            .init();
        tracing::info!("structured_logging: JSON mode (production)");
    }

    // ── Config ──────────────────────────────────────────────────────────
    let settings = config::Settings::from_env();
    let secret_key = settings
        .secret_key
        .map(|s| s.into_bytes())
        .unwrap_or_else(|| b"psicometrics-zero-trust-secret-2024".to_vec());
    tracing::info!(debug = is_debug, "configuration loaded");

    // ── Database ────────────────────────────────────────────────────────
    let database_url = settings.database_url.clone().unwrap_or_else(|| {
        tracing::warn!("DATABASE_URL not set, defaulting to local postgres");
        "postgres://postgres:postgres@localhost:5432/psicometrics".to_string()
    });

    let _db = match database::Database::init(&database_url).await {
        Ok(db) => {
            tracing::info!("Database connected and initialized");
            db
        }
        Err(e) => {
            tracing::error!("Failed to initialize database: {e}");
            tracing::warn!("Starting without database — some features will be unavailable");
            tracing::error!("Cannot connect to PostgreSQL at {database_url}");
            tracing::error!("Please ensure PostgreSQL is running and accessible, or set DATABASE_URL.");
            std::process::exit(1);
        }
    };

    // ── State ───────────────────────────────────────────────────────────
    let zt_provider = Arc::new(ChallengeResponse::new());
    let _verifier = HmacVerifier::new(&secret_key);

    let account_store = Arc::new(accounts::AccountStore::new());
    let rate_limit_layer = Arc::new(mw::rate_limit::RateLimitLayer::new());

    // ── CORS ────────────────────────────────────────────────────────────
    let cors = mw::cors::build();
    tracing::info!(
        allowed_origins = ?settings.allowed_origins,
        "CORS configured"
    );

    // ── Router ──────────────────────────────────────────────────────────
    // Public routes
    let app = Router::new()
        // Health
        .route("/api/v1/health", get(routes::health::health_check))
        .route("/api/v1/health/ready", get(routes::health::readiness_check))
        // Zero Trust auth
        .route("/api/v1/auth/challenge", get(routes::auth::get_challenge))
        .route("/api/v1/auth/verify", post(routes::auth::verify_challenge))
        .route("/api/v1/auth/device/register", post(routes::auth::register_device))
        .route("/api/v1/auth/device/attest", post(routes::auth::attest_device))
        // JWT login
        .route("/api/v1/auth/login", post(routes::accounts::login))
        // Account registration
        .route("/api/v1/accounts/register", post(routes::accounts::register_account))
        // Tests
        .route("/api/v1/tests", get(routes::tests::list_tests))
        .route("/api/v1/tests/{test_type}/metadata", get(routes::tests::get_test_metadata))
        .route("/api/v1/tests/{test_type}", get(routes::tests::get_test_questions))
        .route("/api/v1/tests/{test_type}/submit", post(routes::tests::submit_test))
        .route("/api/v1/tests/{test_type}/report", post(routes::tests::generate_report))
        .route("/api/v1/tests/{test_type}/compare", post(routes::tests::compare_tests))
        // Account by ID
        .route("/api/v1/accounts/{id}", get(routes::accounts::get_account))
        .route("/api/v1/accounts/{id}/results", post(routes::accounts::save_result).get(routes::accounts::get_results))
        .route("/api/v1/accounts/{id}/evolution/{test_type}", get(routes::accounts::get_evolution));

    // Protected routes (JWT required)
    let protected_routes = Router::new()
        .route("/api/v1/accounts/me", get(routes::accounts::get_me))
        .route("/api/v1/accounts/me/delete", delete(routes::accounts::delete_me))
        .route("/api/v1/accounts/me/export", get(routes::accounts::export_me))
        .route_layer(axum::middleware::from_fn(auth::auth_middleware));

    // Merge everything
    let app = app
        .merge(protected_routes)
        .merge(metrics::metrics_router())
        .layer(axum::Extension(zt_provider))
        .layer(axum::Extension(secret_key))
        .layer(axum::Extension(account_store))
        .layer(axum::Extension(rate_limit_layer))
        .layer(
            ServiceBuilder::new()
                .layer(axum::middleware::from_fn(mw::request_id::request_id_middleware))
                .layer(TraceLayer::new_for_http())
                .layer(cors)
                .layer(axum::middleware::from_fn(metrics::metrics_middleware))
                .layer(axum::middleware::from_fn(mw::rate_limit::rate_limit_middleware))
                .layer(axum::middleware::from_fn(security_headers_mw)),
        );

    let addr = "0.0.0.0:8000";
    tracing::info!(addr = %addr, "starting PsicoMetrics API");

    match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            if let Err(e) = axum::serve(listener, app).await {
                tracing::error!("Server error: {e}");
                std::process::exit(1);
            }
        }
        Err(e) => {
            tracing::error!("Failed to bind to {addr}: {e}");
            std::process::exit(1);
        }
    }
}
