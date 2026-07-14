use axum::{extract::Query, Extension, Json};
use std::sync::Arc;
use ztf::challenge::{ChallengeResponse, HmacVerifier};

use crate::models::{ChallengeResponse as Resp, DeviceRegisterRequest, DeviceRegisterResult, VerifyRequest, VerifyResult};

pub async fn get_challenge(
    Extension(zt_provider): Extension<Arc<ChallengeResponse>>,
    Extension(secret_key): Extension<Vec<u8>>,
    Query(params): Query<ChallengeQuery>,
) -> Json<Resp> {
    let agent_type = params.agent_type.as_deref().unwrap_or("unknown");
    let session_id = params.session_id.as_deref().unwrap_or("anon");

    let challenge = zt_provider
        .generate_challenge(session_id, agent_type)
        .unwrap_or_else(|_| panic!("Failed to generate challenge"));

    Json(Resp {
        challenge_id: challenge.id,
        nonce: challenge.nonce,
        expires_at: challenge.expires_at as f64,
    })
}

pub async fn verify_challenge(
    Extension(zt_provider): Extension<Arc<ChallengeResponse>>,
    Extension(secret_key): Extension<Vec<u8>>,
    Json(body): Json<VerifyRequest>,
) -> Json<VerifyResult> {
    let verifier = HmacVerifier::new(&secret_key);
    match zt_provider.verify_and_consume(&body.challenge_id, &body.response, &verifier) {
        Ok(true) => Json(VerifyResult {
            verified: true,
            session_token: Some(uuid::Uuid::new_v4().to_string()),
        }),
        _ => Json(VerifyResult {
            verified: false,
            session_token: None,
        }),
    }
}

pub async fn register_device(
    Json(body): Json<DeviceRegisterRequest>,
) -> Json<DeviceRegisterResult> {
    Json(DeviceRegisterResult {
        device_id: body.device_id,
        trusted: false,
        challenge: Some(uuid::Uuid::new_v4().to_string()),
    })
}

pub async fn attest_device(
    Json(body): Json<DeviceRegisterRequest>,
) -> Json<DeviceRegisterResult> {
    Json(DeviceRegisterResult {
        device_id: body.device_id,
        trusted: true,
        challenge: None,
    })
}

#[derive(serde::Deserialize)]
pub struct ChallengeQuery {
    pub session_id: Option<String>,
    pub agent_type: Option<String>,
}
