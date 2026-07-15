use axum::{
    extract::Path,
    Extension,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::accounts::AccountStore;
use crate::auth::{self, AuthAccountId};
use crate::error::{AppError, AppResult};

// ─── Request / Response types ─────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub account: AccountPublic,
}

#[derive(Serialize)]
pub struct AccountPublic {
    pub id: String,
    pub email: String,
    pub created_at: String,
}

impl From<crate::accounts::Account> for AccountPublic {
    fn from(a: crate::accounts::Account) -> Self {
        AccountPublic {
            id: a.id,
            email: a.email,
            created_at: a.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct AccountResponse {
    pub id: String,
    pub email: String,
    pub created_at: String,
    pub result_count: usize,
}

#[derive(Deserialize)]
pub struct SaveResultRequest {
    pub test_type: String,
    pub result: serde_json::Value,
}

#[derive(Serialize)]
pub struct SaveResultResponse {
    pub result_id: String,
}

#[derive(Deserialize)]
pub struct ResultsQuery {
    pub test_type: Option<String>,
}

#[derive(Serialize)]
pub struct ResultItem {
    pub id: String,
    pub test_type: String,
    pub result: serde_json::Value,
    pub created_at: String,
}

impl From<crate::accounts::StoredResult> for ResultItem {
    fn from(r: crate::accounts::StoredResult) -> Self {
        ResultItem {
            id: r.id,
            test_type: r.test_type,
            result: r.result,
            created_at: r.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct EvolutionItem {
    pub timestamp: String,
    pub scores: serde_json::Value,
}

// ─── Auth endpoints ──────────────────────────────────────────────────────────

pub async fn login(
    Extension(store): Extension<Arc<AccountStore>>,
    Extension(secret_key): Extension<Vec<u8>>,
    Json(body): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    let account = store
        .find_by_email(&body.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid email or password".to_string()))?;

    let valid = auth::verify_password(&body.password, &account.password_hash)?;
    if !valid {
        return Err(AppError::Unauthorized("Invalid email or password".to_string()));
    }

    let token = auth::create_token(&account.id, &secret_key)?;

    Ok(Json(LoginResponse {
        token,
        account: AccountPublic::from(account),
    }))
}

pub async fn register_account(
    Extension(store): Extension<Arc<AccountStore>>,
    Json(body): Json<RegisterRequest>,
) -> AppResult<Json<RegisterResponse>> {
    let password = body.password.unwrap_or_else(|| "changeme123".to_string());
    let password_hash = auth::hash_password(&password)?;
    let account = store.register(&body.email, &password_hash).await?;
    Ok(Json(RegisterResponse {
        id: account.id,
        email: account.email,
    }))
}

// ─── Public account endpoints (by ID) ────────────────────────────────────────

pub async fn get_account(
    Extension(store): Extension<Arc<AccountStore>>,
    Path(id): Path<String>,
) -> AppResult<Json<AccountResponse>> {
    let account = store
        .get_account(&id)
        .await?
        .ok_or_else(|| AppError::NotFound("Account not found".to_string()))?;

    let results = store.get_results(&id).await?;
    Ok(Json(AccountResponse {
        result_count: results.len(),
        id: account.id,
        email: account.email,
        created_at: account.created_at.to_rfc3339(),
    }))
}

pub async fn save_result(
    Extension(store): Extension<Arc<AccountStore>>,
    Path(id): Path<String>,
    Json(body): Json<SaveResultRequest>,
) -> AppResult<Json<SaveResultResponse>> {
    // Verify account exists
    if store.get_account(&id).await?.is_none() {
        return Err(AppError::NotFound("Account not found".to_string()));
    }

    let result_id = store.save_result(&id, &body.test_type, body.result).await?;
    Ok(Json(SaveResultResponse { result_id }))
}

pub async fn get_results(
    Extension(store): Extension<Arc<AccountStore>>,
    Path(id): Path<String>,
    axum::extract::Query(query): axum::extract::Query<ResultsQuery>,
) -> AppResult<Json<Vec<ResultItem>>> {
    if store.get_account(&id).await?.is_none() {
        return Err(AppError::NotFound("Account not found".to_string()));
    }

    let results = match &query.test_type {
        Some(tt) => store.get_results_by_type(&id, tt).await?,
        None => store.get_results(&id).await?,
    };

    let items: Vec<ResultItem> = results.into_iter().map(ResultItem::from).collect();
    Ok(Json(items))
}

pub async fn get_evolution(
    Extension(store): Extension<Arc<AccountStore>>,
    Path((id, test_type)): Path<(String, String)>,
) -> AppResult<Json<Vec<EvolutionItem>>> {
    if store.get_account(&id).await?.is_none() {
        return Err(AppError::NotFound("Account not found".to_string()));
    }

    let mut results = store.get_results_by_type(&id, &test_type).await?;
    results.sort_by_key(|r| r.created_at);

    let items: Vec<EvolutionItem> = results
        .into_iter()
        .map(|r| {
            let scores = r
                .result
                .get("scores")
                .or_else(|| r.result.get("percentages"))
                .or_else(|| r.result.get("percentiles"))
                .cloned()
                .unwrap_or(r.result);
            EvolutionItem {
                timestamp: r.created_at.to_rfc3339(),
                scores,
            }
        })
        .collect();

    Ok(Json(items))
}

// ─── /me endpoints (uses email query param for auth) ────────────────────────

#[derive(Deserialize)]
pub struct EmailQuery {
    pub email: Option<String>,
}

async fn resolve_account_id(
    store: &AccountStore,
    email: Option<&str>,
) -> AppResult<String> {
    let email = email.ok_or_else(|| AppError::BadRequest("email query parameter required".to_string()))?;
    let email = email.trim().to_lowercase();
    let account = store
        .find_by_email(&email)
        .await?
        .ok_or_else(|| AppError::NotFound("Account not found".to_string()))?;
    Ok(account.id)
}

pub async fn get_me(
    Extension(store): Extension<Arc<AccountStore>>,
    Extension(auth_id): Extension<AuthAccountId>,
) -> AppResult<Json<AccountResponse>> {
    let id = &auth_id.0;
    let account = store
        .get_account(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Account not found".to_string()))?;

    let results = store.get_results(id).await?;
    Ok(Json(AccountResponse {
        result_count: results.len(),
        id: account.id,
        email: account.email,
        created_at: account.created_at.to_rfc3339(),
    }))
}

pub async fn delete_me(
    Extension(store): Extension<Arc<AccountStore>>,
    Extension(auth_id): Extension<AuthAccountId>,
) -> AppResult<Json<serde_json::Value>> {
    let deleted = store.delete_account(&auth_id.0).await?;
    if deleted {
        Ok(Json(serde_json::json!({"detail": "Account deleted successfully"})))
    } else {
        Err(AppError::NotFound("Account not found".to_string()))
    }
}

pub async fn export_me(
    Extension(store): Extension<Arc<AccountStore>>,
    Extension(auth_id): Extension<AuthAccountId>,
) -> AppResult<Json<serde_json::Value>> {
    let data = store.get_all_results_for_account(&auth_id.0).await?;
    Ok(Json(data))
}

// ─── Share endpoints ─────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct ShareResponse {
    pub share_url: String,
    pub share_code: String,
}

#[derive(Serialize)]
pub struct PublicProfileResponse {
    pub id: String,
    pub test_type: String,
    pub result: serde_json::Value,
    pub created_at: i64,
}

pub async fn share_result_with_account(
    Extension(store): Extension<Arc<AccountStore>>,
    Path((account_id, result_id)): Path<(String, String)>,
) -> AppResult<Json<ShareResponse>> {
    if store.get_account(&account_id).await?.is_none() {
        return Err(AppError::NotFound("Account not found".to_string()));
    }

    match store.create_share(&result_id, &account_id).await {
        Some(entry) => {
            let share_url = format!("/public/{}", entry.share_code);
            Ok(Json(ShareResponse {
                share_url,
                share_code: entry.share_code,
            }))
        }
        None => Err(AppError::NotFound("Result not found".to_string())),
    }
}

pub async fn get_public_profile(
    Extension(store): Extension<Arc<AccountStore>>,
    Path(share_code): Path<String>,
) -> AppResult<Json<PublicProfileResponse>> {
    match store.get_shared_result(&share_code) {
        Some(entry) => Ok(Json(PublicProfileResponse {
            id: entry.result_id,
            test_type: entry.test_type,
            result: entry.result,
            created_at: entry.created_at,
        })),
        None => Err(AppError::NotFound("Share link not found".to_string())),
    }
}

// ─── GDPR Consent ────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct ConsentResponse {
    pub timestamp: i64,
    pub message: String,
}

pub async fn log_consent(
    Extension(store): Extension<Arc<AccountStore>>,
    axum::extract::Query(query): axum::extract::Query<EmailQuery>,
) -> AppResult<Json<ConsentResponse>> {
    let id = resolve_account_id(&store, query.email.as_deref()).await?;
    let ts = chrono::Utc::now().timestamp();
    store.log_consent(&id);
    Ok(Json(ConsentResponse {
        timestamp: ts,
        message: "Consent logged successfully".to_string(),
    }))
}
