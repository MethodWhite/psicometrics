use std::sync::Arc;

use axum::{
    extract::Path,
    Extension,
    http::HeaderMap,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};
use crate::payments::StripeClient;

#[derive(Deserialize)]
pub struct CreateCheckoutRequest {
    pub tier: String,
    pub account_id: String,
    pub success_url: String,
}

#[derive(Serialize)]
pub struct CheckoutResponse {
    pub url: String,
}

/// POST /api/v1/payments/create-checkout
pub async fn create_checkout(
    Extension(client): Extension<Arc<StripeClient>>,
    Json(body): Json<CreateCheckoutRequest>,
) -> AppResult<Json<CheckoutResponse>> {
    let url = client
        .create_checkout_session(&body.tier, &body.account_id, &body.success_url)
        .await?;
    Ok(Json(CheckoutResponse { url }))
}

/// POST /api/v1/payments/webhook
pub async fn webhook_handler(
    Extension(client): Extension<Arc<StripeClient>>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> AppResult<Json<serde_json::Value>> {
    let sig_header = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::BadRequest("Missing stripe-signature header".to_string()))?;

    let payload_str =
        std::str::from_utf8(&body).map_err(|_| AppError::BadRequest("Invalid UTF-8".to_string()))?;

    let event = client.handle_webhook(payload_str, sig_header)?;

    let event_type = event["type"].as_str().unwrap_or("").to_string();
    tracing::info!(event_type = %event_type, "Stripe webhook received");

    // Process known event types
    match event_type.as_str() {
        "checkout.session.completed" => {
            let account_id = event["data"]["object"]["client_reference_id"]
                .as_str()
                .map(|s| s.to_string());
            let customer_id = event["data"]["object"]["customer"]
                .as_str()
                .map(|s| s.to_string());
            tracing::info!(
                account_id = ?account_id,
                customer_id = ?customer_id,
                "Checkout session completed"
            );
            // TODO: Update account subscription tier in the database
        }
        "customer.subscription.updated" | "customer.subscription.deleted" => {
            let customer_id = event["data"]["object"]["customer"]
                .as_str()
                .map(|s| s.to_string());
            let status = event["data"]["object"]["status"]
                .as_str()
                .unwrap_or("unknown");
            tracing::info!(
                customer_id = ?customer_id,
                status = %status,
                "Subscription updated"
            );
            // TODO: Sync subscription status to database
        }
        _ => {
            tracing::debug!(event_type = %event_type, "Unhandled webhook event");
        }
    }

    Ok(Json(serde_json::json!({"received": true})))
}

/// GET /api/v1/payments/portal/{account_id}
pub async fn portal_session(
    Extension(client): Extension<Arc<StripeClient>>,
    Path(account_id): Path<String>,
) -> AppResult<Json<CheckoutResponse>> {
    // In production, account_id should be the Stripe customer_id
    let url = client.create_portal_session(&account_id).await?;
    Ok(Json(CheckoutResponse { url }))
}

/// GET /api/v1/payments/tiers
pub async fn list_tiers(
    Extension(client): Extension<Arc<StripeClient>>,
) -> Json<Vec<serde_json::Value>> {
    Json(client.tiers_json())
}
