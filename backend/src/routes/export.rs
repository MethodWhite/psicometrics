use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Extension,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::accounts::AccountStore;

#[derive(Deserialize)]
pub struct ExportQuery {
    pub email: Option<String>,
}

fn flatten_scores(result: &serde_json::Value) -> Vec<(String, f64)> {
    let source = result
        .get("scores")
        .or_else(|| result.get("percentages"))
        .or_else(|| result.get("percentiles"))
        .unwrap_or(result);

    let mut pairs = Vec::new();
    if let Some(obj) = source.as_object() {
        for (key, val) in obj {
            if let Some(n) = val.as_f64() {
                pairs.push((key.clone(), n));
            }
        }
    }
    pairs
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

pub async fn export_csv(
    Extension(store): Extension<Arc<AccountStore>>,
    Path(id): Path<String>,
    axum::extract::Query(query): axum::extract::Query<ExportQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Resolve account ID (support /me via email query param)
    let account_id = if id == "me" {
        let email = query.email.as_deref().unwrap_or("");
        if email.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"detail": "email query parameter required for /me"})),
            ));
        }
        match store.find_by_email(email).await {
            Ok(Some(a)) => a.id,
            Ok(None) => {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({"detail": "Account not found"})),
                ))
            }
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"detail": e.to_string()})),
                ))
            }
        }
    } else {
        match store.get_account(&id).await {
            Ok(Some(_)) => id,
            Ok(None) => {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({"detail": "Account not found"})),
                ))
            }
            Err(e) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"detail": e.to_string()})),
                ))
            }
        }
    };

    let results = match store.get_results(&account_id).await {
        Ok(r) => r,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"detail": e.to_string()})),
            ))
        }
    };

    if results.is_empty() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"detail": "No results found"})),
        ));
    }

    // Build CSV
    let mut csv = String::new();

    // Collect all possible factor keys across all results
    let mut all_factors: Vec<String> = Vec::new();
    for r in &results {
        let scores = flatten_scores(&r.result);
        for (factor, _) in scores {
            let h = format!("{} ({})", factor, r.test_type);
            if !all_factors.contains(&h) {
                all_factors.push(h);
            }
        }
    }

    // Header row
    csv.push_str("date,test_type");
    for factor in &all_factors {
        csv.push(',');
        csv.push_str(&escape_csv(factor));
    }
    csv.push('\n');

    // Data rows
    for r in &results {
        let ts = r.created_at.format("%Y-%m-%d %H:%M:%S").to_string();
        csv.push_str(&escape_csv(&ts));
        csv.push(',');
        csv.push_str(&escape_csv(&r.test_type));

        let scores = flatten_scores(&r.result);
        let score_map: std::collections::HashMap<String, f64> = scores.into_iter().collect();

        for factor in &all_factors {
            csv.push(',');
            let base_key = factor.split(" (").next().unwrap_or("");
            if let Some(val) = score_map.get(base_key) {
                csv.push_str(&val.to_string());
            }
        }
        csv.push('\n');
    }

    Ok((
        [
            (axum::http::header::CONTENT_TYPE, "text/csv; charset=utf-8"),
            (
                axum::http::header::CONTENT_DISPOSITION,
                "attachment; filename=\"psicometrics_export.csv\"",
            ),
        ],
        csv,
    ))
}
