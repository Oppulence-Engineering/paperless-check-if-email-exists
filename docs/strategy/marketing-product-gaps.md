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
| G12 | SDK version | npm had `@oppulence/reacher-sdk` 4.0.0; the repo is at 4.3.0. The publish step failed for v4.1.0 and v4.2.0: the package named the repository by its old name, and npm provenance rejected that. | Done. 4.3.0 is `latest` on npm since 2026-10-16, published by the GitHub Actions trusted publisher (OIDC) with provenance. |
| G13 | Quality-gate threshold | The approval checklist read a `tenants.settings` column that does not exist and dropped the error, so it always used 90 and 70. Settings had no control. The sending-platforms page says "threshold set by you". | Done. Two threshold columns with a database check, fields in `PATCH /v1/me/settings`, and inputs in Settings → Verification. |

## Second audit (2026-10-16)

| ID | Gap | Detail | Fix |
| --- | --- | --- | --- |
| G14 | SCIM not limited by plan | The pricing table sells SCIM on Enterprise only. Any workspace could create SCIM credentials. | Done. The SCIM route creates or rotates credentials on Enterprise only, and Settings shows the plan. |
| G15 | Policy per job | The scores page said "choose a policy for the job". The policy is a workspace setting. | Done. The copy now says the workspace policy is set in Settings. |
| G16 | API-only paths on every plan | The outcomes API and pipeline push need an API key, which is Professional and Enterprise only. The pages did not say so. | Done. The copy names the plans. Provider webhooks and the in-app event form work on every plan. |
| G17 | "Hold" a bad upload | The home page said a bad upload is held. The product marks it "Review before sending" and does not block the export. | Done. The copy says "mark". |

## Production checks (2026-10-16)

| ID | Gap | Detail | Fix |
| --- | --- | --- | --- |
| G19 | Sign-in codes failed in production | Outside e2e mode the email OTP plugin got `generateOTP: undefined`, which replaced its default generator, so every code request returned 500. | Done. `generateOTP` is passed only in e2e mode, with a guard test. On production, a code request returns 200 and the form moves to the code step. |
| G20 | Canonical and sitemap URLs pointed at localhost | The image was built without `NEXT_PUBLIC_SITE_URL`, so canonical links, Open Graph URLs, and the sitemap used `http://localhost:3000`. | Done. The image builds with `https://verifier.playbookmedia.io`. All 40 sitemap pages return 200. |
| G21 | Sign-in emails not delivered in production | A code request returns 200 and shows the code step, but no email arrives. Better Auth sends the email in the background and only logs a failure. The log now names the cause: Resend answers `validation_error: API key is invalid`. Production uses the staging key through the deploy fallback. | Open: the owner must set a valid Resend key (see below). The log change is done and deployed. |

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
- npm trusted publisher for `@oppulence/reacher-sdk` (repository `Oppulence-Engineering/paperless-check-if-email-exists`, workflow `sdk-generate.yml`), validated by the 4.3.0 publish.

Also done on 2026-10-16:

- **Production sign-in (G18).** `https://verifier.playbookmedia.io` serves the app from all three clusters (ingress, cert-manager certificate, Cloudflare CNAME to `oppulence.app`). Production has its own `BETTER_AUTH_SECRET` and `SCIM_CREDENTIAL_HASH_SECRET`; only the Resend settings are shared with staging.
- **Stripe live secrets.** `STRIPE_SECRET_KEY` and `STRIPE_WEBHOOK_SECRET` are set; production billing is on. The live webhook points to `https://verifier.playbookmedia.io/api/auth/stripe/webhook` with the four subscription events.
- **Auth migrations,** including `0007_stripe_billing.sql`, run at container start.

Still open, and blocks sign-in:

1. **Resend key (G21).** Create a Resend API key with sending access in the Resend account that owns the sender domain. Run `gh secret set RESEND_API_KEY -R Oppulence-Engineering/paperless-check-if-email-exists` and paste the key. Production now sends from `team@outboundconsole.com`; to use a Verifier address, also set `RESEND_FROM` to a sender on a verified domain. Then deploy and request a code.

Still open, and not a marketing gap:

1. **Staging billing.** Staging has no public address and no Stripe test keys, so billing is off there.
2. **Enterprise.** After a contract, add a row to `auth.subscription` with plan `enterprise` and status `active`.

## Status

G1 to G20 are closed in code, tested, and live on production. G21 waits for a valid Resend key; until then, nobody can sign in to production.

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
