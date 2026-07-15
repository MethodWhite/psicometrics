use axum::{
    extract::{Path, Query},
    http::header,
    Json,
};
use serde::Serialize;
use std::collections::HashMap;

use crate::compatibility;
use crate::data::{get_test_type_list, load_test_data, TestDataNotFound};
use crate::error::{AppError, AppResult};
use crate::i18n::get_string_field;
use crate::scoring;
use crate::validation;

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

fn get_data(test_type: &str) -> AppResult<&'static serde_json::Value> {
    load_test_data(test_type).map_err(|e: TestDataNotFound| {
        if [
            "attachment_style", "big_five", "career_aptitude", "dark_triad", "disc",
            "emotional_intelligence", "enneagram", "human_design", "love_languages",
            "mbti", "via_strengths",
        ].contains(&test_type)
        {
            AppError::Internal(format!("Test data for '{test_type}' is corrupted: {}", e.0))
        } else {
            AppError::NotFound(format!("Test '{test_type}' not found"))
        }
    })
}

fn valid_tests() -> &'static [&'static str] {
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

pub async fn submit_test(
    Path(test_type): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> AppResult<Json<serde_json::Value>> {
    if !valid_tests().contains(&test_type.as_str()) {
        return Err(AppError::NotFound("Test not found".to_string()));
    }

    let lang = body
        .get("language")
        .and_then(|l| l.as_str())
        .unwrap_or("es");

    let raw_answers: HashMap<u32, serde_json::Value> = body["answers"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|a| {
                    let qid = a["question_id"].as_u64()? as u32;
                    let val = a["value"].clone();
                    Some((qid, val))
                })
                .collect()
        })
        .unwrap_or_default();

    let completion_time = body
        .get("completion_time")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32);

    let validity = validation::validate_responses(&test_type, &raw_answers, completion_time);

    let mut result = match test_type.as_str() {
        "attachment_style" => {
            let answers: HashMap<u32, f64> = raw_answers
                .iter()
                .filter_map(|(&qid, v)| v.as_f64().map(|f| (qid, f)))
                .collect();
            scoring::attachment_style::score(&answers, lang)?
        }
        "big_five" => {
            let answers: HashMap<u32, f64> = raw_answers
                .iter()
                .filter_map(|(&qid, v)| v.as_f64().map(|f| (qid, f)))
                .collect();
            scoring::big_five::score(&answers, lang)?
        }
        "career_aptitude" => {
            let answers: HashMap<u32, f64> = raw_answers
                .iter()
                .filter_map(|(&qid, v)| v.as_f64().map(|f| (qid, f)))
                .collect();
            scoring::career_aptitude::score(&answers, lang)?
        }
        "dark_triad" => {
            let answers: HashMap<u32, f64> = raw_answers
                .iter()
                .filter_map(|(&qid, v)| v.as_f64().map(|f| (qid, f)))
                .collect();
            scoring::dark_triad::score(&answers, lang)?
        }
        "disc" => {
            let answers: HashMap<u32, String> = raw_answers
                .iter()
                .filter_map(|(&qid, v)| v.as_str().map(|s| (qid, s.to_string())))
                .collect();
            scoring::disc::score(&answers, lang)?
        }
        "emotional_intelligence" => {
            let answers: HashMap<u32, f64> = raw_answers
                .iter()
                .filter_map(|(&qid, v)| v.as_f64().map(|f| (qid, f)))
                .collect();
            scoring::emotional_intelligence::score(&answers, lang)?
        }
        "enneagram" => {
            let answers: HashMap<u32, f64> = raw_answers
                .iter()
                .filter_map(|(&qid, v)| v.as_f64().map(|f| (qid, f)))
                .collect();
            scoring::enneagram::score(&answers, lang)?
        }
        "human_design" => {
            let bd = body["birth_date"].as_str().unwrap_or("1990-01-01");
            let bt = body["birth_time"].as_str().unwrap_or("12:00");
            let bl = body["birth_location"].as_str().unwrap_or("Unknown");
            scoring::human_design::calculate(bd, bt, bl, lang)
        }
        "love_languages" => {
            let answers: HashMap<u32, f64> = raw_answers
                .iter()
                .filter_map(|(&qid, v)| v.as_f64().map(|f| (qid, f)))
                .collect();
            scoring::love_languages::score(&answers, lang)?
        }
        "mbti" => {
            let answers: HashMap<u32, String> = raw_answers
                .iter()
                .filter_map(|(&qid, v)| v.as_str().map(|s| (qid, s.to_string())))
                .collect();
            scoring::mbti::score(&answers, lang)?
        }
        "via_strengths" => {
            let answers: HashMap<u32, f64> = raw_answers
                .iter()
                .filter_map(|(&qid, v)| v.as_f64().map(|f| (qid, f)))
                .collect();
            scoring::via_strengths::score(&answers, lang)?
        }
        _ => unreachable!(),
    };

    if let Some(obj) = result.as_object_mut() {
        obj.insert(
            "validity".to_string(),
            serde_json::to_value(&validity).unwrap_or_default(),
        );
    }

    Ok(Json(result))
}

