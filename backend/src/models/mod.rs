use serde::{Deserialize, Serialize};

// ========== Answer types ==========

#[derive(Debug, Deserialize)]
pub struct AnswerValue {
    pub question_id: u32,
    pub value: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct BigFiveSubmission {
    pub answers: Vec<AnswerValue>,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MBTISubmission {
    pub answers: Vec<AnswerValue>,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EnneagramSubmission {
    pub answers: Vec<AnswerValue>,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DISCSubmission {
    pub answers: Vec<AnswerValue>,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DarkTriadSubmission {
    pub answers: Vec<AnswerValue>,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct HumanDesignSubmission {
    pub birth_date: String,
    pub birth_time: String,
    pub birth_location: String,
    pub language: Option<String>,
}

// ========== Result types ==========

#[derive(Debug, Serialize)]
pub struct BigFiveResult {
    pub scores: serde_json::Value,
    pub facets: serde_json::Value,
    pub profile_summary: String,
    pub percentiles: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct MBTIResult {
    pub type_code: String,
    pub scores: serde_json::Value,
    pub profile_summary: String,
    pub percentages: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct EnneagramResult {
    pub dominant_type: u32,
    pub wing: u32,
    pub scores: serde_json::Value,
    pub profile_summary: String,
}

#[derive(Debug, Serialize)]
pub struct DISCResult {
    pub primary_style: String,
    pub secondary_style: String,
    pub scores: serde_json::Value,
    pub profile_summary: String,
}

#[derive(Debug, Serialize)]
pub struct DarkTriadResult {
    pub scores: serde_json::Value,
    pub dark_core: f64,
    pub risk_level: String,
    pub profile_summary: String,
}

#[derive(Debug, Serialize)]
pub struct HumanDesignResult {
    #[serde(rename = "type")]
    pub type_name: String,
    pub strategy: String,
    pub authority: String,
    pub profile: String,
    pub centers: serde_json::Value,
    pub summary: String,
}

// ========== Auth types ==========

#[derive(Debug, Serialize)]
pub struct ChallengeResponse {
    pub challenge_id: String,
    pub nonce: String,
    pub expires_at: f64,
}

#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    pub challenge_id: String,
    pub response: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyResult {
    pub verified: bool,
    pub session_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeviceRegisterRequest {
    pub device_id: String,
    pub platform: String,
    pub signed_nonce: String,
    pub app_version: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DeviceRegisterResult {
    pub device_id: String,
    pub trusted: bool,
    pub challenge: Option<String>,
}

// ========== Common ==========

#[derive(Debug, Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct TestInfo {
    pub test_type: String,
    pub name: serde_json::Value,
    pub description: serde_json::Value,
    pub item_count: usize,
    pub test_mode: String,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub detail: String,
    pub code: String,
}
