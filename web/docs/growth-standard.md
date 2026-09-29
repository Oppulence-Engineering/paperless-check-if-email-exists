# Web development guide

The copied template's generator, component library, Storybook, and quality checks remain part of
this application. Run commands from `web/` unless noted otherwise.

- Use `pnpm gen` for product pages, components, hooks, schemas, stores, and stories. The generator
  preserves the `@oppulence-gen` marker and its `*.lit.ts` ownership record.
- Keep Server Components as the default. Use client components for browser state and interaction.
- Read product data through TanStack Query and the generated `@oppulence/reacher-sdk`. Generate
  runtime schemas from `../backend/openapi.json` with `pnpm contracts:generate`.
- Use `pnpm ui:add` for shared primitives. Keep the vendored Sim packages, Sim components, and
  Fumadocs content pipeline when changing the public site.
- Run `pnpm verify:fast` for everyday edits. `pnpm arch`, `pnpm contracts:check`, `pnpm queries:check`,
  `pnpm knip`, `pnpm test`, and `pnpm build` cover the wider source boundary.
- Run `make dev` at the repository root for PostgreSQL, RabbitMQ, the Rust API, and Next.js.

The Rust backend owns verification, lists, history, and tenant authorization. Direct database
access in Next.js is limited to Better Auth identity storage.
