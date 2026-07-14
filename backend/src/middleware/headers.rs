use axum::http::HeaderName;
use axum::response::Response;
use std::sync::LazyLock;

static SECURITY_HEADERS: LazyLock<Vec<(HeaderName, &'static str)>> = LazyLock::new(|| {
    vec![
        (HeaderName::from_static("content-security-policy"),
            "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self'; connect-src 'self'; frame-ancestors 'none'; base-uri 'self'; form-action 'self'"),
        (HeaderName::from_static("x-content-type-options"), "nosniff"),
        (HeaderName::from_static("x-frame-options"), "DENY"),
        (HeaderName::from_static("strict-transport-security"), "max-age=31536000; includeSubDomains; preload"),
        (HeaderName::from_static("referrer-policy"), "strict-origin-when-cross-origin"),
    ]
});

pub fn add_security_headers(response: &mut Response) {
    for (name, value) in SECURITY_HEADERS.iter() {
        response.headers_mut().insert(name.clone(), value.parse().unwrap());
    }
}
