use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Extension,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::accounts::AccountStore;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct AccountResponse {
    pub id: String,
    pub email: String,
    pub created_at: i64,
    pub result_count: usize,
}

#[derive(Deserialize)]
pub struct SaveResultRequest {
    pub test_type: String,
    pub result: serde_json::Value,
}

#[derive(Serialize)]
pub struct SaveResultResponse {
    pub result_id: String,
}

#[derive(Deserialize)]
pub struct ResultsQuery {
    pub test_type: Option<String>,
}

#[derive(Serialize)]
pub struct ResultItem {
    pub id: String,
    pub test_type: String,
    pub result: serde_json::Value,
    pub created_at: i64,
}

#[derive(Serialize)]
pub struct EvolutionItem {
    pub timestamp: i64,
    pub scores: serde_json::Value,
}

pub async fn register_account(
    Extension(store): Extension<Arc<AccountStore>>,
    Json(body): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, Json<serde_json::Value>)> {
    match store.register(&body.email) {
        Ok(account) => Ok(Json(RegisterResponse {
            id: account.id,
            email: account.email,
        })),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"detail": e})),
        )),
    }
}

pub async fn get_account(
    Extension(store): Extension<Arc<AccountStore>>,
    Path(id): Path<String>,
) -> Result<Json<AccountResponse>, (StatusCode, Json<serde_json::Value>)> {
    match store.get_account(&id) {
        Some(account) => {
            let results = store.get_results(&id);
            Ok(Json(AccountResponse {
                result_count: results.len(),
                id: account.id,
                email: account.email,
                created_at: account.created_at,
            }))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"detail": "Account not found"})),
        )),
    }
}

pub async fn save_result(
    Extension(store): Extension<Arc<AccountStore>>,
    Path(id): Path<String>,
    Json(body): Json<SaveResultRequest>,
) -> Result<Json<SaveResultResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Verify account exists
    if store.get_account(&id).is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"detail": "Account not found"})),
        ));
    }

    let result_id = store.save_result(&id, &body.test_type, body.result);
    Ok(Json(SaveResultResponse { result_id }))
}

pub async fn get_results(
    Extension(store): Extension<Arc<AccountStore>>,
    Path(id): Path<String>,
    Query(query): Query<ResultsQuery>,
) -> Result<Json<Vec<ResultItem>>, (StatusCode, Json<serde_json::Value>)> {
    if store.get_account(&id).is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"detail": "Account not found"})),
        ));
    }

    let results = match &query.test_type {
        Some(tt) => store.get_results_by_type(&id, tt),
        None => store.get_results(&id),
    };

    let items: Vec<ResultItem> = results
        .into_iter()
        .map(|r| ResultItem {
            id: r.id,
            test_type: r.test_type,
            result: r.result,
            created_at: r.created_at,
        })
        .collect();

    Ok(Json(items))
}

pub async fn get_evolution(
    Extension(store): Extension<Arc<AccountStore>>,
    Path((id, test_type)): Path<(String, String)>,
) -> Result<Json<Vec<EvolutionItem>>, (StatusCode, Json<serde_json::Value>)> {
    if store.get_account(&id).is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"detail": "Account not found"})),
        ));
    }

    let mut results = store.get_results_by_type(&id, &test_type);
    results.sort_by_key(|r| r.created_at);

    let items: Vec<EvolutionItem> = results
        .into_iter()
        .map(|r| {
            // Extract scores from the stored result
            let scores = r
                .result
                .get("scores")
                .or_else(|| r.result.get("percentages"))
                .or_else(|| r.result.get("percentiles"))
                .cloned()
                .unwrap_or(r.result);
            EvolutionItem {
                timestamp: r.created_at,
                scores,
            }
        })
        .collect();

    Ok(Json(items))
}
