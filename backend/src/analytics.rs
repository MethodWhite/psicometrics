//! In-memory analytics for PsicoMetrics
//!
//! Provides:
//! - POST /api/v1/analytics/track — anonymous event logging (no auth)
//! - GET  /api/v1/analytics/dashboard — aggregated stats

use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

// ── Data types ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventName {
    TestStarted,
    TestCompleted,
    ReportDownloaded,
    AccountCreated,
    PremiumPurchased,
    ComparisonMade,
    PageView,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub event: EventName,
    #[serde(default)]
    pub properties: serde_json::Value,
    #[serde(default)]
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DashboardData {
    pub total_users: u64,
    pub total_tests: u64,
    pub popular_tests: Vec<PopularTest>,
    pub conversion_rate: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PopularTest {
    pub test_type: String,
    pub count: u64,
}

// ── In-memory store ─────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct AnalyticsStore {
    events: RwLock<Vec<AnalyticsEvent>>,
}

impl AnalyticsStore {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn push(&self, event: AnalyticsEvent) {
        self.events.write().push(event);
    }

    pub fn dashboard(&self) -> DashboardData {
        let events = self.events.read();

        let total_users = events
            .iter()
            .filter(|e| matches!(e.event, EventName::AccountCreated))
            .count() as u64;

        let total_tests = events
            .iter()
            .filter(|e| matches!(e.event, EventName::TestCompleted))
            .count() as u64;

        let mut test_counts: std::collections::HashMap<String, u64> =
            std::collections::HashMap::new();
        for e in events.iter() {
            if matches!(e.event, EventName::TestCompleted) {
                if let Some(test_type) = e.properties.get("test_type").and_then(|v| v.as_str()) {
                    *test_counts.entry(test_type.to_string()).or_default() += 1;
                }
            }
        }

        let mut popular_tests: Vec<PopularTest> = test_counts
            .into_iter()
            .map(|(test_type, count)| PopularTest { test_type, count })
            .collect();
        popular_tests.sort_by(|a, b| b.count.cmp(&a.count));
        popular_tests.truncate(10);

        let total_account_interactions = events
            .iter()
            .filter(|e| {
                matches!(
                    e.event,
                    EventName::AccountCreated | EventName::PremiumPurchased
                )
            })
            .count() as f64;

        let conversion_rate = if total_account_interactions > 0.0 {
            let premium = events
                .iter()
                .filter(|e| matches!(e.event, EventName::PremiumPurchased))
                .count() as f64;
            (premium / total_account_interactions * 100.0 * 100.0).round() / 100.0
        } else {
            0.0
        };

        DashboardData {
            total_users,
            total_tests,
            popular_tests,
            conversion_rate,
        }
    }
}

// ── Handlers ────────────────────────────────────────────────────────────

pub async fn track_event(
    axum::Extension(store): axum::Extension<Arc<AnalyticsStore>>,
    Json(event): Json<AnalyticsEvent>,
) -> impl IntoResponse {
    store.push(event);
    (StatusCode::OK, Json(serde_json::json!({ "status": "tracked" })))
}

pub async fn get_dashboard(
    axum::Extension(store): axum::Extension<Arc<AnalyticsStore>>,
) -> Json<DashboardData> {
    Json(store.dashboard())
}

// ── Router ──────────────────────────────────────────────────────────────

pub fn analytics_router() -> Router {
    Router::new()
        .route("/api/v1/analytics/track", post(track_event))
        .route("/api/v1/analytics/dashboard", get(get_dashboard))
}
