use crate::data::load_test_data;
use std::collections::HashMap;

fn normalize(value: f64, min: f64, max: f64) -> f64 {
    ((value - min) / (max - min)) * 100.0
}

pub fn score(answers: &HashMap<u32, f64>, language: &str) -> serde_json::Value {
    let data = load_test_data("enneagram").expect("enneagram data");
    let questions = data["questions"].as_array().expect("questions array");

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

    // Find dominant type
    let dominant = (1..=9)
        .max_by(|&a, &b| {
            let sa = type_scores[&a.to_string()].as_f64().unwrap_or(0.0);
            let sb = type_scores[&b.to_string()].as_f64().unwrap_or(0.0);
            sa.partial_cmp(&sb).unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or(1);

    // Find wing (adjacent type with highest score)
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

    serde_json::json!({
        "dominant_type": dominant,
        "wing": wing,
        "scores": type_scores,
        "profile_summary": summary,
    })
}
