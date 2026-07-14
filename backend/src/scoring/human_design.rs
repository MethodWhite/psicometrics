use crate::interpretation;
use std::collections::HashMap;

/// Simplified Human Design calculator.
/// In production, use Swiss Ephemeris for accurate planetary positions.
fn calculate_planets(day_of_year: u32, hour: f64) -> (HashMap<&'static str, f64>, HashMap<&'static str, f64>) {
    let personality_sun = (day_of_year as f64 * 0.9856 + hour * 0.0417) % 360.0;
    let personality_moon = (day_of_year as f64 * 13.176 + hour * 0.549) % 360.0;
    let personality_mercury = (personality_sun + 30.0 + (day_of_year as f64 % 30.0) * 0.5) % 360.0;
    let personality_venus = (personality_sun + 50.0 + (day_of_year as f64 % 60.0) * 0.3) % 360.0;
    let personality_mars = (personality_sun + 120.0 + (day_of_year as f64 % 90.0) * 0.2) % 360.0;
    let personality_jupiter = (personality_sun + 60.0 + (day_of_year as f64 % 400.0) * 0.08) % 360.0;
    let personality_saturn = (personality_sun + 90.0 + (day_of_year as f64 % 360.0) * 0.03) % 360.0;
    let personality_uranus = (personality_sun + 180.0 + (day_of_year as f64 % 365.0) * 0.01) % 360.0;
    let personality_neptune = (personality_sun + 210.0 + (day_of_year as f64 % 600.0) * 0.006) % 360.0;
    let personality_pluto = (personality_sun + 240.0 + (day_of_year as f64 % 900.0) * 0.004) % 360.0;

    let design_offset = 88.0;
    let design_sun = (personality_sun - design_offset * 0.9856) % 360.0;
    let design_moon = (personality_moon - design_offset * 13.176) % 360.0;
    let design_mercury = (design_sun + 30.0) % 360.0;
    let design_venus = (design_sun + 50.0) % 360.0;
    let design_mars = (design_sun + 120.0) % 360.0;
    let design_jupiter = (design_sun + 60.0) % 360.0;
    let design_saturn = (design_sun + 90.0) % 360.0;
    let design_uranus = (design_sun + 180.0) % 360.0;
    let design_neptune = (design_sun + 210.0) % 360.0;
    let design_pluto = (design_sun + 240.0) % 360.0;

    let planets: Vec<(&str, f64)> = vec![
        ("sun", personality_sun), ("moon", personality_moon),
        ("mercury", personality_mercury), ("venus", personality_venus),
        ("mars", personality_mars), ("jupiter", personality_jupiter),
        ("saturn", personality_saturn), ("uranus", personality_uranus),
        ("neptune", personality_neptune), ("pluto", personality_pluto),
    ];

    let design: Vec<(&str, f64)> = vec![
        ("sun", design_sun), ("moon", design_moon),
        ("mercury", design_mercury), ("venus", design_venus),
        ("mars", design_mars), ("jupiter", design_jupiter),
        ("saturn", design_saturn), ("uranus", design_uranus),
        ("neptune", design_neptune), ("pluto", design_pluto),
    ];

    (planets.into_iter().collect(), design.into_iter().collect())
}

fn calculate_gates(planets: &HashMap<&'static str, f64>) -> Vec<u32> {
    planets.values().map(|&pos| {
        let gate = ((pos % 360.0) / 5.625) as u32 + 1;
        gate.min(64)
    }).collect()
}

