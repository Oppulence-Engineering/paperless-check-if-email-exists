# Email Quality Decision Engine Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the pasted feature set as an email quality decision engine: recommendations, risk policies, list repair, explainability, closed-loop outcomes, source quality analytics, suppression intelligence, and job failure visibility.

**Architecture:** Build a shared decision/policy foundation first, then make list repair, suppressions, source analytics, outcomes, and observability consume that foundation. Keep all response and schema changes additive, persist decisions at processing time, and use existing result-bearing surfaces instead of creating detached decision-only APIs.

**Tech Stack:** Rust, Warp, SQLx, PostgreSQL JSONB and indexes, RabbitMQ worker flow, CSV/NDJSON streaming, existing backend integration tests in `backend/tests`.

---

## Scope And Execution Order

The pasted text covers eight capabilities. Implement them in this order because later features need stable decision metadata:

1. Decision Layer
2. Risk Policies By Use Case
3. Explainable Results
4. Automatic List Repair
5. Suppression Intelligence
6. Job Failure Center
7. Source Quality Intelligence
8. Closed-Loop Learning

The first three ship together as the foundation. The remaining items are separate phases that can be released independently.

## Current-State Anchors

- `backend/src/scoring/response.rs` is the main hook for adding recommendation and policy output because it already computes score, freshness, canonical email, and bounce risk.
- `backend/src/storage/postgres.rs` is the main persistence hook because prepared responses already flow into `v1_task_result` with denormalized score and bounce-risk columns.
- `backend/src/http/csv_shared.rs`, `backend/src/http/v1/jobs/download.rs`, and `backend/src/http/v1/lists/download.rs` are the shared CSV/NDJSON projection surfaces.
- `backend/src/http/v1/query.rs`, `backend/src/http/v1/email_history.rs`, and `backend/src/http/v1/jobs/get_results.rs` are the read APIs that must expose persisted decisions.
- `backend/src/http/v1/lists/post.rs` already preserves original CSV rows and dedupe metadata, which makes remediation plans feasible without mutating source data.
- `backend/src/http/v1/suppressions/*` already provides tenant-scoped suppression CRUD and should be evolved in place.
- `backend/src/http/v1/jobs/*` already exposes progress, events, retry, latency, and downloads, which makes observability a read-model expansion.

## File Structure

### New Modules

- Create `backend/src/decision/mod.rs`
  - Exports decision, policy, explanation, and shared source/outcome enums.
- Create `backend/src/decision/types.rs`
  - Defines `Recommendation`, `RecommendedAction`, `PolicyMode`, `PolicyEvaluation`, `DecisionReason`, `DecisionSeverity`, and `DecisionConfidence`.
- Create `backend/src/decision/engine.rs`
  - Computes first-class recommendations from score, freshness, bounce risk, suppression state, and repair hints.
- Create `backend/src/decision/policy.rs`
  - Evaluates `growth`, `deliverability`, `signup_protection`, `enterprise_strict`, and `custom` policies.
- Create `backend/src/decision/explain.rs`
  - Converts raw signals into stable, customer-facing reason codes and compact evidence.
- Create `backend/src/decision/custom_policy.rs`
  - Validates and evaluates tenant custom policy profiles using a constrained JSON rule schema.
- Create `backend/src/http/v1/lists/remediation.rs`
  - Adds list remediation plan and export endpoints.
- Create `backend/src/http/v1/analytics/mod.rs`
  - Registers source quality analytics endpoints.
- Create `backend/src/http/v1/analytics/sources.rs`
  - Implements source catalog and source rollup reads.
- Create `backend/src/http/v1/provider_endpoints.rs`
  - Manages inbound ESP provider endpoints for outcome ingestion.
- Create `backend/src/http/v1/inbound/mod.rs`
  - Registers inbound provider webhook routes.
- Create `backend/src/http/v1/inbound/providers.rs`
  - Routes provider-specific inbound payloads to adapters.
- Create `backend/src/outcomes/mod.rs`
  - Normalized outcome model, correlation, and ingestion service.
- Create `backend/src/outcomes/adapters/sendgrid.rs`
  - SendGrid signature verification, parsing, and normalization.
- Create `backend/src/outcomes/adapters/mailgun.rs`
  - Mailgun signature verification, parsing, and normalization.
- Create `backend/src/outcomes/adapters/ses.rs`
  - SES/SNS signature verification, parsing, and normalization.
- Create `backend/src/outcomes/adapters/postmark.rs`
  - Postmark token-based verification, parsing, and normalization.
- Create `backend/src/source_quality/mod.rs`
  - Source attribution validation, catalog upsert, and rollup computation.
- Create `backend/src/job_observability/mod.rs`
  - Job summary, failure rows, attempt logging helpers, and download projections.

### Modified Modules

- Modify `backend/src/lib.rs`
  - Add `pub mod decision;`, `pub mod outcomes;`, `pub mod source_quality;`, and `pub mod job_observability;`.
- Modify `backend/src/http/v1/mod.rs`
  - Add `analytics`, `provider_endpoints`, and `inbound` modules.
- Modify `backend/src/http/routes.rs`
  - Register new remediation, analytics, provider endpoint, inbound, and job observability routes.
- Modify `backend/src/http/v0/check_email/post.rs`
  - Add optional `policy_mode` and `policy_profile_key` to the shared `CheckEmailRequest`.
