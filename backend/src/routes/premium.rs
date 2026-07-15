use axum::{
    extract::Path,
    http::StatusCode,
    Extension,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::accounts::AccountStore;

#[derive(Deserialize)]
pub struct PremiumReportRequest {
    pub result: serde_json::Value,
    pub language: Option<String>,
}

#[derive(Serialize)]
pub struct PremiumReportResponse {
    pub report_url: String,
    pub pages: u32,
    pub sections: Vec<String>,
    pub description: String,
}

#[derive(Serialize)]
pub struct CheckoutSessionResponse {
    pub url: String,
    pub session_id: String,
}

/// Generate a premium report for a given test type.
/// Returns metadata about the premium report (no real PDF generation yet).
pub async fn create_premium_report(
    Extension(_store): Extension<Arc<AccountStore>>,
    Path(test_type): Path<String>,
    Json(body): Json<PremiumReportRequest>,
) -> Result<Json<PremiumReportResponse>, (StatusCode, Json<serde_json::Value>)> {
    let lang = body.language.as_deref().unwrap_or("es");

    let sections = vec![
        "Resumen Ejecutivo".to_string(),
        "Análisis por Factor (detallado)".to_string(),
        "Comparación con Población General".to_string(),
        "Tablas de Comparación Inter-Factor".to_string(),
        "Recomendaciones Personalizadas".to_string(),
        "Grid de Desarrollo".to_string(),
        "Plan de Acción (Siguientes 90 Días)".to_string(),
    ];

    let description = format!(
        "Informe premium de {} páginas para {} con análisis detallado por faceta, \
         tablas comparativas y grid de recomendaciones (idioma: {}).",
        5, test_type, lang
    );

    Ok(Json(PremiumReportResponse {
        report_url: format!("/api/v1/tests/{}/premium-report/download", test_type),
        pages: 5,
        sections,
        description,
    }))
}

/// Create a Stripe-like checkout session (mock, no real Stripe keys).
pub async fn create_checkout_session(
) -> Result<Json<CheckoutSessionResponse>, (StatusCode, Json<serde_json::Value>)> {
    let session_id = uuid::Uuid::new_v4().to_string();

    Ok(Json(CheckoutSessionResponse {
        url: format!("https://checkout.stripe.com/pay/{}", session_id),
        session_id,
    }))
}
