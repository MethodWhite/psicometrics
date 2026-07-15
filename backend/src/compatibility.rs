#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_big_five_results() {
        let r1 = serde_json::json!({"scores": {"O": 70.0, "C": 60.0, "E": 50.0, "A": 80.0, "N": 30.0}});
        let r2 = serde_json::json!({"scores": {"O": 70.0, "C": 60.0, "E": 50.0, "A": 80.0, "N": 30.0}});
        let result = compare_results(&[r1, r2], "big_five", "en");
        let score = result["compatibility_score"].as_f64().unwrap();
        assert!(score >= 90.0, "identical results should have high compatibility, got {:.1}", score);
    }

    #[test]
    fn test_opposite_big_five_results() {
        let r1 = serde_json::json!({"scores": {"O": 90.0, "C": 90.0, "E": 90.0, "A": 90.0, "N": 10.0}});
        let r2 = serde_json::json!({"scores": {"O": 10.0, "C": 10.0, "E": 10.0, "A": 10.0, "N": 90.0}});
        let result = compare_results(&[r1, r2], "big_five", "en");
        let score = result["compatibility_score"].as_f64().unwrap();
        assert!(score < 60.0, "opposite results should have low compatibility, got {:.1}", score);
    }

    #[test]
    fn test_identical_mbti_results() {
        let r1 = serde_json::json!({"type_code": "INTJ", "scores": {"EI": 75.0, "SN": 20.0, "TF": 70.0, "JP": 20.0}});
        let r2 = serde_json::json!({"type_code": "INTJ", "scores": {"EI": 75.0, "SN": 20.0, "TF": 70.0, "JP": 20.0}});
        let result = compare_results(&[r1, r2], "mbti", "en");
        let score = result["compatibility_score"].as_f64().unwrap();
        assert!(score >= 85.0, "identical MBTI should have high compatibility, got {:.1}", score);
        assert_eq!(result["match_category"].as_str().unwrap(), "identical");
    }

    #[test]
    fn test_opposite_mbti_results() {
        let r1 = serde_json::json!({"type_code": "INTJ", "scores": {"EI": 75.0, "SN": 75.0, "TF": 75.0, "JP": 75.0}});
        let r2 = serde_json::json!({"type_code": "ESFP", "scores": {"EI": 25.0, "SN": 25.0, "TF": 25.0, "JP": 25.0}});
        let result = compare_results(&[r1, r2], "mbti", "en");
        let score = result["compatibility_score"].as_f64().unwrap();
        assert!(score <= 60.0, "opposite MBTI should have lower compatibility, got {:.1}", score);
    }

    #[test]
    fn test_unsupported_test_type() {
        let r1 = serde_json::json!({"scores": {"1": 50.0}});
        let r2 = serde_json::json!({"scores": {"1": 50.0}});
        let result = compare_results(&[r1, r2], "enneagram", "en");
        assert!(result.get("error").and_then(|v| v.as_bool()).unwrap_or(false),
                "enneagram comparison should return error");
    }

    #[test]
    fn test_big_five_has_factor_details() {
        let r1 = serde_json::json!({"scores": {"O": 50.0, "C": 50.0, "E": 50.0, "A": 50.0, "N": 50.0}});
        let r2 = serde_json::json!({"scores": {"O": 50.0, "C": 50.0, "E": 50.0, "A": 50.0, "N": 50.0}});
        let result = compare_results(&[r1, r2], "big_five", "en");
        let factors = result["factors"].as_object().unwrap();
        for &f in &["O", "C", "E", "A", "N"] {
            assert!(factors.contains_key(f), "missing factor {}", f);
            assert!(factors[f].get("score1").is_some(), "factor {} missing score1", f);
            assert!(factors[f].get("score2").is_some(), "factor {} missing score2", f);
            assert!(factors[f].get("difference").is_some(), "factor {} missing difference", f);
        }
    }

    #[test]
    fn test_compatibility_spanish() {
        let r1 = serde_json::json!({"scores": {"O": 70.0, "C": 60.0, "E": 50.0, "A": 80.0, "N": 30.0}});
        let r2 = serde_json::json!({"scores": {"O": 65.0, "C": 55.0, "E": 45.0, "A": 75.0, "N": 35.0}});
        let result = compare_results(&[r1, r2], "big_five", "es");
        assert!(result["description"].as_str().unwrap_or("").len() > 0);
    }
}

