use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

// ─── Data types ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2BTeam {
    pub id: String,
    pub company_name: String,
    pub email: String,
    pub api_key: String,
    pub team_size: u32,
    pub created_at: String,
    pub members: Vec<B2BMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct B2BMember {
    pub email: String,
    pub name: String,
    pub role: String, // "admin" | "member"
    pub invited_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamReport {
    pub team_id: String,
    pub company_name: String,
    pub total_tests: u64,
    pub member_count: usize,
    pub reports: Vec<serde_json::Value>,
}

// ─── In-memory store ─────────────────────────────────────────────────────────

pub struct B2BStore {
    teams: Mutex<HashMap<String, B2BTeam>>,
    api_keys: Mutex<HashMap<String, String>>, // api_key -> team_id
}

impl B2BStore {
    pub fn new() -> Self {
        Self {
            teams: Mutex::new(HashMap::new()),
            api_keys: Mutex::new(HashMap::new()),
        }
    }

    pub fn register(
        &self,
        company_name: &str,
        email: &str,
        team_size: u32,
    ) -> AppResult<B2BTeam> {
        let id = uuid::Uuid::new_v4().to_string();
        let api_key = format!("pm_b2b_{}", uuid::Uuid::new_v4().to_string().replace('-', ""));

        let team = B2BTeam {
            id: id.clone(),
            company_name: company_name.to_string(),
            email: email.to_string(),
            api_key: api_key.clone(),
            team_size,
            created_at: chrono::Utc::now().to_rfc3339(),
            members: vec![B2BMember {
                email: email.to_string(),
                name: company_name.to_string(),
                role: "admin".to_string(),
                invited_at: chrono::Utc::now().to_rfc3339(),
            }],
        };

        let mut teams = self
            .teams
            .lock()
            .map_err(|e| AppError::Internal(format!("Lock poisoned: {e}")))?;
        let mut keys = self
            .api_keys
            .lock()
            .map_err(|e| AppError::Internal(format!("Lock poisoned: {e}")))?;

        teams.insert(id.clone(), team.clone());
        keys.insert(api_key, id);

        Ok(team)
    }

    pub fn list_teams(&self) -> AppResult<Vec<B2BTeam>> {
        let teams = self
            .teams
            .lock()
            .map_err(|e| AppError::Internal(format!("Lock poisoned: {e}")))?;
        Ok(teams.values().cloned().collect())
    }

    pub fn get_team_reports(&self) -> AppResult<Vec<TeamReport>> {
        let teams = self
            .teams
            .lock()
            .map_err(|e| AppError::Internal(format!("Lock poisoned: {e}")))?;
        Ok(teams
            .values()
            .map(|t| TeamReport {
                team_id: t.id.clone(),
                company_name: t.company_name.clone(),
                total_tests: 0,      // TODO: count from actual test results
                member_count: t.members.len(),
                reports: Vec::new(), // TODO: aggregate from results store
            })
            .collect())
    }

    pub fn invite_member(
        &self,
        team_id: &str,
        email: &str,
        name: &str,
        role: &str,
    ) -> AppResult<B2BMember> {
        let mut teams = self
            .teams
            .lock()
            .map_err(|e| AppError::Internal(format!("Lock poisoned: {e}")))?;

        let team = teams
            .get_mut(team_id)
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        let member = B2BMember {
            email: email.to_string(),
            name: name.to_string(),
            role: role.to_string(),
            invited_at: chrono::Utc::now().to_rfc3339(),
        };

        team.members.push(member.clone());
        Ok(member)
    }

    pub fn verify_api_key(&self, api_key: &str) -> Option<String> {
        let keys = self.api_keys.lock().ok()?;
        keys.get(api_key).cloned()
    }
}

// ─── Route handlers ──────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub company_name: String,
    pub email: String,
    pub team_size: u32,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub api_key: String,
    pub dashboard_url: String,
    pub team_id: String,
}

/// POST /api/v1/b2b/register
pub async fn register(
    Extension(store): Extension<Arc<B2BStore>>,
    Json(body): Json<RegisterRequest>,
) -> AppResult<Json<RegisterResponse>> {
    let team = store.register(&body.company_name, &body.email, body.team_size)?;
    Ok(Json(RegisterResponse {
        api_key: team.api_key,
        dashboard_url: format!("https://psicometrics.app/b2b/{}", team.id),
        team_id: team.id,
    }))
}

/// GET /api/v1/b2b/teams
pub async fn list_teams(
    Extension(store): Extension<Arc<B2BStore>>,
) -> AppResult<Json<Vec<B2BTeam>>> {
    store.list_teams().map(Json)
}

/// GET /api/v1/b2b/reports
pub async fn team_reports(
    Extension(store): Extension<Arc<B2BStore>>,
) -> AppResult<Json<Vec<TeamReport>>> {
    store.get_team_reports().map(Json)
}

#[derive(Deserialize)]
pub struct InviteRequest {
    pub team_id: String,
    pub email: String,
    pub name: String,
    pub role: Option<String>,
}

#[derive(Serialize)]
pub struct InviteResponse {
    pub email: String,
    pub status: String,
}

/// POST /api/v1/b2b/invite
pub async fn invite_member(
    Extension(store): Extension<Arc<B2BStore>>,
    Json(body): Json<InviteRequest>,
) -> AppResult<Json<InviteResponse>> {
    let role = body.role.as_deref().unwrap_or("member");
    store.invite_member(&body.team_id, &body.email, &body.name, role)?;
    Ok(Json(InviteResponse {
        email: body.email,
        status: "invited".to_string(),
    }))
}
