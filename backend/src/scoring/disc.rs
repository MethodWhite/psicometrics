use crate::data::load_test_data;
use std::collections::HashMap;

const DIMENSIONS: [&str; 4] = ["D", "I", "S", "C"];

pub fn score(answers: &HashMap<u32, String>, language: &str) -> serde_json::Value {
    let data = load_test_data("disc").expect("disc data");
    let questions = data["questions"].as_array().expect("questions array");

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

    serde_json::json!({
        "primary_style": primary,
        "secondary_style": secondary,
        "scores": scores_map,
        "profile_summary": summary,
    })
}
