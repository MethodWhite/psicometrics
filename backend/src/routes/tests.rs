use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

use crate::data::{load_test_data, get_test_type_list};
use crate::i18n::get_string_field;
use crate::scoring;

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

pub async fn list_tests() -> Json<Vec<TestInfoResponse>> {
    let results: Vec<TestInfoResponse> = get_test_type_list().iter().map(|&tt| {
        let data = load_test_data(tt).expect("test data");
        let items = data.get("questions")
            .and_then(|q| q.as_array())
            .map(|arr| arr.len())
            .unwrap_or(0);
        TestInfoResponse {
            test_type: tt.to_string(),
            name: serde_json::json!({"es": data["name"]["es"], "en": data["name"]["en"]}),
            description: serde_json::json!({"es": data["description"]["es"], "en": data["description"]["en"]}),
            item_count: items,
            test_mode: if tt == "human_design" { "birth_data".to_string() } else { "questions".to_string() },
        }
    }).collect();
    Json(results)
}

pub async fn get_test_metadata(
    Path(test_type): Path<String>,
) -> Result<Json<TestMetadata>, (StatusCode, Json<serde_json::Value>)> {
    let valid = ["big_five", "mbti", "enneagram", "disc", "dark_triad", "human_design"];
    if !valid.contains(&test_type.as_str()) {
        return Err((StatusCode::NOT_FOUND, Json(serde_json::json!({"detail": "Test not found"}))));
    }

    let data = load_test_data(&test_type).expect("test data");
    let items = data.get("questions")
        .and_then(|q| q.as_array())
        .map(|arr| arr.len())
        .unwrap_or(0);

    Ok(Json(TestMetadata {
        test_type: test_type.clone(),
        name: data["name"].clone(),
        description: data["description"].clone(),
        instructions: data.get("instructions").cloned()
            .unwrap_or(serde_json::json!({"es": "Respondé cada pregunta según corresponda. No hay respuestas correctas o incorrectas.", "en": "Answer each question as it applies to you. There are no right or wrong answers."})),
        consent: data.get("consent").cloned()
            .unwrap_or(serde_json::json!({"es": "Este test es solo para fines educativos y de autoconocimiento. No reemplaza una evaluación psicológica profesional.", "en": "This test is for educational and self-awareness purposes only. It does not replace professional psychological evaluation."})),
        scientific_basis: data.get("scientific_basis").cloned()
            .unwrap_or(serde_json::json!({"es": "","en": ""})),
        item_count: items,
        estimated_minutes: data.get("estimated_minutes").and_then(|v| v.as_u64()).unwrap_or(15) as u32,
        test_mode: if test_type == "human_design" { "birth_data".to_string() } else { "questions".to_string() },
    }))
}

pub async fn get_test_questions(
    Path(test_type): Path<String>,
    Query(q): Query<LangQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let lang = q.lang.as_deref().unwrap_or("es");
    let valid = ["big_five", "mbti", "enneagram", "disc", "dark_triad", "human_design"];

    if !valid.contains(&test_type.as_str()) {
        return Err((StatusCode::NOT_FOUND, Json(serde_json::json!({"detail": "Test not found"}))));
    }

    let data = load_test_data(&test_type).expect("test data");

    let questions: Vec<serde_json::Value> = if test_type == "human_design" {
        vec![]
    } else {
        data["questions"].as_array().map(|arr| {
            arr.iter().map(|q| {
                let text = get_string_field(&q["text"], lang, "");
                let mut localized = serde_json::json!({
                    "id": q["id"],
                    "text": text,
                });
                for field in &["facet", "reverse", "dichotomy", "pole", "type", "dimension", "trait"] {
                    if let Some(v) = q.get(*field) {
                        localized[*field] = v.clone();
                    }
                }
                localized
            }).collect()
        }).unwrap_or_default()
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

#[derive(serde::Deserialize)]
pub struct BigFiveForm {
    pub answers: Vec<QuestionVal>,
    pub language: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct QuestionVal {
    pub question_id: u32,
    pub value: serde_json::Value,
}

pub async fn submit_test(
    Path(test_type): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let lang = body.get("language")
        .and_then(|l| l.as_str())
        .unwrap_or("es");

    let result = match test_type.as_str() {
        "big_five" => {
            let answers: HashMap<u32, f64> = body["answers"].as_array()
                .map(|arr| arr.iter().filter_map(|a| {
                    let qid = a["question_id"].as_u64()? as u32;
                    let val = a["value"].as_f64()?;
                    Some((qid, val))
                }).collect())
                .unwrap_or_default();
            scoring::big_five::score(&answers, lang)
        }
        "mbti" => {
            let answers: HashMap<u32, String> = body["answers"].as_array()
                .map(|arr| arr.iter().filter_map(|a| {
                    let qid = a["question_id"].as_u64()? as u32;
                    let val = a["value"].as_str()?.to_string();
                    Some((qid, val))
                }).collect())
                .unwrap_or_default();
            scoring::mbti::score(&answers, lang)
        }
        "enneagram" => {
            let answers: HashMap<u32, f64> = body["answers"].as_array()
                .map(|arr| arr.iter().filter_map(|a| {
                    let qid = a["question_id"].as_u64()? as u32;
                    let val = a["value"].as_f64()?;
                    Some((qid, val))
                }).collect())
                .unwrap_or_default();
            scoring::enneagram::score(&answers, lang)
        }
        "disc" => {
            let answers: HashMap<u32, String> = body["answers"].as_array()
                .map(|arr| arr.iter().filter_map(|a| {
                    let qid = a["question_id"].as_u64()? as u32;
                    let val = a["value"].as_str()?.to_string();
                    Some((qid, val))
                }).collect())
                .unwrap_or_default();
            scoring::disc::score(&answers, lang)
        }
        "dark_triad" => {
            let answers: HashMap<u32, f64> = body["answers"].as_array()
                .map(|arr| arr.iter().filter_map(|a| {
                    let qid = a["question_id"].as_u64()? as u32;
                    let val = a["value"].as_f64()?;
                    Some((qid, val))
                }).collect())
                .unwrap_or_default();
            scoring::dark_triad::score(&answers, lang)
        }
        "human_design" => {
            let bd = body["birth_date"].as_str().unwrap_or("1990-01-01");
            let bt = body["birth_time"].as_str().unwrap_or("12:00");
            let bl = body["birth_location"].as_str().unwrap_or("Unknown");
            scoring::human_design::calculate(bd, bt, bl, lang)
        }
        _ => return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"detail": "Test not found"})),
        )),
    };

    Ok(Json(result))
}
