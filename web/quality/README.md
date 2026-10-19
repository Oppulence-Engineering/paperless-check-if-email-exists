# Check If Email Exists web quality checks

From `web/`, run `pnpm verify:fast` for formatting, lint, types, generator checks, and backend contract compatibility. The root PR workflow also runs architecture, generated contract, query, unit, production build, and security checks.

Run `pnpm test:e2e` for the real browser journey. It starts the local stack when `E2E_BASE_URL` is unset. Set `E2E_BASE_URL` to test an already running deployment. The journey covers Better Auth, the same-origin backend route, the generated SDK, the Rust API, PostgreSQL, RabbitMQ, tenant isolation, and the Fumadocs article.

The `legacy` entries in `eslint.config.mjs` and `config/architecture/component-baseline.json` are exact-file migration allowances. New files must satisfy the architecture rules. Remove an allowance when its file is updated to meet the rule.

Rule ownership:

- `WEB001`–`WEB014`, `WEB019`, `WEB022`: `@oppulence/eslint-plugin-web`
- `WEB015`–`WEB018`: `quality/repository-policies.test.ts` and contract checks
- `WEB020`: `quality/component-contracts.test.ts` and colocated component tests
- `WEB023`–`WEB029`: generator policies, goldens, and `@oppulence-gen` stamps
- Query keys and client boundaries: `pnpm queries:check`
- Cross-package imports: `pnpm arch`
- Security sinks: `pnpm security:semgrep`
