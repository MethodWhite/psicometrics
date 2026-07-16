"""Email notification service with Resend / SendGrid integration.

Provides an ``EmailClient`` class for sending transactional emails:
welcome, report-ready, password-reset, and an onboarding sequence.
"""

from __future__ import annotations

import logging
import os
from dataclasses import dataclass
from typing import Any

import httpx

logger = logging.getLogger(__name__)


@dataclass
class EmailConfig:
    """Email provider configuration."""

    provider: str = "resend"  # "resend" | "sendgrid"
    api_key: str = ""
    from_email: str = "noreply@psicometrics.app"
    from_name: str = "Psicometrics"
    base_url: str = "https://api.resend.com"  # Resend default


class EmailClient:
    """Transactional email client wrapping Resend or SendGrid APIs."""

    def __init__(self, config: EmailConfig | None = None) -> None:
        self.config = config or EmailConfig(
            api_key=os.getenv("EMAIL_API_KEY", ""),
        )

    # ── helpers ───────────────────────────────────────────────────────

    def _resend_headers(self) -> dict[str, str]:
        return {
            "Authorization": f"Bearer {self.config.api_key}",
            "Content-Type": "application/json",
        }

    def _sendgrid_headers(self) -> dict[str, str]:
        return {
            "Authorization": f"Bearer {self.config.api_key}",
            "Content-Type": "application/json",
        }

    async def _send_resend(self, to: str, subject: str,
                           html: str) -> dict[str, Any]:
        """Send via Resend API."""
        payload = {
            "from": f"{self.config.from_name} <{self.config.from_email}>",
            "to": [to],
            "subject": subject,
            "html": html,
        }
        async with httpx.AsyncClient() as client:
            resp = await client.post(
                f"{self.config.base_url}/emails",
                headers=self._resend_headers(),
                json=payload,
                timeout=15,
            )
            resp.raise_for_status()
            return resp.json()

    async def _send_sendgrid(self, to: str, subject: str,
                             html: str) -> dict[str, Any]:
        """Send via SendGrid v3 Mail Send API."""
        payload = {
            "personalizations": [{"to": [{"email": to}]}],
            "from": {"email": self.config.from_email,
                     "name": self.config.from_name},
            "subject": subject,
            "content": [{"type": "text/html", "value": html}],
        }
        async with httpx.AsyncClient() as client:
            resp = await client.post(
                "https://api.sendgrid.com/v3/mail/send",
                headers=self._sendgrid_headers(),
                json=payload,
                timeout=15,
            )
            resp.raise_for_status()
            return {"status": "sent", "code": resp.status_code}

    async def _send(self, to: str, subject: str,
                    html: str) -> dict[str, Any]:
        """Route to the configured provider."""
        if self.config.provider == "sendgrid":
            return await self._send_sendgrid(to, subject, html)
        return await self._send_resend(to, subject, html)

    # ── transactional emails ──────────────────────────────────────────

    async def send_welcome(self, email: str) -> dict[str, Any]:
        """Send a welcome email upon user registration."""
        subject = "Welcome to Psicometrics! 🎉"
        html = f"""
        <h2>Welcome to Psicometrics!</h2>
        <p>Hi there,</p>
        <p>Thank you for registering. You can now take our full range of
        personality tests and explore your results.</p>
        <p><a href="https://psicometrics.app/dashboard"
              style="background:#3498db;color:#fff;padding:10px 20px;
                     text-decoration:none;border-radius:4px;">
           Go to Dashboard</a></p>
        <p style="color:#888;font-size:12px;">
        The Psicometrics Team</p>
        """
        return await self._send(email, subject, html)

    async def send_report(self, email: str,
                          pdf_url: str) -> dict[str, Any]:
        """Notify user that a report PDF is ready."""
        subject = "Your Psicometrics Report is Ready"
        html = f"""
        <h2>Your Report is Ready</h2>
        <p>Your psicometric assessment report has been generated.</p>
        <p><a href="{pdf_url}"
              style="background:#2ecc71;color:#fff;padding:10px 20px;
                     text-decoration:none;border-radius:4px;">
           View Report</a></p>
        <p style="color:#888;font-size:12px;">
        The Psicometrics Team</p>
        """
        return await self._send(email, subject, html)

    async def send_password_reset(self, email: str,
                                  token: str) -> dict[str, Any]:
        """Send a password reset email with a signed token."""
        reset_url = f"https://psicometrics.app/reset-password?token={token}"
        subject = "Reset Your Psicometrics Password"
        html = f"""
        <h2>Password Reset</h2>
        <p>Click the link below to reset your password. This link
        expires in 60 minutes.</p>
        <p><a href="{reset_url}"
              style="background:#e74c3c;color:#fff;padding:10px 20px;
                     text-decoration:none;border-radius:4px;">
           Reset Password</a></p>
        <p>If you didn't request this, you can safely ignore this email.</p>
        <p style="color:#888;font-size:12px;">
        The Psicometrics Team</p>
        """
        return await self._send(email, subject, html)

    # ── onboarding sequence (5 emails over 14 days) ───────────────────

    _ONBOARDING_TEMPLATES: list[dict[str, str]] = [
        {
            "day": "1",
            "subject": "Step 1: Take your first test",
            "html": """
            <h2>Get started with a test</h2>
            <p>Take the Big Five personality test to discover your
            core traits.</p>
            <p><a href="https://psicometrics.app/tests/big-five"
                  style="background:#3498db;color:#fff;padding:10px 20px;
                         text-decoration:none;border-radius:4px;">
               Start Big Five Test</a></p>
            """,
        },
        {
            "day": "4",
            "subject": "Step 2: Explore your results",
            "html": """
            <h2>Understand your results</h2>
            <p>Your test results include detailed breakdowns per factor,
            percentiles, and a full PDF report.</p>
            <p><a href="https://psicometrics.app/dashboard"
                  style="background:#2ecc71;color:#fff;padding:10px 20px;
                         text-decoration:none;border-radius:4px;">
               View My Results</a></p>
            """,
        },
        {
            "day": "7",
            "subject": "Step 3: Compare with friends",
            "html": """
            <h2>Compare personality types</h2>
            <p>Invite friends to compare your profiles and see your
            compatibility score.</p>
            <p><a href="https://psicometrics.app/community"
                  style="background:#f39c12;color:#fff;padding:10px 20px;
                         text-decoration:none;border-radius:4px;">
               Explore Community</a></p>
            """,
        },
        {
            "day": "10",
            "subject": "Step 4: Try the MBTI test",
            "html": """
            <h2>Discover your MBTI type</h2>
            <p>The Myers-Briggs Type Indicator reveals how you perceive
            the world and make decisions.</p>
            <p><a href="https://psicometrics.app/tests/mbti"
                  style="background:#9b59b6;color:#fff;padding:10px 20px;
                         text-decoration:none;border-radius:4px;">
               Take MBTI Test</a></p>
            """,
        },
        {
            "day": "14",
            "subject": "Step 5: Go Premium",
            "html": """
            <h2>Unlock all features</h2>
            <p>Upgrade to Premium for unlimited reports, advanced
            analytics, and priority support.</p>
            <p><a href="https://psicometrics.app/premium"
                  style="background:#e74c3c;color:#fff;padding:10px 20px;
                         text-decoration:none;border-radius:4px;">
               See Plans</a></p>
            """,
        },
    ]

    async def send_onboarding_email(self, email: str,
                                    step: int) -> dict[str, Any]:
        """Send one email from the 5-step onboarding sequence.

        Parameters
        ----------
        email:
            Recipient address.
        step:
            1-based index of the onboarding step (1-5).
        """
        if step < 1 or step > 5:
            raise ValueError(f"Onboarding step must be 1-5, got {step}")

        template = self._ONBOARDING_TEMPLATES[step - 1]
        html = template["html"] + (
            '<p style="color:#888;font-size:12px;">'
            'The Psicometrics Team</p>'
        )
        return await self._send(email, template["subject"], html)
