use once_cell::sync::Lazy;
use serde_json::Value;

pub static BIG_FIVE: Lazy<Value> = Lazy::new(|| {
    serde_json::from_str(include_str!("../data/big_five_questions.json"))
        .expect("Failed to parse big_five_questions.json")
});

pub static MBTI: Lazy<Value> = Lazy::new(|| {
    serde_json::from_str(include_str!("../data/mbti_questions.json"))
        .expect("Failed to parse mbti_questions.json")
});

pub static ENNEAGRAM: Lazy<Value> = Lazy::new(|| {
    serde_json::from_str(include_str!("../data/enneagram_questions.json"))
        .expect("Failed to parse enneagram_questions.json")
});

pub static DISC: Lazy<Value> = Lazy::new(|| {
    serde_json::from_str(include_str!("../data/disc_questions.json"))
        .expect("Failed to parse disc_questions.json")
});

pub static DARK_TRIAD: Lazy<Value> = Lazy::new(|| {
    serde_json::from_str(include_str!("../data/dark_triad_questions.json"))
        .expect("Failed to parse dark_triad_questions.json")
});

pub static HUMAN_DESIGN: Lazy<Value> = Lazy::new(|| {
    serde_json::from_str(include_str!("../data/human_design_data.json"))
        .expect("Failed to parse human_design_data.json")
});

pub fn load_test_data(test_type: &str) -> Option<&'static Value> {
    match test_type {
        "big_five" => Some(&BIG_FIVE),
        "mbti" => Some(&MBTI),
        "enneagram" => Some(&ENNEAGRAM),
        "disc" => Some(&DISC),
        "dark_triad" => Some(&DARK_TRIAD),
        "human_design" => Some(&HUMAN_DESIGN),
        _ => None,
    }
}

pub fn get_test_type_list() -> Vec<&'static str> {
    vec!["big_five", "mbti", "enneagram", "disc", "dark_triad", "human_design"]
}
