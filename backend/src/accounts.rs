use std::collections::HashMap;
use std::sync::Mutex;

use chrono::{DateTime, Utc};
use rand::Rng;
use serde_json::Value;

use crate::error::{AppError, AppResult};

/// In-memory account representation (matches database::Account shape)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

/// In-memory result representation
#[derive(Debug, Clone, serde::Serialize)]
pub struct StoredResult {
    pub id: String,
    pub account_id: String,
    pub test_type: String,
    pub result: Value,
    pub created_at: DateTime<Utc>,
}

/// Share entry (in-memory)
#[derive(Clone)]
pub struct ShareEntry {
    pub share_code: String,
    pub result_id: String,
    pub account_id: String,
    pub test_type: String,
    pub result: Value,
    pub created_at: i64,
}

/// Consent entry (in-memory)
#[derive(Clone)]
pub struct ConsentEntry {
    pub timestamp: i64,
}

fn generate_share_code() -> String {
    let mut rng = rand::thread_rng();
    (0..12)
        .map(|_| {
            let idx = rng.gen_range(0..36);
            if idx < 10 {
                (b'0' + idx) as char
            } else {
                (b'a' + idx - 10) as char
            }
        })
        .collect()
}

pub struct AccountStore {
    accounts: Mutex<HashMap<String, Account>>,
    results: Mutex<HashMap<String, Vec<StoredResult>>>,
    shares: Mutex<HashMap<String, ShareEntry>>,
    consents: Mutex<HashMap<String, Vec<ConsentEntry>>>,
}

impl AccountStore {
    pub fn new() -> Self {
        Self {
            accounts: Mutex::new(HashMap::new()),
            results: Mutex::new(HashMap::new()),
            shares: Mutex::new(HashMap::new()),
            consents: Mutex::new(HashMap::new()),
        }
    }

    // ─── Account CRUD ────────────────────────────────────────────────────

    pub async fn register(
        &self,
        email: &str,
        password_hash: &str,
    ) -> AppResult<Account> {
        let email = email.trim().to_lowercase();
        if email.is_empty() {
            return Err(AppError::BadRequest("Email cannot be empty".to_string()));
        }

        let mut accounts = self.accounts.lock().map_err(|e| {
            AppError::Internal(format!("Lock poisoned: {e}"))
        })?;

        // Check duplicate
        if accounts.values().any(|a| a.email == email) {
            return Err(AppError::Conflict(
                "An account with this email already exists".to_string(),
            ));
        }

        let account = Account {
            id: uuid::Uuid::new_v4().to_string(),
            email,
            password_hash: password_hash.to_string(),
            created_at: Utc::now(),
        };

        accounts.insert(account.id.clone(), account.clone());
        Ok(account)
    }

    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<Account>> {
        let accounts = self.accounts.lock().map_err(|e| {
            AppError::Internal(format!("Lock poisoned: {e}"))
        })?;
        Ok(accounts.values().find(|a| a.email == email).cloned())
    }

    pub async fn get_account(&self, id: &str) -> AppResult<Option<Account>> {
        let accounts = self.accounts.lock().map_err(|e| {
            AppError::Internal(format!("Lock poisoned: {e}"))
        })?;
        Ok(accounts.get(id).cloned())
    }

    pub async fn delete_account(&self, id: &str) -> AppResult<bool> {
        let existed = {
            let mut accounts = self.accounts.lock().map_err(|e| {
                AppError::Internal(format!("Lock poisoned: {e}"))
            })?;
            accounts.remove(id).is_some()
        };
        // Also clean up results
        if let Ok(mut results) = self.results.lock() {
            results.remove(id);
        }
        Ok(existed)
    }

    pub async fn save_result(
        &self,
        account_id: &str,
        test_type: &str,
        result: Value,
    ) -> AppResult<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let stored = StoredResult {
            id: id.clone(),
            account_id: account_id.to_string(),
            test_type: test_type.to_string(),
            result,
            created_at: Utc::now(),
        };

        let mut results = self.results.lock().map_err(|e| {
            AppError::Internal(format!("Lock poisoned: {e}"))
        })?;
        results.entry(account_id.to_string()).or_default().push(stored);
        Ok(id)
    }

    pub async fn get_results(&self, account_id: &str) -> AppResult<Vec<StoredResult>> {
        let results = self.results.lock().map_err(|e| {
            AppError::Internal(format!("Lock poisoned: {e}"))
        })?;
        Ok(results.get(account_id).cloned().unwrap_or_default())
    }

    pub async fn get_results_by_type(
        &self,
        account_id: &str,
        test_type: &str,
    ) -> AppResult<Vec<StoredResult>> {
        let results = self.results.lock().map_err(|e| {
            AppError::Internal(format!("Lock poisoned: {e}"))
        })?;
        Ok(results
            .get(account_id)
            .map(|v| {
                v.iter()
                    .filter(|r| r.test_type == test_type)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default())
    }

    pub async fn get_all_results_for_account(
        &self,
        account_id: &str,
    ) -> AppResult<Value> {
        let account = self.get_account(account_id).await?;
        let results = self.get_results(account_id).await?;

        let account_info = match account {
            Some(a) => serde_json::json!({
                "id": a.id,
                "email": a.email,
                "created_at": a.created_at.to_rfc3339(),
            }),
            None => serde_json::json!(null),
        };

        Ok(serde_json::json!({
            "account": account_info,
            "results": results.iter().map(|r| serde_json::json!({
                "id": r.id,
                "test_type": r.test_type,
                "result": r.result,
                "created_at": r.created_at.to_rfc3339(),
            })).collect::<Vec<_>>(),
            "export_date": Utc::now().to_rfc3339(),
        }))
    }

    // ─── Sharing (in-memory) ─────────────────────────────────────────────

    pub async fn create_share(
        &self,
        result_id: &str,
        account_id: &str,
    ) -> Option<ShareEntry> {
        let result = {
            let results = self.results.lock().ok()?;
            results
                .get(account_id)?
                .iter()
                .find(|r| r.id == result_id)
                .cloned()?
        };

        let mut shares = self.shares.lock().ok()?;
        let code = generate_share_code();
        let entry = ShareEntry {
            share_code: code.clone(),
            result_id: result.id.clone(),
            account_id: account_id.to_string(),
            test_type: result.test_type.clone(),
            result: result.result.clone(),
            created_at: chrono::Utc::now().timestamp(),
        };
        shares.insert(code.clone(), entry.clone());
        Some(entry)
    }

    pub fn get_shared_result(&self, share_code: &str) -> Option<ShareEntry> {
        let shares = self.shares.lock().ok()?;
        shares.get(share_code).cloned()
    }

    // ─── GDPR Consent (in-memory) ────────────────────────────────────────

    pub fn log_consent(&self, account_id: &str) {
        if let Ok(mut consents) = self.consents.lock() {
            consents
                .entry(account_id.to_string())
                .or_default()
                .push(ConsentEntry {
                    timestamp: chrono::Utc::now().timestamp(),
                });
        }
    }

    pub fn get_consents(&self, account_id: &str) -> Vec<ConsentEntry> {
        let consents = match self.consents.lock() {
            Ok(c) => c,
            Err(_) => return Vec::new(),
        };
        consents.get(account_id).cloned().unwrap_or_default()
    }
}