pub async fn generate_report(
    Path(test_type): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> AppResult<(axum::http::HeaderMap, Vec<u8>)> {
    if !valid_tests().contains(&test_type.as_str()) {
        return Err(AppError::NotFound("Test not found".to_string()));
    }

    let lang = body
        .get("language")
        .and_then(|l| l.as_str())
        .unwrap_or("es");

    let result = match test_type.as_str() {
        "attachment_style" => {
            let answers: HashMap<u32, f64> = body["answers"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|a| {
                            let qid = a["question_id"].as_u64()? as u32;
                            let val = a["value"].as_f64()?;
                            Some((qid, val))
                        })
                        .collect()
                })
                .unwrap_or_default();
            scoring::attachment_style::score(&answers, lang)?
        }
        "big_five" => {
            let answers: HashMap<u32, f64> = body["answers"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|a| {
                            let qid = a["question_id"].as_u64()? as u32;
                            let val = a["value"].as_f64()?;
                            Some((qid, val))
                        })
                        .collect()
                })
                .unwrap_or_default();
            scoring::big_five::score(&answers, lang)?
        }
        "career_aptitude" => {
            let answers: HashMap<u32, f64> = body["answers"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|a| {
                            let qid = a["question_id"].as_u64()? as u32;
                            let val = a["value"].as_f64()?;
                            Some((qid, val))
                        })
                        .collect()
                })
                .unwrap_or_default();
            scoring::career_aptitude::score(&answers, lang)?
        }
        "dark_triad" => {
            let answers: HashMap<u32, f64> = body["answers"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|a| {
                            let qid = a["question_id"].as_u64()? as u32;
                            let val = a["value"].as_f64()?;
                            Some((qid, val))
                        })
                        .collect()
                })
                .unwrap_or_default();
            scoring::dark_triad::score(&answers, lang)?
        }
        "disc" => {
            let answers: HashMap<u32, String> = body["answers"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|a| {
                            let qid = a["question_id"].as_u64()? as u32;
                            let val = a["value"].as_str()?.to_string();
                            Some((qid, val))
                        })
                        .collect()
                })
                .unwrap_or_default();
            scoring::disc::score(&answers, lang)?
        }
        "emotional_intelligence" => {
            let answers: HashMap<u32, f64> = body["answers"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|a| {
                            let qid = a["question_id"].as_u64()? as u32;
                            let val = a["value"].as_f64()?;
                            Some((qid, val))
                        })
                        .collect()
                })
                .unwrap_or_default();
            scoring::emotional_intelligence::score(&answers, lang)?
        }
        "enneagram" => {
            let answers: HashMap<u32, f64> = body["answers"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|a| {
                            let qid = a["question_id"].as_u64()? as u32;
                            let val = a["value"].as_f64()?;
                            Some((qid, val))
                        })
                        .collect()
                })
                .unwrap_or_default();
            scoring::enneagram::score(&answers, lang)?
        }
        "human_design" => {
            let bd = body["birth_date"].as_str().unwrap_or("1990-01-01");
            let bt = body["birth_time"].as_str().unwrap_or("12:00");
            let bl = body["birth_location"].as_str().unwrap_or("Unknown");
            scoring::human_design::calculate(bd, bt, bl, lang)
        }
        "love_languages" => {
            let answers: HashMap<u32, f64> = body["answers"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|a| {
                            let qid = a["question_id"].as_u64()? as u32;
                            let val = a["value"].as_f64()?;
                            Some((qid, val))
                        })
                        .collect()
                })
                .unwrap_or_default();
            scoring::love_languages::score(&answers, lang)?
        }
        "mbti" => {
            let answers: HashMap<u32, String> = body["answers"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|a| {
                            let qid = a["question_id"].as_u64()? as u32;
                            let val = a["value"].as_str()?.to_string();
                            Some((qid, val))
                        })
                        .collect()
                })
                .unwrap_or_default();
            scoring::mbti::score(&answers, lang)?
        }
        "via_strengths" => {
            let answers: HashMap<u32, f64> = body["answers"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|a| {
                            let qid = a["question_id"].as_u64()? as u32;
                            let val = a["value"].as_f64()?;
                            Some((qid, val))
                        })
                        .collect()
                })
                .unwrap_or_default();
            scoring::via_strengths::score(&answers, lang)?
        }
        _ => unreachable!(),
    };

    let pdf_bytes = crate::report::generate_report(&test_type, &result, lang);

    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/pdf"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str(&format!(
            "attachment; filename=\"{}-report.pdf\"",
            test_type
        ))
        .unwrap_or_else(|_| header::HeaderValue::from_static("attachment")),
    );

    Ok((headers, pdf_bytes))
}

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
