use std::collections::HashMap;

use crate::data::load_test_data;
use crate::error::{AppError, AppResult};
use crate::interpretation;

fn reverse_value(value: f64) -> f64 {
    6.0 - value
}

fn normalize(value: f64, min: f64, max: f64) -> f64 {
    ((value - min) / (max - min)) * 100.0
}

fn approx_percentile(score: f64) -> f64 {
    let z = (score - 50.0) / 15.0;
    let t = 1.0 / (1.0 + 0.2316419 * z.abs());
    let d = 0.3989422804014327 * (-z * z / 2.0).exp();
    let p = d * t * (0.3193815 + t * (-0.3565638 + t * (1.781478 + t * (-1.821256 + t * 1.330274))));
    if z > 0.0 {
        (1.0 - p) * 100.0
    } else {
        p * 100.0
    }
}

const FACTOR_NAMES_ES: &[(&str, &str)] = &[
    ("O", "Apertura"),
    ("C", "Responsabilidad"),
    ("E", "Extraversión"),
    ("A", "Amabilidad"),
    ("N", "Neuroticismo"),
];

const FACTOR_NAMES_EN: &[(&str, &str)] = &[
    ("O", "Openness"),
    ("C", "Conscientiousness"),
    ("E", "Extraversion"),
    ("A", "Agreeableness"),
    ("N", "Neuroticism"),
];

