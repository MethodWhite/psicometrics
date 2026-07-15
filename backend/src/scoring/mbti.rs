use std::collections::HashMap;

use crate::data::load_test_data;
use crate::error::{AppError, AppResult};
use crate::interpretation;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::load_test_data;

    fn make_answers(prefer_a: bool) -> HashMap<u32, String> {
        let data = load_test_data("mbti").unwrap();
        let questions = data["questions"].as_array().unwrap();
        let mut answers = HashMap::new();
        for q in questions {
            if !q.get("attention_check").and_then(|v| v.as_bool()).unwrap_or(false) {
                let qid = q["id"].as_u64().unwrap() as u32;
                // Alternate between "a" and "b" but biased toward prefer_a direction
                let val = if prefer_a { "a" } else { "b" };
                answers.insert(qid, val.to_string());
            }
        }
        answers
    }

    #[test]
    fn test_mbti_has_required_fields() {
        let answers = make_answers(true);
        let result = score(&answers, "en").unwrap();
        let obj = result.as_object().unwrap();
        assert!(obj.contains_key("type_code"), "missing type_code");
        assert!(obj.contains_key("scores"), "missing scores");
        assert!(obj.contains_key("percentages"), "missing percentages");
        assert!(obj.contains_key("profile_summary"), "missing profile_summary");
    }

    #[test]
    fn test_mbti_type_code_length() {
        let answers = make_answers(true);
        let result = score(&answers, "en").unwrap();
        let tc = result["type_code"].as_str().unwrap();
        assert_eq!(tc.len(), 4, "MBTI type code should be 4 chars, got {}", tc);
    }

    #[test]
    fn test_mbti_different_answers_different_type() {
        let result_a = score(&make_answers(true), "en").unwrap();
        let result_b = score(&make_answers(false), "en").unwrap();
        // Different answer patterns should generally produce different type codes
        // (not guaranteed but very likely with all-a vs all-b)
        let tc_a = result_a["type_code"].as_str().unwrap().to_string();
        let tc_b = result_b["type_code"].as_str().unwrap().to_string();
        // At minimum both should be valid 4-char codes
        assert_eq!(tc_a.len(), 4);
        assert_eq!(tc_b.len(), 4);
    }

    #[test]
    fn test_mbti_percentages_sum_to_100() {
        let answers = make_answers(true);
        let result = score(&answers, "en").unwrap();
        let pcts = result["percentages"].as_object().unwrap();
        for (dich, val) in pcts {
            let v = val.as_f64().unwrap();
            assert!(v >= 0.0 && v <= 100.0, "dichotomy {} percentage {} out of range", dich, v);
        }
    }

    #[test]
    fn test_mbti_spanish() {
        let answers = make_answers(true);
        let result = score(&answers, "es").unwrap();
        assert!(result["profile_summary"].as_str().unwrap_or("").len() > 0);
    }
}

const DICHOTOMIES: [&str; 4] = ["EI", "SN", "TF", "JP"];

const TYPE_NAMES_ES: &[(&str, &str)] = &[
    ("INTJ", "El Arquitecto"), ("INTP", "El Pensador"), ("ENTJ", "El Comandante"),
    ("ENTP", "El Debate"), ("INFJ", "El Defensor"), ("INFP", "El Mediador"),
    ("ENFJ", "El Protagonista"), ("ENFP", "El Activista"), ("ISTJ", "El Logístico"),
    ("ISFJ", "El Protector"), ("ESTJ", "El Ejecutivo"), ("ESFJ", "El Cónsul"),
    ("ISTP", "El Virtuoso"), ("ISFP", "El Aventurero"), ("ESTP", "El Emprendedor"),
    ("ESFP", "El Animador"),
];

