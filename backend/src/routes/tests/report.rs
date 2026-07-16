use axum::{extract::Path, http::header, Json};
use std::collections::HashMap;

use crate::error::AppResult;
use crate::scoring;

use super::valid_tests;

pub async fn generate_report(
    Path(test_type): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> AppResult<(axum::http::HeaderMap, Vec<u8>)> {
    if !valid_tests().contains(&test_type.as_str()) {
        return Err(crate::error::AppError::NotFound("Test not found".to_string()));
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