- Modify `backend/src/http/v1/bulk/post.rs`
  - Accept source and policy fields, persist effective source and policy on jobs and tasks.
- Modify `backend/src/http/v1/lists/post.rs`
  - Accept source and policy form fields, persist effective source and policy on lists and tasks.
- Modify `backend/src/http/v1/pipelines.rs`
  - Add source and policy mode to pipeline create/update and scheduled task metadata.
- Modify `backend/src/scoring/response.rs`
  - Attach `recommendation` and `policy_evaluation` to prepared verification responses.
- Modify `backend/src/storage/postgres.rs`
  - Persist recommendation, policy evaluation, policy mode, policy profile, and source key in `v1_task_result`.
- Modify `backend/src/http/csv_shared.rs`
  - Add recommendation, policy, remediation, and failure projection columns.
- Modify `backend/src/http/v1/jobs/get_results.rs`, `backend/src/http/v1/jobs/download.rs`, `backend/src/http/v1/lists/download.rs`, `backend/src/http/v1/query.rs`, and `backend/src/http/v1/email_history.rs`
  - Read and return persisted recommendation and policy data.
- Modify `backend/src/worker/do_work.rs`
  - Propagate decision, policy, source, and observability fields to duplicate list rows and attempt logs.
- Modify `backend/src/worker/actions.rs`
  - Use policy decisions and richer suppression metadata for auto-suppression.
- Modify `backend/src/http/v1/suppressions/add.rs`, `check.rs`, `list.rs`, `delete.rs`
  - Expand suppression fields and preserve backward compatibility.
- Modify `backend/src/http/openapi.rs`
  - Add schemas and paths for all new fields and endpoints.

### New Migrations

- Create `backend/migrations/20260630000001_decision_policy_foundation.up.sql`
- Create `backend/migrations/20260630000001_decision_policy_foundation.down.sql`
- Create `backend/migrations/20260630000002_remediation_suppression_intelligence.up.sql`
- Create `backend/migrations/20260630000002_remediation_suppression_intelligence.down.sql`
- Create `backend/migrations/20260630000003_source_outcome_observability.up.sql`
- Create `backend/migrations/20260630000003_source_outcome_observability.down.sql`

## Target Database Shape

### Decision And Policy Foundation

`20260630000001_decision_policy_foundation.up.sql`:

```sql
ALTER TABLE tenants
    ADD COLUMN default_policy_mode TEXT NOT NULL DEFAULT 'deliverability';

CREATE TABLE tenant_policy_profiles (
    id SERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    profile_key TEXT NOT NULL,
    display_name TEXT NOT NULL,
    rules JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, profile_key)
);

ALTER TABLE v1_bulk_job
    ADD COLUMN policy_mode TEXT,
    ADD COLUMN policy_profile_key TEXT,
    ADD COLUMN source_key TEXT;

ALTER TABLE v1_lists
    ADD COLUMN policy_mode TEXT,
    ADD COLUMN policy_profile_key TEXT,
    ADD COLUMN source_key TEXT;

ALTER TABLE v1_task_result
    ADD COLUMN recommendation JSONB,
    ADD COLUMN recommendation_action TEXT,
    ADD COLUMN recommendation_confidence TEXT,
    ADD COLUMN recommendation_priority TEXT,
    ADD COLUMN policy_mode TEXT,
    ADD COLUMN policy_profile_key TEXT,
    ADD COLUMN policy_evaluation JSONB,
    ADD COLUMN policy_decision TEXT,
    ADD COLUMN policy_evaluated_at TIMESTAMPTZ,
    ADD COLUMN source_key TEXT;

CREATE INDEX idx_v1_task_result_recommendation_action
    ON v1_task_result (tenant_id, recommendation_action)
    WHERE recommendation_action IS NOT NULL;

CREATE INDEX idx_v1_task_result_policy_decision
    ON v1_task_result (tenant_id, policy_decision)
    WHERE policy_decision IS NOT NULL;

CREATE INDEX idx_v1_task_result_source_key
    ON v1_task_result (tenant_id, source_key)
    WHERE source_key IS NOT NULL;
```

`down.sql` must drop the three indexes, drop the added columns, drop `tenant_policy_profiles`, and drop `tenants.default_policy_mode`.

### Remediation And Suppression Intelligence

`20260630000002_remediation_suppression_intelligence.up.sql`:

