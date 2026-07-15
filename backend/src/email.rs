//! Email service — transactional emails & onboarding sequence
//!
//! Uses Resend API for delivery.
//!
//! Onboarding sequence:
//!   Day 0: Welcome + verify email
//!   Day 1: "Try the Big Five test"
//!   Day 3: "Your results explained"
//!   Day 7: "Compare with friends"
//!   Day 14: "Upgrade to Premium"
//!
//! All emails include an unsubscribe link and open-tracker pixel.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::error::AppResult;

// ── Config ──────────────────────────────────────────────────────────────

const UNSUBSCRIBE_BASE: &str = "https://psicometrics.com/api/v1/onboarding/unsubscribe";

// ── Email message type ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailMessage {
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub html: String,
    pub text: String,
}

/// Resend-compatible request body
#[derive(Debug, Serialize)]
struct ResendPayload {
    from: String,
    to: Vec<String>,
    subject: String,
    html: String,
    text: String,
}

// ── EmailClient (production, uses Resend) ───────────────────────────────

#[derive(Clone)]
pub struct EmailClient {
    api_key: String,
    from: String,
    http_client: reqwest::Client,
}

impl EmailClient {
    pub fn new(api_key: &str, from: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            from: from.to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn send(&self, msg: &EmailMessage) -> AppResult<()> {
        if self.api_key == "unset" || self.api_key.is_empty() {
            tracing::info!(
                to = ?msg.to,
                subject = %msg.subject,
                "email skipped (no API key)"
            );
            return Ok(());
        }

        let payload = ResendPayload {
            from: msg.from.clone(),
            to: msg.to.clone(),
            subject: msg.subject.clone(),
            html: msg.html.clone(),
            text: msg.text.clone(),
        };

        let resp = self
            .http_client
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
            .send()
            .await
            .map_err(|e| crate::error::AppError::Internal(format!("Email send error: {e}")))?;

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            tracing::error!("Resend API error: {body}");
        } else {
            tracing::info!(to = ?msg.to, subject = %msg.subject, "email sent");
        }

        Ok(())
    }
}

// ── Onboarding state ────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct OnboardingState {
    pub email: String,
    pub started_at: DateTime<Utc>,
    pub current_step: u8,
    pub completed: bool,
    pub unsubscribed: bool,
}

#[derive(Default)]
pub struct EmailStore {
    pub users: RwLock<Vec<OnboardingState>>,
}

impl EmailStore {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn start_onboarding(&self, email: &str) -> OnboardingState {
        let mut users = self.users.write();
        users.retain(|u| u.email != email);
        let state = OnboardingState {
            email: email.to_string(),
            started_at: Utc::now(),
            current_step: 0,
            completed: false,
            unsubscribed: false,
        };
        users.push(state.clone());
        state
    }

    pub fn find_by_email(&self, email: &str) -> Option<OnboardingState> {
        let users = self.users.read();
        users.iter().find(|u| u.email == email).cloned()
    }

    pub fn unsubscribe(&self, email: &str) -> bool {
        let mut users = self.users.write();
        if let Some(user) = users.iter_mut().find(|u| u.email == email) {
            user.unsubscribed = true;
            true
        } else {
            false
        }
    }

    pub fn advance_step(&self, email: &str) -> Option<OnboardingState> {
        let mut users = self.users.write();
        if let Some(user) = users.iter_mut().find(|u| u.email == email) {
            if user.current_step >= 4 {
                user.completed = true;
            } else {
                user.current_step += 1;
            }
            Some(user.clone())
        } else {
            None
        }
    }

    pub fn status(&self, email: &str) -> Option<serde_json::Value> {
        let users = self.users.read();
        users.iter().find(|u| u.email == email).map(|u| {
            serde_json::json!({
                "email": u.email,
                "started_at": u.started_at,
                "current_step": u.current_step,
                "completed": u.completed,
                "unsubscribed": u.unsubscribed,
            })
        })
    }
}