const FACTOR_NAMES_ES: [(&str, &str); 5] = [
    ("O", "Apertura"),
    ("C", "Responsabilidad"),
    ("E", "Extraversión"),
    ("A", "Amabilidad"),
    ("N", "Neuroticismo"),
];

const FACTOR_NAMES_EN: [(&str, &str); 5] = [
    ("O", "Openness"),
    ("C", "Conscientiousness"),
    ("E", "Extraversion"),
    ("A", "Agreeableness"),
    ("N", "Neuroticism"),
];

fn factor_names(lang: &str) -> &[(&str, &str); 5] {
    if lang == "en" {
        &FACTOR_NAMES_EN
    } else {
        &FACTOR_NAMES_ES
    }
}

fn clamped_diff(a: f64, b: f64) -> f64 {
    ((a - b).abs() * 10.0).round() / 10.0
}

fn clamp_score(v: f64) -> f64 {
    v.max(0.0).min(100.0)
}

pub fn compare_results(
    results: &[serde_json::Value],
    test_type: &str,
    lang: &str,
) -> serde_json::Value {
    match test_type {
        "big_five" => compare_big_five(results, lang),
        "mbti" => compare_mbti(results, lang),
        _ => serde_json::json!({
            "error": true,
            "detail": format!("Comparison not supported for test type '{}'", test_type),
        }),
    }
}

fn get_score(result: &serde_json::Value, factor: &str) -> f64 {
    result
        .get("scores")
        .and_then(|s| s.get(factor))
        .and_then(|v| v.as_f64())
        .unwrap_or(50.0)
}

fn compare_big_five(results: &[serde_json::Value], lang: &str) -> serde_json::Value {
    let names = factor_names(lang);
    let factors = ["O", "C", "E", "A", "N"];

    let r1 = results.first();
    let r2 = results.get(1);

    let mut total_diff = 0.0;
    let mut factor_details = serde_json::Map::new();

    for (i, f) in factors.iter().enumerate() {
        let s1 = r1.map(|r| get_score(r, f)).unwrap_or(50.0);
        let s2 = r2.map(|r| get_score(r, f)).unwrap_or(50.0);
        let diff = clamped_diff(s1, s2);
        total_diff += diff;

        let (_, label) = names[i];
        let both_high = s1 >= 60.0 && s2 >= 60.0;
        let both_low = s1 <= 40.0 && s2 <= 40.0;
        let desc = if both_high {
            if lang == "en" {
                format!("Both high in {}: you share this trait strongly.", label)
            } else {
                format!("Ambos altos en {}: comparten este rasgo fuertemente.", label)
            }
        } else if both_low {
            if lang == "en" {
                format!("Both low in {}: you share this tendency.", label)
            } else {
                format!("Ambos bajos en {}: comparten esta tendencia.", label)
            }
        } else if diff <= 15.0 {
            if lang == "en" {
                format!("Similar in {} (diff: {:.0}%).", label, diff)
            } else {
                format!("Similares en {} (dif: {:.0}%).", label, diff)
            }
        } else {
            if lang == "en" {
                format!(
                    "Complementary in {}: you balance each other (diff: {:.0}%).",
                    label, diff
                )
            } else {
                format!(
                    "Complementarios en {}: se equilibran mutuamente (dif: {:.0}%).",
                    label, diff
                )
            }
        };

        factor_details.insert(
            f.to_string(),
            serde_json::json!({
                "score1": (s1 * 10.0).round() / 10.0,
                "score2": (s2 * 10.0).round() / 10.0,
                "difference": diff,
                "description": desc,
            }),
        );
    }

    let avg_diff = total_diff / factors.len() as f64;
    let compatibility = clamp_score(100.0 - avg_diff);

    let summary = if lang == "en" {
        format!(
            "Compatibility: {:.0}% — {}",
            compatibility,
            if compatibility >= 80.0 {
                "Very compatible! You share similar traits."
            } else if compatibility >= 60.0 {
                "Good compatibility with some complementary differences."
            } else {
                "Opposite traits create a challenging dynamic."
            }
        )
    } else {
        format!(
            "Compatibilidad: {:.0}% — {}",
            compatibility,
            if compatibility >= 80.0 {
                "¡Muy compatibles! Comparten rasgos similares."
            } else if compatibility >= 60.0 {
                "Buena compatibilidad con diferencias complementarias."
            } else {
                "Rasgos opuestos crean una dinámica desafiante."
            }
        )
    };

    serde_json::json!({
        "test_type": "big_five",
        "compatibility_score": compatibility,
        "description": summary,
        "factors": factor_details,
    })
}

