#![allow(dead_code)]

/// PsicoMetrics Rust Backend
/// Tier S++ SecDevOps - Zero Trust, TPM, Defense in Depth

use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};

pub mod accounts;
pub mod analytics;
pub mod auth;
pub mod community;
pub mod community_seed;
pub mod compatibility;
pub mod config;
pub mod data;
pub mod database;
pub mod email;
pub mod error;
pub mod i18n;
pub mod interpretation;
pub mod metrics;
pub mod middleware;
pub mod models;
pub mod payments;
pub mod report;
pub mod routes;
pub mod scoring;
pub mod validation;

/// Newtype for injecting the authenticated account ID into request extensions.
#[derive(Clone, Debug)]
pub struct AuthAccountId(pub String);

/// Build the application router for testing (in-memory store, no database needed).
/// This exposes the same routes as the production app but with minimal middleware.
pub fn create_test_router(store: Arc<accounts::AccountStore>) -> Router {
    let secret_key = b"psicometrics-zero-trust-secret-2024".to_vec();
    let zt_provider = Arc::new(ztf::challenge::ChallengeResponse::new());
    let rate_limit_layer = Arc::new(middleware::rate_limit::RateLimitLayer::new());
    let community_store = Arc::new(community::CommunityStore::new());
    community_seed::seed_community(&community_store);

    let app = Router::new()
        .route("/api/v1/health", get(routes::health::health_check))
        .route("/api/v1/health/ready", get(routes::health::readiness_check))
        .route("/api/v1/auth/challenge", get(routes::auth::get_challenge))
        .route("/api/v1/auth/verify", post(routes::auth::verify_challenge))
        .route("/api/v1/auth/device/register", post(routes::auth::register_device))
        .route("/api/v1/auth/device/attest", post(routes::auth::attest_device))
        .route("/api/v1/auth/login", post(routes::accounts::login))
        .route("/api/v1/accounts/register", post(routes::accounts::register_account))
        .route("/api/v1/tests", get(routes::tests::list_tests))
        .route("/api/v1/tests/{test_type}/metadata", get(routes::tests::get_test_metadata))
        .route("/api/v1/tests/{test_type}", get(routes::tests::get_test_questions))
        .route("/api/v1/tests/{test_type}/submit", post(routes::tests::submit_test))
        .route("/api/v1/tests/{test_type}/report", post(routes::tests::generate_report))
        .route("/api/v1/tests/{test_type}/compare", post(routes::tests::compare_tests))
        .route("/api/v1/accounts/{id}", get(routes::accounts::get_account))
        .route("/api/v1/accounts/{id}/results", post(routes::accounts::save_result).get(routes::accounts::get_results))
        .route("/api/v1/accounts/{id}/evolution/{test_type}", get(routes::accounts::get_evolution))
        .route("/api/v1/accounts/me", get(routes::accounts::get_me))
        .route("/api/v1/accounts/me/delete", delete(routes::accounts::delete_me))
        .route("/api/v1/accounts/me/export", get(routes::accounts::export_me))
        // Community
        .route("/api/v1/community/posts", get(routes::community::get_posts).post(routes::community::create_post))
        .route("/api/v1/community/posts/{id}", get(routes::community::get_post))
        .route("/api/v1/community/posts/{id}/like", post(routes::community::like_post))
        .route("/api/v1/community/posts/{id}/comments", get(routes::community::get_comments).post(routes::community::create_comment))
        .route("/api/v1/community/testimonials", get(routes::community::get_testimonials).post(routes::community::create_testimonial))
        .route("/api/v1/community/stories", get(routes::community::get_stories).post(routes::community::create_story))
        .route("/api/v1/community/stories/{id}/like", post(routes::community::like_story))
        .route("/api/v1/community/types/{mbti_type}/stats", get(routes::community::get_type_stats))
        .layer(axum::Extension(zt_provider))
        .layer(axum::Extension(secret_key))
        .layer(axum::Extension(store))
        .layer(axum::Extension(community_store))
        .layer(axum::Extension(rate_limit_layer))
        .layer(axum::middleware::from_fn(security_headers_mw))
        .layer(middleware::cors::build());

    app
}

async fn security_headers_mw(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let mut res = next.run(req).await;
    middleware::headers::add_security_headers(&mut res);
    res
}
