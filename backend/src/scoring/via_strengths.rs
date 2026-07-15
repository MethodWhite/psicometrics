use std::collections::HashMap;

use crate::data::load_test_data;
use crate::error::{AppError, AppResult};
use crate::interpretation;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::load_test_data;

    fn make_answers(value: f64) -> HashMap<u32, f64> {
        let data = load_test_data("via_strengths").unwrap();
        let questions = data["questions"].as_array().unwrap();
        let mut answers = HashMap::new();
        for q in questions {
            let qid = q["id"].as_u64().unwrap() as u32;
            answers.insert(qid, value);
        }
        answers
    }

    #[test]
    fn test_via_strengths_has_required_fields() {
        let answers = make_answers(4.0);
        let result = score(&answers, "en").unwrap();
        let obj = result.as_object().unwrap();
        assert!(obj.contains_key("scores"));
        assert!(obj.contains_key("signature_strengths"));
        assert!(obj.contains_key("profile_summary"));
    }

    #[test]
    fn test_via_strengths_scores_in_range() {
        let answers = make_answers(3.0);
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        for (dim, val) in scores {
            let v = val.as_f64().unwrap();
            assert!(v >= 0.0 && v <= 100.0, "dimension {} score {} out of [0,100]", dim, v);
        }
    }

    #[test]
    fn test_via_strengths_all_dimensions_present() {
        let answers = make_answers(3.0);
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        for dim in &["wisdom", "courage", "humanity", "justice", "temperance", "transcendence"] {
            assert!(scores.contains_key(*dim), "missing dimension: {}", dim);
        }
    }

    #[test]
    fn test_via_strengths_signature_strengths_nonempty() {
        let answers = make_answers(5.0);
        let result = score(&answers, "en").unwrap();
        let sig = result["signature_strengths"].as_array().unwrap();
        assert!(!sig.is_empty(), "signature strengths should not be empty");
    }

    #[test]
    fn test_via_strengths_high_scores() {
        let answers = make_answers(5.0);
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        for (dim, val) in scores {
            let v = val.as_f64().unwrap();
            assert!(v > 50.0, "dimension {} should be >50 with all-5 answers, got {}", dim, v);
        }
    }

    #[test]
    fn test_via_strengths_spanish() {
        let answers = make_answers(3.0);
        let result = score(&answers, "es").unwrap();
        assert!(result["profile_summary"].as_str().unwrap_or("").len() > 0);
    }
}

fn normalize(value: f64, min: f64, max: f64) -> f64 {
    ((value - min) / (max - min)) * 100.0
}

const DIMENSIONS: &[&str] = &[
    "wisdom", "courage", "humanity",
    "justice", "temperance", "transcendence",
];

pub fn score(answers: &HashMap<u32, f64>, language: &str) -> AppResult<serde_json::Value> {
    let data = load_test_data("via_strengths").map_err(|e| {
        AppError::Internal(format!("Failed to load via_strengths data: {}", e.0))
    })?;
    let questions = data["questions"].as_array().ok_or_else(|| {
        AppError::Internal("via_strengths data missing questions array".to_string())
    })?;

    let mut dim_raw: HashMap<&str, Vec<f64>> = HashMap::new();
    for d in DIMENSIONS {
        dim_raw.insert(d, Vec::new());
    }

    for q in questions {
        let qid = q["id"].as_u64().unwrap_or(0) as u32;
        if let Some(&value) = answers.get(&qid) {
            let dim = q["dimension"].as_str().unwrap_or("");
            dim_raw.entry(dim).or_default().push(value);
        }
    }

    let mut scores_map = serde_json::Map::new();
    for d in DIMENSIONS {
        let values = dim_raw.get(d).cloned().unwrap_or_default();
        let score = if values.is_empty() {
            0.0
        } else {
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            (normalize(mean, 1.0, 5.0) * 10.0).round() / 10.0
        };
        scores_map.insert(d.to_string(), serde_json::json!(score));
    }

    // Rank dimensions to find signature strengths
    let mut ranked: Vec<(&str, f64)> = DIMENSIONS.iter()
        .filter_map(|d| {
            let score = scores_map.get(*d).and_then(|v| v.as_f64())?;
            Some((*d, score))
        })
        .collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Signature strengths = dimensions scored >= 70
    let signature: Vec<&str> = ranked.iter()
        .filter(|(_, s)| *s >= 70.0)
        .map(|(d, _)| *d)
        .collect();

    let dimension_names = data["dimensions"].as_object().unwrap();

    let summary = if language == "en" {
        let sig_str = if signature.is_empty() {
            "No signature strengths above 70%.".to_string()
        } else {
            let names: Vec<String> = signature.iter()
                .map(|d| d.replace('_', " "))
                .collect();
            format!("Signature Strengths: {}.", names.join(", "))
        };
        let top = ranked.first().map(|(d, s)| format!("{} ({:.0}%)", d.replace('_', " "), s)).unwrap_or_default();
        format!("{} Top: {}.", sig_str, top)
    } else {
        let sig_str = if signature.is_empty() {
            "Ninguna fortaleza supera el 70%.".to_string()
        } else {
            let names: Vec<String> = signature.iter()
                .filter_map(|d| dimension_names.get(*d).and_then(|v| v.get("es")).and_then(|v| v.as_str()).map(|s| s.to_string()))
                .collect();
            format!("Fortalezas Firma: {}.", names.join(", "))
        };
        let top_d = ranked.first().map(|(d, _)| *d).unwrap_or("wisdom");
        let top_name = dimension_names.get(top_d).and_then(|v| v.get("es")).and_then(|v| v.as_str()).unwrap_or(top_d);
        let top_score = ranked.first().map(|(_, s)| s).unwrap_or(&0.0);
        format!("{} Principal: {} ({:.0}%).", sig_str, top_name, top_score)
    };

    let scores_hash: HashMap<String, f64> = scores_map.iter()
        .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f)))
        .collect();

    let interp = interpretation::get_interpretation("via_strengths", &scores_hash, language);
    let recs = interpretation::get_recommendations("via_strengths", &scores_hash, language);

    let mut result = serde_json::Map::new();
    result.insert("scores".into(), serde_json::json!(scores_map));
    result.insert("signature_strengths".into(), serde_json::json!(signature));
    result.insert("profile_summary".into(), serde_json::json!(summary));
    result.insert("interpretation".into(), interp);
    if let Some(obj) = recs.as_object() {
        for (k, v) in obj {
            result.insert(k.clone(), v.clone());
        }
    }
    Ok(serde_json::Value::Object(result))
}
