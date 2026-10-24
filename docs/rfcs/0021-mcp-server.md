# RFC 0021: MCP Server

## Verify disposition — 2026-10-23

**Deferred; distribution-demand gate.** MCP is a delivery channel after stable, useful audit APIs and demonstrated demand. Phase-one tools should serve actual research, review, and export workflows, with scopes and explicit authorization for paid actions. A verification call can spend money and must not be classified as read-only merely because it returns data.

[Current build order](README.md) · [Codebase audit](../audits/2026-10-23-verify-north-star.md). This disposition supersedes conflicting historical scope, priority, or baseline statements below; it does not certify every original acceptance criterion.

| | |
|---|---|
| **Status** | Deferred; distribution-demand gate; audited 2026-10-23 |
| **Created** | 2026-10-21 |
| **Owners** | To assign: web lead, backend lead |
| **Parent** | RFC 0033, item 8 |
| **Depends on** | None for phase 1. RFC 0017 for the provider report tool. |
| **Release gates** | Security review of auth and tool scopes; Definition Of Done met |

## Summary

This RFC adds a remote Model Context Protocol (MCP) server at `/mcp` on the public host. AI clients such as Claude and ChatGPT use it to check addresses, read history and list quality, manage suppressions, and create segments from a chat. The server is a thin layer over the existing REST API. It has no access that the caller's API key does not have.

## Motivation / Problem

- Marketing operations staff ask questions such as "why did this address bounce?" or "which lead source is worst this month?". Today they must use the app or write API code.
- AI clients can call tools through MCP. An MCP server makes our data available inside the tools that our buyer already uses.
- Without a server of our own, users paste exports into chats. That is slower and less safe than a scoped tool.

## Goals

- A remote MCP server with the Streamable HTTP transport.
- Read tools for the most common questions, and a few safe write tools.
- The same auth, scopes, quotas, and audit trail as the REST API.
- Phase 2: OAuth, so that clients which need OAuth can connect.

## Non-Goals

- An LLM call inside our server. The AI client's model does the reasoning. We do not pay for model use.
- Tools for bulk jobs, deletes, settings, API keys, or webhooks in v1.
- A local (stdio) MCP package in v1.

## Current State In Reacher

- The public host runs the Next.js app. It sends workspace API-key requests for `/v1/*` to the Rust API.
- The Rust API accepts `Authorization: Bearer rch_live_...` API keys, and Better Auth JWTs from the Next.js server (`backend/src/http/mod.rs`).
- API keys are for the Professional and Enterprise plans only (gap G5 in `docs/strategy/marketing-product-gaps.md`).
- Scopes are in `backend/src/tenant/context.rs`: `verify`, `bulk`, `find`, `lists`, `suppressions`, `reputation`, `settings`, `pipelines.*`, `outcomes.read`, `outcomes.write`, and `admin`.
- The TypeScript SDK `@oppulence/reacher-sdk` is generated from `backend/openapi.json`.
- Segment filters are checked by `validate_segment_filter` (`backend/src/list_intelligence.rs`).
- The web app already uses Better Auth with organizations, and it signs short-lived JWTs for the Rust API.

## Proposed Design

### Hosting

- A Next.js route handler at `web/app/mcp/route.ts` serves `POST` and `GET` for the Streamable HTTP transport.
- It uses the official TypeScript SDK `@modelcontextprotocol/sdk` in stateless mode. Each request stands alone, so any replica can serve it.
- Each tool calls the Rust API through the generated SDK, with the caller's credential. The MCP route never uses a service key.

### Tools (v1)

| Tool | REST call | Scope | Kind |
|---|---|---|---|
| `check_suppression` | `GET /v1/suppressions/check` | `suppressions` | Read |
| `get_email_history` | `GET /v1/emails/{email}/history` | `verify` | Read |
| `list_lists` | `GET /v1/lists` | `lists` | Read |
| `get_list_quality` | `GET /v1/lists/{id}/quality` | `lists` | Read |
| `get_source_quality` | `GET /v1/sources/quality` | None today (see below) | Read |
| `list_alerts` | `GET /v1/alerts` | `verify` | Read |
| `get_usage` | `GET /v1/me/usage` | None | Read |
| `query_results` | `GET /v1/query` | `bulk` | Read |
| `get_provider_breakdown` | RFC 0017 endpoints | `outcomes.read` | Read (after RFC 0017) |
| `verify_email` | `POST /v1/check_email` | `verify` | Write, uses 1 credit |
| `add_suppression` | `POST /v1/suppressions` | `suppressions` | Write |
| `create_segment` | `POST /v1/segments` | `lists` | Write |

- With the Paperless focus (`docs/strategy/company-focus.md`, updated 2026-10-20), phase 1 starts with the Paperless tools. These are `start_research` (`POST /v1/research`), `get_research` (`GET /v1/research/{session_id}`), `export_research` (`POST /v1/research/{session_id}/export`), and `find_email` (`POST /v1/find_email`). They use the `find` scope. Research and finder calls can spend credits, so they are write tools with the same daily cap as `verify_email`.
- `GET /v1/sources/quality` checks no scope today, so any key can read it. Before release, add an `outcomes.read` check to that endpoint, because the report contains feedback data. The MCP tool then needs `outcomes.read`.
- Read tools set the MCP annotation `readOnlyHint: true`. Write tools set `readOnlyHint: false`. MCP clients use these hints to ask the user before a write.
- The `verify_email` description says that it uses one credit.
- Each tool has a JSON input schema and returns structured content plus a short text summary.
- List results return at most 50 rows. The output includes `next_offset` when more rows exist.

