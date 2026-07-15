//! Onboarding email sequence routes
//!
//! POST /api/v1/onboarding/start   — starts the onboarding sequence
//! POST /api/v1/onboarding/unsubscribe — unsubscribe from emails
//! GET  /api/v1/onboarding/status  — get current step

use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::email::{self, EmailClient, EmailStore};
use crate::error::{AppError, AppResult};

// ── Handlers ────────────────────────────────────────────────────────────

pub async fn start_onboarding(
    axum::Extension(store): axum::Extension<Arc<EmailStore>>,
    axum::Extension(email_client): axum::Extension<EmailClient>,
    Json(payload): Json<serde_json::Value>,
) -> AppResult<impl IntoResponse> {
    let email = payload
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("email is required".to_string()))?;

    if email.trim().is_empty() {
        return Err(AppError::BadRequest("Email is required".to_string()));
    }

    let token = Uuid::new_v4().to_string();
    store.start_onboarding(email);

    let msg = email::welcome_email(email, &token);
    email_client.send(&msg).await?;

    tracing::info!(email = %email, "onboarding started");

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "onboarding_started",
            "email": email,
        })),
    ))
}

pub async fn unsubscribe(
    axum::Extension(store): axum::Extension<Arc<EmailStore>>,
    Json(payload): Json<serde_json::Value>,
) -> AppResult<impl IntoResponse> {
    let email = payload
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("email is required".to_string()))?;

    store.unsubscribe(email);
    tracing::info!(email = %email, "user unsubscribed");

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({ "status": "unsubscribed" })),
    ))
}

pub async fn get_status(
    axum::Extension(store): axum::Extension<Arc<EmailStore>>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> AppResult<Json<serde_json::Value>> {
    let email = params
        .get("email")
        .ok_or_else(|| AppError::BadRequest("email query parameter required".to_string()))?;

    match store.status(email) {
        Some(status) => Ok(Json(status)),
        None => Ok(Json(serde_json::json!({ "status": "not_found" }))),
    }
}

// ── Router ──────────────────────────────────────────────────────────────

pub fn onboarding_router() -> Router {
    Router::new()
        .route("/api/v1/onboarding/start", post(start_onboarding))
        .route("/api/v1/onboarding/unsubscribe", post(unsubscribe))
        .route("/api/v1/onboarding/status", get(get_status))
}
