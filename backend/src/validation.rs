use crate::data::load_test_data;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub warnings: Vec<String>,
    pub attention_check_failed: bool,
    pub response_pattern: String,
    pub completion_time_seconds: Option<u32>,
}

/// Validate test responses for careless/random answering patterns.
///
/// Checks performed:
/// 1. Straight line — all or >80% answers identical
/// 2. Alternating — A-B-A-B or 1-2-1-2 pattern
/// 3. Attention checks — embedded catch questions
/// 4. Missing answers — too many skipped
/// 5. Completion time — unrealistically fast submission
pub fn validate_responses(
    test_type: &str,
    answers: &HashMap<u32, serde_json::Value>,
    completion_time_seconds: Option<u32>,
) -> ValidationResult {
    let mut warnings: Vec<String> = Vec::new();
    let mut attention_check_failed = false;
    let mut response_pattern = "normal".to_string();

    // Human Design has no questions — skip all checks
    if test_type == "human_design" {
        return ValidationResult {
            is_valid: true,
            warnings,
            attention_check_failed,
            response_pattern,
            completion_time_seconds,
        };
    }

    // 1. Straight-line detection
    if is_straight_line(answers) {
        warnings.push(
            "All or most answers are identical, which may indicate non-serious responding."
                .to_string(),
        );
        response_pattern = "straight_line".to_string();
    }

    // 2. Alternating pattern detection
    if is_alternating(answers) {
        warnings.push(
            "Answer pattern alternates suspiciously (e.g., A-B-A-B or 1-2-1-2).".to_string(),
        );
        if response_pattern == "normal" {
            response_pattern = "alternating".to_string();
        }
    }

    // 3. Attention checks
    let (att_failed, att_warnings) = check_attention_checks(test_type, answers);
    if att_failed {
        attention_check_failed = true;
    }
    warnings.extend(att_warnings);

    // 4. Missing answers
    let (missing, missing_warnings) = check_missing_answers(test_type, answers);
    if missing {
        warnings.extend(missing_warnings);
    }

    // 5. Completion time
    let (time_warning, time_warnings) =
        check_completion_time(test_type, completion_time_seconds);
    if time_warning {
        warnings.extend(time_warnings);
    }

    let is_valid = !attention_check_failed && response_pattern == "normal";

    ValidationResult {
        is_valid,
        warnings,
        attention_check_failed,
        response_pattern,
        completion_time_seconds,
    }
}

/// Returns the count of non-attention-check questions for a test type.
fn get_real_question_count(test_type: &str) -> usize {
    let data = match load_test_data(test_type) {
        Some(d) => d,
        None => return 0,
    };
    let questions = match data["questions"].as_array() {
        Some(q) => q,
        None => return 0,
    };
    questions
        .iter()
        .filter(|q| !q.get("attention_check").and_then(|v| v.as_bool()).unwrap_or(false))
        .count()
}

// ─── Straight line ───────────────────────────────────────────────────────────

fn is_straight_line(answers: &HashMap<u32, serde_json::Value>) -> bool {
    if answers.len() < 5 {
        return false;
    }

    let numeric_values: Vec<f64> = answers
        .values()
        .filter_map(|v| v.as_f64())
        .collect();
    let string_values: Vec<&str> = answers.values().filter_map(|v| v.as_str()).collect();

    let threshold = (answers.len() as f64 * 0.8).ceil() as usize;

    // Numeric: check if >80% have (nearly) the same value
    if !numeric_values.is_empty() {
        let counts = count_numeric_frequencies(&numeric_values);
        if counts.values().any(|&c| c >= threshold) {
            return true;
        }
    }

    // String: check if >80% are the same value
    if !string_values.is_empty() {
        let counts = count_string_frequencies(&string_values);
        if counts.values().any(|&c| c >= threshold) {
            return true;
        }
    }

    false
}

fn count_numeric_frequencies(values: &[f64]) -> HashMap<u32, usize> {
    let mut counts: HashMap<u32, usize> = HashMap::new();
    for &v in values {
        let bucket = (v * 10.0).round() as u32;
        *counts.entry(bucket).or_insert(0) += 1;
    }
    counts
}

fn count_string_frequencies<'a>(values: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for &s in values {
        *counts.entry(s).or_insert(0) += 1;
    }
    counts
}

// ─── Alternating pattern ─────────────────────────────────────────────────────

