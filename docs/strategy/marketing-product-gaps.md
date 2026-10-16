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
| G1 | No payment | Paid plans Starter ($49), Professional ($149), Enterprise | No checkout, no subscription, no billing portal. Paid-plan buttons open sign-up only. `/billing/success` and `/billing/cancel` are placeholders. | Done | Stripe through `@better-auth/stripe`. Checkout, signed webhook, billing portal, and a Plan card in Settings → Usage. Billing stays off until the Stripe keys are set. The Starter and Professional buttons on the pricing page sign up, then open the plan card with that plan chosen. Tested end to end in Stripe test mode on 2026-10-16: checkout, webhook, plan and allowance in the app and the backend, billing portal, and cancel back to Free. |
| G2 | Check allowance not set | Free 100, Starter 4,000, Professional 12,000 checks each month | A new workspace is created with `plan_tier = free` and `monthly_email_limit = NULL`, which the backend treats as unlimited (`backend/src/tenant/jwt.rs`, `backend/src/tenant/quota.rs`). | Done | The backend JWT carries the plan. The backend sets the plan and allowance on each request. An Enterprise limit set by hand is kept. |
| G3 | Plan names disagree | Professional | The web app uses `pro`; the backend uses `professional`. A Professional workspace reads as Free in the web app (`web/lib/entitlements`). The session never carries the plan, so every workspace reads as Free. | Done | One name, `professional`. The session reads the plan from the Stripe subscription. |
| G4 | Seat limit not enforced | Free 3, Starter 10, Professional 50 seats, Enterprise unlimited | The invite screen checks seats, but with the wrong plan (G3). The server has no member limit. | Done | Better Auth `membershipLimit` uses the plan's seat limit. Tested live: Free accepts the 2nd and 3rd member and refuses the 4th; after an upgrade to Starter, the 4th member joins. |
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
| G12 | SDK version | npm has `@oppulence/reacher-sdk` 4.0.0; the repo is at 4.3.0. The publish step failed for v4.1.0 and v4.2.0: the package names the repository by its old name, and npm provenance rejects that. The npm token is also from December 2025. | Fixed in code: correct repository URL, and npm 11.6.2 for trusted publishing. The owner must add the trusted publisher on npmjs.com (or renew `NPM_TOKEN`). The next release then publishes 4.3.0. |
| G13 | Quality-gate threshold | The approval checklist read a `tenants.settings` column that does not exist and dropped the error, so it always used 90 and 70. Settings had no control. The sending-platforms page says "threshold set by you". | Done. Two threshold columns with a database check, fields in `PATCH /v1/me/settings`, and inputs in Settings → Verification. |

## Second audit (2026-10-16)

| ID | Gap | Detail | Fix |
| --- | --- | --- | --- |
| G14 | SCIM not limited by plan | The pricing table sells SCIM on Enterprise only. Any workspace could create SCIM credentials. | Done. The SCIM route creates or rotates credentials on Enterprise only, and Settings shows the plan. |
| G15 | Policy per job | The scores page said "choose a policy for the job". The policy is a workspace setting. | Done. The copy now says the workspace policy is set in Settings. |
| G16 | API-only paths on every plan | The outcomes API and pipeline push need an API key, which is Professional and Enterprise only. The pages did not say so. | Done. The copy names the plans. Provider webhooks and the in-app event form work on every plan. |
| G17 | "Hold" a bad upload | The home page said a bad upload is held. The product marks it "Review before sending" and does not block the export. | Done. The copy says "mark". |

## Product features the site did not market

| Feature | Decision |
| --- | --- |
| Pipelines | Marketed on Ongoing re-checks. Push needs the API, so the page names Professional and Enterprise. |
| Comments | Marketed on Team workspace. |
| Analytics | Source quality is on Marketing operations. The result explorer and the domain reputation check (8 blocklists, SPF, DMARC) are now on Marketing operations and Email marketing. |
| API explorer | A tool for API users. The developer docs cover the API. |
| Domains | Workspace administration. It makes no promise to a customer. |
| Find email | Marketed on 2026-10-16 at `/features/find-email`, in the Features menu, and in the pricing table (every plan). The page says that each address tried counts as one check (up to 13), and that the sender needs a lawful reason to contact the person. |

Every check path counts against the plan allowance: single checks, lists, bulk jobs, retries, re-checks, pipelines, and Find email.

## What the owner must do

Done on 2026-10-16:

