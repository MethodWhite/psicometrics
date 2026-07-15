use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database_url: Option<String>,
    pub redis_url: Option<String>,
    pub secret_key: Option<String>,
    pub allowed_origins: Option<String>,
    pub debug: bool,

    // Stripe
    pub stripe_secret_key: Option<String>,
    pub stripe_webhook_secret: Option<String>,

    // Email (Resend)
    pub resend_api_key: Option<String>,
    pub email_from: Option<String>,
}

impl Settings {
    pub fn from_env() -> Self {
        envy::from_env().unwrap_or(Settings {
            database_url: None,
            redis_url: None,
            secret_key: None,
            allowed_origins: None,
            debug: false,
            stripe_secret_key: None,
            stripe_webhook_secret: None,
            resend_api_key: None,
            email_from: None,
        })
    }
}
