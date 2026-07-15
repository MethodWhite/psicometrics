use std::collections::HashMap;

use crate::data::load_test_data;
use crate::error::{AppError, AppResult};
use crate::interpretation;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::load_test_data;

    fn make_answers(value: f64) -> HashMap<u32, f64> {
        let data = load_test_data("career_aptitude").unwrap();
        let questions = data["questions"].as_array().unwrap();
        let mut answers = HashMap::new();
        for q in questions {
            let qid = q["id"].as_u64().unwrap() as u32;
            answers.insert(qid, value);
        }
        answers
    }

    #[test]
    fn test_career_aptitude_has_required_fields() {
        let answers = make_answers(4.0);
        let result = score(&answers, "en").unwrap();
        let obj = result.as_object().unwrap();
        assert!(obj.contains_key("scores"));
        assert!(obj.contains_key("holland_code"));
        assert!(obj.contains_key("profile_summary"));
    }

    #[test]
    fn test_career_aptitude_scores_in_range() {
        let answers = make_answers(3.0);
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        for (dim, val) in scores {
            let v = val.as_f64().unwrap();
            assert!(v >= 0.0 && v <= 100.0, "dimension {} score {} out of [0,100]", dim, v);
        }
    }

    #[test]
    fn test_career_aptitude_holland_code_length() {
        let answers = make_answers(4.0);
        let result = score(&answers, "en").unwrap();
        let code = result["holland_code"].as_str().unwrap();
        assert_eq!(code.len(), 3, "Holland code should be 3 letters, got: {}", code);
    }

    #[test]
    fn test_career_aptitude_all_dimensions_present() {
        let answers = make_answers(3.0);
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        for dim in &["realistic", "investigative", "artistic", "social", "enterprising", "conventional"] {
            assert!(scores.contains_key(*dim), "missing dimension: {}", dim);
        }
    }

    #[test]
    fn test_career_aptitude_spanish() {
        let answers = make_answers(3.0);
        let result = score(&answers, "es").unwrap();
        assert!(result["profile_summary"].as_str().unwrap_or("").len() > 0);
    }
}

fn normalize(value: f64, min: f64, max: f64) -> f64 {
    ((value - min) / (max - min)) * 100.0
}

const DIMENSIONS: &[&str] = &[
    "realistic", "investigative", "artistic",
    "social", "enterprising", "conventional",
];

const DIM_CODES: &[(&str, &str)] = &[
    ("realistic", "R"),
    ("investigative", "I"),
    ("artistic", "A"),
    ("social", "S"),
    ("enterprising", "E"),
    ("conventional", "C"),
];

pub fn score(answers: &HashMap<u32, f64>, language: &str) -> AppResult<serde_json::Value> {
    let data = load_test_data("career_aptitude").map_err(|e| {
        AppError::Internal(format!("Failed to load career_aptitude data: {}", e.0))
    })?;
    let questions = data["questions"].as_array().ok_or_else(|| {
        AppError::Internal("career_aptitude data missing questions array".to_string())
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

    // Generate Holland Code (top 3 in order)
    let mut ranked: Vec<(String, String, f64)> = DIM_CODES.iter()
        .filter_map(|(dim, code)| {
            let score = scores_map.get(*dim).and_then(|v| v.as_f64())?;
            Some((dim.to_string(), code.to_string(), score))
        })
        .collect();
    ranked.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let holland_code: String = ranked.iter().take(3).map(|(_, code, _)| code.as_str()).collect();

    let dimension_names = data["dimensions"].as_object().unwrap();

    let summary = if language == "en" {
        let top1 = ranked.first().map(|(dim, _, score)| (dim.clone(), *score)).unwrap_or(("realistic".to_string(), 0.0));
        let top2 = ranked.get(1).map(|(dim, _, score)| (dim.clone(), *score)).unwrap_or(("investigative".to_string(), 0.0));
        let top3 = ranked.get(2).map(|(dim, _, score)| (dim.clone(), *score)).unwrap_or(("artistic".to_string(), 0.0));
        format!(
            "Holland Code: {}. Top areas: {} ({:.0}%), {} ({:.0}%), {} ({:.0}%).",
            holland_code,
            top1.0.replace('_', " "), top1.1,
            top2.0.replace('_', " "), top2.1,
            top3.0.replace('_', " "), top3.1,
        )
    } else {
        let top1 = ranked.first().map(|(dim, _, score)| (dim.clone(), *score)).unwrap_or(("realistic".to_string(), 0.0));
        let top2 = ranked.get(1).map(|(dim, _, score)| (dim.clone(), *score)).unwrap_or(("investigative".to_string(), 0.0));
        let top3 = ranked.get(2).map(|(dim, _, score)| (dim.clone(), *score)).unwrap_or(("artistic".to_string(), 0.0));
        let n1 = dimension_names.get(top1.0.as_str()).and_then(|v| v.get("es")).and_then(|v| v.as_str()).unwrap_or(&top1.0);
        let n2 = dimension_names.get(top2.0.as_str()).and_then(|v| v.get("es")).and_then(|v| v.as_str()).unwrap_or(&top2.0);
        let n3 = dimension_names.get(top3.0.as_str()).and_then(|v| v.get("es")).and_then(|v| v.as_str()).unwrap_or(&top3.0);
        format!(
            "Código Holland: {}. Áreas principales: {} ({:.0}%), {} ({:.0}%), {} ({:.0}%).",
            holland_code, n1, top1.1, n2, top2.1, n3, top3.1,
        )
    };

    let scores_hash: HashMap<String, f64> = scores_map.iter()
        .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f)))
        .collect();

    let interp = interpretation::get_interpretation("career_aptitude", &scores_hash, language);
    let recs = interpretation::get_recommendations("career_aptitude", &scores_hash, language);

    let mut result = serde_json::Map::new();
    result.insert("scores".into(), serde_json::json!(scores_map));
    result.insert("holland_code".into(), serde_json::json!(holland_code));
    result.insert("profile_summary".into(), serde_json::json!(summary));
    result.insert("interpretation".into(), interp);
    if let Some(obj) = recs.as_object() {
        for (k, v) in obj {
            result.insert(k.clone(), v.clone());
        }
    }
    Ok(serde_json::Value::Object(result))
}
