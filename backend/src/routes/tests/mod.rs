use axum::{
    extract::{Path, Query},
    Json,
};
use serde::Serialize;

use crate::data::{get_test_type_list, load_test_data, TestDataNotFound};
use crate::error::{AppError, AppResult};
use crate::i18n::get_string_field;

pub mod compare;
pub mod report;
pub mod submit;

pub use compare::compare_tests;
pub use report::generate_report;
pub use submit::submit_test;

// ─── Shared types ────────────────────────────────────────────────────────────

#[derive(serde::Deserialize)]
pub struct LangQuery {
    pub lang: Option<String>,
}

#[derive(Serialize)]
pub struct TestInfoResponse {
    pub test_type: String,
    pub name: serde_json::Value,
    pub description: serde_json::Value,
    pub item_count: usize,
    pub test_mode: String,
}

#[derive(Serialize)]
pub struct TestMetadata {
    pub test_type: String,
    pub name: serde_json::Value,
    pub description: serde_json::Value,
    pub instructions: serde_json::Value,
    pub consent: serde_json::Value,
    pub scientific_basis: serde_json::Value,
    pub item_count: usize,
    pub estimated_minutes: u32,
    pub test_mode: String,
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

pub(crate) fn get_data(test_type: &str) -> AppResult<&'static serde_json::Value> {
    load_test_data(test_type).map_err(|e: TestDataNotFound| {
        if [
            "attachment_style", "big_five", "career_aptitude", "dark_triad", "disc",
            "emotional_intelligence", "enneagram", "human_design", "love_languages",
            "mbti", "via_strengths",
        ]
        .contains(&test_type)
        {
            AppError::Internal(format!("Test data for '{test_type}' is corrupted: {}", e.0))
        } else {
            AppError::NotFound(format!("Test '{test_type}' not found"))
        }
    })
}

pub(crate) fn valid_tests() -> &'static [&'static str] {
    &[
        "attachment_style", "big_five", "career_aptitude", "dark_triad", "disc",
        "emotional_intelligence", "enneagram", "human_design", "love_languages",
        "mbti", "via_strengths",
    ]
}

// ─── Handlers ────────────────────────────────────────────────────────────────

pub async fn list_tests() -> Json<Vec<TestInfoResponse>> {
    let results: Vec<TestInfoResponse> = get_test_type_list()
        .iter()
        .filter_map(|&tt| {
            let data = get_data(tt).ok()?;
            let items = data
                .get("questions")
                .and_then(|q| q.as_array())
                .map(|arr| arr.len())
                .unwrap_or(0);
            Some(TestInfoResponse {
                test_type: tt.to_string(),
                name: serde_json::json!({"es": data["name"]["es"], "en": data["name"]["en"]}),
                description: serde_json::json!({
                    "es": data["description"]["es"],
                    "en": data["description"]["en"]
                }),
                item_count: items,
                test_mode: if tt == "human_design" {
                    "birth_data".to_string()
                } else {
                    "questions".to_string()
                },
            })
        })
        .collect();
    Json(results)
}

pub async fn get_test_metadata(
    Path(test_type): Path<String>,
) -> AppResult<Json<TestMetadata>> {
    if !valid_tests().contains(&test_type.as_str()) {
        return Err(AppError::NotFound("Test not found".to_string()));
    }

    let data = get_data(&test_type)?;
    let items = data
        .get("questions")
        .and_then(|q| q.as_array())
        .map(|arr| arr.len())
        .unwrap_or(0);

    Ok(Json(TestMetadata {
        test_type: test_type.clone(),
        name: data["name"].clone(),
        description: data["description"].clone(),
        instructions: data
            .get("instructions")
            .cloned()
            .unwrap_or(serde_json::json!({
                "es": "Respondé cada pregunta según corresponda. No hay respuestas correctas o incorrectas.",
                "en": "Answer each question as it applies to you. There are no right or wrong answers."
            })),
        consent: data.get("consent").cloned().unwrap_or(serde_json::json!({
            "es": "Este test es solo para fines educativos y de autoconocimiento. No reemplaza una evaluación psicológica profesional.",
            "en": "This test is for educational and self-awareness purposes only. It does not replace professional psychological evaluation."
        })),
        scientific_basis: data
            .get("scientific_basis")
            .cloned()
            .unwrap_or(serde_json::json!({"es": "", "en": ""})),
        item_count: items,
        estimated_minutes: data
            .get("estimated_minutes")
            .and_then(|v| v.as_u64())
            .unwrap_or(15) as u32,
        test_mode: if test_type == "human_design" {
            "birth_data".to_string()
        } else {
            "questions".to_string()
        },
    }))
}

pub async fn get_test_questions(
    Path(test_type): Path<String>,
    Query(q): Query<LangQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let lang = q.lang.as_deref().unwrap_or("es");

    if !valid_tests().contains(&test_type.as_str()) {
        return Err(AppError::NotFound("Test not found".to_string()));
    }

    let data = get_data(&test_type)?;

    let questions: Vec<serde_json::Value> = if test_type == "human_design" {
        vec![]
    } else {
        data["questions"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|q| {
                        let text = get_string_field(&q["text"], lang, "");
                        let mut localized = serde_json::json!({
                            "id": q["id"],
                            "text": text,
                        });
                        for field in &[
                            "facet",
                            "reverse",
                            "dichotomy",
                            "pole",
                            "type",
                            "dimension",
                            "trait",
                        ] {
                            if let Some(v) = q.get(*field) {
                                localized[*field] = v.clone();
                            }
                        }
                        localized
                    })
                    .collect()
            })
            .unwrap_or_default()
    };

    let result = serde_json::json!({
        "test_type": test_type,
        "name": get_string_field(&data["name"], lang, ""),
        "description": get_string_field(&data["description"], lang, ""),
        "questions": questions,
        "test_mode": if test_type == "human_design" { "birth_data" } else { "questions" },
    });

    Ok(Json(result))
}
