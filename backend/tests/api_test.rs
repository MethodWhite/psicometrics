use std::sync::Arc;

use psicometrics_backend::accounts::AccountStore;
use psicometrics_backend::create_test_router;

/// Helper: start the server on a random port and return the base URL + shutdown handle.
async fn setup_server() -> (String, tokio::task::JoinHandle<()>) {
    let store = Arc::new(AccountStore::new());
    let app = create_test_router(store);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base_url = format!("http://{}", addr);

    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    (base_url, handle)
}

// ─── Health endpoints ─────────────────────────────────────────────────────

#[tokio::test]
async fn test_health_check() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let resp = client.get(format!("{}/api/v1/health", base))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"], "ok");
    assert_eq!(body["version"], "1.0.0");
}

#[tokio::test]
async fn test_readiness_check() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let resp = client.get(format!("{}/api/v1/health/ready", base))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"], "ready");
}

// ─── Auth endpoints ────────────────────────────────────────────────────────

#[tokio::test]
async fn test_auth_challenge() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let resp = client.get(format!("{}/api/v1/auth/challenge", base))
        .query(&[("session_id", "test-123"), ("agent_type", "test")])
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("challenge_id").is_some());
    assert!(body.get("nonce").is_some());
}

#[tokio::test]
async fn test_auth_verify() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let resp = client.post(format!("{}/api/v1/auth/verify", base))
        .json(&serde_json::json!({
            "challenge_id": "test",
            "response": "invalid"
        }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["verified"], false);
}

#[tokio::test]
async fn test_device_register() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let resp = client.post(format!("{}/api/v1/auth/device/register", base))
        .json(&serde_json::json!({
            "device_id": "dev-001",
            "platform": "test",
            "signed_nonce": "abc123"
        }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["device_id"], "dev-001");
}

// ─── Test list endpoint ────────────────────────────────────────────────────

#[tokio::test]
async fn test_list_tests() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let resp = client.get(format!("{}/api/v1/tests", base))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let tests: Vec<serde_json::Value> = resp.json().await.unwrap();
    assert_eq!(tests.len(), 11);
    let types: Vec<&str> = tests.iter().filter_map(|t| t["test_type"].as_str()).collect();
    assert!(types.contains(&"big_five"));
    assert!(types.contains(&"mbti"));
    assert!(types.contains(&"enneagram"));
    assert!(types.contains(&"disc"));
    assert!(types.contains(&"dark_triad"));
    assert!(types.contains(&"human_design"));
}

// ─── Test questions endpoint ───────────────────────────────────────────────

#[tokio::test]
async fn test_get_questions_big_five() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let resp = client.get(format!("{}/api/v1/tests/big_five", base))
        .query(&[("lang", "en")])
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["test_type"], "big_five");
    let questions = body["questions"].as_array().unwrap();
    assert!(!questions.is_empty(), "should have questions");
    // Verify first question has an id and text
    assert!(questions[0].get("id").is_some());
    assert!(questions[0].get("text").is_some());
}

#[tokio::test]
async fn test_get_questions_invalid_type() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let resp = client.get(format!("{}/api/v1/tests/invalid_type", base))
        .send().await.unwrap();
    assert_eq!(resp.status(), 404);
}

// ─── Test metadata endpoint ────────────────────────────────────────────────

#[tokio::test]
async fn test_get_metadata() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let resp = client.get(format!("{}/api/v1/tests/big_five/metadata", base))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["test_type"], "big_five");
    assert!(body.get("name").is_some());
    assert!(body.get("description").is_some());
    assert!(body.get("instructions").is_some());
    assert!(body.get("consent").is_some());
    assert!(body.get("item_count").is_some());
    assert!(body.get("estimated_minutes").is_some());
}

// ─── Submit test endpoints ─────────────────────────────────────────────────

fn make_answers_big_five(count: u32) -> Vec<serde_json::Value> {
    (1..=count).map(|i| {
        serde_json::json!({"question_id": i, "value": ((i * 7) % 5 + 1)})
    }).collect()
}

