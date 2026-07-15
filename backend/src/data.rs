use once_cell::sync::Lazy;
use serde_json::Value;

macro_rules! load_json {
    ($path:literal) => {
        serde_json::from_str(include_str!($path)).unwrap_or_else(|e| {
            panic!("Failed to parse embedded data file `{}`: {e}", $path)
        })
    };
}

pub static BIG_FIVE: Lazy<Value> = Lazy::new(|| load_json!("../data/big_five_questions.json"));
pub static MBTI: Lazy<Value> = Lazy::new(|| load_json!("../data/mbti_questions.json"));
pub static ENNEAGRAM: Lazy<Value> = Lazy::new(|| load_json!("../data/enneagram_questions.json"));
pub static DISC: Lazy<Value> = Lazy::new(|| load_json!("../data/disc_questions.json"));
pub static DARK_TRIAD: Lazy<Value> = Lazy::new(|| load_json!("../data/dark_triad_questions.json"));
pub static HUMAN_DESIGN: Lazy<Value> = Lazy::new(|| load_json!("../data/human_design_data.json"));

#[derive(Debug)]
pub struct TestDataNotFound(pub String);

pub fn load_test_data(test_type: &str) -> Result<&'static Value, TestDataNotFound> {
    match test_type {
        "big_five" => Ok(&BIG_FIVE),
        "mbti" => Ok(&MBTI),
        "enneagram" => Ok(&ENNEAGRAM),
        "disc" => Ok(&DISC),
        "dark_triad" => Ok(&DARK_TRIAD),
        "human_design" => Ok(&HUMAN_DESIGN),
        _ => Err(TestDataNotFound(test_type.to_string())),
    }
}

pub fn get_test_type_list() -> Vec<&'static str> {
    vec!["big_five", "mbti", "enneagram", "disc", "dark_triad", "human_design"]
}