```sql
ALTER TABLE v1_suppression_entries
    ADD COLUMN canonical_email TEXT,
    ADD COLUMN status TEXT NOT NULL DEFAULT 'active',
    ADD COLUMN reason_code TEXT,
    ADD COLUMN reason_detail TEXT,
    ADD COLUMN source_type TEXT,
    ADD COLUMN source_ref TEXT,
    ADD COLUMN created_by TEXT,
    ADD COLUMN expires_at TIMESTAMPTZ,
    ADD COLUMN last_seen_at TIMESTAMPTZ,
    ADD COLUMN metadata JSONB NOT NULL DEFAULT '{}'::jsonb;

UPDATE v1_suppression_entries
SET canonical_email = lower(trim(email)),
    reason_code = COALESCE(reason::TEXT, 'manual'),
    source_type = COALESCE(source, 'manual'),
    last_seen_at = created_at
WHERE canonical_email IS NULL;

ALTER TABLE v1_suppression_entries
    ALTER COLUMN canonical_email SET NOT NULL;

CREATE UNIQUE INDEX idx_v1_suppression_active_canonical
    ON v1_suppression_entries (tenant_id, canonical_email)
    WHERE status = 'active';

CREATE TABLE v1_suppression_events (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    entry_id INTEGER REFERENCES v1_suppression_entries(id) ON DELETE SET NULL,
    canonical_email TEXT NOT NULL,
    event_type TEXT NOT NULL,
    from_status TEXT,
    to_status TEXT,
    reason_code TEXT,
    reason_detail TEXT,
    source_type TEXT,
    source_ref TEXT,
    actor_type TEXT NOT NULL DEFAULT 'api',
    actor_id TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_v1_suppression_events_entry
    ON v1_suppression_events (tenant_id, entry_id, created_at DESC);

CREATE TABLE v1_remediation_plans (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    list_id INTEGER NOT NULL REFERENCES v1_lists(id) ON DELETE CASCADE,
    job_id INTEGER REFERENCES v1_bulk_job(id) ON DELETE SET NULL,
    status TEXT NOT NULL DEFAULT 'completed',
    rule_version TEXT NOT NULL,
    options JSONB NOT NULL,
    result_state_digest TEXT NOT NULL,
    summary_counts JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, list_id, rule_version, result_state_digest, options)
);

CREATE TABLE v1_remediation_rows (
    id BIGSERIAL PRIMARY KEY,
    plan_id BIGINT NOT NULL REFERENCES v1_remediation_plans(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    list_id INTEGER NOT NULL REFERENCES v1_lists(id) ON DELETE CASCADE,
    task_result_id INTEGER REFERENCES v1_task_result(id) ON DELETE SET NULL,
    row_index INTEGER NOT NULL,
    classification TEXT NOT NULL,
    rule_id TEXT NOT NULL,
    confidence TEXT NOT NULL,
    original_email TEXT NOT NULL,
    effective_email TEXT NOT NULL,
    before JSONB NOT NULL,
    after JSONB NOT NULL,
    reasons JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (plan_id, row_index)
);

CREATE INDEX idx_v1_remediation_rows_partition
    ON v1_remediation_rows (tenant_id, plan_id, classification, row_index);

CREATE TABLE v1_remediation_exports (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    plan_id BIGINT NOT NULL REFERENCES v1_remediation_plans(id) ON DELETE CASCADE,
    partitions TEXT[] NOT NULL,
    format TEXT NOT NULL DEFAULT 'csv',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Source, Outcome, And Observability

`20260630000003_source_outcome_observability.up.sql`:

```sql
CREATE TABLE v1_sources (
    id SERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    source_key TEXT NOT NULL,
    source_display_name TEXT NOT NULL,
    source_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, source_key)
);

CREATE TABLE v1_source_quality_daily (
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    source_key TEXT NOT NULL,
    bucket_date DATE NOT NULL,
    total_records INTEGER NOT NULL DEFAULT 0,
    avg_score DOUBLE PRECISION,
    valid_count INTEGER NOT NULL DEFAULT 0,
    risky_count INTEGER NOT NULL DEFAULT 0,
    unknown_count INTEGER NOT NULL DEFAULT 0,
    invalid_count INTEGER NOT NULL DEFAULT 0,
    safe_to_send_count INTEGER NOT NULL DEFAULT 0,
    suppressed_count INTEGER NOT NULL DEFAULT 0,
    outcome_delivered_count INTEGER,
    outcome_bounce_count INTEGER,
    computed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (tenant_id, source_key, bucket_date)
);

