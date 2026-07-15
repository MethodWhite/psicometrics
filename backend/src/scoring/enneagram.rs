use std::collections::HashMap;

use crate::data::load_test_data;
use crate::error::{AppError, AppResult};
use crate::interpretation;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::load_test_data;

    fn make_answers(dominant_type: u32, value: f64) -> HashMap<u32, f64> {
        let data = load_test_data("enneagram").unwrap();
        let questions = data["questions"].as_array().unwrap();
        let mut answers = HashMap::new();
        for q in questions {
            if !q.get("attention_check").and_then(|v| v.as_bool()).unwrap_or(false) {
                let qid = q["id"].as_u64().unwrap() as u32;
                let qtype = q["type"].as_u64().unwrap_or(1) as u32;
                // Give high values to the dominant type, low to others
                let ans = if qtype == dominant_type { value } else { 1.0 };
                answers.insert(qid, ans);
            }
        }
        answers
    }

    #[test]
    fn test_enneagram_has_required_fields() {
        let answers = make_answers(4, 4.0);
        let result = score(&answers, "en").unwrap();
        let obj = result.as_object().unwrap();
        assert!(obj.contains_key("dominant_type"));
        assert!(obj.contains_key("wing"));
        assert!(obj.contains_key("scores"));
        assert!(obj.contains_key("profile_summary"));
    }

    #[test]
    fn test_enneagram_dominant_type_4() {
        let answers = make_answers(4, 4.0);
        let result = score(&answers, "en").unwrap();
        assert_eq!(result["dominant_type"].as_u64().unwrap(), 4);
    }

    #[test]
    fn test_enneagram_dominant_type_9() {
        let answers = make_answers(9, 4.0);
        let result = score(&answers, "en").unwrap();
        assert_eq!(result["dominant_type"].as_u64().unwrap(), 9);
    }

    #[test]
    fn test_enneagram_dominant_type_1() {
        let answers = make_answers(1, 4.0);
        let result = score(&answers, "en").unwrap();
        assert_eq!(result["dominant_type"].as_u64().unwrap(), 1);
    }

    #[test]
    fn test_enneagram_wing_is_adjacent() {
        let answers = make_answers(5, 4.0);
        let result = score(&answers, "en").unwrap();
        let dom = result["dominant_type"].as_u64().unwrap();
        let wing = result["wing"].as_u64().unwrap();
        // Wing must be adjacent to dominant type (mod 9)
        let adjacent = [if dom == 1 { 9 } else { dom - 1 }, if dom == 9 { 1 } else { dom + 1 }];
        assert!(adjacent.contains(&wing), "wing {} not adjacent to dominant {}", wing, dom);
    }

    #[test]
    fn test_enneagram_scores_in_range() {
        let answers = make_answers(4, 3.0);
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        for (t, val) in scores {
            let v = val.as_f64().unwrap();
            assert!(v >= 0.0 && v <= 100.0, "type {} score {} out of [0,100]", t, v);
        }
    }

    #[test]
    fn test_enneagram_spanish() {
        let answers = make_answers(2, 4.0);
        let result = score(&answers, "es").unwrap();
        assert!(result["profile_summary"].as_str().unwrap_or("").len() > 0);
    }
}

fn normalize(value: f64, min: f64, max: f64) -> f64 {
    ((value - min) / (max - min)) * 100.0
}

pub fn score(answers: &HashMap<u32, f64>, language: &str) -> AppResult<serde_json::Value> {
    let data = load_test_data("enneagram").map_err(|e| {
        AppError::Internal(format!("Failed to load enneagram data: {}", e.0))
    })?;
    let questions = data["questions"].as_array().ok_or_else(|| {
        AppError::Internal("enneagram data missing questions array".to_string())
    })?;

    let mut type_raw: HashMap<u32, Vec<f64>> = HashMap::new();
    for t in 1..=9 { type_raw.insert(t, Vec::new()); }

    for q in questions {
        let qid = q["id"].as_u64().unwrap_or(0) as u32;
        if let Some(&value) = answers.get(&qid) {
            let etype = q["type"].as_u64().unwrap_or(1) as u32;
            type_raw.entry(etype).or_default().push(value);
        }
    }

    let mut type_scores = serde_json::Map::new();
    for t in 1..=9 {
        let values = type_raw.get(&t).cloned().unwrap_or_default();
        let score = if values.is_empty() {
            0.0
        } else {
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            (normalize(mean, 1.0, 4.0) * 10.0).round() / 10.0
        };
        type_scores.insert(t.to_string(), serde_json::json!(score));
    }

    let dominant = (1..=9)
        .max_by(|&a, &b| {
            let sa = type_scores[&a.to_string()].as_f64().unwrap_or(0.0);
            let sb = type_scores[&b.to_string()].as_f64().unwrap_or(0.0);
            sa.partial_cmp(&sb).unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or(1);

    let adjacent = [if dominant == 1 { 9 } else { dominant - 1 }, if dominant == 9 { 1 } else { dominant + 1 }];
    let wing = adjacent.into_iter().max_by(|&a, &b| {
        let sa = type_scores[&a.to_string()].as_f64().unwrap_or(0.0);
        let sb = type_scores[&b.to_string()].as_f64().unwrap_or(0.0);
        sa.partial_cmp(&sb).unwrap_or(std::cmp::Ordering::Equal)
    }).unwrap_or(adjacent[0]);

    let dom_score = type_scores[&dominant.to_string()].as_f64().unwrap_or(0.0);
    let wing_score = type_scores[&wing.to_string()].as_f64().unwrap_or(0.0);

    let summary = if language == "en" {
        format!("Type {}w{}. Highest scores: Type {} ({:.0}%), Type {} ({:.0}%).", dominant, wing, dominant, dom_score, wing, wing_score)
    } else {
        format!("Tipo {}w{}. Puntajes más altos: Tipo {} ({:.0}%), Tipo {} ({:.0}%).", dominant, wing, dominant, dom_score, wing, wing_score)
    };

    let scores_hash: HashMap<String, f64> = type_scores.iter()
        .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f)))
        .collect();

    let interp = interpretation::get_interpretation("enneagram", &scores_hash, language);
    let recs = interpretation::get_recommendations("enneagram", &scores_hash, language);

    let mut result = serde_json::Map::new();
    result.insert("dominant_type".into(), serde_json::json!(dominant));
    result.insert("wing".into(), serde_json::json!(wing));
    result.insert("scores".into(), serde_json::json!(type_scores));
    result.insert("profile_summary".into(), serde_json::json!(summary));
    result.insert("interpretation".into(), interp);
    if let Some(obj) = recs.as_object() {
        for (k, v) in obj {
            result.insert(k.clone(), v.clone());
        }
    }
    Ok(serde_json::Value::Object(result))
}
