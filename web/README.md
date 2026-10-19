# Check If Email Exists web app

This Next.js application is copied from `nextjs-app-template` and lives in this repository as normal source. It uses the existing Rust backend and generated TypeScript SDK for verification, lists, and history. Better Auth owns users, sessions, and organizations.

## Local development

From the repository root, run `make dev`. It starts PostgreSQL, RabbitMQ, the Rust API, and this app. Open `http://localhost:3000`. Local email sign-in uses code `123456`; the fixed code is enabled only for the local loopback development stack. The Rust API and Next.js reload on source changes. `Ctrl+C` stops the application processes; the Compose services remain available for later runs.

The root [`.env.example`](../.env.example) lists backend and web defaults. This directory has a [web-only example](./.env.example). Use separate secrets and a public HTTPS `BETTER_AUTH_URL` in production.

## Request and data boundaries

Browser requests go to the same-origin `/api/backend/*` route. The BFF checks the Better Auth session and active organization, then sends a short-lived JWT to the private Rust API. The Rust API validates the JWT and enforces tenant permissions. The browser does not receive the backend signing key or JWT. External API clients use `/v1/*` on the same public host with a tenant API key created in the app's API keys settings. That path forwards requests and responses to Rust without exposing the private backend port or accepting the backend secret.

```bash
curl "$APP_URL/v1/check_email" \
  -H "Authorization: Bearer $REACHER_API_KEY" \
  -H 'Content-Type: application/json' \
  --data '{"to_email":"test@valid.example.com","sandbox":true}'
```

Set the generated SDK's base URL to the app host when calling it from another service. Provider callbacks under `/v1/inbound/providers/*` use a delivery token and provider authentication instead of a workspace API key. Provider endpoints with an IP allowlist reject proxied callbacks because Rust sees the local proxy address. Platform admin and legacy v0 routes still require the private Rust service and its backend secret. Public `/v1/check-email-with-onboard` is disabled; create workspaces through app sign-up.

Product screens use TanStack Query and the generated `@oppulence/reacher-sdk` package. The SDK is generated from `backend/openapi.json`; Orval provides runtime schemas. Check, list, and history data live behind Rust API routes. Direct PostgreSQL access in this app is limited to Better Auth identity storage and platform administration of identity records.

The copied template's Fumadocs content pipeline, Sim packages and components, Storybook, generator, React development tools, architecture checks, lint, Vitest, and Playwright remain available. Product routes are under `app/(product)/app`. The template's public route inventory and Sim page layouts are under `app/(marketing)`; their visible claims describe email verification. Legacy URLs remain available with updated copy. The legal routes retain their layout but show product-specific drafts marked for legal review.

## Production image

Build the combined image from the repository root with `docker build -t check-if-email-exists .`. It includes Next.js standalone, the Rust API and worker, Chromium, and ChromeDriver. It exposes one public port, 3000. PostgreSQL and RabbitMQ are external services. Supply `DATABASE_URL`, `BETTER_AUTH_URL`, `BETTER_AUTH_SECRET`, `SCIM_CREDENTIAL_HASH_SECRET`, `RESEND_API_KEY`, `RESEND_FROM`, `RCH__HEADER_SECRET`, `RCH__WORKER__RABBITMQ__URL`, and `BACKEND_JWT_AUDIENCE` at runtime. `/healthz` reports web liveness; `/readyz` checks configuration, PostgreSQL, and the Rust API.

## Checks

Run `pnpm typecheck`, `pnpm lint`, and `pnpm test` here. Run `pnpm contracts:compatibility` after an API contract change. `pnpm test:e2e` starts the real local stack unless `E2E_BASE_URL` points to an existing one. The focused browser journey covers authentication, a sample verification, list upload, history, and sign-out.