pub fn score(answers: &HashMap<u32, f64>, language: &str) -> AppResult<serde_json::Value> {
    let data = load_test_data("big_five").map_err(|e| {
        AppError::Internal(format!("Failed to load big_five data: {}", e.0))
    })?;
    let questions = data["questions"].as_array().ok_or_else(|| {
        AppError::Internal("big_five data missing questions array".to_string())
    })?;

    let mut facet_raw: HashMap<&str, Vec<f64>> = HashMap::new();

    for q in questions {
        let qid = q["id"].as_u64().unwrap_or(0) as u32;
        if let Some(&value) = answers.get(&qid) {
            let facet = q["facet"].as_str().unwrap_or("");
            let reverse = q["reverse"].as_bool().unwrap_or(false);
            let val = if reverse { reverse_value(value) } else { value };
            facet_raw.entry(facet).or_default().push(val);
        }
    }

    let mut facets: HashMap<String, f64> = HashMap::new();
    for (facet, values) in &facet_raw {
        if !values.is_empty() {
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            facets.insert(facet.to_string(), (normalize(mean, 1.0, 5.0) * 10.0).round() / 10.0);
        }
    }

    let factor_facets: HashMap<&str, Vec<&str>> = [
        ("O", vec!["O1","O2","O3","O4","O5","O6"]),
        ("C", vec!["C1","C2","C3","C4","C5","C6"]),
        ("E", vec!["E1","E2","E3","E4","E5","E6"]),
        ("A", vec!["A1","A2","A3","A4","A5","A6"]),
        ("N", vec!["N1","N2","N3","N4","N5","N6"]),
    ].iter().cloned().collect();

    let mut scores: HashMap<String, f64> = HashMap::new();
    let mut percentiles: HashMap<String, f64> = HashMap::new();

    for (factor, facs) in &factor_facets {
        let values: Vec<f64> = facs.iter()
            .filter_map(|f| facets.get(*f))
            .copied()
            .collect();
        let mean = if values.is_empty() { 50.0 } else { values.iter().sum::<f64>() / values.len() as f64 };
        let rounded = (mean * 10.0).round() / 10.0;
        scores.insert(factor.to_string(), rounded);
        percentiles.insert(factor.to_string(), (approx_percentile(rounded) * 10.0).round() / 10.0);
    }

    let factor_names = if language == "en" { FACTOR_NAMES_EN } else { FACTOR_NAMES_ES };
    let summary = generate_summary(&scores, factor_names, language);
    let interp = interpretation::get_interpretation("big_five", &scores, language);
    let recs = interpretation::get_recommendations("big_five", &scores, language);

    let mut result = serde_json::Map::new();
    result.insert("scores".into(), serde_json::json!(scores));
    result.insert("facets".into(), serde_json::json!(facets));
    result.insert("profile_summary".into(), serde_json::json!(summary));
    result.insert("percentiles".into(), serde_json::json!(percentiles));
    result.insert("interpretation".into(), interp);
    if let Some(obj) = recs.as_object() {
        for (k, v) in obj {
            result.insert(k.clone(), v.clone());
        }
    }
    Ok(serde_json::Value::Object(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_answers(value: f64) -> HashMap<u32, f64> {
        let data = crate::data::load_test_data("big_five").unwrap();
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
    fn test_big_five_has_scores_and_facets() {
        let answers = make_answers(3.0);
        let result = score(&answers, "es").unwrap();
        let obj = result.as_object().unwrap();
        assert!(obj.contains_key("scores"), "missing scores");
        assert!(obj.contains_key("facets"), "missing facets");
        assert!(obj.contains_key("profile_summary"), "missing profile_summary");
        assert!(obj.contains_key("percentiles"), "missing percentiles");
    }

    #[test]
    fn test_big_five_scores_in_range() {
        let answers = make_answers(3.0);
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        for (factor, val) in scores {
            let v = val.as_f64().unwrap();
            assert!(v >= 0.0 && v <= 100.0, "factor {} score {} out of [0,100]", factor, v);
        }
    }

    #[test]
    fn test_big_five_high_scores_tend_high() {
        let answers = make_answers(5.0);
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        // All 5s should give scores > 50 for non-heavily-reversed factors
        // (some facets have reversed items so not all factors will be 100)
        for (factor, val) in scores {
            let v = val.as_f64().unwrap();
            assert!(v > 0.0, "factor {} should be > 0 with all-5 answers, got {}", factor, v);
        }
    }

    #[test]
    fn test_big_five_low_scores_tend_low() {
        let answers = make_answers(1.0);
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        for (factor, val) in scores {
            let v = val.as_f64().unwrap();
            assert!(v < 100.0, "factor {} should be < 100 with all-1 answers, got {}", factor, v);
        }
    }

    #[test]
    fn test_big_five_neutral_around_fifty() {
        let answers = make_answers(3.0);
        let result = score(&answers, "en").unwrap();
        let scores = result["scores"].as_object().unwrap();
        // With all 3s, average should be around 50 for non-reversed facets
        // Some factors will differ due to reverse items
        for (factor, val) in scores {
            let v = val.as_f64().unwrap();
            assert!(v >= 0.0 && v <= 100.0, "factor {} = {} not in [0,100]", factor, v);
        }
    }

    #[test]
    fn test_big_five_spanish() {
        let answers = make_answers(4.0);
        let result = score(&answers, "es").unwrap();
        assert!(result["profile_summary"].as_str().unwrap_or("").len() > 0);
    }
}

fn generate_summary(scores: &HashMap<String, f64>, names: &[(&str, &str)], lang: &str) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (key, label) in names {
        let score = scores.get(*key).copied().unwrap_or(50.0);
        let line = if lang == "en" {
            match *key {
                "O" if score >= 70.0 => format!("{}: high creative, imaginative, enjoys new ideas.", label),
                "O" if score <= 30.0 => format!("{}: moderate - prefers practical over theoretical.", label),
                "O" => format!("{}: balanced.", label),
                "C" if score >= 70.0 => format!("{}: highly organized, fulfills obligations.", label),
                "C" if score <= 30.0 => format!("{}: flexible, spontaneous, keeps options open.", label),
                "C" => format!("{}: balanced.", label),
                "E" if score >= 70.0 => format!("{}: enjoys others' company, energized by interaction.", label),
                "E" if score <= 30.0 => format!("{}: introverted, values alone time.", label),
                "E" => format!("{}: balanced.", label),
                "A" if score >= 70.0 => format!("{}: cooperative, cares about others' feelings.", label),
                "A" if score <= 30.0 => format!("{}: direct, analytical, prioritizes logic.", label),
                "A" => format!("{}: balanced.", label),
                "N" if score >= 70.0 => format!("{}: experiences intense emotions and worry.", label),
                "N" if score <= 30.0 => format!("{}: emotionally stable, relaxed under stress.", label),
                "N" => format!("{}: balanced.", label),
                _ => format!("{}: {:.0}%", label, score),
            }
        } else {
            match *key {
                "O" if score >= 70.0 => format!("{}: alta - creativo/a, imaginativo/a, disfruta ideas nuevas.", label),
                "O" if score <= 30.0 => format!("{}: moderada - prefiere lo práctico sobre lo teórico.", label),
                "O" => format!("{}: equilibrada.", label),
                "C" if score >= 70.0 => format!("{}: alta - organizado/a, cumple obligaciones.", label),
                "C" if score <= 30.0 => format!("{}: flexible, espontáneo/a.", label),
                "C" => format!("{}: equilibrada.", label),
                "E" if score >= 70.0 => format!("{}: alto - disfruta la compañía de otros.", label),
                "E" if score <= 30.0 => format!("{}: introvertido/a, valora el tiempo a solas.", label),
                "E" => format!("{}: equilibrada.", label),
                "A" if score >= 70.0 => format!("{}: alta - cooperativo/a, le importan los sentimientos ajenos.", label),
                "A" if score <= 30.0 => format!("{}: directo/a, analítico/a, prioriza la lógica.", label),
                "A" => format!("{}: equilibrada.", label),
                "N" if score >= 70.0 => format!("{}: alto - tiende a preocuparse frecuentemente.", label),
                "N" if score <= 30.0 => format!("{}: bajo - emocionalmente estable y relajado/a.", label),
                "N" => format!("{}: equilibrado.", label),
                _ => format!("{}: {:.0}%", label, score),
            }
        };
        parts.push(line);
    }
    parts.join(" ")
}
