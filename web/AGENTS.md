# Check If Email Exists web app

This is the copied `nextjs-app-template` application, adapted to the Rust backend in the parent repository. Preserve the template's reusable tooling, Fumadocs, and Sim packages and components.

- Keep browser API traffic same-origin through `/api/backend/*`. The BFF checks Better Auth sessions and sends short-lived backend JWTs. Never expose backend signing credentials to the browser.
- Use the generated `@oppulence/reacher-sdk` package for product API operations. The Rust backend owns verification, lists, history, and tenant authorization. Keep Drizzle and direct PostgreSQL access within Better Auth identity work.
- Use `pnpm gen` for new product routes, components, hooks, and schemas. Keep its `@oppulence-gen` stamps and `*.lit.ts` files; see `docs/growth-standard.md`. Adapt generated code to the Rust OpenAPI contract and SDK.
- Use TanStack Query for product reads and writes and Orval schemas for runtime validation. Keep server-only modules out of Client Components.
- Run the local stack with `make dev` from the parent directory. Build the combined production image from that directory's `Dockerfile`.
- Check changes with focused tests, `pnpm typecheck`, `pnpm lint`, and `pnpm contracts:compatibility` when the API contract changes.

<!-- BEGIN:nextjs-agent-rules -->

# This is NOT the Next.js you know

This version has breaking changes — APIs, conventions, and file structure may all differ from your training data. Read the relevant guide in `node_modules/next/dist/docs/` (resolved from this file's directory; in monorepos the `next` package may not be visible from the repo root) before writing any code. Heed deprecation notices.

This block is written and re-added by `next dev` — verify at `node_modules/next/dist/server/lib/generate-agent-files.js`. Removing it from a diff only re-creates the uncommitted change; committing it with your work keeps the tree clean.

<!-- END:nextjs-agent-rules -->