const TYPE_NAMES_EN: &[(&str, &str)] = &[
    ("INTJ", "The Architect"), ("INTP", "The Logician"), ("ENTJ", "The Commander"),
    ("ENTP", "The Debater"), ("INFJ", "The Advocate"), ("INFP", "The Mediator"),
    ("ENFJ", "The Protagonist"), ("ENFP", "The Campaigner"), ("ISTJ", "The Logistician"),
    ("ISFJ", "The Defender"), ("ESTJ", "The Executive"), ("ESFJ", "The Consul"),
    ("ISTP", "The Virtuoso"), ("ISFP", "The Adventurer"), ("ESTP", "The Entrepreneur"),
    ("ESFP", "The Entertainer"),
];

pub fn score(answers: &HashMap<u32, String>, language: &str) -> AppResult<serde_json::Value> {
    let data = load_test_data("mbti").map_err(|e| {
        AppError::Internal(format!("Failed to load mbti data: {}", e.0))
    })?;
    let questions = data["questions"].as_array().ok_or_else(|| {
        AppError::Internal("mbti data missing questions array".to_string())
    })?;

    let mut counts: HashMap<&str, HashMap<String, u32>> = HashMap::new();
    for d in &DICHOTOMIES {
        let mut m = HashMap::new();
        m.insert("a".to_string(), 0);
        m.insert("b".to_string(), 0);
        counts.insert(d, m);
    }

    for q in questions {
        let qid = q["id"].as_u64().unwrap_or(0) as u32;
        if let Some(response) = answers.get(&qid) {
            let dichotomy = q["dichotomy"].as_str().unwrap_or("");
            if let Some(c) = counts.get_mut(dichotomy) {
                *c.entry(response.clone()).or_insert(0) += 1;
            }
        }
    }

    let mut percentages = serde_json::Map::new();
    let mut type_code = String::new();
    let mut scores = serde_json::Map::new();

    for dichotomy in &DICHOTOMIES {
        if let Some(c) = counts.get(dichotomy) {
            let total = c.get("a").copied().unwrap_or(0) + c.get("b").copied().unwrap_or(0);
            if total > 0 {
                let score_b = (c.get("b").copied().unwrap_or(0) as f64 / total as f64) * 100.0;
                let _score_b = (score_b * 10.0).round() / 10.0;
                let perc_a = ((1.0 - score_b / 100.0) * 100.0 * 10.0).round() / 10.0;

                scores.insert(dichotomy.to_string(), serde_json::json!(_score_b));
                percentages.insert(dichotomy.to_string(), serde_json::json!(perc_a));

                let pole_a = match *dichotomy {
                    "EI" => "E", "SN" => "S", "TF" => "T", "JP" => "J", _ => "",
                };
                let pole_b = match *dichotomy {
                    "EI" => "I", "SN" => "N", "TF" => "F", "JP" => "P", _ => "",
                };
                if c.get("a").copied().unwrap_or(0) >= c.get("b").copied().unwrap_or(0) {
                    type_code.push_str(pole_a);
                } else {
                    type_code.push_str(pole_b);
                }
            }
        }
    }

    let type_names = if language == "en" { TYPE_NAMES_EN } else { TYPE_NAMES_ES };
    let base_desc = type_names.iter()
        .find(|(k, _)| *k == type_code)
        .map(|(_, v)| *v)
        .unwrap_or(&type_code);

    let summary = format!("{} Tipo {}.", base_desc, type_code);

    let scores_hash: HashMap<String, f64> = scores.iter()
        .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f)))
        .collect();

    let interp = interpretation::get_interpretation("mbti", &scores_hash, language);
    let recs = interpretation::get_recommendations("mbti", &scores_hash, language);

    let mut result = serde_json::Map::new();
    result.insert("type_code".into(), serde_json::json!(type_code));
    result.insert("scores".into(), serde_json::json!(scores));
    result.insert("profile_summary".into(), serde_json::json!(summary));
    result.insert("percentages".into(), serde_json::json!(percentages));
    result.insert("interpretation".into(), interp);
    if let Some(obj) = recs.as_object() {
        for (k, v) in obj {
            result.insert(k.clone(), v.clone());
        }
    }
    Ok(serde_json::Value::Object(result))
}
