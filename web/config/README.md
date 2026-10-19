# Repository Configuration

Repository-owned configuration is grouped by the boundary it protects. Keep framework-discovered
configuration at the application root; put manually invoked policy and generation configuration
here.

| Directory       | Ownership                                                            | Invoked by                                                |
| --------------- | -------------------------------------------------------------------- | --------------------------------------------------------- |
| `architecture/` | Dependency boundaries and exact migration baselines                  | ESLint and `pnpm arch`                                    |
| `contracts/`    | OpenAPI generation, backend capability manifest, compatibility gates | `pnpm contracts:generate`, `pnpm contracts:compatibility` |
| `generate/`     | Growth-standard templates, Zod CLI schemas, Plop                     | `pnpm gen`                                                |
| `quality/`      | Formatting, dead-code, and security policy                           | Prettier, Knip, OSV, and Semgrep                          |

Configuration moves must update `package.json`, repository-policy tests, contributor documentation,
and any scripts that invoke the tool directly. Do not add a new root-level policy file when the tool
supports an explicit configuration path.

The repository root `Dockerfile` packages the web runtime with the Rust backend. CI runs from
the repository root `.github/workflows/pr.yml` workflow.