// ── Convenience methods on EmailClient ──────────────────────────────────

impl EmailClient {
    /// Send a welcome email on registration.
    pub async fn send_welcome(&self, email: &str, name: &str) -> AppResult<()> {
        let subject = "Welcome to PsicoMetrics!";
        let body = format!(
            r#"<h2>Welcome to PsicoMetrics, {name}!</h2>
<p>Thank you for creating your account. Start discovering your personality profile.</p>
<ul>
<li>Complete your first assessment</li>
<li>View detailed reports and interpretations</li>
<li>Track your evolution over time</li>
<li>Upgrade to Premium for unlimited reports</li>
</ul>
<p><a href="https://psicometrics.app/login" style="display:inline-block;padding:12px 24px;background:#4F46E5;color:#fff;text-decoration:none;border-radius:6px;">Get Started</a></p>
<p>— The PsicoMetrics Team</p>"#,
            name = name
        );
        let msg = build_email(email, subject, &body, Some(email));
        self.send(&msg).await
    }

    /// Send an email with a link to download a premium report PDF.
    pub async fn send_report(&self, email: &str, pdf_url: &str) -> AppResult<()> {
        let subject = "Your PsicoMetrics Premium Report is Ready";
        let body = format!(
            r#"<h2>Your Premium Report is Ready</h2>
<p>Your detailed personality assessment report has been generated.</p>
<p><a href="{pdf_url}" style="display:inline-block;padding:12px 24px;background:#4F46E5;color:#fff;text-decoration:none;border-radius:6px;">Download Report</a></p>
<p>The download link will expire in 7 days.</p>
<p>— The PsicoMetrics Team</p>"#,
            pdf_url = pdf_url
        );
        let msg = build_email(email, subject, &body, None);
        self.send(&msg).await
    }

    /// Send a password reset email with a token link.
    pub async fn send_password_reset(&self, email: &str, token: &str) -> AppResult<()> {
        let subject = "PsicoMetrics — Password Reset";
        let reset_url = format!("https://psicometrics.app/reset-password?token={token}");
        let body = format!(
            r#"<h2>Password Reset</h2>
<p>We received a request to reset your password. Click the button below to set a new one.</p>
<p><a href="{reset_url}" style="display:inline-block;padding:12px 24px;background:#4F46E5;color:#fff;text-decoration:none;border-radius:6px;">Reset Password</a></p>
<p>If you did not request this, you can safely ignore this email.</p>
<p>— The PsicoMetrics Team</p>"#,
            reset_url = reset_url
        );
        let msg = build_email(email, subject, &body, None);
        self.send(&msg).await
    }
}

// ── Email builders ──────────────────────────────────────────────────────

fn build_email(to: &str, subject: &str, body_html: &str, token: Option<&str>) -> EmailMessage {
    let unsubscribe_html = match token {
        Some(t) => format!(
            r#"<br><hr><small style="color:#888"><a href="{}/{}">Unsubscribe</a></small>"#,
            UNSUBSCRIBE_BASE, t
        ),
        None => String::new(),
    };

    let tracker_pixel = match token {
        Some(t) => format!(
            r#"<img src="{}/open/{}" width="1" height="1" alt="" />"#,
            UNSUBSCRIBE_BASE, t
        ),
        None => String::new(),
    };

    let html = format!(
        r#"<!DOCTYPE html><html><body style="font-family:sans-serif;max-width:600px;margin:0 auto;padding:20px">
{}{}{}</body></html>"#,
        body_html, tracker_pixel, unsubscribe_html
    );

    let text = html
        .replace("<br>", "\n")
        .replace("<hr>", "\n---\n")
        .replace("<li>", "• ")
        .replace("</li>", "\n");

    EmailMessage {
        from: "noreply@psicometrics.com".to_string(),
        to: vec![to.to_string()],
        subject: subject.to_string(),
        html,
        text,
    }
}

// ── Onboarding email templates ──────────────────────────────────────────