- Live Stripe products and monthly prices: Verifier Starter `price_1URB8AJ5a5YKQUvYA0SsEw8c` ($49) and Verifier Professional `price_1URB9UJ5a5YKQUvYW1AHWiSM` ($149), lookup keys `verifier_starter_monthly` and `verifier_professional_monthly`, tax code SaaS business use.
- GitHub secrets `STRIPE_PRICE_STARTER`, `STRIPE_PRICE_PROFESSIONAL` (live) and `STRIPE_PRICE_STARTER_STAGING`, `STRIPE_PRICE_PROFESSIONAL_STAGING` (test).
- Customer portal, live and test: customers can switch between the two Verifier plans only, with prorated charges invoiced at once. Other products are not affected. Tested in test mode: Starter → Professional.
- npm trusted publisher for `@oppulence/reacher-sdk` (repository `Oppulence-Engineering/paperless-check-if-email-exists`, workflow `sdk-generate.yml`). It is pending validation: a publish through it must run before 2026-10-18 13:30 UTC.

Still open:

1. **Production sign-in (G18).** Done on 2026-10-16. `https://verifier.playbookmedia.io` serves the app from all three clusters (ingress, cert-manager certificate, Cloudflare CNAME to `oppulence.app`). Production has its own `BETTER_AUTH_SECRET` and `SCIM_CREDENTIAL_HASH_SECRET`; only the Resend settings are shared with staging. The deploy also needed `--server-side=true` for Helm and the database CA for the auth migration.
2. **Stripe secret keys.** Set `STRIPE_SECRET_KEY` (live) and `STRIPE_SECRET_KEY_STAGING` (test) with `gh secret set <NAME>`, and paste the key when asked. Until then, billing stays off and the plan card says so.
3. **Stripe webhooks.** Add an endpoint at `https://verifier.playbookmedia.io/api/auth/stripe/webhook` in live mode and one at `<BETTER_AUTH_URL_STAGING>/api/auth/stripe/webhook` in test mode, with the events `checkout.session.completed`, `customer.subscription.created`, `customer.subscription.updated`, and `customer.subscription.deleted`. Put each signing secret in `STRIPE_WEBHOOK_SECRET` and `STRIPE_WEBHOOK_SECRET_STAGING`.
4. **Auth migration.** Apply `config/auth/migrations/0007_stripe_billing.sql`.
5. **Enterprise.** Add a row to `auth.subscription` with plan `enterprise` and status `active` after a contract.

## Status legend

- **Missing**: the product does not do it.
- **Partial**: the backend does it, but a customer cannot reach it, or it is wrong for some cases.
- **Wrong**: the product does it differently from what we say.
- **Done**: fixed in code and tested. The last column says how.

## P0: a customer cannot pay, or does not get what they paid for

| ID | Gap | What we market | What the product did before the fix | Status | Fix |
| --- | --- | --- | --- | --- | --- |
| G1 | No payment | Paid plans Starter ($49), Professional ($149), Enterprise | No checkout, no subscription, no billing portal. Paid-plan buttons open sign-up only. `/billing/success` and `/billing/cancel` are placeholders. | Done | Stripe through `@better-auth/stripe`. Checkout, signed webhook, billing portal, and a Plan card in Settings → Usage. Billing stays off until the Stripe keys are set. The Starter and Professional buttons on the pricing page sign up, then open the plan card with that plan chosen. Tested end to end in Stripe test mode on 2026-10-16: checkout, webhook, plan and allowance in the app and the backend, billing portal, and cancel back to Free. |
| G2 | Check allowance not set | Free 100, Starter 4,000, Professional 12,000 checks each month | A new workspace is created with `plan_tier = free` and `monthly_email_limit = NULL`, which the backend treats as unlimited (`backend/src/tenant/jwt.rs`, `backend/src/tenant/quota.rs`). | Done | The backend JWT carries the plan. The backend sets the plan and allowance on each request. An Enterprise limit set by hand is kept. |
| G3 | Plan names disagree | Professional | The web app uses `pro`; the backend uses `professional`. A Professional workspace reads as Free in the web app (`web/lib/entitlements`). The session never carries the plan, so every workspace reads as Free. | Done | One name, `professional`. The session reads the plan from the Stripe subscription. |
| G4 | Seat limit not enforced | Free 3, Starter 10, Professional 50 seats, Enterprise unlimited | The invite screen checks seats, but with the wrong plan (G3). The server has no member limit. | Done | Better Auth `membershipLimit` uses the plan's seat limit. Tested live: Free accepts the 2nd and 3rd member and refuses the 4th; after an upgrade to Starter, the 4th member joins. |
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
| G12 | SDK version | npm has `@oppulence/reacher-sdk` 4.0.0; the repo is at 4.3.0. The publish step failed for v4.1.0 and v4.2.0: the package names the repository by its old name, and npm provenance rejects that. The npm token is also from December 2025. | Fixed in code: correct repository URL, and npm 11.6.2 for trusted publishing. The owner must add the trusted publisher on npmjs.com (or renew `NPM_TOKEN`). The next release then publishes 4.3.0. |
| G13 | Quality-gate threshold | The approval checklist read a `tenants.settings` column that does not exist and dropped the error, so it always used 90 and 70. Settings had no control. The sending-platforms page says "threshold set by you". | Done. Two threshold columns with a database check, fields in `PATCH /v1/me/settings`, and inputs in Settings → Verification. |