### Segments From A Description

- The `create_segment` input schema includes the full filter schema, with field names, operators, and examples.
- The AI client's model turns the user's words into a filter. Our server checks the filter with `validate_segment_filter`, through the REST API.
- When the filter is not valid, the tool returns the error message, so the model can fix the filter and try again.

### Auth

**Phase 1: API key.** The client sends `Authorization: Bearer rch_live_...`. The MCP route passes it to the Rust API. Clients that let the user set a header (for example Claude Code and Claude Desktop) work in phase 1.

**Phase 2: OAuth.** Some clients connect only with OAuth, as the MCP authorization specification describes. In phase 2:

1. The web app acts as the OAuth authorization server. Evaluate the Better Auth MCP plugin for this, and confirm that it works with our Better Auth version.
2. The user signs in, picks a workspace, and approves a list of scopes.
3. The MCP route checks the access token, then signs a short-lived backend JWT for the workspace and the approved scopes. This is the same path that browser sessions use today.
4. A workspace admin can see and revoke MCP grants in Settings.

### Limits

- The tenant throttle and the monthly quota apply, as for any API call.
- A daily cap on paid tool calls (`verify_email`) for each API key or OAuth grant, default 200. An admin can change it in Settings.
- A workspace setting `mcp_enabled` turns the server off for the workspace. The default is on for plans with API access.

### Audit

- Each tool call writes a log line with the tool name, the key prefix or grant ID, the tenant, the duration, and the result status. It never logs tool inputs or outputs.
- Write tools mark their records. For example, `add_suppression` sets `created_by` to `mcp:<key prefix>`, so the suppression event log shows the source.

### Data Safety

- Tool outputs contain the tenant's own data, including email addresses. The setup guide says that the AI client sends this data to its model provider.
- Text fields in our data (for example list names, notes, and comments) can contain words that look like instructions. Tools return these fields inside structured content, with field names, and never as part of the tool description.

## Public API / Interface Changes

| Path | Change |
|---|---|
| `/mcp` | New. MCP Streamable HTTP endpoint. |
| `/.well-known/oauth-protected-resource` | New in phase 2. OAuth metadata for MCP clients. |
| `GET`, `PATCH` `/v1/me/settings` | New fields `mcp_enabled` and `mcp_daily_paid_call_limit`. |

The REST API does not change in phase 1.

## Data Model / Storage Changes

- Phase 1: the daily cap needs a counter. Use a row in a new table `v1_mcp_usage (tenant_id, credential_ref, day, paid_calls)`.
- Phase 2: OAuth clients, grants, and tokens live in the Better Auth schema, managed by its plugin migrations.

## Auth, Permissions, And Tenant Isolation

- The MCP route has no privilege of its own. Each call uses the caller's key or grant, so tenant isolation is the same as for the REST API.
- An OAuth grant has only the scopes that the user approved, and never `admin`.
- A revoked API key or grant stops working on the next request.

## Failure Modes And Edge Cases

| Case | Behavior |
|---|---|
| The key lacks a scope | The tool returns a clear error that names the missing scope. |
| The plan has no API access | The MCP route returns `403` with a link to the plan page. |
| `verify_email` waits longer than 20 seconds | The tool returns the pending status and the operation ID. The model can call `get_email_history` later. |
| The model sends a bad filter | `create_segment` returns the validation message. |
| A client sends many parallel calls | The tenant throttle applies. The tool returns the retry time. |

## Alternatives Considered

1. **A local stdio server package.** Rejected for v1. Remote servers need no install, and our API is remote anyway.
2. **A separate MCP service in Rust.** Rejected. The web app already has the auth, the generated SDK, and the public route.
3. **Run an LLM in our server to build segments.** Rejected. The client already has a model. It would add cost and a second place for mistakes.

## Rollout / Migration / Compatibility

1. Phase 1 behind `MCP_ENABLED` in the web app. Test with Claude Code and Claude Desktop.
2. Publish a setup guide for each supported client.
3. Phase 2: OAuth, then list the server in client directories.

No existing API changes.

## Metrics And Success Criteria

- Count of workspaces with MCP calls each week.
- Tool error rate. Target: under 2%, not counting scope errors.
- Share of `create_segment` calls that pass validation on the first try.

## Test Strategy

- Unit tests for each tool: input schema, REST call mapping, output shape, and the 50-row limit.
- A contract test that each tool's REST call exists in `backend/openapi.json`.
- An end-to-end test with the MCP SDK client against the local stack. It lists the tools, calls each read tool, and calls each write tool with a test key.
- Tests for the scope errors, the daily cap, and `mcp_enabled = false`.
- Phase 2: tests for the OAuth flow, scope approval, and grant revocation.

## Definition Of Done

1. All tools work against the local stack with a test key.
2. The contract test passes. CI fails when a tool calls an endpoint that does not exist.
3. The setup guide covers Claude Code and Claude Desktop.
4. The security review approves the auth path and the write tools.
5. Phase 2 has its own done list when it starts.

## Open Questions

1. Which clients do we support at launch, and in what order? Owner: product owner.
2. What daily cap fits the research and finder tools? One `find_email` call can use up to 13 checks, and one research run can use many. Owner: product owner.

## Unresolved Risks

1. **Spec changes.** MCP is young, and the auth parts change often. Pin the SDK version and check each release.
2. **Wrong writes.** A model can call a write tool with wrong arguments. Write tools are few, need approval in the client, and are logged.
