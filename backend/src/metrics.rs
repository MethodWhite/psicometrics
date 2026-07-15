//! Prometheus metrics for PsicoMetrics
//!
//! Tracks HTTP request rates, durations, test completions, and account gauges.

use axum::{
    extract::Request,
    http::header,
    middleware::Next,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use once_cell::sync::Lazy;
use prometheus::{
    gather, register_counter_vec, register_gauge, register_histogram_vec,
    CounterVec, Encoder, Gauge, HistogramVec, TextEncoder,
};
use std::time::Instant;

// ── Counters ────────────────────────────────────────────────────────────

pub static REQUESTS_TOTAL: Lazy<CounterVec> = Lazy::new(|| {
    register_counter_vec!(
        "psicometrics_requests_total",
        "Total number of HTTP requests",
        &["method", "path", "status"]
    )
    .expect("REQUESTS_TOTAL metric")
});

pub static TESTS_COMPLETED: Lazy<CounterVec> = Lazy::new(|| {
    register_counter_vec!(
        "psicometrics_tests_completed_total",
        "Total number of tests completed",
        &["test_type"]
    )
    .expect("TESTS_COMPLETED metric")
});

// ── Histograms ──────────────────────────────────────────────────────────

pub static REQUEST_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "psicometrics_request_duration_seconds",
        "HTTP request duration in seconds",
        &["path"]
    )
    .expect("REQUEST_DURATION metric")
});

// ── Gauges ──────────────────────────────────────────────────────────────

pub static ACCOUNTS_TOTAL: Lazy<Gauge> = Lazy::new(|| {
    register_gauge!(
        "psicometrics_accounts_total",
        "Total number of registered accounts"
    )
    .expect("ACCOUNTS_TOTAL metric")
});

pub static ACTIVE_USERS: Lazy<Gauge> = Lazy::new(|| {
    register_gauge!(
        "psicometrics_active_users",
        "Number of active users"
    )
    .expect("ACTIVE_USERS metric")
});

// ── Metrics endpoint ────────────────────────────────────────────────────

pub async fn metrics_handler() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let mut buffer = String::new();
    let metric_families = gather();
    encoder.encode_utf8(&metric_families, &mut buffer).unwrap();

    (
        [(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
        buffer,
    )
}

pub fn metrics_router() -> Router {
    Router::new().route("/api/v1/metrics", get(metrics_handler))
}

// ── Middleware ──────────────────────────────────────────────────────────

pub async fn metrics_middleware(req: Request, next: Next) -> Response {
    let path = req.uri().path().to_string();
    let method = req.method().to_string();
    let start = Instant::now();

    let response = next.run(req).await;

    let duration = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    REQUESTS_TOTAL
        .with_label_values(&[&method, &path, &status])
        .inc();
    REQUEST_DURATION
        .with_label_values(&[&path])
        .observe(duration);

    response
}
