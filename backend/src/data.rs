use once_cell::sync::Lazy;
use serde_json::Value;

macro_rules! load_json {
    ($path:literal) => {
        serde_json::from_str(include_str!($path)).unwrap_or_else(|e| {
            panic!("Failed to parse embedded data file `{}`: {e}", $path)
        })
    };
}

pub static ATTACHMENT_STYLE: Lazy<Value> = Lazy::new(|| load_json!("../data/attachment_style_questions.json"));
pub static BIG_FIVE: Lazy<Value> = Lazy::new(|| load_json!("../data/big_five_questions.json"));
pub static CAREER_APTITUDE: Lazy<Value> = Lazy::new(|| load_json!("../data/career_aptitude_questions.json"));
pub static DARK_TRIAD: Lazy<Value> = Lazy::new(|| load_json!("../data/dark_triad_questions.json"));
pub static DISC: Lazy<Value> = Lazy::new(|| load_json!("../data/disc_questions.json"));
pub static EMOTIONAL_INTELLIGENCE: Lazy<Value> = Lazy::new(|| load_json!("../data/eq_questions.json"));
pub static ENNEAGRAM: Lazy<Value> = Lazy::new(|| load_json!("../data/enneagram_questions.json"));
pub static HUMAN_DESIGN: Lazy<Value> = Lazy::new(|| load_json!("../data/human_design_data.json"));
pub static LOVE_LANGUAGES: Lazy<Value> = Lazy::new(|| load_json!("../data/love_languages_questions.json"));
pub static MBTI: Lazy<Value> = Lazy::new(|| load_json!("../data/mbti_questions.json"));
pub static VIA_STRENGTHS: Lazy<Value> = Lazy::new(|| load_json!("../data/via_strengths_questions.json"));

#[derive(Debug)]
pub struct TestDataNotFound(pub String);

pub fn load_test_data(test_type: &str) -> Result<&'static Value, TestDataNotFound> {
    match test_type {
        "attachment_style" => Ok(&ATTACHMENT_STYLE),
        "big_five" => Ok(&BIG_FIVE),
        "career_aptitude" => Ok(&CAREER_APTITUDE),
        "dark_triad" => Ok(&DARK_TRIAD),
        "disc" => Ok(&DISC),
        "emotional_intelligence" => Ok(&EMOTIONAL_INTELLIGENCE),
        "enneagram" => Ok(&ENNEAGRAM),
        "human_design" => Ok(&HUMAN_DESIGN),
        "love_languages" => Ok(&LOVE_LANGUAGES),
        "mbti" => Ok(&MBTI),
        "via_strengths" => Ok(&VIA_STRENGTHS),
        _ => Err(TestDataNotFound(test_type.to_string())),
    }
}

pub fn get_test_type_list() -> Vec<&'static str> {
    vec![
        "attachment_style", "big_five", "career_aptitude", "dark_triad", "disc",
        "emotional_intelligence", "enneagram", "human_design", "love_languages",
        "mbti", "via_strengths",
    ]
}
