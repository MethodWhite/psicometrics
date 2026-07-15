use std::collections::HashMap;

use crate::data::load_test_data;
use crate::error::{AppError, AppResult};
use crate::interpretation;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::load_test_data;

    fn make_answers(value: f64) -> HashMap<u32, f64> {
        let data = load_test_data("attachment_style").unwrap();
        let questions = data["questions"].as_array().unwrap();
        let mut answers = HashMap::new();
        for q in questions {
            let qid = q["id"].as_u64().unwrap() as u32;
            answers.insert(qid, value);
        }
        answers
    }

    #[test]
    fn test_attachment_style_has_required_fields() {
        let answers = make_answers(4.0);
        let result = score(&answers, "en").unwrap();
        let obj = result.as_object().unwrap();
        assert!(obj.contains_key("scores"));
        assert!(obj.contains_key("primary_style"));
        assert!(obj.contains_key("profile_summary"));
    }

    #[test]
    fn test_attachment_style_scores_in_range() {
        let answers = make_answers(3.0);
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        for (style, val) in scores {
            let v = val.as_f64().unwrap();
            assert!(v >= 0.0 && v <= 100.0, "style {} score {} out of [0,100]", style, v);
        }
    }

    #[test]
    fn test_attachment_style_primary_high() {
        let answers = make_answers(5.0);
        let result = score(&answers, "en").unwrap();
        let primary = result["primary_style"].as_str().unwrap();
        let scores = result["scores"].as_object().unwrap();
        assert!(scores.contains_key(primary));
    }

    #[test]
    fn test_attachment_style_high_secure_detected() {
        let mut answers = HashMap::new();
        let data = load_test_data("attachment_style").unwrap();
        let questions = data["questions"].as_array().unwrap();
        for q in questions {
            let qid = q["id"].as_u64().unwrap() as u32;
            let dim = q["dimension"].as_str().unwrap_or("");
            let val = if dim == "secure" { 7.0 } else { 1.0 };
            answers.insert(qid, val);
        }
        let result = score(&answers, "en").unwrap();
        assert_eq!(result["primary_style"].as_str().unwrap(), "secure");
    }

    #[test]
    fn test_attachment_style_spanish() {
        let answers = make_answers(3.0);
        let result = score(&answers, "es").unwrap();
        assert!(result["profile_summary"].as_str().unwrap_or("").len() > 0);
    }
}

fn normalize(value: f64, min: f64, max: f64) -> f64 {
    ((value - min) / (max - min)) * 100.0
}

const DIMENSIONS: &[&str] = &["secure", "anxious", "avoidant"];

pub fn score(answers: &HashMap<u32, f64>, language: &str) -> AppResult<serde_json::Value> {
    let data = load_test_data("attachment_style").map_err(|e| {
        AppError::Internal(format!("Failed to load attachment_style data: {}", e.0))
    })?;
    let questions = data["questions"].as_array().ok_or_else(|| {
        AppError::Internal("attachment_style data missing questions array".to_string())
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
            // Secure uses 1-7 scale, same as anxious/avoidant
            // Higher secure = better; higher anxious/avoidant = more of that style
            (normalize(mean, 1.0, 7.0) * 10.0).round() / 10.0
        };
        scores_map.insert(d.to_string(), serde_json::json!(score));
    }

    let secure_score = scores_map.get("secure").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let anxious_score = scores_map.get("anxious").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let avoidant_score = scores_map.get("avoidant").and_then(|v| v.as_f64()).unwrap_or(0.0);

    // Classify: if secure >= 60 and both insecure < 60 → secure
    // Otherwise, the highest among insecure wins
    let primary_style = if secure_score >= 60.0 && anxious_score < 60.0 && avoidant_score < 60.0 {
        "secure"
    } else if anxious_score >= avoidant_score && anxious_score > secure_score {
        "anxious"
    } else if avoidant_score > anxious_score && avoidant_score > secure_score {
        "avoidant"
    } else if secure_score >= anxious_score && secure_score >= avoidant_score {
        "secure"
    } else {
        "secure"
    };

    let dimension_names = data["dimensions"].as_object().unwrap();

    let summary = if language == "en" {
        format!(
            "Primary Attachment Style: {} (Secure: {:.0}%, Anxious: {:.0}%, Avoidant: {:.0}%).",
            primary_style, secure_score, anxious_score, avoidant_score
        )
    } else {
        let name = dimension_names.get(primary_style)
            .and_then(|v| v.get("es"))
            .and_then(|v| v.as_str())
            .unwrap_or(primary_style);
        format!(
            "Estilo de Apego Principal: {} (Seguro: {:.0}%, Ansioso: {:.0}%, Evitativo: {:.0}%).",
            name, secure_score, anxious_score, avoidant_score
        )
    };

    let scores_hash: HashMap<String, f64> = scores_map.iter()
        .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f)))
        .collect();

    let interp = interpretation::get_interpretation("attachment_style", &scores_hash, language);
    let recs = interpretation::get_recommendations("attachment_style", &scores_hash, language);

    let mut result = serde_json::Map::new();
    result.insert("scores".into(), serde_json::json!(scores_map));
    result.insert("primary_style".into(), serde_json::json!(primary_style));
    result.insert("profile_summary".into(), serde_json::json!(summary));
    result.insert("interpretation".into(), interp);
    if let Some(obj) = recs.as_object() {
        for (k, v) in obj {
            result.insert(k.clone(), v.clone());
        }
    }
    Ok(serde_json::Value::Object(result))
}
