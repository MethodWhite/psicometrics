use axum::{extract::Path, Json};
use std::collections::HashMap;

use crate::error::AppResult;
use crate::scoring;
use crate::validation;

use super::valid_tests;

pub async fn submit_test(
    Path(test_type): Path<String>,
    Json(body): Json<serde_json::Value>,
) -> AppResult<Json<serde_json::Value>> {
    if !valid_tests().contains(&test_type.as_str()) {
        return Err(crate::error::AppError::NotFound("Test not found".to_string()));
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