pub fn calculate(_birth_date: &str, _birth_time: &str, _birth_location: &str, language: &str) -> serde_json::Value {
    let (personality_planets, design_planets) = calculate_planets(365, 14.5);

    let personality_gates = calculate_gates(&personality_planets);
    let design_gates = calculate_gates(&design_planets);

    let all_gates: std::collections::HashSet<u32> = personality_gates.iter().chain(design_gates.iter()).copied().collect();

    // Simplified center determination
    let centers = serde_json::json!({
        "head": all_gates.iter().any(|g| [64, 61, 63].contains(g)),
        "ajna": all_gates.iter().any(|g| [47, 24, 4, 17, 43, 11].contains(g)),
        "throat": all_gates.iter().any(|g| [62, 56, 35, 12, 45, 33, 8, 31, 20, 16].contains(g)),
        "g": all_gates.iter().any(|g| [1, 13, 25, 46, 2, 15, 10, 7].contains(g)),
        "heart": all_gates.iter().any(|g| [21, 51, 26, 40].contains(g)),
        "sacral": all_gates.iter().any(|g| [5, 14, 29, 59, 9, 3, 42, 27, 34].contains(g)),
        "solar_plexus": all_gates.iter().any(|g| [36, 22, 37, 6, 49, 55, 30].contains(g)),
        "splenic": all_gates.iter().any(|g| [48, 57, 44, 50, 32, 28, 18].contains(g)),
        "root": all_gates.iter().any(|g| [53, 60, 52, 19, 39, 41, 58, 38, 54].contains(g)),
    });

    let has_sacral = centers["sacral"].as_bool().unwrap_or(false);
    let has_throat = centers["throat"].as_bool().unwrap_or(false);
    let has_motor = centers["solar_plexus"].as_bool().unwrap_or(false)
        || centers["heart"].as_bool().unwrap_or(false);

    let hd_type = if has_sacral {
        if has_motor && has_throat { "Manifesting Generator" } else { "Generator" }
    } else if has_motor && has_throat {
        "Manifestor"
    } else if !all_gates.is_empty() {
        "Projector"
    } else {
        "Reflector"
    };

    let strategy = match hd_type {
        "Manifestor" => if language == "en" { "Inform" } else { "Informar" },
        "Generator" => if language == "en" { "Wait to Respond" } else { "Esperar a responder" },
        "Manifesting Generator" => if language == "en" { "Wait to Respond, then Inform" } else { "Esperar a responder, luego informar" },
        "Projector" => if language == "en" { "Wait for the Invitation" } else { "Esperar la invitación" },
        "Reflector" => if language == "en" { "Wait a Lunar Cycle" } else { "Esperar un ciclo lunar" },
        _ => "",
    };

    let summary = if language == "en" {
        format!("You are a {}. Your strategy is: {}.", hd_type, strategy)
    } else {
        format!("Eres un/una {}. Tu estrategia es: {}.", hd_type, strategy)
    };

    // Build synthetic scores for interpretation
    let type_value: f64 = match hd_type {
        "Manifestor" | "Manifestador" => 1.0,
        "Generator" | "Generador" => 2.0,
        "Manifesting Generator" | "Generador Manifestador" => 3.0,
        "Projector" | "Proyector" => 4.0,
        _ => 5.0,
    };
    let mut scores_hash = HashMap::new();
    scores_hash.insert("type_value".to_string(), type_value);
    scores_hash.insert("type_key".to_string(), type_value);

    let interp = interpretation::get_interpretation("human_design", &scores_hash, language);
    let recs = interpretation::get_recommendations("human_design", &scores_hash, language);

    let mut result = serde_json::Map::new();
    result.insert("type".into(), serde_json::json!(hd_type));
    result.insert("strategy".into(), serde_json::json!(strategy));
    result.insert("authority".into(), serde_json::json!("Emotional"));
    result.insert("profile".into(), serde_json::json!("1/3"));
    result.insert("centers".into(), centers);
    result.insert("personality_gates".into(), serde_json::json!(personality_gates));
    result.insert("design_gates".into(), serde_json::json!(design_gates));
    result.insert("summary".into(), serde_json::json!(summary));
    result.insert("interpretation".into(), interp);
    if let Some(obj) = recs.as_object() {
        for (k, v) in obj {
            result.insert(k.clone(), v.clone());
        }
    }
    serde_json::Value::Object(result)
}
