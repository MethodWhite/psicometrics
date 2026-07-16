use axum::{extract::Path, Json};

use crate::compatibility;
use crate::error::{AppError, AppResult};

use super::valid_tests;

#[derive(serde::Deserialize)]
pub struct CompareRequest {
    pub results: Vec<serde_json::Value>,
    pub language: Option<String>,
}

pub async fn compare_tests(
    Path(test_type): Path<String>,
    Json(body): Json<CompareRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let lang = body.language.as_deref().unwrap_or("es");

    if !valid_tests().contains(&test_type.as_str()) {
        return Err(AppError::NotFound("Test not found".to_string()));
    }

    if body.results.len() != 2 {
        return Err(AppError::BadRequest(
            "Exactly 2 results required for comparison".to_string(),
        ));
    }

    let result = compatibility::compare_results(&body.results, &test_type, lang);

    if result.get("error").and_then(|e| e.as_bool()).unwrap_or(false) {
        return Err(AppError::BadRequest(result.to_string()));
    }

    Ok(Json(result))
}