fn compare_mbti(results: &[serde_json::Value], lang: &str) -> serde_json::Value {
    let r1 = results.first();
    let r2 = results.get(1);

    let code1 = r1
        .and_then(|r| r.get("type_code"))
        .and_then(|c| c.as_str())
        .unwrap_or("");
    let code2 = r2
        .and_then(|r| r.get("type_code"))
        .and_then(|c| c.as_str())
        .unwrap_or("");

    let dichotomies = ["EI", "SN", "TF", "JP"];
    let mut matching = 0u32;
    let mut factor_details = serde_json::Map::new();

    let dichotomy_labels_en = ["E/I", "S/N", "T/F", "J/P"];
    let dichotomy_labels_es = ["E/I", "S/N", "T/F", "J/P"];
    let labels = if lang == "en" {
        &dichotomy_labels_en[..]
    } else {
        &dichotomy_labels_es[..]
    };

    for (i, d) in dichotomies.iter().enumerate() {
        let s1 = r1.map(|r| get_score(r, d)).unwrap_or(50.0);
        let s2 = r2.map(|r| get_score(r, d)).unwrap_or(50.0);

        let pole1 = pole_for(d, s1);
        let pole2 = pole_for(d, s2);
        let is_match = pole1 == pole2;
        if is_match {
            matching += 1;
        }

        let desc = if is_match {
            if lang == "en" {
                format!(
                    "{}: Both lean {} — aligned preference.",
                    labels[i], pole1
                )
            } else {
                format!(
                    "{}: Ambos inclinan hacia {} — preferencia alineada.",
                    labels[i], pole1
                )
            }
        } else {
            if lang == "en" {
                format!(
                    "{}: {} vs {} — complementary perspective.",
                    labels[i], pole1, pole2
                )
            } else {
                format!(
                    "{}: {} vs {} — perspectiva complementaria.",
                    labels[i], pole1, pole2
                )
            }
        };

        factor_details.insert(
            d.to_string(),
            serde_json::json!({
                "score1": (s1 * 10.0).round() / 10.0,
                "score2": (s2 * 10.0).round() / 10.0,
                "pole1": pole1,
                "pole2": pole2,
                "match": is_match,
                "description": desc,
            }),
        );
    }

    let (category, compatibility, summary) = match matching {
        4 => (
            "identical",
            90.0,
            if lang == "en" {
                "Identical types! You share the same MBTI type — strong mutual understanding."
            } else {
                "¡Tipos idénticos! Comparten el mismo tipo MBTI — fuerte entendimiento mutuo."
            },
        ),
        3 => (
            "similar",
            75.0,
            if lang == "en" {
                "Very similar types with only minor differences in one dichotomy."
            } else {
                "Tipos muy similares con solo diferencias menores en una dicotomía."
            },
        ),
        2 => (
            "complementary",
            60.0,
            if lang == "en" {
                "Complementary types — you balance each other across the dichotomies."
            } else {
                "Tipos complementarios — se equilibran mutuamente en las dicotomías."
            },
        ),
        _ => (
            "challenging",
            40.0,
            if lang == "en" {
                "Opposite types — your differences may create a challenging dynamic."
            } else {
                "Tipos opuestos — sus diferencias pueden crear una dinámica desafiante."
            },
        ),
    };

    serde_json::json!({
        "test_type": "mbti",
        "type_code1": code1,
        "type_code2": code2,
        "compatibility_score": compatibility,
        "match_category": category,
        "description": summary,
        "factors": factor_details,
    })
}

fn pole_for(dichotomy: &str, score: f64) -> &'static str {
    match dichotomy {
        "EI" => {
            if score < 50.0 {
                "E"
            } else {
                "I"
            }
        }
        "SN" => {
            if score < 50.0 {
                "S"
            } else {
                "N"
            }
        }
        "TF" => {
            if score < 50.0 {
                "T"
            } else {
                "F"
            }
        }
        "JP" => {
            if score < 50.0 {
                "J"
            } else {
                "P"
            }
        }
        _ => "",
    }
}