fn make_answers_mbti(count: u32) -> Vec<serde_json::Value> {
    (1..=count).map(|i| {
        let val = if i % 2 == 0 { "a" } else { "b" };
        serde_json::json!({"question_id": i, "value": val})
    }).collect()
}

fn make_answers_disc(count: u32) -> Vec<serde_json::Value> {
    (1..=count).map(|i| {
        let dim = match (i - 1) % 4 {
            0 => "a", 1 => "b", 2 => "c", _ => "d",
        };
        serde_json::json!({"question_id": i, "value": dim})
    }).collect()
}

#[tokio::test]
async fn test_submit_big_five() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let answers = make_answers_big_five(120);
    let resp = client.post(format!("{}/api/v1/tests/big_five/submit", base))
        .json(&serde_json::json!({"answers": answers, "language": "en", "completion_time": 300}))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("scores").is_some(), "missing scores: {:?}", body);
    assert!(body.get("facets").is_some());
    assert!(body.get("profile_summary").is_some());
    assert!(body.get("validity").is_some());
}

#[tokio::test]
async fn test_submit_mbti() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let answers = make_answers_mbti(72);
    let resp = client.post(format!("{}/api/v1/tests/mbti/submit", base))
        .json(&serde_json::json!({"answers": answers, "language": "en", "completion_time": 180}))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("type_code").is_some(), "missing type_code");
    assert!(body.get("scores").is_some());
}

#[tokio::test]
async fn test_submit_enneagram() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let answers: Vec<serde_json::Value> = (1..=81).map(|i| {
        let val = if i <= 9 { 4.0 } else { 1.0 };
        serde_json::json!({"question_id": i, "value": val})
    }).collect();
    let resp = client.post(format!("{}/api/v1/tests/enneagram/submit", base))
        .json(&serde_json::json!({"answers": answers, "language": "en", "completion_time": 200}))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("dominant_type").is_some());
    assert!(body.get("wing").is_some());
}

#[tokio::test]
async fn test_submit_disc() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let answers = make_answers_disc(28);
    let resp = client.post(format!("{}/api/v1/tests/disc/submit", base))
        .json(&serde_json::json!({"answers": answers, "language": "en", "completion_time": 120}))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("primary_style").is_some());
    assert!(body.get("secondary_style").is_some());
}

#[tokio::test]
async fn test_submit_dark_triad() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let answers: Vec<serde_json::Value> = (1..=27).map(|i| {
        serde_json::json!({"question_id": i, "value": ((i * 3) % 5 + 1)})
    }).collect();
    let resp = client.post(format!("{}/api/v1/tests/dark_triad/submit", base))
        .json(&serde_json::json!({"answers": answers, "language": "en", "completion_time": 150}))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("dark_core").is_some());
    assert!(body.get("risk_level").is_some());
}

#[tokio::test]
async fn test_submit_human_design() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let resp = client.post(format!("{}/api/v1/tests/human_design/submit", base))
        .json(&serde_json::json!({
            "birth_date": "1990-01-01",
            "birth_time": "12:00",
            "birth_location": "New York",
            "language": "en"
        }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("type").is_some());
    assert!(body.get("strategy").is_some());
}

#[tokio::test]
async fn test_submit_invalid_type() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let resp = client.post(format!("{}/api/v1/tests/nonexistent/submit", base))
        .json(&serde_json::json!({"answers": []}))
        .send().await.unwrap();
    assert_eq!(resp.status(), 404);
}

// ─── Report endpoint ───────────────────────────────────────────────────────

#[tokio::test]
async fn test_generate_report_pdf() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let answers = make_answers_big_five(120);
    let resp = client.post(format!("{}/api/v1/tests/big_five/report", base))
        .json(&serde_json::json!({"answers": answers, "language": "en"}))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    // Check content type
    let content_type = resp.headers().get("content-type").unwrap().to_str().unwrap().to_string();
    assert!(content_type.contains("pdf"), "expected PDF content type, got: {}", content_type);
    // Check PDF magic bytes
    let bytes = resp.bytes().await.unwrap();
    assert!(bytes.len() > 1000, "PDF should be > 1000 bytes, got {}", bytes.len());
    assert_eq!(&bytes[..5], b"%PDF-", "should start with PDF magic bytes");
}