## Second audit (2026-10-16)

| ID | Gap | Detail | Fix |
| --- | --- | --- | --- |
| G14 | SCIM not limited by plan | The pricing table sells SCIM on Enterprise only. Any workspace could create SCIM credentials. | Done. The SCIM route creates or rotates credentials on Enterprise only, and Settings shows the plan. |
| G15 | Policy per job | The scores page said "choose a policy for the job". The policy is a workspace setting. | Done. The copy now says the workspace policy is set in Settings. |
| G16 | API-only paths on every plan | The outcomes API and pipeline push need an API key, which is Professional and Enterprise only. The pages did not say so. | Done. The copy names the plans. Provider webhooks and the in-app event form work on every plan. |
| G17 | "Hold" a bad upload | The home page said a bad upload is held. The product marks it "Review before sending" and does not block the export. | Done. The copy says "mark". |

## Product features the site did not market

| Feature | Decision |
| --- | --- |
| Pipelines | Marketed on Ongoing re-checks. Push needs the API, so the page names Professional and Enterprise. |
| Comments | Marketed on Team workspace. |
| Analytics | Source quality is on Marketing operations. The result explorer and the domain reputation check (8 blocklists, SPF, DMARC) are now on Marketing operations and Email marketing. |
| API explorer | A tool for API users. The developer docs cover the API. |
| Domains | Workspace administration. It makes no promise to a customer. |
| Find email | Marketed on 2026-10-16 at `/features/find-email`, in the Features menu, and in the pricing table (every plan). The page says that each address tried counts as one check (up to 13), and that the sender needs a lawful reason to contact the person. |

Every check path counts against the plan allowance: single checks, lists, bulk jobs, retries, re-checks, pipelines, and Find email.

## What the owner must do

1. Create the two monthly prices in Stripe live mode: Starter $49 and Professional $149. Test mode already has them (lookup keys `verifier_starter_monthly` and `verifier_professional_monthly`).
2. Set these values in the web environment and in the deploy secrets:
   - `STRIPE_SECRET_KEY`
   - `STRIPE_WEBHOOK_SECRET`
   - `STRIPE_PRICE_STARTER`
   - `STRIPE_PRICE_PROFESSIONAL`
3. Add a Stripe webhook to `<BETTER_AUTH_URL>/api/auth/stripe/webhook`. For a local test, run `stripe listen --events checkout.session.completed,customer.subscription.created,customer.subscription.updated,customer.subscription.deleted --forward-to localhost:3000/api/auth/stripe/webhook`.
4. Apply the auth migration `config/auth/migrations/0007_stripe_billing.sql`.
5. For Enterprise, add a row to `auth.subscription` with plan `enterprise` and status `active`.
6. Save a customer portal configuration in Stripe live mode (Settings > Billing > Customer portal). Test mode works today.
7. For G12: add a trusted publisher for `@oppulence/reacher-sdk` on npmjs.com (repository `Oppulence-Engineering/paperless-check-if-email-exists`, workflow `sdk-generate.yml`), or replace the `NPM_TOKEN` secret. The next release then publishes the SDK.

## Status

Every gap above is closed in code and tested. G12 waits for one owner step (item 7 above); the code fix is in place.

End-to-end checks on 2026-10-16, local stack, Stripe test mode:

1. Pricing → Choose Starter → sign-up → plan card with Starter marked.
2. Checkout with a Stripe test card → back to Settings → subscription `starter active`.
3. Plan card and backend: Starter, 4,000 checks, 10 seats.
4. Billing portal opens.
5. Cancel → Free, 100 checks, 3 seats, in the app and the backend.
6. Seats: Free refuses the 4th member; Starter accepts it.
7. API keys: blocked on Free with a plan message; available on Professional.

## Decisions

1. **Payment provider.** Decided: Stripe. Stripe does not collect sales tax for us. Turn on Stripe Tax if we must collect it.
2. **Enterprise checkout.** Applied: no self-serve checkout for Enterprise. Set the plan by hand after a contract.
3. **API on Starter.** Applied: the published rule (Professional and Enterprise only).
