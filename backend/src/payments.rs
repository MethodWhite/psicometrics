use stripe::Webhook;

use crate::error::{AppError, AppResult};

/// Subscription tier descriptor.
pub struct SubscriptionTier {
    pub name: &'static str,
    pub price_id: &'static str,
    pub amount_cents: u32,
    pub features: &'static [&'static str],
}

pub const TIERS: &[SubscriptionTier] = &[
    SubscriptionTier {
        name: "Free",
        price_id: "free",
        amount_cents: 0,
        features: &["2 reports/mo", "basic interpretation"],
    },
    SubscriptionTier {
        name: "Premium",
        price_id: "price_premium",
        amount_cents: 999,
        features: &[
            "Unlimited reports",
            "Detailed interpretation",
            "Comparison",
            "Evolution tracking",
            "CSV export",
        ],
    },
    SubscriptionTier {
        name: "Pro",
        price_id: "price_pro",
        amount_cents: 1999,
        features: &[
            "All Premium features",
            "Bulk testing (1000+)",
            "White-label reports",
            "API access",
            "Team management",
        ],
    },
];

fn find_tier(name: &str) -> Option<&'static SubscriptionTier> {
    TIERS.iter().find(|t| t.name.eq_ignore_ascii_case(name))
}

/// High-level Stripe client using reqwest for API calls + async-stripe for webhooks.
pub struct StripeClient {
    secret_key: String,
    webhook_secret: String,
    http: reqwest::Client,
}

impl StripeClient {
    pub fn new(secret_key: &str, webhook_secret: &str) -> Self {
        Self {
            secret_key: secret_key.to_string(),
            webhook_secret: webhook_secret.to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// Create a Stripe Checkout Session and return the hosted URL.
    pub async fn create_checkout_session(
        &self,
        tier_name: &str,
        account_id: &str,
        success_url: &str,
    ) -> AppResult<String> {
        let tier = find_tier(tier_name)
            .ok_or_else(|| AppError::BadRequest(format!("Unknown tier: {tier_name}")))?;

        if tier.amount_cents == 0 {
            return Ok("no_charge_needed".into());
        }

        let params = serde_json::json!({
            "mode": "subscription",
            "success_url": success_url,
            "client_reference_id": account_id,
            "line_items": [{
                "price": tier.price_id,
                "quantity": 1
            }]
        });

        let resp = self
            .http
            .post("https://api.stripe.com/v1/checkout/sessions")
            .header("Authorization", format!("Bearer {}", self.secret_key))
            .header("Content-Type", "application/json")
            .json(&params)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Stripe request failed: {e}")))?;

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Stripe parse error: {e}")))?;

        body["url"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| {
                let err = body
                    .get("error")
                    .map(|e| e.to_string())
                    .unwrap_or_default();
                AppError::Internal(format!("Stripe checkout error: {err}"))
            })
    }

    /// Create a Billing Portal session URL.
    pub async fn create_portal_session(&self, customer_id: &str) -> AppResult<String> {
        let params = serde_json::json!({
            "customer": customer_id,
            "return_url": "https://psicometrics.app/account/billing"
        });

        let resp = self
            .http
            .post("https://api.stripe.com/v1/billing_portal/sessions")
            .header("Authorization", format!("Bearer {}", self.secret_key))
            .header("Content-Type", "application/json")
            .json(&params)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Stripe portal request failed: {e}")))?;

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Stripe portal parse error: {e}")))?;

        body["url"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| {
                let err = body
                    .get("error")
                    .map(|e| e.to_string())
                    .unwrap_or_default();
                AppError::Internal(format!("Stripe portal error: {err}"))
            })
    }

    /// Verify a webhook signature using async-stripe and return the parsed event JSON.
    pub fn handle_webhook(
        &self,
        payload: &str,
        sig_header: &str,
    ) -> AppResult<serde_json::Value> {
        // Verifies the Stripe signature using async-stripe's Webhook
        Webhook::construct_event(payload, sig_header, &self.webhook_secret)
            .map_err(|e| AppError::BadRequest(format!("Webhook verification failed: {e}")))?;

        // Return parsed JSON for the route handler
        serde_json::from_str(payload)
            .map_err(|e| AppError::BadRequest(format!("Invalid webhook payload: {e}")))
    }

    /// Return all subscription tiers as JSON.
    pub fn tiers_json(&self) -> Vec<serde_json::Value> {
        TIERS
            .iter()
            .map(|t| {
                serde_json::json!({
                    "name": t.name,
                    "price_id": t.price_id,
                    "amount_cents": t.amount_cents,
                    "features": t.features,
                })
            })
            .collect()
    }
}
