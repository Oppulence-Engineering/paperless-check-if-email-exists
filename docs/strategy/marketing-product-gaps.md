# Gaps between the public site and the product

Date: 2026-10-16. Scope: every promise on the public site (home, features, use cases, integrations, guides, pricing, security) compared with the signed-in app (`/app`), the BFF, and the Rust backend.

Goal for this week: everything we market is available to a customer as soon as they sign up and pay.

## How the audit was done

1. Listed each promise on the public pages and in the pricing table (`web/app/(marketing)/sim-landing/subpages/pricing-comparison-data.ts`).
2. Signed in to the local stack as a new user and opened every `/app` area.
3. Traced each promise to the UI component, the BFF route, the backend handler, and the database.

## Status legend

- **Missing**: the product does not do it.
- **Partial**: the backend does it, but a customer cannot reach it, or it is wrong for some cases.
- **Wrong**: the product does it differently from what we say.
- **Done**: fixed in code and tested. The last column says how.

## P0: a customer cannot pay, or does not get what they paid for

| ID | Gap | What we market | What the product did before the fix | Status | Fix |
| --- | --- | --- | --- | --- | --- |
| G1 | No payment | Paid plans Starter ($49), Professional ($149), Enterprise | No checkout, no subscription, no billing portal. Paid-plan buttons open sign-up only. `/billing/success` and `/billing/cancel` are placeholders. | Done (needs Stripe keys) | Stripe through `@better-auth/stripe`. Checkout, signed webhook, billing portal, and a Plan card in Settings → Usage. Billing stays off until the Stripe keys are set. |
| G2 | Check allowance not set | Free 100, Starter 4,000, Professional 12,000 checks each month | A new workspace is created with `plan_tier = free` and `monthly_email_limit = NULL`, which the backend treats as unlimited (`backend/src/tenant/jwt.rs`, `backend/src/tenant/quota.rs`). | Done | The backend JWT carries the plan. The backend sets the plan and allowance on each request. An Enterprise limit set by hand is kept. |
| G3 | Plan names disagree | Professional | The web app uses `pro`; the backend uses `professional`. A Professional workspace reads as Free in the web app (`web/lib/entitlements`). The session never carries the plan, so every workspace reads as Free. | Done | One name, `professional`. The session reads the plan from the Stripe subscription. |
| G4 | Seat limit not enforced | Free 3, Starter 10, Professional 50 seats, Enterprise unlimited | The invite screen checks seats, but with the wrong plan (G3). The server has no member limit. | Done | Better Auth `membershipLimit` uses the plan's seat limit. |
| G5 | API access not limited by plan | REST API and SDKs on Professional and Enterprise | Any workspace can create API keys and call the API. | Done | API keys and API-key calls return 403 below Professional. The app's own calls work on every plan. |

## P1: marketed features that a customer cannot use

| ID | Gap | What we market | What the product did before the fix | Status | Fix |
| --- | --- | --- | --- | --- | --- |
| G6 | Scheduled re-checks cannot be turned on | "Scheduled re-checks" on every plan; "Set in days, per workspace" | The scheduler and `GET /v1/reverification/status` exist, but no endpoint creates or changes a schedule. Settings shows the status only. | Done | `PUT /v1/reverification` and controls in Settings → Verification. SDKs regenerated. |
| G7 | Amazon SES events are rejected | Amazon SES integration: bounces and complaints | The setup form sends no SNS topic ARN, and the backend rejects every SES event without one. SNS subscription confirmation is not handled, so the SNS subscription never becomes active. | Done | Topic ARN field added. The backend confirms the SNS subscription after the signature check. |
| G8 | Only one export file | Export files: safe, fixed, review, drop | The list page exports the "safe to send" file only. The backend supports every file. | Done | The list page downloads each file. |
| G9 | Address history lacks delivery events | "See the delivery events for each address in its history" | History shows check results only. Delivery events are on the Outcomes page. | Done | History shows the delivery events for the address. |

## P2: accuracy and polish

| ID | Gap | Detail | Fix |
| --- | --- | --- | --- |
| G10 | Mailgun bounces | Current Mailgun webhooks report a bounce as `failed` with `severity: permanent`. The backend maps the legacy `bounced` event only. The public page now lists complaints and unsubscribes only. | Done. Bounces are back on the Mailgun page. |
| G11 | Postmark complaints and unsubscribes | Postmark sends `SpamComplaint` and `SubscriptionChange`. The backend does not map them. The public page now lists bounces only. | Done. The Postmark page lists both events. |
| G12 | SDK version | npm has `@oppulence/reacher-sdk` 4.0.0; the repo is at 4.3.0. | Open. Needs an npm publish by the owner. |
| G13 | Quality-gate threshold | The approval checklist read a `tenants.settings` column that does not exist and dropped the error, so it always used 90 and 70. Settings had no control. The sending-platforms page says "threshold set by you". | Done. Two threshold columns with a database check, fields in `PATCH /v1/me/settings`, and inputs in Settings → Verification. |

## Product features we do not market

These work in the app but are not on the public site: Find email, Domains, Pipelines (scheduled verification of a saved list or push source), Analytics (result explorer, domain reputation), Comments, and the API explorer. Decide which to market; none blocks this week's goal.

## What the owner must do

1. Create two monthly prices in Stripe (test mode first): Starter $49 and Professional $149.
2. Set these values in the web environment and in the deploy secrets:
   - `STRIPE_SECRET_KEY`
   - `STRIPE_WEBHOOK_SECRET`
   - `STRIPE_PRICE_STARTER`
   - `STRIPE_PRICE_PROFESSIONAL`
3. Add a Stripe webhook to `<BETTER_AUTH_URL>/api/auth/stripe/webhook`. For a local test, run `stripe listen --forward-to localhost:3000/api/auth/stripe/webhook`.
4. Apply the auth migration `config/auth/migrations/0007_stripe_billing.sql`.
5. For Enterprise, add a row to `auth.subscription` with plan `enterprise` and status `active`.

## Plan for this week

| Day | Work | Gaps |
| --- | --- | --- |
| 1 | One plan model. Allowance and seats from the plan. Plan in the session. | G2, G3, G4 |
| 1 | API access by plan. | G5 |
| 2 | Re-check controls. SES topic ARN and SNS confirmation. Export files. History events. | G6, G7, G8, G9 |
| 2–3 | Payment: checkout, webhook, billing portal, Upgrade on Usage, billing result pages. | G1 |
| 3 | Mailgun and Postmark event mapping. Quality-gate threshold. | G10, G11, G13 |
| 4 | End-to-end test: sign up, pay in test mode, use each paid feature. | All |

## Decisions

1. **Payment provider.** Decided: Stripe. Stripe does not collect sales tax for us. Turn on Stripe Tax if we must collect it.
2. **Enterprise checkout.** Recommended: no self-serve checkout for Enterprise. Set the plan by hand after a contract.
3. **API on Starter.** Recommended: keep the published rule (Professional and Enterprise only).
