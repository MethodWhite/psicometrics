"""Payment / subscription endpoints (Stripe integration)."""

from __future__ import annotations

import uuid
from datetime import UTC, datetime
from typing import Any

from fastapi import APIRouter, Request

from app.config import settings

router = APIRouter(prefix="/api/v1/payments", tags=["Payments"])

# ── In-memory stores ──────────────────────────────────────────────────────
_customers: dict[str, dict[str, Any]] = {}  # account_id -> stripe customer
_checkout_sessions: dict[str, dict[str, Any]] = {}

# ── Routes ────────────────────────────────────────────────────────────────


@router.post("/create-checkout")
async def create_checkout(payload: dict):
    """Create a Stripe Checkout Session for subscription purchase."""
    account_id = payload.get("account_id", "")
    price_id = payload.get("price_id", settings.stripe_price_monthly)

    # In production this would call ``stripe.checkout.Session.create(...)``.
    session_id = f"cs_{uuid.uuid4().hex}"
    _checkout_sessions[session_id] = {
        "id": session_id,
        "account_id": account_id,
        "price_id": price_id,
        "status": "pending",
        "created_at": datetime.now(UTC).isoformat(),
    }
    return {
        "session_id": session_id,
        "url": f"https://checkout.stripe.com/pay/{session_id}",
        "status": "pending",
    }


@router.post("/webhook")
async def stripe_webhook(request: Request):
    """Receive Stripe webhook events.

    In production, verify the signature with ``stripe.Webhook.construct_event()``
    using ``settings.stripe_webhook_secret``.
    """
    # Production: verify signature with ``stripe.Webhook.construct_event(
    #     await request.body(), sig_header, settings.stripe_webhook_secret)``
    
    return {"received": True}


@router.get("/portal/{account_id}")
async def billing_portal(account_id: str):
    """Return a Stripe Customer Portal URL for the given account."""
    customer = _customers.get(account_id)
    if not customer:
        # Auto-create a stub customer for demo purposes.
        _customers[account_id] = {
            "id": account_id,
            "stripe_customer_id": f"cus_{uuid.uuid4().hex}",
        }
        customer = _customers[account_id]

    # In production this would call ``stripe.billing_portal.Session.create(...)``.
    return {
        "url": f"https://billing.stripe.com/p/session/{customer['stripe_customer_id']}",
        "customer_id": customer["stripe_customer_id"],
    }


@router.get("/tiers")
async def list_tiers():
    """List available subscription tiers and pricing."""
    return [
        {
            "id": "free",
            "name": "Free",
            "price_monthly": 0,
            "price_yearly": 0,
            "features": ["3 tests", "Basic insights", "Community access"],
        },
        {
            "id": "premium",
            "name": "Premium",
            "price_monthly": 9.99,
            "price_yearly": 79.99,
            "features": [
                "Unlimited tests",
                "Detailed reports (PDF)",
                "Evolution tracking",
                "Advanced analytics",
                "Priority support",
            ],
        },
        {
            "id": "professional",
            "name": "Professional",
            "price_monthly": 29.99,
            "price_yearly": 249.99,
            "features": [
                "Everything in Premium",
                "B2B team management",
                "API access",
                "Custom branding",
                "Dedicated account manager",
            ],
        },
    ]
