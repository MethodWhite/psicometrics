"""Stripe-based payment service.

Handles subscription checkout sessions and webhook event processing
for three tiers: Free, Premium ($9.99/mo), Pro ($19.99/mo).
"""

from __future__ import annotations

import logging
from dataclasses import dataclass
from typing import Any

import stripe

logger = logging.getLogger(__name__)


@dataclass
class SubscriptionTier:
    name: str
    price_monthly: float
    price_yearly: float | None = None
    stripe_price_id: str = ""


TIERS: dict[str, SubscriptionTier] = {
    "free": SubscriptionTier("Free", 0.0),
    "premium": SubscriptionTier("Premium", 9.99, 99.99),
    "pro": SubscriptionTier("Pro", 19.99, 199.99),
}


class StripeClient:
    """Client for Stripe payment operations.

    Requires ``stripe_secret_key`` and optionally
    ``stripe_webhook_secret``.
    """

    def __init__(self, api_key: str | None = None,
                 webhook_secret: str | None = None) -> None:
        self.api_key = api_key or ""
        self.webhook_secret = webhook_secret or ""
        if self.api_key:
            stripe.api_key = self.api_key

    # ── helpers ───────────────────────────────────────────────────────

    def _get_price_id(self, tier: str) -> str:
        """Look up the Stripe price ID for a tier.

        Override by providing the price ID in the tier config or
        by setting ``stripe_price_monthly`` / ``stripe_price_yearly``
        in app config.
        """
        sub = TIERS.get(tier)
        if sub and sub.stripe_price_id:
            return sub.stripe_price_id
        # Fall back to namespaced env keys
        if tier == "premium":
            price_id = stripe.api_key and "price_premium_monthly"
            logger.warning("No Stripe price ID for premium; using placeholder.")
        return f"price_{tier}_monthly"  # placeholder

    # ── checkout ──────────────────────────────────────────────────────

    def create_checkout_session(
        self,
        tier: str,
        account_id: str,
        success_url: str,
        cancel_url: str | None = None,
    ) -> dict[str, Any]:
        """Create a Stripe Checkout Session for the given tier.

        Parameters
        ----------
        tier:
            One of ``"premium"``, ``"pro"``.
        account_id:
            Internal account ID to attach to the session
            (stored in ``client_reference_id``).
        success_url:
            Redirect URL after successful payment.
        cancel_url:
            Redirect URL if the user cancels (defaults to
            the success URL).

        Returns
        -------
        Dict with at least ``session_id`` and ``url``.
        """
        if tier == "free":
            return {"session_id": "", "url": success_url,
                    "message": "Free tier — no payment needed."}

        if not self.api_key:
            raise RuntimeError("Stripe API key not configured.")

        cancel = cancel_url or success_url

        session = stripe.checkout.Session.create(
            client_reference_id=account_id,
            success_url=success_url,
            cancel_url=cancel,
            line_items=[
                {
                    "price": self._get_price_id(tier),
                    "quantity": 1,
                },
            ],
            mode="subscription",
            metadata={"tier": tier, "account_id": account_id},
        )
        return {"session_id": session.id, "url": session.url,
                "tier": tier}

    # ── webhooks ──────────────────────────────────────────────────────

    def handle_webhook(self, payload: bytes | str,
                       sig_header: str) -> dict[str, Any]:
        """Validate and process a Stripe webhook event.

        Parameters
        ----------
        payload:
            Raw request body.
        sig_header:
            ``Stripe-Signature`` header value.

        Returns
        -------
        Dict with ``event_type`` and ``status``.

        Supported events:
            ``checkout.session.completed``
            ``customer.subscription.updated``
            ``customer.subscription.deleted``
            ``invoice.paid``
        """
        if not self.webhook_secret:
            logger.warning("Stripe webhook secret not configured; "
                           "skipping signature verification.")

        # Verify signature
        event: stripe.Event | None = None
        if self.webhook_secret:
            try:
                event = stripe.Webhook.construct_event(
                    payload, sig_header, self.webhook_secret
                )
            except ValueError:
                raise
            except stripe.error.SignatureVerificationError:
                raise
        else:
            # Fallback: parse without verification (dev only)
            data = stripe.Event.construct_from(
                stripe.util.json.loads(
                    payload if isinstance(payload, str)
                    else payload.decode("utf-8")
                ),
                stripe.api_key,
            )
            event = data

        if event is None:
            return {"event_type": "unknown", "status": "ignored"}

        event_type = event.type
        result: dict[str, Any] = {"event_type": event_type, "status": "ok"}

        if event_type == "checkout.session.completed":
            session = event.data.object
            tier = session.get("metadata", {}).get("tier", "unknown")
            account_id = session.get("client_reference_id", "")
            result.update({"tier": tier, "account_id": account_id,
                           "payment_status": session.get("payment_status")})

        elif event_type in ("customer.subscription.updated",
                            "customer.subscription.deleted"):
            subscription = event.data.object
            result.update({
                "subscription_id": subscription.get("id"),
                "status": subscription.get("status"),
            })

        elif event_type == "invoice.paid":
            invoice = event.data.object
            result.update({
                "invoice_id": invoice.get("id"),
                "amount_paid": invoice.get("amount_paid"),
                "currency": invoice.get("currency"),
            })

        return result
