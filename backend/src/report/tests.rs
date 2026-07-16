use super::*;

#[test]
fn test_generate_big_five_pdf() {
    let result = serde_json::json!({
        "scores": {"O": 72.0, "C": 45.0, "E": 58.0, "A": 80.0, "N": 30.0},
        "profile_summary": "Openness: high creative, imaginative, enjoys new ideas. Conscientiousness: balanced. Extraversion: balanced. Agreeableness: high cooperative, cares about others. Neuroticism: low emotionally stable."
    });
    let pdf = generate_report("big_five", &result, "en");
    assert!(pdf.len() > 1000);
    assert_eq!(&pdf[..5], b"%PDF-");
}

#[test]
fn test_generate_mbti_pdf() {
    let result = serde_json::json!({
        "type_code": "INFJ",
        "scores": {"EI": 75.0, "SN": 20.0, "TF": 35.0, "JP": 80.0},
        "percentages": {"EI": 75.0, "SN": 20.0, "TF": 35.0, "JP": 80.0},
        "profile_summary": "The Advocate. Type INFJ."
    });
    let pdf = generate_report("mbti", &result, "en");
    assert!(pdf.len() > 1000);
    assert_eq!(&pdf[..5], b"%PDF-");
}

#[test]
fn test_generate_spanish_pdf() {
    let result = serde_json::json!({
        "scores": {"O": 65.0, "C": 50.0, "E": 72.0, "A": 40.0, "N": 55.0},
        "profile_summary": "Apertura: alta creativo. Responsabilidad: equilibrada. Extraversión: alta. Amabilidad: equilibrada. Neuroticismo: equilibrado."
    });
    let pdf = generate_report("big_five", &result, "es");
    assert!(pdf.len() > 1000);
    assert_eq!(&pdf[..5], b"%PDF-");
}

#[test]
fn test_generate_enneagram_pdf() {
    let result = serde_json::json!({
        "dominant_type": 4,
        "wing": 5,
        "scores": {
            "1": 30.0, "2": 45.0, "3": 40.0, "4": 85.0, "5": 70.0,
            "6": 35.0, "7": 50.0, "8": 25.0, "9": 55.0
        },
        "profile_summary": "Type 4w5. Highest scores: Type 4 (85%), Type 5 (70%)."
    });
    let pdf = generate_report("enneagram", &result, "en");
    assert!(pdf.len() > 1000);
    assert_eq!(&pdf[..5], b"%PDF-");
}
