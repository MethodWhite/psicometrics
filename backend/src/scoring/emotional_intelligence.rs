use std::collections::HashMap;

use crate::data::load_test_data;
use crate::error::{AppError, AppResult};
use crate::interpretation;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::load_test_data;

    fn make_answers(value: f64) -> HashMap<u32, f64> {
        let data = load_test_data("emotional_intelligence").unwrap();
        let questions = data["questions"].as_array().unwrap();
        let mut answers = HashMap::new();
        for q in questions {
            let qid = q["id"].as_u64().unwrap() as u32;
            answers.insert(qid, value);
        }
        answers
    }

    #[test]
    fn test_eq_has_required_fields() {
        let answers = make_answers(4.0);
        let result = score(&answers, "en").unwrap();
        let obj = result.as_object().unwrap();
        assert!(obj.contains_key("scores"));
        assert!(obj.contains_key("eq_total"));
        assert!(obj.contains_key("profile_summary"));
    }

    #[test]
    fn test_eq_scores_in_range() {
        let answers = make_answers(3.0);
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        for (dim, val) in scores {
            let v = val.as_f64().unwrap();
            assert!(v >= 0.0 && v <= 100.0, "dimension {} score {} out of [0,100]", dim, v);
        }
        let total = result["eq_total"].as_f64().unwrap();
        assert!(total >= 0.0 && total <= 100.0, "eq_total {} out of [0,100]", total);
    }

    #[test]
    fn test_eq_total_is_average() {
        let answers = make_answers(5.0);
        let result = score(&answers, "en").unwrap();
        let total = result["eq_total"].as_f64().unwrap();
        assert!(total > 50.0, "eq_total {:.1} should be >50 for all-5 answers", total);
    }

    #[test]
    fn test_eq_all_dimensions_present() {
        let answers = make_answers(4.0);
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        for dim in &["self_awareness", "self_regulation", "empathy", "social_skills"] {
            assert!(scores.contains_key(*dim), "missing dimension: {}", dim);
        }
    }

    #[test]
    fn test_eq_spanish() {
        let answers = make_answers(3.0);
        let result = score(&answers, "es").unwrap();
        assert!(result["profile_summary"].as_str().unwrap_or("").len() > 0);
    }
}

fn normalize(value: f64, min: f64, max: f64) -> f64 {
    ((value - min) / (max - min)) * 100.0
}

const DIMENSIONS: &[&str] = &[
    "self_awareness",
    "self_regulation",
    "empathy",
    "social_skills",
];

pub fn score(answers: &HashMap<u32, f64>, language: &str) -> AppResult<serde_json::Value> {
    let data = load_test_data("emotional_intelligence").map_err(|e| {
        AppError::Internal(format!("Failed to load emotional_intelligence data: {}", e.0))
    })?;
    let questions = data["questions"].as_array().ok_or_else(|| {
        AppError::Internal("emotional_intelligence data missing questions array".to_string())
    })?;

    let mut dim_raw: HashMap<&str, Vec<f64>> = HashMap::new();
    for d in DIMENSIONS {
        dim_raw.insert(d, Vec::new());
    }

    for q in questions {
        let qid = q["id"].as_u64().unwrap_or(0) as u32;
        if let Some(&value) = answers.get(&qid) {
            let dim = q["dimension"].as_str().unwrap_or("");
            let val = if qid == 19 { 6.0 - value } else { value }; // Reverse only question 19
            dim_raw.entry(dim).or_default().push(val);
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

    let dim_scores: Vec<f64> = DIMENSIONS.iter()
        .filter_map(|d| scores_map.get(*d).and_then(|v| v.as_f64()))
        .collect();
    let eq_total = if dim_scores.is_empty() {
        0.0
    } else {
        let mean = dim_scores.iter().sum::<f64>() / dim_scores.len() as f64;
        (mean * 10.0).round() / 10.0
    };

    let dimension_names = data["dimensions"].as_object().unwrap();

    let summary = if language == "en" {
        format!(
            "Overall EQ: {:.0}%. Self-Awareness: {:.0}%, Self-Regulation: {:.0}%, Empathy: {:.0}%, Social Skills: {:.0}%.",
            eq_total,
            scores_map.get("self_awareness").and_then(|v| v.as_f64()).unwrap_or(0.0),
            scores_map.get("self_regulation").and_then(|v| v.as_f64()).unwrap_or(0.0),
            scores_map.get("empathy").and_then(|v| v.as_f64()).unwrap_or(0.0),
            scores_map.get("social_skills").and_then(|v| v.as_f64()).unwrap_or(0.0),
        )
    } else {
        let sa = dimension_names.get("self_awareness").and_then(|v| v.get("es")).and_then(|v| v.as_str()).unwrap_or("Autoconciencia");
        let sr = dimension_names.get("self_regulation").and_then(|v| v.get("es")).and_then(|v| v.as_str()).unwrap_or("Autorregulación");
        let em = dimension_names.get("empathy").and_then(|v| v.get("es")).and_then(|v| v.as_str()).unwrap_or("Empatía");
        let ss = dimension_names.get("social_skills").and_then(|v| v.get("es")).and_then(|v| v.as_str()).unwrap_or("Habilidades Sociales");
        format!(
            "IE Total: {:.0}%. {}: {:.0}%, {}: {:.0}%, {}: {:.0}%, {}: {:.0}%.",
            eq_total, sa,
            scores_map.get("self_awareness").and_then(|v| v.as_f64()).unwrap_or(0.0),
            sr,
            scores_map.get("self_regulation").and_then(|v| v.as_f64()).unwrap_or(0.0),
            em,
            scores_map.get("empathy").and_then(|v| v.as_f64()).unwrap_or(0.0),
            ss,
            scores_map.get("social_skills").and_then(|v| v.as_f64()).unwrap_or(0.0),
        )
    };

    let scores_hash: HashMap<String, f64> = scores_map.iter()
        .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f)))
        .collect();

    let interp = interpretation::get_interpretation("emotional_intelligence", &scores_hash, language);
    let recs = interpretation::get_recommendations("emotional_intelligence", &scores_hash, language);

    let mut result = serde_json::Map::new();
    result.insert("scores".into(), serde_json::json!(scores_map));
    result.insert("eq_total".into(), serde_json::json!(eq_total));
    result.insert("profile_summary".into(), serde_json::json!(summary));
    result.insert("interpretation".into(), interp);
    if let Some(obj) = recs.as_object() {
        for (k, v) in obj {
            result.insert(k.clone(), v.clone());
        }
    }
    Ok(serde_json::Value::Object(result))
}
