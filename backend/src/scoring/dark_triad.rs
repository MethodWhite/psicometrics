use std::collections::HashMap;

use crate::data::load_test_data;
use crate::error::{AppError, AppResult};
use crate::interpretation;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::load_test_data;

    fn make_answers(value: f64) -> HashMap<u32, f64> {
        let data = load_test_data("dark_triad").unwrap();
        let questions = data["questions"].as_array().unwrap();
        let mut answers = HashMap::new();
        for q in questions {
            if !q.get("attention_check").and_then(|v| v.as_bool()).unwrap_or(false) {
                let qid = q["id"].as_u64().unwrap() as u32;
                answers.insert(qid, value);
            }
        }
        answers
    }

    #[test]
    fn test_dark_triad_has_required_fields() {
        let answers = make_answers(3.0);
        let result = score(&answers, "en").unwrap();
        let obj = result.as_object().unwrap();
        assert!(obj.contains_key("scores"));
        assert!(obj.contains_key("dark_core"));
        assert!(obj.contains_key("risk_level"));
        assert!(obj.contains_key("profile_summary"));
    }

    #[test]
    fn test_dark_triad_scores_in_range() {
        let answers = make_answers(4.0);
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        for (trait_name, val) in scores {
            let v = val.as_f64().unwrap();
            assert!(v >= 0.0 && v <= 100.0, "trait {} score {} out of [0,100]", trait_name, v);
        }
    }

    #[test]
    fn test_dark_triad_dark_core_average_of_traits() {
        let answers = make_answers(5.0);
        let result = score(&answers, "en").unwrap();
        let dark_core = result["dark_core"].as_f64().unwrap();
        assert!(dark_core > 0.0);
    }

    #[test]
    fn test_dark_triad_risk_level_string() {
        let answers = make_answers(1.0);
        let result = score(&answers, "en").unwrap();
        let risk = result["risk_level"].as_str().unwrap();
        assert!(["minimal", "low", "moderate", "high"].contains(&risk),
                "unknown risk level: {}", risk);
    }

    #[test]
    fn test_dark_triad_high_scores_high_risk() {
        let answers = make_answers(5.0);
        let result = score(&answers, "en").unwrap();
        let dark_core = result["dark_core"].as_f64().unwrap();
        let risk = result["risk_level"].as_str().unwrap();
        // All 5s should produce high scores
        assert!(dark_core > 50.0, "dark core {:.1} should be >50 for all-5 answers", dark_core);
        assert_eq!(risk, "high", "all-5 answers should be 'high' risk, got '{}'", risk);
    }

    #[test]
    fn test_dark_triad_spanish() {
        let answers = make_answers(2.0);
        let result = score(&answers, "es").unwrap();
        assert!(result["profile_summary"].as_str().unwrap_or("").len() > 0);
    }
}

fn normalize(value: f64, min: f64, max: f64) -> f64 {
    ((value - min) / (max - min)) * 100.0
}

pub fn score(answers: &HashMap<u32, f64>, language: &str) -> AppResult<serde_json::Value> {
    let data = load_test_data("dark_triad").map_err(|e| {
        AppError::Internal(format!("Failed to load dark_triad data: {}", e.0))
    })?;
    let questions = data["questions"].as_array().ok_or_else(|| {
        AppError::Internal("dark_triad data missing questions array".to_string())
    })?;

    let mut trait_raw: HashMap<&str, Vec<f64>> = HashMap::new();
    trait_raw.insert("M", Vec::new());
    trait_raw.insert("N", Vec::new());
    trait_raw.insert("P", Vec::new());

    for q in questions {
        let qid = q["id"].as_u64().unwrap_or(0) as u32;
        if let Some(&value) = answers.get(&qid) {
            let trait_name = q["trait"].as_str().unwrap_or("");
            trait_raw.entry(trait_name).or_default().push(value);
        }
    }

    let mut scores_map = serde_json::Map::new();
    for (trait_name, values) in &trait_raw {
        let score = if values.is_empty() {
            0.0
        } else {
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            (normalize(mean, 1.0, 5.0) * 10.0).round() / 10.0
        };
        let key = match *trait_name {
            "M" => "machiavellianism",
            "N" => "narcissism",
            "P" => "psychopathy",
            _ => *trait_name,
        };
        scores_map.insert(key.to_string(), serde_json::json!(score));
    }

    let dark_core = {
        let sum: f64 = scores_map.values().filter_map(|v| v.as_f64()).sum();
        (sum / 3.0 * 10.0).round() / 10.0
    };

    let risk_level = if dark_core >= 75.0 { "high" }
        else if dark_core >= 50.0 { "moderate" }
        else if dark_core >= 25.0 { "low" }
        else { "minimal" };

    let summary = if language == "en" {
        format!("Dark Core: {:.0}% ({} risk).", dark_core, risk_level)
    } else {
        format!("Dark Core: {:.0}% (riesgo {}).", dark_core, risk_level)
    };

    let scores_hash: HashMap<String, f64> = scores_map.iter()
        .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f)))
        .collect();

    let interp = interpretation::get_interpretation("dark_triad", &scores_hash, language);
    let recs = interpretation::get_recommendations("dark_triad", &scores_hash, language);

    let mut result = serde_json::Map::new();
    result.insert("scores".into(), serde_json::json!(scores_map));
    result.insert("dark_core".into(), serde_json::json!(dark_core));
    result.insert("risk_level".into(), serde_json::json!(risk_level));
    result.insert("profile_summary".into(), serde_json::json!(summary));
    result.insert("interpretation".into(), interp);
    if let Some(obj) = recs.as_object() {
        for (k, v) in obj {
            result.insert(k.clone(), v.clone());
        }
    }
    Ok(serde_json::Value::Object(result))
}
