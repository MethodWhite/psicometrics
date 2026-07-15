use std::collections::HashMap;

use crate::data::load_test_data;
use crate::error::{AppError, AppResult};
use crate::interpretation;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::load_test_data;

    fn make_answers(preferred: &str) -> HashMap<u32, String> {
        let data = load_test_data("disc").unwrap();
        let questions = data["questions"].as_array().unwrap();
        let mut answers = HashMap::new();
        for q in questions {
            if !q.get("attention_check").and_then(|v| v.as_bool()).unwrap_or(false) {
                let qid = q["id"].as_u64().unwrap() as u32;
                let ans = match preferred {
                    "D" => "a",
                    "I" => "b",
                    "S" => "c",
                    "C" => "d",
                    _ => "a",
                };
                answers.insert(qid, ans.to_string());
            }
        }
        answers
    }

    #[test]
    fn test_disc_has_required_fields() {
        let answers = make_answers("D");
        let result = score(&answers, "en").unwrap();
        let obj = result.as_object().unwrap();
        assert!(obj.contains_key("primary_style"));
        assert!(obj.contains_key("secondary_style"));
        assert!(obj.contains_key("scores"));
        assert!(obj.contains_key("profile_summary"));
    }

    #[test]
    fn test_disc_primary_dominance() {
        let answers = make_answers("D");
        let result = score(&answers, "en").unwrap();
        assert_eq!(result["primary_style"].as_str().unwrap(), "D");
    }

    #[test]
    fn test_disc_primary_influence() {
        let answers = make_answers("I");
        let result = score(&answers, "en").unwrap();
        assert_eq!(result["primary_style"].as_str().unwrap(), "I");
    }

    #[test]
    fn test_disc_primary_steadiness() {
        let answers = make_answers("S");
        let result = score(&answers, "en").unwrap();
        assert_eq!(result["primary_style"].as_str().unwrap(), "S");
    }

    #[test]
    fn test_disc_primary_compliance() {
        let answers = make_answers("C");
        let result = score(&answers, "en").unwrap();
        assert_eq!(result["primary_style"].as_str().unwrap(), "C");
    }

    #[test]
    fn test_disc_scores_sum_to_100() {
        let answers = make_answers("D");
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        let sum: f64 = scores.values().filter_map(|v| v.as_f64()).sum();
        // Should be close to 100 (within rounding tolerance)
        assert!((sum - 100.0).abs() < 1.0, "DISC scores sum to {:.1}, expected ~100", sum);
    }

    #[test]
    fn test_disc_spanish() {
        let answers = make_answers("I");
        let result = score(&answers, "es").unwrap();
        assert!(result["profile_summary"].as_str().unwrap_or("").len() > 0);
    }
}

const DIMENSIONS: [&str; 4] = ["D", "I", "S", "C"];

pub fn score(answers: &HashMap<u32, String>, language: &str) -> AppResult<serde_json::Value> {
    let data = load_test_data("disc").map_err(|e| {
        AppError::Internal(format!("Failed to load disc data: {}", e.0))
    })?;
    let questions = data["questions"].as_array().ok_or_else(|| {
        AppError::Internal("disc data missing questions array".to_string())
    })?;

    let mut counts: HashMap<&str, u32> = HashMap::new();
    for d in &DIMENSIONS { counts.insert(d, 0); }

    for q in questions {
        let qid = q["id"].as_u64().unwrap_or(0) as u32;
        if let Some(response) = answers.get(&qid) {
            let dim = match response.as_str() {
                "a" => "D", "b" => "I", "c" => "S", "d" => "C",
                _ => continue,
            };
            *counts.entry(dim).or_insert(0) += 1;
        }
    }

    let total: u32 = counts.values().sum();
    let mut scores_map = serde_json::Map::new();
    for d in &DIMENSIONS {
        let pct = if total > 0 {
            (counts.get(d).copied().unwrap_or(0) as f64 / total as f64 * 100.0 * 10.0).round() / 10.0
        } else {
            25.0
        };
        scores_map.insert(d.to_string(), serde_json::json!(pct));
    }

    let mut sorted: Vec<(&&str, f64)> = DIMENSIONS.iter()
        .map(|d| (d, scores_map[*d].as_f64().unwrap_or(0.0)))
        .collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let primary = sorted[0].0;
    let secondary = sorted[1].0;

    let summary = if language == "en" {
        format!("Primary: {} ({:.0}%), Secondary: {} ({:.0}%).", primary, sorted[0].1, secondary, sorted[1].1)
    } else {
        format!("Primario: {} ({:.0}%), Secundario: {} ({:.0}%).", primary, sorted[0].1, secondary, sorted[1].1)
    };

    let scores_hash: HashMap<String, f64> = scores_map.iter()
        .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f)))
        .collect();

    let interp = interpretation::get_interpretation("disc", &scores_hash, language);
    let recs = interpretation::get_recommendations("disc", &scores_hash, language);

    let mut result = serde_json::Map::new();
    result.insert("primary_style".into(), serde_json::json!(primary));
    result.insert("secondary_style".into(), serde_json::json!(secondary));
    result.insert("scores".into(), serde_json::json!(scores_map));
    result.insert("profile_summary".into(), serde_json::json!(summary));
    result.insert("interpretation".into(), interp);
    if let Some(obj) = recs.as_object() {
        for (k, v) in obj {
            result.insert(k.clone(), v.clone());
        }
    }
    Ok(serde_json::Value::Object(result))
}
