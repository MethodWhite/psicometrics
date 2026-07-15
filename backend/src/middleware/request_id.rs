//! Request-ID middleware.
//!
//! Injects a `x-request-id` header + extension on every request.
//! If the client already sent one, it is reused (useful for tracing across
//! services).  Otherwise a new UUID v4 is generated.

use axum::{
    extract::Request,
    http::{HeaderName, HeaderValue},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

static REQUEST_ID_HEADER: &str = "x-request-id";

pub async fn request_id_middleware(mut req: Request, next: Next) -> Response {
    // 1. Reuse or generate
    let request_id = req
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok().map(|s| s.to_string()))
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    // 2. Insert into extensions so downstream middleware / handlers can read it
    req.extensions_mut().insert(request_id.clone());

    // 3. Run the inner service
    let mut response = next.run(req).await;

    // 4. Set the response header
    if let Ok(val) = HeaderValue::from_str(&request_id) {
        response.headers_mut().insert(
            HeaderName::from_static("x-request-id"),
            val,
        );
    }

    response
}