#[tokio::test]
async fn test_generate_report_invalid_type() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let resp = client.post(format!("{}/api/v1/tests/invalid/report", base))
        .json(&serde_json::json!({"answers": []}))
        .send().await.unwrap();
    assert_eq!(resp.status(), 404);
}

// ─── Compare endpoint ──────────────────────────────────────────────────────

#[tokio::test]
async fn test_compare_big_five() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let resp = client.post(format!("{}/api/v1/tests/big_five/compare", base))
        .json(&serde_json::json!({
            "results": [
                {"scores": {"O": 70.0, "C": 60.0, "E": 50.0, "A": 80.0, "N": 30.0}},
                {"scores": {"O": 65.0, "C": 55.0, "E": 45.0, "A": 75.0, "N": 35.0}}
            ],
            "language": "en"
        }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.get("compatibility_score").is_some());
    assert!(body.get("factors").is_some());
}

#[tokio::test]
async fn test_compare_wrong_number_of_results() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let resp = client.post(format!("{}/api/v1/tests/big_five/compare", base))
        .json(&serde_json::json!({
            "results": [{"scores": {"O": 50.0}}],
            "language": "en"
        }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 400);
}

// ─── Account endpoints ─────────────────────────────────────────────────────

#[tokio::test]
async fn test_account_register_and_get() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();

    // Register
    let resp = client.post(format!("{}/api/v1/accounts/register", base))
        .json(&serde_json::json!({"email": "test@example.com", "password": "secret123"}))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let reg: serde_json::Value = resp.json().await.unwrap();
    let account_id = reg["id"].as_str().unwrap().to_string();
    assert_eq!(reg["email"], "test@example.com");

    // Get account
    let resp = client.get(format!("{}/api/v1/accounts/{}", base, account_id))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let acct: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(acct["email"], "test@example.com");
}

#[tokio::test]
async fn test_account_not_found() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();
    let resp = client.get(format!("{}/api/v1/accounts/nonexistent", base))
        .send().await.unwrap();
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn test_save_and_get_results() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();

    // Register account
    let resp = client.post(format!("{}/api/v1/accounts/register", base))
        .json(&serde_json::json!({"email": "results@test.com", "password": "pass123"}))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let reg: serde_json::Value = resp.json().await.unwrap();
    let id = reg["id"].as_str().unwrap().to_string();

    // Save a result
    let resp = client.post(format!("{}/api/v1/accounts/{}/results", base, id))
        .json(&serde_json::json!({
            "test_type": "big_five",
            "result": {"scores": {"O": 50.0, "C": 50.0, "E": 50.0, "A": 50.0, "N": 50.0}}
        }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);

    // Get results
    let resp = client.get(format!("{}/api/v1/accounts/{}/results", base, id))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let results: Vec<serde_json::Value> = resp.json().await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["test_type"], "big_five");
}

#[tokio::test]
async fn test_evolution_endpoint() {
    let (base, _handle) = setup_server().await;
    let client = reqwest::Client::new();

    // Register account
    let resp = client.post(format!("{}/api/v1/accounts/register", base))
        .json(&serde_json::json!({"email": "evo@test.com", "password": "pass123"}))
        .send().await.unwrap();
    let reg: serde_json::Value = resp.json().await.unwrap();
    let id = reg["id"].as_str().unwrap().to_string();

    // Save two results
    for _ in 0..2 {
        client.post(format!("{}/api/v1/accounts/{}/results", base, id))
            .json(&serde_json::json!({
                "test_type": "big_five",
                "result": {"scores": {"O": 50.0, "C": 50.0, "E": 50.0, "A": 50.0, "N": 50.0}}
            }))
            .send().await.unwrap();
    }

    // Get evolution
    let resp = client.get(format!("{}/api/v1/accounts/{}/evolution/big_five", base, id))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let evo: Vec<serde_json::Value> = resp.json().await.unwrap();
    assert_eq!(evo.len(), 2);
}
