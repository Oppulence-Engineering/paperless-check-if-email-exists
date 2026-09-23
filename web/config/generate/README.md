# Growth-standard generators

This directory is the template and input-schema home for `pnpm gen`.

| Path               | Ownership                                              |
| ------------------ | ------------------------------------------------------ |
| `input-schemas.ts` | Zod contracts for every CLI kind                       |
| `templates/`       | Handlebars sources (the standard made files)           |
| `plopfile.ts`      | Plop registration; the public door is still `pnpm gen` |

`gen hook`, `gen mutation`, and `gen upload` bind Orval Zod contracts. Prefer
`--operation <operationId>` so path, method, and schema names are resolved
from `BACKEND_OPENAPI_PATH` or the vendored sample. Do not generate local API stubs.

`gen realtime` accepts an explicit generated event schema and writes a parser,
parser test, and reconnecting client hook. Event code generation stays schema-led
until the backend publishes an AsyncAPI or equivalent event catalog.

Do not add a root-level `plopfile.js`. Wire new kinds here and in
`scripts/generate/plan.ts` in the same change.

See `docs/growth-standard.md`.
