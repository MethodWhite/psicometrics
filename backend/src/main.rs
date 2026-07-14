use std::sync::Arc;

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

use ztf::challenge::{ChallengeResponse, HmacVerifier};

mod accounts;
mod compatibility;
mod config;
mod data;
mod i18n;
mod interpretation;
mod middleware;
mod models;
mod report;
mod routes;
mod scoring;
mod validation;

async fn security_headers_mw(req: Request, next: Next) -> Response {
    let mut res = next.run(req).await;
    crate::middleware::headers::add_security_headers(&mut res);
    res
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let secret_key = b"psicometrics-zero-trust-secret-2024".to_vec();

    // Initialize Zero Trust provider
    let zt_provider = Arc::new(ChallengeResponse::new());
    let _verifier = HmacVerifier::new(&secret_key);

    let account_store = Arc::new(accounts::AccountStore::new());

    // Build router
    let app = Router::new()
        .route("/api/v1/health", get(routes::health::health_check))
        .route("/api/v1/health/ready", get(routes::health::readiness_check))
        .route("/api/v1/auth/challenge", get(routes::auth::get_challenge))
        .route("/api/v1/auth/verify", post(routes::auth::verify_challenge))
        .route("/api/v1/auth/device/register", post(routes::auth::register_device))
        .route("/api/v1/auth/device/attest", post(routes::auth::attest_device))
        .route("/api/v1/tests", get(routes::tests::list_tests))
        .route("/api/v1/tests/{test_type}/metadata", get(routes::tests::get_test_metadata))
        .route("/api/v1/tests/{test_type}", get(routes::tests::get_test_questions))
        .route("/api/v1/tests/{test_type}/submit", post(routes::tests::submit_test))
        .route("/api/v1/tests/{test_type}/report", post(routes::tests::generate_report))
        .route("/api/v1/tests/{test_type}/compare", post(routes::tests::compare_tests))
        .route("/api/v1/accounts/register", post(routes::accounts::register_account))
        .route("/api/v1/accounts/{id}", get(routes::accounts::get_account))
        .route("/api/v1/accounts/{id}/results", post(routes::accounts::save_result).get(routes::accounts::get_results))
        .route("/api/v1/accounts/{id}/evolution/{test_type}", get(routes::accounts::get_evolution))
        .layer(axum::Extension(zt_provider))
        .layer(axum::Extension(secret_key))
        .layer(axum::Extension(account_store))
        .layer(axum::middleware::from_fn(security_headers_mw))
        .layer(CorsLayer::very_permissive());

    let addr = "0.0.0.0:8000";
    tracing::info!("Starting PsicoMetrics API on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
