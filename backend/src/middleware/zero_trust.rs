use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use ztf::challenge::{ChallengeResponse, HmacVerifier};

#[derive(Clone)]
pub struct ZeroTrustState {
    pub provider: Arc<ChallengeResponse>,
    pub secret_key: Vec<u8>,
}

pub async fn zero_trust_middleware(
    state: Arc<ZeroTrustState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let path = request.uri().path().to_string();

    if method == axum::http::Method::POST && path.starts_with("/api/v1/tests/") {
        let challenge_id = request
            .headers()
            .get("x-challenge-id")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let response = request
            .headers()
            .get("x-challenge-response")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if !challenge_id.is_empty() && !response.is_empty() {
            let verifier = HmacVerifier::new(&state.secret_key);
            let _ = state.provider.verify_and_consume(challenge_id, response, &verifier);
        }
    }

    next.run(request).await
}