fn is_alternating(answers: &HashMap<u32, serde_json::Value>) -> bool {
    if answers.len() < 6 {
        return false;
    }

    // Sort by question_id to reconstruct presentation order
    let mut sorted: Vec<(u32, &serde_json::Value)> =
        answers.iter().map(|(k, v)| (*k, v)).collect();
    sorted.sort_by_key(|(k, _)| *k);

    let values: Vec<&serde_json::Value> = sorted.iter().map(|(_, v)| *v).collect();

    // Try period 2 alternation: A-B-A-B...
    if let (Some(first), Some(second)) = (values.first(), values.get(1)) {
        if first != second && values.iter().enumerate().all(|(i, v)| {
            if i % 2 == 0 {
                *v == *first
            } else {
                *v == *second
            }
        }) {
            return true;
        }
    }

    // Try period 3 alternation: A-B-C-A-B-C...
    if values.len() >= 6 {
        if let (Some(a), Some(b), Some(c)) = (values.first(), values.get(1), values.get(2)) {
            if a != b && b != c && a != c
                && values.iter().enumerate().all(|(i, v)| {
                    match i % 3 {
                        0 => *v == *a,
                        1 => *v == *b,
                        _ => *v == *c,
                    }
                })
            {
                return true;
            }
        }
    }

    false
}

// ─── Attention checks ────────────────────────────────────────────────────────

fn check_attention_checks(
    test_type: &str,
    answers: &HashMap<u32, serde_json::Value>,
) -> (bool, Vec<String>) {
    let mut failed = false;
    let mut warnings: Vec<String> = Vec::new();

    let data = match load_test_data(test_type) {
        Some(d) => d,
        None => return (false, vec![]),
    };
    let questions = match data["questions"].as_array() {
        Some(q) => q,
        None => return (false, vec![]),
    };

    for q in questions {
        let is_attention = q.get("attention_check").and_then(|v| v.as_bool()).unwrap_or(false);
        if !is_attention {
            continue;
        }

        let qid = q["id"].as_u64().unwrap_or(0) as u32;
        let expected = q.get("expected_value");

        match expected {
            // Leave-blank check (expected_value is null)
            Some(val) if val.is_null() => {
                if answers.contains_key(&qid) {
                    failed = true;
                    warnings.push(format!(
                        "Attention check #{} should have been left blank but was answered.",
                        qid
                    ));
                }
            }
            // Specific-value check
            Some(val) => match answers.get(&qid) {
                Some(answer) if answer != val => {
                    failed = true;
                    warnings.push(format!(
                        "Attention check #{} failed: expected {:?}, got {:?}.",
                        qid, val, answer
                    ));
                }
                None => {
                    failed = true;
                    warnings.push(format!("Attention check #{} was not answered.", qid));
                }
                _ => { /* correct */ }
            },
            None => { /* no expected_value set — skip */ }
        }
    }

    (failed, warnings)
}

// ─── Missing answers ─────────────────────────────────────────────────────────

fn check_missing_answers(
    test_type: &str,
    answers: &HashMap<u32, serde_json::Value>,
) -> (bool, Vec<String>) {
    let total = get_real_question_count(test_type);
    if total == 0 {
        return (false, vec![]);
    }

    let answered = answers.len();
    let ratio = answered as f64 / total as f64;

    if ratio < 0.5 {
        return (
            true,
            vec![format!(
                "Only {}/{} questions answered ({:.0}%). Many responses are missing.",
                answered,
                total,
                ratio * 100.0
            )],
        );
    }
    if ratio < 0.8 {
        return (
            true,
            vec![format!(
                "Only {}/{} questions answered ({:.0}%). Consider completing all questions.",
                answered,
                total,
                ratio * 100.0
            )],
        );
    }

    (false, vec![])
}

// ─── Completion time ─────────────────────────────────────────────────────────

fn check_completion_time(
    test_type: &str,
    completion_time_seconds: Option<u32>,
) -> (bool, Vec<String>) {
    let time = match completion_time_seconds {
        Some(t) => t,
        None => return (false, vec![]),
    };

    let total = get_real_question_count(test_type);
    if total == 0 {
        return (false, vec![]);
    }

    if time < 120 && total > 50 {
        return (
            true,
            vec![format!(
                "Completed in {} seconds for {} questions. This is unusually fast.",
                time, total
            )],
        );
    }

    (false, vec![])
}
