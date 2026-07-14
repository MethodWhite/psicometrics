use crate::data::load_test_data;
use std::collections::HashMap;

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

pub fn score(answers: &HashMap<u32, String>, language: &str) -> serde_json::Value {
    let data = load_test_data("mbti").expect("mbti data");
    let questions = data["questions"].as_array().expect("questions array");

    // Count per dichotomy
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

                // Determine type letter
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

    serde_json::json!({
        "type_code": type_code,
        "scores": scores,
        "profile_summary": summary,
        "percentages": percentages,
    })
}