CREATE TABLE provider_endpoints (
    id TEXT PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    label TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    delivery_token_hash TEXT NOT NULL,
    provider_config JSONB NOT NULL DEFAULT '{"version":1,"settings":{}}'::jsonb,
    allowed_ips TEXT[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_provider_endpoints_tenant_provider
    ON provider_endpoints (tenant_id, provider, status);

CREATE TABLE delivery_outcome_receipts (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    endpoint_id TEXT REFERENCES provider_endpoints(id) ON DELETE SET NULL,
    provider TEXT NOT NULL,
    request_headers JSONB NOT NULL DEFAULT '{}'::jsonb,
    raw_payload BYTEA NOT NULL,
    payload_sha256 TEXT NOT NULL,
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    validation_status TEXT NOT NULL,
    validation_error TEXT,
    normalized_count INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE delivery_outcomes (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    endpoint_id TEXT REFERENCES provider_endpoints(id) ON DELETE SET NULL,
    provider TEXT NOT NULL,
    provider_event_id TEXT NOT NULL,
    provider_message_id TEXT,
    event_type TEXT NOT NULL,
    event_family TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    recipient_email TEXT NOT NULL,
    canonical_email TEXT NOT NULL,
    job_id INTEGER REFERENCES v1_bulk_job(id) ON DELETE SET NULL,
    list_id INTEGER REFERENCES v1_lists(id) ON DELETE SET NULL,
    source_key TEXT,
    correlation_status TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, provider, provider_event_id)
);

CREATE INDEX idx_delivery_outcomes_email
    ON delivery_outcomes (tenant_id, canonical_email, occurred_at DESC);
CREATE INDEX idx_delivery_outcomes_source
    ON delivery_outcomes (tenant_id, source_key, occurred_at DESC)
    WHERE source_key IS NOT NULL;

CREATE TABLE v1_task_attempts (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    job_id INTEGER REFERENCES v1_bulk_job(id) ON DELETE CASCADE,
    task_id INTEGER REFERENCES v1_task_result(id) ON DELETE CASCADE,
    attempt_number INTEGER NOT NULL,
    state TEXT NOT NULL,
    queued_at TIMESTAMPTZ,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    failure_code TEXT,
    failure_message TEXT,
    retryable BOOLEAN NOT NULL DEFAULT false,
    worker_id TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (task_id, attempt_number)
);

CREATE INDEX idx_v1_task_attempts_job_state
    ON v1_task_attempts (tenant_id, job_id, state, completed_at DESC);

ALTER TABLE job_events
    ADD COLUMN tenant_id UUID REFERENCES tenants(id),
    ADD COLUMN normalized_event_type TEXT;

CREATE INDEX idx_job_events_tenant_job_event
    ON job_events (tenant_id, job_id, normalized_event_type, created_at DESC);
```

## Phase 1: Decision Layer, Policies, And Explainability

### Task 1: Add Decision Types And Unit Tests

**Files:**
- Create: `backend/src/decision/types.rs`
- Create: `backend/src/decision/mod.rs`
- Modify: `backend/src/lib.rs`
- Test: `cargo test -p reacher_backend decision::types`

- [ ] Create enums with exact serialized values:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendedAction {
    Send,
    SendWithCaution,
    Review,
    Suppress,
    Drop,
    FixThenSend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyMode {
    Growth,
    Deliverability,
    SignupProtection,
    EnterpriseStrict,
    Custom,
}
```

- [ ] Add `Recommendation`, `PolicyEvaluation`, and `DecisionReason` structs with fields: `action`, `policy_mode`, `policy_profile_key`, `confidence`, `priority`, `summary`, `reasons`, `suggested_email`, `engine_version`, and `evaluated_at`.
- [ ] Add serde round-trip tests proving the actions serialize as `send`, `send_with_caution`, `review`, `suppress`, `drop`, and `fix_then_send`.
- [ ] Run `cargo test -p reacher_backend decision::types`.
- [ ] Commit with `git commit -m "feat: add decision type model"`.

### Task 2: Implement Explainable Reason Extraction

**Files:**
- Create: `backend/src/decision/explain.rs`
- Modify: `backend/src/decision/mod.rs`
- Test: `cargo test -p reacher_backend decision::explain`

- [ ] Implement reason codes for the pasted examples:
  - `catch_all_corporate_domain`
  - `possible_domain_typo`
  - `stale_verification`
  - `role_account`
  - `disposable_provider`
  - `weak_mail_infrastructure`
  - `previously_bounced_for_tenant`
- [ ] Map existing score and result signals to these codes:
  - catch-all: `score.signals.smtp_is_catch_all`
  - typo: `score.domain_suggestion` or `output.syntax.suggestion`
  - stale: freshness age greater than policy threshold
  - role: `score.signals.is_role_account`
  - disposable: `score.signals.is_disposable`
  - weak infrastructure: no MX, SMTP unreachable, SMTP error, or bounce-risk high
  - previous bounce: delivery outcome `bounce_hard` for the same tenant and canonical email
- [ ] Include compact evidence only from scalar values: score, category, boolean flags, age days, bounce-risk category, outcome event type, and suggested email.
- [ ] Add tests for each reason code using hand-built `EmailScore` values and JSON evidence assertions.
- [ ] Run `cargo test -p reacher_backend decision::explain`.
- [ ] Commit with `git commit -m "feat: add explainable decision reasons"`.

### Task 3: Implement Recommendation Engine

**Files:**
- Create: `backend/src/decision/engine.rs`
- Modify: `backend/src/decision/mod.rs`
- Modify: `backend/src/scoring/response.rs`
- Test: `cargo test -p reacher_backend decision::engine scoring::response`

- [ ] Define `DecisionInput` with score, completed_at, canonical_email, domain suggestion, bounce risk, suppression state, prior outcome summary, and selected policy.
- [ ] Implement action precedence:
  - hard invalid or active suppression -> `drop`
  - domain typo with safe corrected address -> `fix_then_send`
  - spam trap, disposable under strict policies, or hard negative prior outcome -> `suppress`
  - risky, unknown, stale, role, catch-all, weak infrastructure -> `review`
  - safe with medium bounce risk -> `send_with_caution`
  - clean safe result -> `send`
- [ ] Attach ordered reasons and a human-readable summary string.
- [ ] Set `engine_version` to `decision_v1`.
- [ ] Add tests for each action.
- [ ] Modify `PreparedVerificationResponse` to include `recommendation: Option<Recommendation>` and `policy_evaluation: Option<PolicyEvaluation>`.
- [ ] Inject `recommendation` and `policy_evaluation` into the response JSON before `body` is serialized.
- [ ] Run `cargo test -p reacher_backend decision::engine scoring::response`.
- [ ] Commit with `git commit -m "feat: compute email recommendations"`.

### Task 4: Implement Preset And Custom Policy Evaluation

**Files:**
- Create: `backend/src/decision/policy.rs`
- Create: `backend/src/decision/custom_policy.rs`
- Modify: `backend/src/http/v0/check_email/post.rs`
- Modify: `backend/src/http/v1/tenant_settings.rs`
- Test: `cargo test -p reacher_backend decision::policy decision::custom_policy`

- [ ] Add request fields to `CheckEmailRequest`:

```rust
pub policy_mode: Option<crate::decision::types::PolicyMode>,
pub policy_profile_key: Option<String>,
```

- [ ] Implement preset policy semantics:
  - `growth`: send valid and most risky rows unless disposable, spam trap, active suppression, hard invalid, or dangerous bounce risk.
  - `deliverability`: send only safe rows with bounce risk below high and verification age not older than 30 days.
  - `signup_protection`: drop invalid, suppress disposable/spam-trap/high-risk domains, review catch-all and unknown SMTP.
  - `enterprise_strict`: send only safe rows with no catch-all, no role account, no disposable, no suppression, bounce risk safe or low, and verification age not older than 14 days.
  - `custom`: load `tenant_policy_profiles.rules` and evaluate constrained JSON rules.
- [ ] Use `deliverability` as the system default when no tenant or request mode is present.
- [ ] Add `default_policy_mode` to settings GET/PATCH responses.
- [ ] Reject unsupported mode values with `400`.
- [ ] Reject `policy_mode=custom` without an active `policy_profile_key`.
- [ ] Add tests proving the same score can produce different decisions across all four preset modes.
- [ ] Run `cargo test -p reacher_backend decision::policy decision::custom_policy`.
- [ ] Commit with `git commit -m "feat: add risk policy modes"`.

### Task 5: Persist Decisions And Read Them Back

**Files:**
- Add migrations: `20260630000001_decision_policy_foundation.*.sql`
- Modify: `backend/src/storage/postgres.rs`
- Modify: `backend/src/worker/do_work.rs`
- Modify: `backend/src/http/v1/jobs/retry.rs`
- Modify: `backend/src/http/v1/jobs/get_results.rs`
- Modify: `backend/src/http/v1/jobs/download.rs`
- Modify: `backend/src/http/v1/query.rs`
- Modify: `backend/src/http/v1/email_history.rs`
- Modify: `backend/src/http/csv_shared.rs`
- Test: `cargo test -p reacher_backend storage:: tenant:: http::csv_shared`

- [ ] Add the decision/policy migration exactly as defined above.
- [ ] Extend `SuccessColumns` in `storage/postgres.rs` with recommendation and policy fields.
- [ ] Store `recommendation`, `recommendation_action`, `recommendation_confidence`, `recommendation_priority`, `policy_mode`, `policy_profile_key`, `policy_evaluation`, `policy_decision`, and `policy_evaluated_at`.
- [ ] Clear the decision and policy columns in `store_error` and `jobs/retry`.
- [ ] Propagate decision and policy columns in duplicate-row sync inside `worker/do_work.rs`.
- [ ] Extend `TaskResultRecord` and CSV/NDJSON projections with:
  - `recommended_action`
  - `recommendation_confidence`
  - `recommendation_priority`
  - `recommendation_reasons`
  - `policy_mode`
  - `policy_decision`
  - `policy_reasons`
- [ ] Extend query filters with optional `recommendation_action`, `policy_mode`, and `policy_decision`.
- [ ] Add read tests proving decisions survive storage and are returned by job results, history, query, and downloads.
- [ ] Run `cargo test -p reacher_backend storage:: http::csv_shared`.
- [ ] Commit with `git commit -m "feat: persist decision and policy metadata"`.

## Phase 2: Automatic List Repair

### Task 6: Add Remediation Plan Tables And Classifier

**Files:**
- Add migrations: `20260630000002_remediation_suppression_intelligence.*.sql`
- Create: `backend/src/http/v1/lists/remediation.rs`
- Modify: `backend/src/http/v1/lists/mod.rs`
- Modify: `backend/src/http/routes.rs`
- Test: `cargo test -p reacher_backend lists::remediation`

- [ ] Implement deterministic remediation classes:
  - `fixed`
  - `safe`
  - `review`
  - `drop`
- [ ] Implement automatic fixes:
  - trim leading/trailing whitespace
  - lowercase domain portion
  - strip zero-width and control characters
  - apply one unambiguous domain typo suggestion
  - use canonical normalized email when it only changes casing, Gmail dots, Gmail plus tag, or `googlemail.com` alias
- [ ] Classify `fix_then_send` recommendations as `fixed` when the effective email differs from the original.
- [ ] Classify `send` as `safe`.
- [ ] Classify `send_with_caution` and `review` as `review`.
- [ ] Classify `drop` and `suppress` as `drop`.
- [ ] Store one immutable row per original CSV row in `v1_remediation_rows`.
- [ ] Add `POST /v1/lists/{list_id}/remediation-plan` and `GET /v1/lists/{list_id}/remediation-plan`.
- [ ] Reject plan creation for lists that are not completed unless `allow_partial=true`.
- [ ] Add tests for typo repair, normalization repair, duplicate review, suppressed drop, and completed-list enforcement.
- [ ] Run `cargo test -p reacher_backend lists::remediation`.
- [ ] Commit with `git commit -m "feat: add list remediation plans"`.

### Task 7: Add Remediation Exports

**Files:**
- Modify: `backend/src/http/v1/lists/remediation.rs`
- Modify: `backend/src/http/openapi.rs`
- Test: `cargo test -p reacher_backend lists::remediation`

- [ ] Add `POST /v1/lists/{list_id}/remediation-exports`.
- [ ] Add `GET /v1/lists/{list_id}/remediation-exports/{export_id}/download`.
- [ ] Support CSV partitions:
  - `fixed`
  - `safe`
  - `review`
  - `drop`
  - `combined_clean`
- [ ] Append provenance columns:
  - `_reacher_classification`
  - `_reacher_rule_id`
  - `_reacher_confidence`
  - `_reacher_original_email`
  - `_reacher_effective_email`
  - `_reacher_reasons`
- [ ] Reject unsupported partitions with `400`.
- [ ] Stream large exports in batches of 500 rows.
- [ ] Add tests for each partition and for combined clean including only fixed plus safe rows.
- [ ] Run `cargo test -p reacher_backend lists::remediation`.
- [ ] Commit with `git commit -m "feat: export remediated list partitions"`.

## Phase 3: Suppression Intelligence

### Task 8: Upgrade Suppression Storage And API Fields

**Files:**
- Modify migration: `20260630000002_remediation_suppression_intelligence.up.sql`
- Modify: `backend/src/http/v1/suppressions/add.rs`
- Modify: `backend/src/http/v1/suppressions/list.rs`
- Modify: `backend/src/http/v1/suppressions/check.rs`
- Modify: `backend/src/http/v1/suppressions/delete.rs`
- Test: `cargo test -p reacher_backend suppressions`

- [ ] Keep legacy request compatibility: `emails`, `reason`, `source`, and `notes` continue to work.
- [ ] Add fields: `canonical_email`, `status`, `reason_code`, `reason_detail`, `source_type`, `source_ref`, `created_by`, `expires_at`, `last_seen_at`, and `metadata`.
- [ ] Treat only `status='active'` and unexpired rows as blocking.
- [ ] Make DELETE set `status='revoked'` instead of physically deleting.
- [ ] Write one `v1_suppression_events` row for create, refresh, revoke, import, and expiry transitions.
- [ ] Enforce same-tenant lookup for every suppression operation.
- [ ] Add tests for manual precedence, automatic refresh, expired non-blocking rows, revoke behavior, and legacy response compatibility.
- [ ] Run `cargo test -p reacher_backend suppressions`.
- [ ] Commit with `git commit -m "feat: add suppression intelligence"`.

### Task 9: Add Suppression Import, Export, And Policy Auto-Suppression

**Files:**
- Create: `backend/src/http/v1/suppressions/import.rs`
- Create: `backend/src/http/v1/suppressions/export.rs`
- Modify: `backend/src/http/v1/suppressions/mod.rs`
- Modify: `backend/src/http/routes.rs`
- Modify: `backend/src/worker/actions.rs`
- Test: `cargo test -p reacher_backend suppressions worker::actions`

- [ ] Add `POST /v1/suppressions/import` with a hard limit of 10,000 rows.
- [ ] Add `GET /v1/suppressions/export?format=csv|ndjson`.
- [ ] Add `GET /v1/suppressions/{id}/events`.
- [ ] Store import batch ID in event metadata.
- [ ] Make worker auto-suppression use policy decisions:
  - `suppress` creates or refreshes an automatic suppression.
  - `drop` does not create durable suppression unless tenant settings explicitly enable drop-to-suppression.
- [ ] Add tests for import dedupe, invalid email rejects, event history, CSV export, NDJSON export, and policy-created suppressions.
- [ ] Run `cargo test -p reacher_backend suppressions worker::actions`.
- [ ] Commit with `git commit -m "feat: add suppression import export and policy actions"`.

## Phase 4: Job Failure Center

### Task 10: Add Attempt Logging And Observability Summary

**Files:**
- Add migration: `20260630000003_source_outcome_observability.*.sql`
- Create: `backend/src/job_observability/mod.rs`
- Modify: `backend/src/worker/do_work.rs`
- Modify: `backend/src/worker/consume.rs`
- Create: `backend/src/http/v1/jobs/observability.rs`
- Modify: `backend/src/http/v1/jobs/mod.rs`
- Modify: `backend/src/http/routes.rs`
- Test: `cargo test -p reacher_backend job_observability worker::do_work`

- [ ] Add best-effort attempt logging for queued, started, completed, retried, failed, cancelled, and dead-lettered tasks.
- [ ] Populate `job_events.tenant_id` and `job_events.normalized_event_type` for new events.
- [ ] Add `GET /v1/jobs/{job_id}/observability`.
- [ ] Return job status, timestamps, task counts, retry counts, dead-letter counts, tasks per minute, ETA when enough data exists, and last failure.
- [ ] Reconcile counts from `v1_task_result` even when attempt rows are missing.
- [ ] Add tests for a job with no attempts, a completed job, a retrying job, and a cancelled job.
- [ ] Run `cargo test -p reacher_backend job_observability`.
- [ ] Commit with `git commit -m "feat: add job observability summary"`.

### Task 11: Add Failure Rows And Downloads

**Files:**
- Create: `backend/src/http/v1/jobs/failures.rs`
- Create: `backend/src/http/v1/jobs/failure_download.rs`
- Modify: `backend/src/http/v1/jobs/mod.rs`
- Modify: `backend/src/http/routes.rs`
- Test: `cargo test -p reacher_backend job_observability`

- [ ] Add `GET /v1/jobs/{job_id}/failures`.
- [ ] Add `GET /v1/jobs/{job_id}/failures/download`.
- [ ] Include original input, canonical email, task state, retry count, retryable, failure code, failure message, and last attempt timestamp.
- [ ] Support `format=csv` and `format=ndjson`.
- [ ] Support `retryable_only=true` and `task_state=failed|dead_lettered|cancelled`.
- [ ] Ensure tasks that later succeed disappear from `/failures`.
- [ ] Add streaming tests for CSV and NDJSON output.
- [ ] Run `cargo test -p reacher_backend job_observability`.
- [ ] Commit with `git commit -m "feat: add job failure center"`.

## Phase 5: Source Quality Intelligence

### Task 12: Add Source Attribution To Writes

**Files:**
- Create: `backend/src/source_quality/mod.rs`
- Modify migration: `20260630000003_source_outcome_observability.up.sql`
- Modify: `backend/src/http/v1/bulk/post.rs`
- Modify: `backend/src/http/v1/lists/post.rs`
- Modify: `backend/src/http/v1/pipelines.rs`
- Modify: `backend/src/storage/postgres.rs`
- Test: `cargo test -p reacher_backend source_quality`

- [ ] Validate `source_key` as lowercase ASCII letters, digits, and hyphen, length 1 through 64, not starting or ending with hyphen.
- [ ] Support `source_type` values:
  - `apollo_import`
  - `hubspot_list`
  - `salesforce_campaign`
  - `signup_form`
  - `csv_vendor`
  - `enrichment_tool`
  - `manual_upload`
  - `api`
  - `unknown`
- [ ] Upsert `v1_sources` when source fields are supplied.
- [ ] Default missing source to `unknown`.
- [ ] Reject display name without source key with `400`.
- [ ] Reject existing source key reused with different source type with `409`.
- [ ] Persist effective source key to jobs, lists, pipelines, and task results.
- [ ] Add tests for validation, unknown fallback, conflicting type, and tenant isolation.
- [ ] Run `cargo test -p reacher_backend source_quality`.
- [ ] Commit with `git commit -m "feat: add source attribution"`.

### Task 13: Add Source Rollups And Compare API

**Files:**
- Create: `backend/src/http/v1/analytics/sources.rs`
- Create: `backend/src/http/v1/analytics/mod.rs`
- Modify: `backend/src/http/v1/mod.rs`
- Modify: `backend/src/http/routes.rs`
- Test: `cargo test -p reacher_backend source_quality analytics`

- [ ] Add `GET /v1/analytics/sources`.
- [ ] Add `GET /v1/analytics/sources/{source_key}`.
- [ ] Add `GET /v1/analytics/sources/compare?source_key=a&source_key=b`.
- [ ] Compute daily rollups from `v1_task_result` and `delivery_outcomes`.
- [ ] Return metrics:
  - total records
  - average score
  - valid, risky, unknown, invalid percentages
  - safe-to-send percentage
  - suppressed percentage
  - delivered percentage when outcomes exist
  - bounce percentage when outcomes exist
- [ ] Make outcome metrics `null` when no outcome data exists.
- [ ] Limit compare endpoint to 2 through 5 sources.
- [ ] Add tests for daily, weekly, monthly grouping and compare deltas.
- [ ] Run `cargo test -p reacher_backend source_quality analytics`.
- [ ] Commit with `git commit -m "feat: add source quality analytics"`.

## Phase 6: Closed-Loop Learning

### Task 14: Add Provider Endpoint Management

**Files:**
- Create: `backend/src/http/v1/provider_endpoints.rs`
- Create: `backend/src/outcomes/mod.rs`
- Modify: `backend/src/http/v1/mod.rs`
- Modify: `backend/src/http/routes.rs`
- Test: `cargo test -p reacher_backend outcomes provider_endpoints`

- [ ] Add provider endpoint CRUD:
  - `GET /v1/provider-endpoints`
  - `POST /v1/provider-endpoints`
  - `PATCH /v1/provider-endpoints/{endpoint_id}`
  - `DELETE /v1/provider-endpoints/{endpoint_id}`
- [ ] Generate opaque endpoint IDs and delivery tokens.
- [ ] Store only token hashes.
- [ ] Return webhook URLs in create responses.
- [ ] Support providers: `sendgrid`, `ses`, `mailgun`, `postmark`.
- [ ] Require the existing `settings` scope.
- [ ] Add tests for create, list, patch pause, delete disable, bad token hash behavior, and tenant isolation.
- [ ] Run `cargo test -p reacher_backend outcomes provider_endpoints`.
- [ ] Commit with `git commit -m "feat: add provider outcome endpoints"`.

### Task 15: Add Outcome Ingestion And Correlation

**Files:**
- Create: `backend/src/http/v1/inbound/mod.rs`
- Create: `backend/src/http/v1/inbound/providers.rs`
- Create: `backend/src/outcomes/adapters/sendgrid.rs`
- Create: `backend/src/outcomes/adapters/mailgun.rs`
- Create: `backend/src/outcomes/adapters/ses.rs`
- Create: `backend/src/outcomes/adapters/postmark.rs`
- Modify: `backend/src/http/routes.rs`
- Modify: `backend/src/worker/actions.rs`
- Test: `cargo test -p reacher_backend outcomes`

- [ ] Add `POST /v1/inbound/providers/{provider}/{endpoint_id}/{delivery_token}`.
- [ ] Persist `delivery_outcome_receipts` before normalization.
- [ ] Normalize events:
  - `bounced` -> `bounce_hard` or `bounce_soft`
  - `delivered` -> `delivered`
  - `opened` -> `open`
  - `clicked` -> `click`
  - `complained` -> `complaint`
  - `unsubscribed` -> `unsubscribe`
- [ ] Deduplicate by `(tenant_id, provider, provider_event_id)`.
- [ ] Correlate by explicit job/list/source metadata first, then by most recent canonical email verification in the previous 30 days.
- [ ] Feed hard bounce, complaint, and unsubscribe outcomes into suppression intelligence with `source_type='provider_event'`.
- [ ] Make decision explanations include `previously_bounced_for_tenant` after outcomes exist.
- [ ] Add golden adapter tests for each provider, duplicate replay tests, unmatched outcome tests, and suppression side-effect tests.
- [ ] Run `cargo test -p reacher_backend outcomes`.
- [ ] Commit with `git commit -m "feat: ingest delivery outcomes"`.

## Phase 7: Public Contract, SDKs, And Docs

### Task 16: Update OpenAPI And SDK Surfaces

**Files:**
- Modify: `backend/src/http/openapi.rs`
- Modify: `backend/openapi.json`
- Modify: `sdks/typescript/*`
- Test: `cargo run -p reacher_backend --bin generate_openapi`

- [ ] Add schemas for recommendation, policy evaluation, remediation plans, suppression entries/events, source analytics, provider endpoints, outcomes, observability, and failure rows.
- [ ] Ensure route registry and OpenAPI contain the same new endpoints.
- [ ] Regenerate `backend/openapi.json`.
- [ ] Regenerate the TypeScript SDK using the repo's existing SDK generation flow.
- [ ] Run `cargo run -p reacher_backend --bin generate_openapi`.
- [ ] Run the SDK generation command from `Makefile` if available in the current branch.
- [ ] Commit with `git commit -m "docs: update openapi and sdk contracts"`.

### Task 17: Update Customer-Facing Docs

**Files:**
- Modify: `docs/getting-started/service-capabilities.md`
- Modify: `docs/advanced/customer-feature-backlog.md`
- Add: `docs/advanced/decision-policies.md`
- Add: `docs/advanced/list-remediation.md`
- Add: `docs/advanced/outcome-ingestion.md`
- Add: `docs/advanced/source-quality.md`
- Add: `docs/advanced/job-observability.md`

- [ ] Document recommendation actions and when each appears.
- [ ] Document policy modes and custom policy rule schema.
- [ ] Document remediation plan lifecycle and export partitions.
- [ ] Document suppression status, provenance, expiry, events, import, and export.
- [ ] Document provider endpoint setup for SendGrid, SES, Mailgun, and Postmark.
- [ ] Document source attribution fields and source analytics endpoints.
- [ ] Document job failure center endpoints and download formats.
- [ ] Commit with `git commit -m "docs: document email quality workflows"`.

## Verification Gates

Run these commands after each phase:

```bash
cargo fmt --all
cargo test -p reacher_backend --lib
cargo test -p reacher_backend --test e2e_tier1_features
cargo test -p reacher_backend --test e2e_job_lifecycle
cargo test -p reacher_backend --test e2e_worker_storage
```

Run these commands before marking the full pasted scope complete:

```bash
cargo test --workspace --lib
cargo test -p reacher_backend --tests
cargo run -p reacher_backend --bin generate_openapi
git diff --check
```

Expected final evidence:

- Every result-bearing response can include `recommendation` and `policy_evaluation`.
- Bulk, list, history, query, CSV, and NDJSON reads preserve persisted decision metadata.
- List remediation creates durable plans and exports fixed, safe, review, drop, and combined clean partitions.
- Suppressions include provenance, status, expiry, events, import, export, and policy-created auto-suppression.
- Job observability exposes summary, failures, and failure downloads.
- Sources can be attached to jobs/lists/pipelines and compared with quality metrics.
- Provider outcomes can be ingested, normalized, correlated, deduplicated, and used for suppressions and explanations.

## Requirement Coverage Audit

- Decision Layer: Tasks 1, 3, and 5 implement first-class recommendations with all pasted actions.
- Risk Policies By Use Case: Task 4 implements `growth`, `deliverability`, `signup_protection`, `enterprise_strict`, and constrained `custom` policy profiles.
- Automatic List Repair: Tasks 6 and 7 implement automatic fixes, normalized exports, dedupe-aware review, safe/review/drop partitions, and changed-row reports through remediation rows.
- Explainable Results: Task 2 implements all pasted explanation examples and Task 3 attaches explanations to every decision.
- Closed-Loop Learning: Tasks 14 and 15 ingest ESP/CRM-style outcomes, normalize events, feed suppression intelligence, source metrics, and future recommendations.
- Source Quality Intelligence: Tasks 12 and 13 implement source tracking and source quality reports for the pasted source categories.
- Suppression Intelligence: Tasks 8 and 9 implement reason, source, timestamp, expiry, confidence through metadata, owner through `created_by`, import/export, audit trail, and policy-created auto-suppression.
- Job Failure Center: Tasks 10 and 11 implement progress, failed rows, retryable rows, permanent failures, worker retries, ETA, and downloadable failure reports.

## Execution Handoff

Plan complete when this file exists, the requirements above are covered, and the current repo state has been inspected for all referenced paths. Implementation should start with Phase 1 and should not skip the decision/policy foundation.

Two execution options:

1. Subagent-Driven (recommended) - dispatch a fresh subagent per task, review between tasks, fast iteration.
2. Inline Execution - execute tasks in this session using executing-plans, batch execution with checkpoints.
