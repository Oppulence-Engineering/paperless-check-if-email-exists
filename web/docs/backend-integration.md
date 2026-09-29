# Backend integration

The browser calls the same-origin `/api/backend/*` route. That route verifies the Better Auth
session and active organization, mints a short-lived JWT, and forwards the request to the private
Rust API. The Rust API validates the JWT through `/api/auth/jwks`, derives the tenant from the
signed organization claim, and enforces permissions. Browser-supplied identity headers are
replaced at the BFF boundary.

`BACKEND_API_URL` is the private Rust origin, and `BACKEND_JWT_AUDIENCE` must match
`RCH__AUTH__AUDIENCE`. `RCH__AUTH__ISSUER` must match `BETTER_AUTH_URL`. The Rust API remains the
source of truth for product data. Better Auth uses its own PostgreSQL schema for identity data.

The source contract is `../backend/openapi.json`. Build the local TypeScript SDK in
`../sdks/typescript/src`, then run `pnpm contracts:generate` after API changes.
`pnpm contracts:check` checks generated files for drift, and `pnpm contracts:compatibility`
checks the BFF capability manifest against the source contract.

`make dev` starts the local stack. `pnpm test:e2e` covers the browser, Better Auth, BFF, SDK,
Rust API, and PostgreSQL path. The root `Dockerfile` builds the combined production image.
