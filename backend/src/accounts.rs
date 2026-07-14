use std::collections::HashMap;
use std::sync::Mutex;

pub struct AccountStore {
    accounts: Mutex<HashMap<String, Account>>,
    results: Mutex<HashMap<String, Vec<StoredResult>>>,
}

#[derive(Clone)]
pub struct Account {
    pub id: String,
    pub email: String,
    pub created_at: i64,
}

#[derive(Clone)]
pub struct StoredResult {
    pub id: String,
    pub test_type: String,
    pub result: serde_json::Value,
    pub created_at: i64,
}

impl AccountStore {
    pub fn new() -> Self {
        AccountStore {
            accounts: Mutex::new(HashMap::new()),
            results: Mutex::new(HashMap::new()),
        }
    }

    pub fn register(&self, email: &str) -> Result<Account, String> {
        let email = email.trim().to_lowercase();
        if email.is_empty() {
            return Err("Email cannot be empty".to_string());
        }

        let mut accounts = self.accounts.lock().map_err(|e| e.to_string())?;

        // Check if email already registered
        for account in accounts.values() {
            if account.email == email {
                return Ok(account.clone());
            }
        }

        let account = Account {
            id: uuid::Uuid::new_v4().to_string(),
            email,
            created_at: chrono::Utc::now().timestamp(),
        };

        accounts.insert(account.id.clone(), account.clone());
        Ok(account)
    }

    pub fn get_account(&self, id: &str) -> Option<Account> {
        let accounts = self.accounts.lock().ok()?;
        accounts.get(id).cloned()
    }

    pub fn save_result(
        &self,
        account_id: &str,
        test_type: &str,
        result: serde_json::Value,
    ) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let stored = StoredResult {
            id: id.clone(),
            test_type: test_type.to_string(),
            result,
            created_at: chrono::Utc::now().timestamp(),
        };

        if let Ok(mut results) = self.results.lock() {
            results
                .entry(account_id.to_string())
                .or_default()
                .push(stored);
        }

        id
    }

    pub fn get_results(&self, account_id: &str) -> Vec<StoredResult> {
        let results = match self.results.lock() {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };
        results
            .get(account_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_results_by_type(
        &self,
        account_id: &str,
        test_type: &str,
    ) -> Vec<StoredResult> {
        let results = match self.results.lock() {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };
        results
            .get(account_id)
            .map(|v| {
                v.iter()
                    .filter(|r| r.test_type == test_type)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
}