pub fn welcome_email(to: &str, token: &str) -> EmailMessage {
    build_email(
        to,
        "Welcome to PsicoMetrics!",
        &format!(
            r#"<h2>Welcome to PsicoMetrics!</h2>
<p>Thank you for creating your account. You now have access to all our scientifically validated personality tests.</p>
<p><strong>Get started:</strong></p>
<ul>
  <li>Take the <a href="https://psicometrics.com/test/big_five">Big Five / OCEAN</a> test</li>
  <li>Save your results to your account</li>
  <li>Compare with friends and colleagues</li>
</ul>
<p>If you didn't create this account, you can safely ignore this email.</p>
<p>— The PsicoMetrics Team</p>"#
        ),
        Some(token),
    )
}

pub fn day1_try_big_five(to: &str, token: &str) -> EmailMessage {
    build_email(
        to,
        "Try the Big Five personality test",
        &format!(
            r#"<h2>Discover your personality profile</h2>
<p>The Big Five / OCEAN model is the most scientifically validated framework in personality psychology.</p>
<p>Take the test now and discover your Openness, Conscientiousness, Extraversion, Agreeableness, and Neuroticism scores.</p>
<p><a href="https://psicometrics.com/test/big_five" style="display:inline-block;padding:12px 24px;background:#4F46E5;color:#fff;text-decoration:none;border-radius:6px;">Start the Big Five Test →</a></p>"#
        ),
        Some(token),
    )
}

pub fn day3_results_explained(to: &str, token: &str) -> EmailMessage {
    build_email(
        to,
        "Your PsicoMetrics results explained",
        &format!(
            r#"<h2>Understanding your scores</h2>
<p>Each personality test on PsicoMetrics comes with a detailed interpretation guide for your unique profile.</p>
<p><a href="https://psicometrics.com/account" style="display:inline-block;padding:12px 24px;background:#4F46E5;color:#fff;text-decoration:none;border-radius:6px;">View Your Results →</a></p>"#
        ),
        Some(token),
    )
}

pub fn day7_compare_with_friends(to: &str, token: &str) -> EmailMessage {
    build_email(
        to,
        "Compare your personality with friends",
        &format!(
            r#"<h2>How do you compare?</h2>
<p>With PsicoMetrics you can compare any two test results side by side, see compatibility scores, and share your results via a public profile link.</p>
<p><a href="https://psicometrics.com/compare" style="display:inline-block;padding:12px 24px;background:#4F46E5;color:#fff;text-decoration:none;border-radius:6px;">Compare Results →</a></p>"#
        ),
        Some(token),
    )
}

pub fn day14_upgrade_premium(to: &str, token: &str) -> EmailMessage {
    build_email(
        to,
        "Unlock premium features on PsicoMetrics",
        &format!(
            r#"<h2>Ready for more?</h2>
<p>Upgrade to PsicoMetrics Premium and unlock unlimited test history, advanced reports, priority support, and early access to new tests.</p>
<p><a href="https://psicometrics.com/premium" style="display:inline-block;padding:12px 24px;background:#4F46E5;color:#fff;text-decoration:none;border-radius:6px;">Upgrade Now →</a></p>"#
        ),
        Some(token),
    )
}

// ── Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_email_contains_unsubscribe() {
        let email = welcome_email("test@example.com", "tok123");
        assert!(email.html.contains("Unsubscribe"));
        assert_eq!(email.to, vec!["test@example.com".to_string()]);
    }

    #[test]
    fn test_onboarding_store() {
        let store = EmailStore::new();
        store.start_onboarding("a@b.com");
        let state = store.find_by_email("a@b.com").unwrap();
        assert_eq!(state.current_step, 0);

        store.advance_step("a@b.com");
        let state = store.find_by_email("a@b.com").unwrap();
        assert_eq!(state.current_step, 1);

        store.unsubscribe("a@b.com");
        assert!(store.find_by_email("a@b.com").unwrap().unsubscribed);
    }
}
