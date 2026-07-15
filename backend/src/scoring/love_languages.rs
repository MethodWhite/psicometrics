use std::collections::HashMap;

use crate::data::load_test_data;
use crate::error::{AppError, AppResult};
use crate::interpretation;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::load_test_data;

    fn make_answers(value: f64) -> HashMap<u32, f64> {
        let data = load_test_data("love_languages").unwrap();
        let questions = data["questions"].as_array().unwrap();
        let mut answers = HashMap::new();
        for q in questions {
            let qid = q["id"].as_u64().unwrap() as u32;
            answers.insert(qid, value);
        }
        answers
    }

    #[test]
    fn test_love_languages_has_required_fields() {
        let answers = make_answers(4.0);
        let result = score(&answers, "en").unwrap();
        let obj = result.as_object().unwrap();
        assert!(obj.contains_key("scores"));
        assert!(obj.contains_key("primary_language"));
        assert!(obj.contains_key("secondary_language"));
        assert!(obj.contains_key("language_ranking"));
        assert!(obj.contains_key("profile_summary"));
    }

    #[test]
    fn test_love_languages_scores_in_range() {
        let answers = make_answers(3.0);
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        for (lang, val) in scores {
            let v = val.as_f64().unwrap();
            assert!(v >= 0.0 && v <= 100.0, "language {} score {} out of [0,100]", lang, v);
        }
    }

    #[test]
    fn test_love_languages_primary_high() {
        let answers = make_answers(5.0);
        let result = score(&answers, "en").unwrap();
        let primary = result["primary_language"].as_str().unwrap();
        let scores = result["scores"].as_object().unwrap();
        let primary_score = scores.get(primary).and_then(|v| v.as_f64()).unwrap();
        // With all 5s, primary should be one of the languages
        assert!(primary_score > 0.0);
        assert!(result["secondary_language"].as_str().is_some());
    }

    #[test]
    fn test_love_languages_spanish() {
        let answers = make_answers(2.0);
        let result = score(&answers, "es").unwrap();
        assert!(result["profile_summary"].as_str().unwrap_or("").len() > 0);
    }

    #[test]
    fn test_love_languages_all_languages_present() {
        let answers = make_answers(3.0);
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        for lang in &["words_of_affirmation", "quality_time", "receiving_gifts", "acts_of_service", "physical_touch"] {
            assert!(scores.contains_key(*lang), "missing language: {}", lang);
        }
    }
}

fn normalize(value: f64, min: f64, max: f64) -> f64 {
    ((value - min) / (max - min)) * 100.0
}

const DIMENSIONS: &[&str] = &[
    "words_of_affirmation",
    "quality_time",
    "receiving_gifts",
    "acts_of_service",
    "physical_touch",
];

pub fn score(answers: &HashMap<u32, f64>, language: &str) -> AppResult<serde_json::Value> {
    let data = load_test_data("love_languages").map_err(|e| {
        AppError::Internal(format!("Failed to load love_languages data: {}", e.0))
    })?;
    let questions = data["questions"].as_array().ok_or_else(|| {
        AppError::Internal("love_languages data missing questions array".to_string())
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

    // Sort by score descending to get ranking
    let mut ranked: Vec<(&str, f64)> = DIMENSIONS.iter()
        .filter_map(|d| {
            let score = scores_map.get(*d).and_then(|v| v.as_f64())?;
            Some((*d, score))
        })
        .collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let primary = ranked.first().map(|(k, _)| *k).unwrap_or("words_of_affirmation");
    let secondary = ranked.get(1).map(|(k, _)| *k).unwrap_or("quality_time");

    let primary_score = scores_map.get(primary).and_then(|v| v.as_f64()).unwrap_or(0.0);
    let secondary_score = scores_map.get(secondary).and_then(|v| v.as_f64()).unwrap_or(0.0);

    let summary = if language == "en" {
        format!(
            "Primary Love Language: {} ({:.0}%). Secondary: {} ({:.0}%).",
            primary.replace('_', " "),
            primary_score,
            secondary.replace('_', " "),
            secondary_score,
        )
    } else {
        let lang_names = data["dimensions"].as_object().unwrap();
        let primary_name = lang_names.get(primary)
            .and_then(|v| v.get("es"))
            .and_then(|v| v.as_str())
            .unwrap_or(primary);
        let secondary_name = lang_names.get(secondary)
            .and_then(|v| v.get("es"))
            .and_then(|v| v.as_str())
            .unwrap_or(secondary);
        format!(
            "Lenguaje del Amor Principal: {} ({:.0}%). Secundario: {} ({:.0}%).",
            primary_name,
            primary_score,
            secondary_name,
            secondary_score,
        )
    };

    let scores_hash: HashMap<String, f64> = scores_map.iter()
        .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f)))
        .collect();

    let interp = interpretation::get_interpretation("love_languages", &scores_hash, language);
    let recs = interpretation::get_recommendations("love_languages", &scores_hash, language);

    let mut result = serde_json::Map::new();
    result.insert("scores".into(), serde_json::json!(scores_map));
    result.insert("primary_language".into(), serde_json::json!(primary));
    result.insert("secondary_language".into(), serde_json::json!(secondary));
    result.insert("language_ranking".into(), serde_json::json!(
        ranked.iter().map(|(k, v)| serde_json::json!({"language": k, "score": v})).collect::<Vec<_>>()
    ));
    result.insert("profile_summary".into(), serde_json::json!(summary));
    result.insert("interpretation".into(), interp);
    if let Some(obj) = recs.as_object() {
        for (k, v) in obj {
            result.insert(k.clone(), v.clone());
        }
    }
    Ok(serde_json::Value::Object(result))
}
