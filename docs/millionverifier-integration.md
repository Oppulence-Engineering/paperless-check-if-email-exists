# MillionVerifier integration and activation plan

Use MillionVerifier as an HTTPS verification backend while keeping the platform's jobs,
tenant access, quotas, history, exports, policies and webhooks. No SMTP proxy is required
on this path. The provider [explicitly permits API resale](https://help.millionverifier.com/millionverifier/resellers).
Its [official API documentation](https://developer.millionverifier.com/) supplies the
OpenAPI source for our generated Rust client.

## Implementation

1. Pin the upstream OpenAPI document; generate the local `millionverifier-sdk` Rust crate.
   Keep reproducible schema corrections separate from generated code.
2. Select `reacher` or `millionverifier` with `RCH__VERIFICATION_BACKEND`. Capture that
   selection in each admitted task. Credentials never enter task payloads or browser code.
3. Use the existing durable queue for single checks, list rows, finder candidates,
   scheduled checks and pipeline runs. Invalid syntax is rejected locally before paying.
   Finder domain prechecks use DNS only; catch-all evidence comes from candidate results.
4. Preserve provider result/code/subresult as `external_verification`. Map `ok` to valid,
   `catch_all` and `disposable` to risky, `unknown` to unknown, and `invalid` to invalid.
   Local disposable/role/spam-trap checks and suggestions can reduce the verdict.
   Provider errors, demo responses and inconsistent results fail the operation; they do
   not label a customer's mailbox invalid. Existing suppression/outcome policies still apply.
5. Record each paid attempt before sending HTTP and save the response before scoring and
   completion. Replayed deliveries reuse the stored response. An interrupted attempt with
   no recorded response fails without automatically buying another check. The existing
   quota lifecycle refunds a failed operation. Manual retries are explicit new attempts.
6. Report local SMTP diagnostics as unavailable. Do not synthesize SMTP success or use
   the SMTP-based bounce-risk model for external results. The provider's result remains
   authoritative even when positive history exists. No numeric confidence is published
   for external results because the API does not supply a calibrated confidence value.

## Configuration

Store `RCH__MILLIONVERIFIER__API_KEY` in the app's Infisical environment and provision it
through the existing Kubernetes secret mechanism. Keep the key out of values files,
source control, logs, screenshots, URLs in support messages, and browser environments.

Set the server configuration:

```text
RCH__VERIFICATION_BACKEND=millionverifier
RCH__MILLIONVERIFIER__API_KEY=<from secret store>
```

Postgres is required. The application default remains `reacher`; the staging and
production Helm overlays select `millionverifier` and require its synced secret. MillionVerifier
bypasses SMTP readiness/probes; `/readyz` checks provider configuration, not the provider's
live credit balance or upstream availability. The generated credits endpoint can inspect
balance without verifying an address.

Requests use a 20-second provider timeout, a 25-second HTTP deadline and at most four
concurrent provider calls per backend process. Review limits before scaling replicas.
API failures and unknown results do not automatically fall back to a second paid vendor.

## Validation and rollout

1. Run mapping/HTTP tests, paid-attempt recovery tests against an isolated Postgres test
   database, the existing backend suite, contract generation, and focused web checks.
2. Run the opt-in public demo contract test. Demo keys return synthetic results; they
   prove API wiring only and cannot be used to process customer addresses.
3. The user created the MillionVerifier account, and its live key is saved in Paperless's
   staging and production `/frontend/apps/sim` folders. Readback confirmed that the key
   is synced to Kubernetes in all three regions of both environments. No credit package
   was purchased.
4. Staging uses `check_email_staging`; production retains `check_email`. They share a
   broker but have separate databases, so publishers, retries and consumers must all
   use the configured queue (`RCH__WORKER__RABBITMQ__QUEUE`). Finder, pipeline and
   scheduled tasks now enter through the same durable outbox as API submissions.
   Deploy to staging with admission paused and old workers drained. This branch also
   includes the preceding audit branch's migrations; follow
   `docs/audits/2026-10-20-remediation.md` and
   `docs/audits/mailbox-identity-migration.md`. Apply migration
   `20261020000006_verification_provider_attempts.up.sql` before enabling this provider.
5. Verify a small, consented set of controlled addresses through the app: single check,
   uploaded CSV with duplicates, catch-all/unknown display, quota deduction, webhook,
   history/export, restart recovery and provider error. Check provider credit usage
   against paid-attempt rows and confirm the deployed revision.
6. Activate production only after the staging flow passes. Drain work before changing
   the provider setting in every region. Changing it with queued tasks fails those tasks
   explicitly rather than silently moving them to another provider.

The deprecated `/v0/bulk` SQL queue remains SMTP-only and rejects MillionVerifier admission;
use `/v1/bulk` or `/v1/lists`. The operator onboarding endpoint returns the same durable
operation result, which may be pending while verification finishes.

## Local validation completed on 2026-10-20

- Backend library suite: 378 tests passed, including the opt-in public demo API test.
- Postgres recovery tests: two passed, covering saved-response replay and an interrupted
  paid attempt without automatic retry or retained customer quota.
- Focused verification UI tests: 22 passed. Web typecheck, lint, contract compatibility,
  and generated-contract checks passed; lint still reports existing warnings.
- Workspace/all-target Clippy, Rust formatting, TypeScript SDK build, Go SDK tests, and
  staging/production Helm lint passed. Clippy reports existing and generated-code warnings.
- MillionVerifier SDK regeneration produced identical file hashes.

### Live account validation

The opt-in `live_mailboxes_complete_through_the_durable_worker` test passed against the
real provider using the two addresses supplied by the user and an isolated local Postgres
database. Both provider responses had `livemode: true`, and both application operations
returned HTTP 200:

- The corporate-domain address returned `catch_all`; the app reported risky, score 70,
  and `safe_to_send: false`.
- The Gmail address returned `ok`; the app reported valid, score 100, and
  `safe_to_send: true`.

Replaying each completed task retained exactly two provider-attempt rows and two customer
quota units in total. The provider balance was 500 before the test and 499 at the readback
afterward; this observation does not establish a general billing policy for risky results.

The live test is ignored by default. Running it requires a securely supplied
`RCH__MILLIONVERIFIER__API_KEY`, `MILLIONVERIFIER_LIVE_EMAILS` (one or two comma-separated,
explicitly authorized addresses), and a new `MILLIONVERIFIER_LIVE_REPORT` file path:

```sh
SQLX_OFFLINE=true cargo test -p reacher_backend --test e2e_millionverifier \
  live_mailboxes_complete_through_the_durable_worker -- --ignored --exact --nocapture
```

The report includes mailbox data and should stay outside source control. The test refuses
an existing report path or configured shared test database. Live requests consume provider
credits; rerunning with a new report path is an explicit new verification attempt.

This proves the live provider and local durable worker integration. The new code has not
been deployed, and verification through the deployed app remains outstanding.

## Deliberately deferred

Vendor-side bulk file uploads/polling, automatic vendor failover, new customer pricing,
and new credit-purchasing automation. Platform list uploads work with the existing
per-address queue. Benchmark that path before adding a separate bulk orchestration system.
