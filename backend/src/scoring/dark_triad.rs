use crate::data::load_test_data;
use std::collections::HashMap;

fn normalize(value: f64, min: f64, max: f64) -> f64 {
    ((value - min) / (max - min)) * 100.0
}

pub fn score(answers: &HashMap<u32, f64>, language: &str) -> serde_json::Value {
    let data = load_test_data("dark_triad").expect("dark_triad data");
    let questions = data["questions"].as_array().expect("questions array");

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

    serde_json::json!({
        "scores": scores_map,
        "dark_core": dark_core,
        "risk_level": risk_level,
        "profile_summary": summary,
    })
}
