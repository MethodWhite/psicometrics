use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};

use crate::error::AppResult;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct StoredResult {
    pub id: String,
    pub account_id: String,
    pub test_type: String,
    pub result: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn init(url: &str) -> AppResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(url)
            .await
            .map_err(|e| {
                tracing::error!("Failed to connect to database: {e}");
                crate::error::AppError::Internal(format!("Database connection failed: {e}"))
            })?;
        let db = Self { pool };
        db.create_tables().await?;
        Ok(db)
    }

    async fn create_tables(&self) -> AppResult<()> {
        sqlx::query(include_str!("../migrations/001_init.sql"))
            .execute(&self.pool)
            .await?;
        tracing::info!("Database tables initialized");
        Ok(())
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    // ─── Account CRUD ────────────────────────────────────────────────────────

    pub async fn register(
        &self,
        email: &str,
        password_hash: &str,
    ) -> AppResult<Account> {
        let id = uuid::Uuid::new_v4();
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO accounts (id, email, password_hash, created_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(id)
        .bind(email)
        .bind(password_hash)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e {
                if db_err.constraint() == Some("accounts_email_key") {
                    return crate::error::AppError::Conflict(
                        "An account with this email already exists".to_string(),
                    );
                }
            }
            crate::error::AppError::Database(e)
        })?;

        Ok(Account {
            id: id.to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            created_at: now,
        })
    }

    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<Account>> {
        let row = sqlx::query("SELECT id, email, password_hash, created_at FROM accounts WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| Account {
            id: r.get::<uuid::Uuid, _>("id").to_string(),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            created_at: r.get("created_at"),
        }))
    }

    pub async fn find_by_id(&self, id: &str) -> AppResult<Option<Account>> {
        let uuid = match uuid::Uuid::parse_str(id) {
            Ok(u) => u,
            Err(_) => return Ok(None),
        };
        let row = sqlx::query("SELECT id, email, password_hash, created_at FROM accounts WHERE id = $1")
            .bind(uuid)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| Account {
            id: r.get::<uuid::Uuid, _>("id").to_string(),
            email: r.get("email"),
            password_hash: r.get("password_hash"),
            created_at: r.get("created_at"),
        }))
    }

    pub async fn delete_account(&self, id: &str) -> AppResult<bool> {
        let uuid = match uuid::Uuid::parse_str(id) {
            Ok(u) => u,
            Err(_) => return Ok(false),
        };
        let result = sqlx::query("DELETE FROM accounts WHERE id = $1")
            .bind(uuid)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // ─── Results CRUD ────────────────────────────────────────────────────────

    pub async fn save_result(
        &self,
        account_id: &str,
        test_type: &str,
        result: &Value,
    ) -> AppResult<String> {
        let id = uuid::Uuid::new_v4();
        let account_uuid = uuid::Uuid::parse_str(account_id)
            .map_err(|_| crate::error::AppError::BadRequest("Invalid account ID".to_string()))?;
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO results (id, account_id, test_type, result, created_at) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(id)
        .bind(account_uuid)
        .bind(test_type)
        .bind(result)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(id.to_string())
    }

    pub async fn get_results(&self, account_id: &str) -> AppResult<Vec<StoredResult>> {
        let account_uuid = match uuid::Uuid::parse_str(account_id) {
            Ok(u) => u,
            Err(_) => return Ok(Vec::new()),
        };
        let rows = sqlx::query(
            "SELECT id, account_id, test_type, result, created_at FROM results WHERE account_id = $1 ORDER BY created_at DESC",
        )
        .bind(account_uuid)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| StoredResult {
                id: r.get::<uuid::Uuid, _>("id").to_string(),
                account_id: r.get::<uuid::Uuid, _>("account_id").to_string(),
                test_type: r.get("test_type"),
                result: r.get("result"),
                created_at: r.get("created_at"),
            })
            .collect())
    }

    pub async fn get_results_by_type(
        &self,
        account_id: &str,
        test_type: &str,
    ) -> AppResult<Vec<StoredResult>> {
        let account_uuid = match uuid::Uuid::parse_str(account_id) {
            Ok(u) => u,
            Err(_) => return Ok(Vec::new()),
        };
        let rows = sqlx::query(
            "SELECT id, account_id, test_type, result, created_at FROM results WHERE account_id = $1 AND test_type = $2 ORDER BY created_at DESC",
        )
        .bind(account_uuid)
        .bind(test_type)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| StoredResult {
                id: r.get::<uuid::Uuid, _>("id").to_string(),
                account_id: r.get::<uuid::Uuid, _>("account_id").to_string(),
                test_type: r.get("test_type"),
                result: r.get("result"),
                created_at: r.get("created_at"),
            })
            .collect())
    }

    // ─── GDPR Export ─────────────────────────────────────────────────────────

    pub async fn get_all_results_for_account(
        &self,
        account_id: &str,
    ) -> AppResult<Value> {
        let account = self.find_by_id(account_id).await?;
        let results = self.get_results(account_id).await?;

        let account_info = match account {
            Some(a) => serde_json::json!({
                "id": a.id,
                "email": a.email,
                "created_at": a.created_at.to_rfc3339(),
            }),
            None => serde_json::json!(null),
        };

        let export = serde_json::json!({
            "account": account_info,
            "results": results.iter().map(|r| serde_json::json!({
                "id": r.id,
                "test_type": r.test_type,
                "result": r.result,
                "created_at": r.created_at.to_rfc3339(),
            })).collect::<Vec<_>>(),
            "export_date": Utc::now().to_rfc3339(),
        });

        Ok(export)
    }
}
