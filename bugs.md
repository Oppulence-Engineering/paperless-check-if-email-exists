# Email verification infrastructure and backend audit

Audited on **2026-10-20**. Source: `master` at `da6741786ebf26e80166dda546f3e8a36b81e4e1`. Live network observations were collected at approximately **08:48–08:53 UTC / 04:48–04:53 America/New_York**. This is an audit and remediation proposal; application code and deployed configuration were not changed.

## Main conclusions

**SMTP verification currently has two independent connectivity failures:** direct outbound TCP/25 timed out from every tested production and staging pod, and the configured SOCKS5 proxy authenticated successfully but explicitly refused connections to the tested mail servers on TCP/25. A reachable proxy is not sufficient; its exit must permit SMTP. Both backend and web readiness endpoints nevertheless returned HTTP 200 in all six deployments.

The report contains **36 findings**: 3 supported by live observations, 12 reproduced locally, and 21 derived from source review. Priorities are 1 P0, 23 P1, and 12 P2. They include operational misconfiguration, correctness defects, security weaknesses, and failure-recovery defects. Findings identify their evidence level individually. Live observations and local reproductions are stronger evidence than source-derived failure scenarios; a source finding is not a claim that the corresponding incident has already occurred in production. Passing tests do not establish absence of these defects.

The highest priorities are restoring an approved SMTP route, distinguishing provider/network failure from an invalid mailbox, preserving suppression decisions, closing outbound destination-validation gaps, and making queue/database/quota transitions recoverable. Restoring port 25 alone will not resolve the classification and persistence defects.

## What was examined and tested

The review covered core syntax/MX/SMTP behavior; backend configuration and readiness; tenant authentication, quotas, and throttling; single-check request handling; idempotency; bulk/list submission, retries, deletion, and exports; worker persistence and acknowledgements; webhook validation; suppressions and campaign outcomes; policy/scoring integration; reputation DNS checks; and maintenance deletion. The code graph was used for orientation, then findings were checked against current source because the graph's indexed revision was older than the checkout.

Existing tests:

| Command | Result |
| --- | --- |
| `SQLX_OFFLINE=true cargo test -p check-if-email-exists --lib` | 46 passed, 4 ignored |
| `SQLX_OFFLINE=true cargo test -p reacher_backend --lib` | 354 passed |
| Temporary audit integration tests, with external database/broker overrides unset | 10 passed; assertions intentionally demonstrate the defects below |

The temporary integration suite used real public core/backend functions, a localhost scripted SMTP server, and an isolated local PostgreSQL test container. It did not mutate production or staging databases. One test function exercised four database scenarios; another recorded international-address support limitations. The audit source was retained outside the checkout, then removed from the repository so this deliverable changes only this document.

| Reproduction | Observed result | Finding |
| --- | --- | --- |
| Request a different HELO, sender, SMTP port, and timeout without a request-level proxy | Configured values remain in the resulting verification methods | B27 |
| Run two throttle checks before either increments, with a limit of one | Both admitted | B15 |
| Canonicalize `Sales@company.example` and `sales@company.example` | Identical identity | B29 |
| Random-recipient reply `451 4.7.1 Greylisted`; target reply `250 OK` | `is_deliverable=true`, `is_catch_all=false` | B05 |
| Random recipient accepted; target would return `550 5.1.1` | Target RCPT never sent; deliverable and catch-all both true | B06 |
| Target accepted; server stalls on QUIT | Known result replaced with `Timeout(300ms)` | B08 |
| Target reply `550 5.7.1 Recipient address rejected: Authentication required` | Normal SMTP result with `is_deliverable=false` | B04 |
| Export a stored input beginning `=1+1` | CSV begins `=1+1,` without neutralization | B30 |
| Expire an active suppression, then insert a new adverse event for that address | New insertion returns false; address remains unsuppressed | B23 |
| Store `a.b+tag@gmail.com`, then check `ab@gmail.com` | Active suppression not found | B24 |
| Inject a PostgreSQL trigger failure during complaint suppression, remove failure, retry same outcome | Outcome accepted; suppression still absent after retry | B25 |
| Resolve an exhausted tenant, close database pool, call quota admission | `Allowed` | B17 |

Machine-readable evidence and logs are retained locally at `/tmp/paperless-audit-20261020/`: `direct-port25.json`, `live-smtp-diagnostics.json`, `live-socks-and-health.json`, `core-tests.log`, `backend-tests.log`, `adversarial-tests.log`, and `audit_probe.rs`. These are local scratch artifacts, not committed evidence or durable production telemetry. No proxy passwords or database credentials are reproduced here.

## Port 25 investigation

### Observations from the running pods

Each direct test used a five-second TCP connection timeout against the Google, Yahoo, and Microsoft MX endpoints listed below. Each proxy test performed SOCKS5 authentication and CONNECT using the pod's existing configuration. No recipient probes or email messages were sent to those providers.

| Kubernetes context | Namespace | Direct MX connections | HTTPS control | SOCKS5 authentication | SOCKS5 MX CONNECT | Web/backend readiness |
| --- | --- | --- | --- | --- | --- | --- |
| `oppulence-eu-central` | `paperless-production` | 3/3 timeouts | Success | Success | 2/2 refused, reply `0x02` | Both 200 |
| `oppulence-eu-central` | `paperless-staging` | 3/3 timeouts | Success | Success | 2/2 refused, reply `0x02` | Both 200 |
| `oppulence-us-east` | `paperless-production` | 3/3 timeouts | Success | Success | 2/2 refused, reply `0x02` | Both 200 |
| `oppulence-us-east` | `paperless-staging` | 3/3 timeouts | Success | Success | 2/2 refused, reply `0x02` | Both 200 |
| `oppulence-us-west` | `paperless-production` | 3/3 timeouts | Success | Success | 2/2 refused, reply `0x02` | Both 200 |
| `oppulence-us-west` | `paperless-staging` | 3/3 timeouts | Success | Success | 2/2 refused, reply `0x02` | Both 200 |

Direct targets were `gmail-smtp-in.l.google.com:25`, `mx1.mail.yahoo.com:25`, and `outlook-com.olc.protection.outlook.com:25`; the control was `example.com:443`. Proxy targets were the Google and Yahoo hosts. That is **18 direct SMTP timeouts, 6 successful HTTPS controls, and 12 explicit proxy refusals after successful authentication**. SOCKS5 reply `0x02` denotes a ruleset denial; it is distinct from an authentication failure. [SOCKS5 protocol, RFC 1928](https://datatracker.ietf.org/doc/html/rfc1928#section-6).

All six workloads used `pr.oxylabs.io:7777` and had worker mode enabled. Startup diagnostics also recorded the proxy denial and the identities `hello_name="localhost"` and `from_email="hello@localhost"`. No environment override for either identity was present. Backend readiness was checked on port **8081**, web readiness on **3000**; the backend response reported PostgreSQL and RabbitMQ healthy.

| Environment | Image tag | Running image digest |
| --- | --- | --- |
| Production, all three regions | `sha-da6741786ebf26e80166dda546f3e8a36b81e4e1` | `sha256:507f7635c94efdd5621ffc47e3d994f475756a4a5afadc9c324f64f5cc5c24ca` |
| Staging, all three regions | `sha-6cd7b3059ce4fb32cde898b4e6c7c4754343a505` | `sha256:94cc0f49e08ba925e4bccce051256d84da3ca41c99e0e3b29e5e3052e8cf5872` |

The image repository is `ghcr.io/oppulence-engineering/check-if-email-exists`. All six inspected pods were running, ready, and had zero restarts at inspection. The source audit therefore matches the production image tag; staging runs a different revision, so individual source findings should be checked against that revision before attributing them to staging.

| Context/environment | Pod | Node |
| --- | --- | --- |
| EU production | `check-if-email-exists-5c4557b487-rnpn6` | `oppulence-eu-central-agent-fsn1-ixw` |
| EU staging | `check-if-email-exists-staging-7bf6d74548-9hs7k` | `oppulence-eu-central-agent-fsn1-ixw` |
| US east production | `check-if-email-exists-76d57ff7f8-hfnhj` | `oppulence-us-east-agent-ash-ntz` |
| US east staging | `check-if-email-exists-staging-7fc88df5f-r6wmc` | `oppulence-us-east-agent-ash-ntz` |
| US west production | `check-if-email-exists-5c47cf8686-r4q9h` | `oppulence-us-west-agent-hil-fvi` |
| US west staging | `check-if-email-exists-staging-db56d5f54-87g69` | `oppulence-us-west-worker-v2-hil-upf` |

### What the evidence establishes

The tested direct IPv4 SMTP paths are unusable, while general outbound HTTPS works. The configured proxy account/endpoint also refuses the tested SMTP destinations. The proxy failure is explicit; the precise source of the direct-path timeouts is not established by a timeout alone.

No matching namespace NetworkPolicy was found for the verifier, and no Cilium cluster-wide network policies were listed in the three clusters. Production's `paperless-sim-app` policy selects the separate Sim application, not these verifier pods; staging had no namespaced policies. This reduces the likelihood of those Kubernetes policy objects being responsible, but does not rule out host rules, cloud firewalls, routing, or provider restrictions.

Hetzner documents default outbound blocking of ports 25 and 465, enforced per account. Eligible customers can request removal after a month and payment of the first invoice, subject to review. **Inference:** that documented policy is consistent with the direct-path evidence, but this audit did not inspect the account's actual restriction/unblock status or establish causality through provider support. Opening a Kubernetes Service or ingress port would not repair an outbound provider restriction. [Hetzner Cloud server FAQ](https://docs.hetzner.com/cloud/servers/faq/).

### Would a proxy work?

**Yes, provided its exit is explicitly allowed to connect to arbitrary public recipient MX servers on TCP/25.** The application already supports SOCKS5, so a different transport implementation is not the first requirement. The current endpoint's successful authentication and subsequent refusal demonstrate why “supports SOCKS5” alone is insufficient. The observation applies to this configured endpoint/account, not every product offered by its vendor.

The intended route is:

```text
Verifier pod
  -> authenticated, restricted SOCKS5 service on an allowed outbound port
  -> stable public exit IP
  -> recipient domain's actual MX server on TCP/25
```

My recommendation is a small, controlled egress service with a dedicated IP, or a managed SOCKS5 service whose provider explicitly approves SMTP verification and port 25. Require a stable exit, controllable reverse DNS, a matching owned hostname, a valid envelope sender, destination restrictions, and measured connection/concurrency limits. Test the exact account and exit before committing to it. A proxy placed on another server subject to the same outbound restriction does not help. Rotating through arbitrary shared exits makes diagnosis and reputation control harder; it is not a substitute for approved access.

An HTTP-only proxy is not directly interchangeable with the existing SOCKS client. A general SMTP submission relay on port 587 is also not equivalent: it accepts submissions under its own policy and does not expose the recipient MX's immediate mailbox decision. Port 587 is the submission service; SMTP relay traffic uses port 25. [Message submission, RFC 6409](https://datatracker.ietf.org/doc/html/rfc6409#section-3.1).

Acceptance should require an authenticated tunnel, a successful TCP/25 connection, a valid `220` greeting, the expected observed exit IP and DNS identity, then controlled verification cases using addresses owned for testing. Exercise known-valid, known-invalid, catch-all, policy-blocked, and temporarily deferred cases separately. Prove this from every intended region and retain route/provider/error metrics. Neither a successful TCP connection nor a proxy vendor's marketing description proves verification accuracy.

## Research: how to build reliable verification

The following combines primary standards, open-source implementations, and operational documentation. Proposed product behavior is my engineering synthesis; it is not a claim that a standard mandates this exact product architecture.

### Separate the questions the product answers

Syntax validity, DNS mail capability, SMTP acceptance, ownership, and expected deliverability are different facts. A successful RCPT response is evidence about a particular server interaction, not proof that a person owns the address or that a future message will reach an inbox. Servers can accept recipients and reject later; Postfix's verification documentation also explains probing cost, blocking risk, and caching tradeoffs. Keep “accepted by MX,” “catch-all,” “unknown,” and “policy blocked” distinguishable. [Postfix address verification](https://www.postfix.org/ADDRESS_VERIFICATION_README.html).

For ownership, use an explicit confirmation flow when the product needs that guarantee. For deliverability prediction, expose evidence age and uncertainty rather than presenting a score as a measured probability without calibration. A boolean `safe_to_send` should be a policy output grounded in suppression and evidence, not a synonym for TCP reachability.

### Normalize conservatively

Use a maintained parser and retain the original address alongside its normalized representation. Domain normalization and internationalized domains need deliberate handling; quoted addresses and SMTPUTF8 support need an explicit support policy. The open-source `python-email-validator` project is a useful reference for normalization, IDNA, syntax policy, and DNS checks, but its DNS checks do not establish mailbox existence. Provider-specific equivalence should remain scoped: consumer Gmail ignores dots, whereas Google documents different behavior for work/school domains. [python-email-validator](https://github.com/JoshData/python-email-validator), [Google's address-dot guidance](https://support.google.com/mail/answer/7436150).

Do not lowercase every local part as a universal identity rule. RFC 5321 preserves local-part case sensitivity while domain names are case-insensitive. Its routing rules also require ordered MX handling and an implicit mail exchanger when no MX exists but the domain has an address record. Null MX is a separate declaration that a domain accepts no mail. [SMTP, RFC 5321](https://datatracker.ietf.org/doc/html/rfc5321), [Null MX, RFC 7505](https://datatracker.ietf.org/doc/html/rfc7505).

If internationalized local parts are supported, negotiate SMTPUTF8 and preserve UTF-8 correctly throughout parsing, queues, persistence, and SMTP commands. If they are not supported, return a specific unsupported-capability result rather than implying the mailbox is known not to exist. [SMTPUTF8, RFC 6531](https://datatracker.ietf.org/doc/html/rfc6531).

### Preserve error meaning

Record the operation stage, basic SMTP code, enhanced status code, provider text, selected MX, and route. A policy/security rejection such as `5.7.1` is different from an address-status failure such as `5.1.1`; provider text can be ambiguous. Temporary failures and transport errors must remain inconclusive. Provider-specific parsers need captured fixtures and explicit precedence for enhanced codes. Google's and Microsoft's error documentation are useful primary fixture sources, not universal parsing rules. [Enhanced status codes, RFC 3463](https://datatracker.ietf.org/doc/html/rfc3463), [Google SMTP errors](https://support.google.com/mail/answer/3726730?hl=en), [Microsoft 5.7.1 guidance](https://learn.microsoft.com/en-us/troubleshoot/exchange/email-delivery/ndr/fix-error-code-550-5-7-1-in-exchange-online).

Treat catch-all detection as a separate observation with a possible unknown state. Accepting a random local part is evidence of broad acceptance behavior at that time; it cannot prove that a particular target is deliverable. A timeout or greylist during that random probe is not evidence that catch-all is false. Avoid increasing confidence simply because the server declined to disclose mailbox information.

### Retry according to the failure

Greylisting may key decisions on the source IP, sender, and recipient. A retry must preserve the relevant identity and wait long enough to be meaningful. Immediate retry loops and changing exits can repeatedly reproduce the first-attempt rejection. Use bounded delayed jobs, provider-aware backoff, and a stable probe identity where needed; preserve the original result as pending/unknown until the retry resolves. [Greylisting guidance, RFC 6647](https://datatracker.ietf.org/doc/html/rfc6647).

Maintain an overall deadline across DNS, connection, greeting, commands, enrichment, and cleanup. Distinguish retryable infrastructure failures from definitive recipient responses. Record which MX was tried and whether failover was exhausted. Apply per-MX/provider and per-exit concurrency limits as well as tenant limits; a tenant API rate limit alone does not protect a shared exit's reputation.

### Make the network identity intentional

The upstream Reacher implementation documents the need for outbound port 25. Its open-source code is a useful implementation baseline, not a guarantee that every cloud provider or proxy permits the traffic. Maintain owned, resolvable identities, stable egress, low probe volume, and route diagnostics. When a route is blocked, report an infrastructure problem and stop repeatedly spending quota on equivalent failures. [Reacher upstream](https://github.com/reacherhq/check-if-email-exists).

For a proxy deployment, the recipient observes the proxy's exit, so DNS identity and reputation checks must describe that exit rather than merely the Kubernetes node. Keep connections to the approved proxy restricted, and avoid exposing an unrestricted public proxy. Acceptance testing should measure observed behavior with the exact credentials and route that production will use.

### Do not turn domain reputation hints into mailbox facts

SPF is an authorization policy for a sending identity, not evidence that a recipient mailbox exists. An SPF record without `all` or `redirect` can still be syntactically valid and reach the default neutral result. Parsing validity and assessing strictness are different operations. DKIM selectors are chosen by domain owners; absence at a handful of guessed selector names cannot establish absence of DKIM, and a TXT record is not necessarily a usable key. [SPF, RFC 7208](https://datatracker.ietf.org/doc/html/rfc7208#section-4.7), [DKIM, RFC 6376](https://datatracker.ietf.org/doc/html/rfc6376).

For each reputation signal, retain `observed`, `absent`, `unknown`, or `query_failed` rather than assigning reassuring values after an outage. Distinguish the reputation of inbound MX infrastructure from that of outbound senders. Shared hosting, forwarding, and provider topology prevent a simple one-to-one interpretation. We should calibrate the value of these features against outcomes before assigning them strong scoring weight.

### Treat suppressions and outcomes as durable safety inputs

Keep complaints, unsubscribes, hard bounces, and operator suppressions separate from probabilistic verification scores. A recent positive SMTP result should not erase a suppression. Use one tenant-scoped identity policy across ingestion, lookup, import, verification, and export; make suppression updates transactional with their audit events. Expiry and reactivation must have explicit semantics.

For outcomes, atomically commit either the event and its required action or the event and a durable pending action. Deduplication must not skip an action that previously failed. For decisions that prohibit sending, use a source with the required consistency rather than an eventually consistent read whose lag can briefly reverse the decision.

### Design for interruption, duplicate delivery, and uncertain outcomes

Every user operation should have a durable identity independent of an HTTP connection or broker delivery. An HTTP timeout does not establish that the worker did nothing. Persist operation state and quota ownership so a retry can find the same work. Use a transactional outbox for database-to-broker publication where correctness crosses both systems; consumers still need idempotent state transitions because outbox delivery can repeat.

Store the result and required downstream work before acknowledging the message or returning success. Ensure final job/list state is monotonic, and reconcile unfinished transitions after crashes. Test failures between each durable step: quota reservation, job insertion, task insertion, publication, result storage, response publication, acknowledgement, suppression, and parent finalization.

### Secure destinations and exports

An email domain, proxy address, or webhook URL can become a network destination. Validate resolved addresses, including IPv6 and non-public ranges, and ensure the connection actually uses the validated addresses. DNS validation followed by a second unrestricted resolution leaves a rebinding window. Apply the same policy to redirects and proxy-mediated resolution. [OWASP SSRF prevention](https://cheatsheetseries.owasp.org/cheatsheets/Server_Side_Request_Forgery_Prevention_Cheat_Sheet.html).

CSV quoting solves CSV syntax, not spreadsheet formula execution. Imported cells, email strings, error text, and arbitrary original columns need spreadsheet-safe export handling when opened as spreadsheets. Treat this as a shared serialization concern across download endpoints. [OWASP CSV injection](https://community.owasp.org/attacks/CSV_Injection).

### Measure accuracy with ground truth

Build an owned test corpus covering accepted/rejected recipients, catch-all exceptions, greylisting, policy denials, null MX, implicit MX, MX failover, Unicode support boundaries, SMTP stalls, duplicate broker deliveries, and database failures. Track results by provider, route, exit, and evidence age. Report false-invalid and false-safe rates separately; “unknown” is often the correct outcome and should not be optimized away.

Use permitted delivery outcomes and explicit ownership confirmation where available to evaluate predictions. Separate transport availability, classification accuracy, suppression correctness, and business recommendation quality in dashboards. A high aggregate score or a passing readiness probe cannot replace any of these measurements.

## Findings

Priorities: **P0** = current blocker for the affected capability; **P1** = high impact to correctness, safety, security, or durable operations; **P2** = important bounded or conditional defect. Each finding below is one paragraph containing the bug, root cause, and proposed fix. Source references are relative to the audited revision.

### B01 — P0 — Both available SMTP routes fail

**Bug:** SMTP methods cannot reach the tested public MX servers from any of the six deployed workloads. **Root cause/evidence — live:** direct TCP/25 timed out in all 18 probes, while the configured `pr.oxylabs.io:7777` SOCKS5 endpoint successfully authenticated and returned ruleset denial `0x02` for all 12 SMTP CONNECT attempts; HTTPS controls succeeded. Direct failure is consistent with Hetzner's documented restriction, but the actual account/firewall cause remains unconfirmed. **Fix:** obtain approved outbound SMTP access or use an explicitly SMTP-capable controlled proxy exit, then prove TCP connection, SMTP greeting, and controlled recipient behavior from each region. Expose route failure separately from mailbox validity and avoid charging/retrying equivalent checks during a known outage. Changing only the proxy hostname or opening inbound Kubernetes port 25 is not a verified repair.

### B02 — P1 — Production SMTP identity uses localhost

**Bug:** verification sessions identify themselves as `localhost` with sender `hello@localhost`, which can trigger policy rejection even after network access is repaired. **Root cause/evidence — live and source:** all six startup diagnostics report these values, matching defaults in [backend_config.toml](backend/backend_config.toml):28 and :34, without relevant environment overrides. **Fix:** configure an owned, resolvable HELO hostname and valid sender identity; align forward/reverse DNS and any sender authorization with the actual egress IP, including a proxy exit when used. Reject or prominently flag placeholder identities for production SMTP configuration and test against controlled servers. This finding establishes unsuitable configuration, not that it caused the currently observed proxy ruleset denial.

### B03 — P1 — Readiness stays healthy while SMTP is unusable

**Bug:** operations can see a ready service even when its SMTP verification route is unavailable. **Root cause/evidence — live and source:** both readiness endpoints returned 200 everywhere during B01; [readiness.rs](backend/src/http/health/readiness.rs) checks database/broker dependencies, while [smtp_egress.rs](backend/src/smtp_egress.rs) provides startup diagnostics rather than a maintained capability signal. Kubernetes probes target the web endpoints. **Fix:** maintain a bounded, periodic SMTP-route capability status, publish metrics/alerts, and make the API's behavior during route failure explicit; decide separately whether an SMTP-dependent worker should accept new work. Do not make liveness depend on a third-party MX or issue SMTP probes for every health request, which would create restart loops or unnecessary traffic.

### B04 — P1 — Policy rejection is classified as an invalid recipient

**Bug:** `550 5.7.1 Recipient address rejected: Authentication required` becomes a normal nondeliverable mailbox result, which the core classifier can treat as invalid. **Root cause/evidence — reproduced:** broad text matching in [smtp/parser.rs](core/src/smtp/parser.rs):25–81 recognizes “address rejected,” while the policy-error exceptions do not cover this response; the localhost SMTP fixture returned `is_deliverable=false`. **Fix:** classify enhanced status codes and command context before generic text, preserving security/policy failures as blocked or unknown and reserving invalid-recipient results for supported mailbox evidence such as applicable `5.1.1` responses. Add this fixture and contrasting missing-mailbox fixtures, and verify the resulting score, recommendation, exported fields, and suppression behavior.

### B05 — P1 — Failed catch-all detection becomes a confident false

**Bug:** an inconclusive random-recipient probe is reported as `is_catch_all=false`, allowing downstream decisions to use absence of catch-all risk without evidence. **Root cause/evidence — reproduced:** [smtp/connect.rs](core/src/smtp/connect.rs):287–289 applies `.unwrap_or(false)` to the catch-all check; a fixture returning `451 4.7.1` for the random recipient and `250` for the target produced deliverable true and catch-all false. **Fix:** preserve a tri-state catch-all observation or an explicit probe error, propagate it through scoring/contracts, and use a conservative recommendation or delayed recheck when needed. Test policy blocks, timeouts, and temporary failures separately from a definitive rejection of the random recipient.

### B06 — P2 — Catch-all acceptance skips the requested mailbox

**Bug:** the target address can be marked deliverable without its RCPT ever being tested. **Root cause/evidence — reproduced:** [smtp/connect.rs](core/src/smtp/connect.rs):290 constructs successful delivery details immediately when the random address is accepted; the fixture that would reject the target with `550 5.1.1` recorded only the random RCPT. Catch-all behavior can coexist with explicit per-recipient rejection or disabled-address exceptions. **Fix:** evaluate the requested recipient independently, retain the catch-all observation as separate uncertainty, and do not claim target-specific acceptance when it was not observed. Add a server fixture that accepts the random recipient but rejects the target.

### B07 — P2 — Only one MX is tried

**Bug:** an unavailable preferred MX can make a domain appear unverifiable even when another advertised exchanger is reachable. **Root cause/evidence — source:** [core/src/lib.rs](core/src/lib.rs):310–320 selects a single minimum-preference MX and passes it to SMTP checking; retries remain tied to that selected host rather than exhausting appropriate alternatives. **Fix:** attempt exchangers in preference order, balance equivalent preferences, and apply a bounded overall deadline while retaining each attempt's transport result. Fail over on connection/temporary transport failures without overriding a definitive recipient decision merely because another server is more permissive. Verify a dead primary with a working backup and bounded all-host failure. Current null-MX and implicit-MX handling should be preserved.

### B08 — P2 — QUIT can erase an already known result

**Bug:** a server that stalls during QUIT converts a completed recipient check into a timeout. **Root cause/evidence — reproduced:** [smtp/connect.rs](core/src/smtp/connect.rs):325 awaits cleanup inside the same timed operation that produces the verdict; the fixture accepted the target, then stalled during QUIT, yielding `Timeout(300ms)`. **Fix:** finalize the recipient evidence before cleanup, use a short bounded best-effort QUIT, and preserve the result if only shutdown fails. Retain cleanup failure as diagnostic metadata rather than mailbox evidence; test both accepted and rejected recipients with hanging QUIT.

### B09 — P1 — SMTP and proxy destinations lack private-network protection

**Bug:** tenant-controlled email/MX data and request-level routing settings can direct backend connections toward non-public services. **Root cause/evidence — source:** [smtp/connect.rs](core/src/smtp/connect.rs):51 opens the selected destination without a resolved-address public-network check, and [check_email/post.rs](backend/src/http/v0/check_email/post.rs):24–76 accepts a proxy host/port and SMTP port. DNS-controlled MX records and caller-supplied proxies cross the network trust boundary; the emitted protocol is SMTP/SOCKS, so this is not a claim of arbitrary HTTP request capability. **Fix:** restrict tenant routing overrides, validate and pin all resolved destination addresses, reject loopback/private/link-local/metadata destinations including IPv6, and apply equivalent restrictions when a proxy resolves the target remotely. Add isolated negative tests and enforce outbound network policy as defense in depth; no production exploitation was attempted.

### B10 — P1 — Webhook validation does not pin the connection address

**Bug:** a webhook hostname can pass public-IP validation and subsequently resolve to a prohibited address for the HTTP connection. **Root cause/evidence — source:** [worker/do_work.rs](backend/src/worker/do_work.rs):1099 validates a DNS lookup, but `send_task_webhook` later uses a separately resolving reqwest client; related validation/send paths exist in [pipelines.rs](backend/src/pipelines.rs):2869 and :3420. Redirects are already restricted, which does not close this DNS rebinding gap. **Fix:** connect only to the addresses validated for that attempt while retaining correct Host/SNI behavior, validate every refreshed address, and share the implementation across webhook callers. Add a controlled resolver-change test and maintain egress restrictions. This is a source-established validation/connection mismatch, not a demonstrated production compromise.

### B11 — P1 — A timed-out idempotent request can dispatch and charge again

**Bug:** retrying the same idempotency key after an HTTP worker timeout can create another verification and another quota reservation while the original worker still runs. **Root cause/evidence — source:** [shared/check_email.rs](backend/src/http/shared/check_email.rs):251 marks worker errors failed, including the timeout at :422, while retaining quota because the task may finish; [idempotency.rs](backend/src/http/idempotency.rs):116 permits failed keys to be recreated. Its five-minute stale-processing cleanup also lacks proof that the underlying operation stopped. **Fix:** persist an operation/task identity and reservation independently of HTTP waiting, leave uncertain outcomes recoverable through that same operation, and reconcile its durable result on retry instead of dispatching anew. Use fenced ownership for abandoned operations and test timeout followed by late completion and repeated identical requests.

### B12 — P2 — Reclaiming a failed idempotency key races

**Bug:** simultaneous retries of one failed idempotency key can produce an internal error instead of one owner and a deterministic in-progress/cached response. **Root cause/evidence — source:** [idempotency.rs](backend/src/http/idempotency.rs):116–143 deletes the failed row and inserts a replacement in separate statements; two requests can both observe failed state, after which the second insertion conflicts with the first. **Fix:** atomically transition the existing failed record using a conditional update and ownership token, or use a transaction with an appropriate lock and return the winning operation to the loser. Preserve body/path mismatch checks and test concurrent reclamation with a barrier at the old failed state.

### B13 — P1 — Worker RPC success is published before result storage

**Bug:** a caller can receive success even though the result subsequently fails to persist, and broker redelivery can rerun the check. **Root cause/evidence — source:** `delivery_finalize` in [worker/do_work.rs](backend/src/worker/do_work.rs):850 sends the single-shot reply before calling `store_prepared`/`store_error`; storage failure then leaves delivery finalization unsuccessful. **Fix:** durably store the operation result before publishing success, and make redelivery retrieve that stored result using a stable operation identity rather than repeating SMTP work. A reply-publication failure must also be recoverable without losing the stored result. Test storage failure before reply and reply failure after storage, including client retry and quota accounting.

### B14 — P2 — HTTP worker timeout is shorter than permitted verification work

**Bug:** valid checks can return a gateway timeout while their permitted SMTP operation is still running, leaving clients without the result and retaining their reservation. **Root cause/evidence — source:** [shared/check_email.rs](backend/src/http/shared/check_email.rs):31 fixes RPC waiting at 20 seconds, while [backend_config.toml](backend/backend_config.toml):43 permits a 45-second SMTP timeout before queue delay and other work are considered. **Fix:** define an end-to-end deadline and a durable operation retrieval contract; slow work should return a pending operation or have an explicitly aligned synchronous budget. Merely increasing one timeout leaves queueing and B11 unresolved. Verify slow SMTP, queue backlog, and late completion without issuing duplicate checks.

### B15 — P1 — Concurrent requests bypass throttling

**Bug:** more requests than the configured limit can enter verification concurrently. **Root cause/evidence — reproduced:** [throttle.rs](backend/src/throttle.rs):173–193 checks and increments under separate mutex acquisitions; the direct HTTP path increments after work and the worker path increments after earlier admission steps. Two concurrent checks with a one-request limit were both admitted before either increment. **Fix:** perform check-and-reserve atomically at admission, then explicitly decide which pre-dispatch failures release the reservation. Update all callers to use the shared atomic operation and test concurrency rather than only sequential counter increments.

### B16 — P2 — Tenant throttles are independent per process

**Bug:** a tenant routed across multiple instances can consume each instance's full nominal rate allowance. **Root cause/evidence — source:** [config.rs](backend/src/config.rs):298 stores tenant managers in a process-local map, and [throttle.rs](backend/src/throttle.rs) stores counters only in memory. B15's atomicity fix within one process would not address multiplication across replicas or regions. **Fix:** use a shared atomic limiter for a tenant-wide contract, or explicitly define and enforce a different contract at a common ingress. Keep per-provider/exit protection separate. Test requests distributed over two processes; live exploitation or actual tenant traffic distribution was not measured.

### B17 — P1 — Quota enforcement fails open on database errors

**Bug:** an exhausted metered tenant can be admitted when quota storage is unavailable. **Root cause/evidence — reproduced:** [tenant/quota.rs](backend/src/tenant/quota.rs):81–151 returns `Allowed` on update failure, missing pool, or failure to read the tenant after a zero-row update; the isolated database test resolved a tenant at 10/10 usage, closed the pool, and received `Allowed`. **Fix:** distinguish intentionally unlimited/legacy contexts from operational failure for metered tenants, return a retryable service error when reservation cannot be proven, and dispatch no work without a recorded reservation. Cover pool failure, missing tenant, stale context, and concurrent last-credit requests.

### B18 — P1 — List creation leaks quota and partial rows on database failure

**Bug:** a failed upload can consume credits and leave incomplete list/job/task records without corresponding published work. **Root cause/evidence — source:** [lists/post.rs](backend/src/http/v1/lists/post.rs):109 reserves quota before separate job/list/row insertions, whose `?` exits do not undo the reservation or earlier writes; compensation later in the function addresses publication failure rather than all creation failures. **Fix:** atomically commit quota reservation, list/job/task records, and publication intent, preferably through the existing database transaction plus an outbox; otherwise compensate every pre-publication failure and reconcile uncertain partial publication. Inject failure after each insertion and verify no orphaned charge or invisible queued work remains.

### B19 — P1 — Bulk creation can overwrite a terminal job state

**Bug:** a small, quickly processed bulk job can revert from completed/failed/cancelled to running and remain misleadingly active. **Root cause/evidence — source:** after publishing tasks, [bulk/post.rs](backend/src/http/v1/bulk/post.rs):299 unconditionally writes `status='running'`; a worker or cancellation path can already have advanced the job while publication was in progress. **Fix:** use a conditional pending-to-running transition, preserve terminal/cancellation states, and ensure reconciliation cannot move a job backwards. Test with a consumer that finishes before the publisher performs its final update. The corresponding list code already has a conditional transition and should not be replaced with the faulty bulk pattern.

### B20 — P1 — Manual retry publishes before its database transaction commits

**Bug:** retry messages can exist in RabbitMQ while the database still reports the old failed tasks, causing duplicate retry eligibility, inconsistent results, or additional charging. **Root cause/evidence — source:** [jobs/retry.rs](backend/src/http/v1/jobs/retry.rs):136 publishes while holding the transaction, then resets task state and commits; a later publish or commit failure rolls back database changes but cannot retract already accepted messages. **Fix:** commit retry intent, quota ownership, and state transition atomically, publish through an outbox, and have consumers claim the exact retry generation idempotently. Test partial batch publication and commit failure; publisher confirms alone do not make a broker publish atomic with PostgreSQL.

### B21 — P1 — Database count failures can finalize unfinished jobs

**Bug:** transient read errors can be interpreted as no remaining work or no failed tasks, producing an incorrect completed state. **Root cause/evidence — source:** `sync_task_entities` in [worker/do_work.rs](backend/src/worker/do_work.rs):895 uses `.unwrap_or(0)` for task counts, including nonterminal and failed counts at approximately :1016 and :1025, then performs finalization updates; similar list-finalization counts have the same pattern. **Fix:** propagate or record count failures and defer finalization until a successful consistent read establishes terminal state. Retry reconciliation independently, preserve cancellation precedence, and inject a count-query failure followed by a successful update to verify that no completion is fabricated.

### B22 — P1 — Required post-completion work occurs after broker acknowledgement

**Bug:** a worker crash can permanently skip auto-suppression, outcome-driven suppression, events, or parent synchronization after a task result has been stored. **Root cause/evidence — source:** [worker/do_work.rs](backend/src/worker/do_work.rs):484 acknowledges the delivery before later state/event updates and `evaluate_post_completion_actions` / `apply_post_verification_outcome_check`, with related-entity synchronization still outstanding. The result may already be durable; the lost work is the required follow-up. **Fix:** durably record required actions and finalization intent before acknowledgement, execute them idempotently, and reconcile pending actions after restart. Test process termination at the acknowledgement boundary; merely moving external calls before ACK without durable idempotency can introduce duplicate effects.

### B23 — P1 — Expired active suppression prevents new suppression

**Bug:** a new complaint or bounce can fail to reactivate suppression for an address whose old row expired. **Root cause/evidence — reproduced:** [suppression.rs](backend/src/suppression.rs):57 inserts with `ON CONFLICT ... WHERE status='active' DO NOTHING`, while reads exclude expired rows; an expired row still marked active occupies the unique key but no longer suppresses sending. The isolated database test reproduced exactly that state. **Fix:** atomically transition expired rows or renew them according to explicit precedence/expiry rules, preserving history and the new event. Test expiry followed by complaint, unsubscribe, and repeated ingestion, including concurrent reactivation.

### B24 — P1 — Suppression identity differs between write paths

**Bug:** one Gmail spelling can be suppressed while an equivalent mailbox spelling passes the suppression check. **Root cause/evidence — reproduced and caller review:** suppression API paths store lowercased addresses, while other paths use Gmail canonicalization; [suppression.rs](backend/src/suppression.rs):26 checks the input's lowercase and canonical strings but cannot discover every differently stored alias. Storing `a.b+tag@gmail.com` and checking `ab@gmail.com` returned false. **Fix:** use one provider-aware identity function on every suppression write/read/import/outcome path, migrate existing alias rows with conflict/history handling, and retain the supplied address separately. Do not extend consumer Gmail equivalence to unrelated domains or preserve B29's universal local-part lowercasing.

### B25 — P1 — Accepted campaign outcomes can permanently lose suppression

**Bug:** an adverse campaign outcome can be accepted while its suppression write fails, and retrying the same event does not repair the missing action. **Root cause/evidence — reproduced:** [outcomes.rs](backend/src/outcomes.rs):291–370 commits/deduplicates the outcome separately from the suppression action; the suppression error path at approximately :439–466 returns false, while duplicate ingestion skips that action. A local trigger forced the insert failure; after removing it, retry still left the complaint address unsuppressed. **Fix:** transactionally commit event and action, or persist a retryable pending-action record and revisit it on duplicate ingestion. Scope this finding to campaign-outcome ingestion; the separate contact-outcome ingestion path already uses a transaction and should be reused where appropriate.

### B26 — P1 — Suppression decisions may read a lagging replica

**Bug:** a newly suppressed address can temporarily receive a send recommendation when a read replica lags. **Root cause/evidence — source, conditional on replica configuration:** [scoring/response.rs](backend/src/scoring/response.rs):331 checks active suppression using `read_pool`, which [config.rs](backend/src/config.rs):273 obtains from the configured read pool rather than necessarily the primary. **Fix:** read safety-critical suppression state from the primary or enforce a demonstrated read-after-write consistency mechanism; keep ordinary analytics on replicas if desired. Add a test where the replica lacks a just-committed suppression. This audit did not measure live replica lag or establish that production is configured with a distinct replica.

### B27 — P2 — SMTP request overrides are silently ignored without a proxy

**Bug:** a caller's HELO, sender, port, and timeout settings do not reliably affect verification. **Root cause/evidence — reproduced:** [check_email/post.rs](backend/src/http/v0/check_email/post.rs):48–76 computes those overrides but applies them only when `body.proxy` is present; otherwise it returns `config.get_verif_method()`. The shared request type is used by v0/v1 paths. The test requested port 2525 and two seconds but retained port 25 and the configured 60 seconds. **Fix:** apply permitted explicit fields to the selected SMTP methods independently of proxy presence while preserving provider-specific methods, or reject unsupported overrides clearly. Test both proxy/no-proxy paths and maintain B09's destination restrictions.

### B28 — P1 — Requested policy mode and profile are ignored

**Bug:** a caller can request one risk policy and receive a decision computed under the tenant default or a different active custom profile. **Root cause/evidence — source:** `policy_mode` and `policy_profile_key` are accepted in [check_email/post.rs](backend/src/http/v0/check_email/post.rs):39–41 but are not propagated into decision preparation; [scoring/response.rs](backend/src/scoring/response.rs):317 loads the tenant default and :337 loads the active profile. **Fix:** validate the requested tenant-owned policy/profile, carry it through direct and queued execution, and use it consistently in response, storage, idempotency replay, and export; alternatively remove unsupported request fields with an explicit contract change. Test a strict request against a permissive tenant default and a non-active named profile.

### B29 — P1 — Canonicalization merges potentially distinct non-Gmail mailboxes

**Bug:** list deduplication and related history/suppression identity can merge distinct local parts or verify a changed address. **Root cause/evidence — reproduced:** [lists/canonicalize.rs](backend/src/http/v1/lists/canonicalize.rs):11 lowercases the complete address, so `Sales@company.example` and `sales@company.example` become identical; list processing uses the canonical address for checks. **Fix:** lowercase/normalize domains while preserving local-part case except under an explicitly supported provider equivalence rule, store the original address, and separate display/routing address from deduplication identity. Plan migration of existing merged identities without silently deleting history, and test case-sensitive local SMTP fixtures and consumer Gmail aliases.

### B30 — P1 — CSV exports preserve executable formula prefixes

**Bug:** opening exported attacker-controlled values in spreadsheet software can evaluate them as formulas. **Root cause/evidence — reproduced:** [csv_shared.rs](backend/src/http/csv_shared.rs):59 serializes values with ordinary CSV escaping, which does not neutralize formula syntax; a stored invalid input `=1+1` remained the first CSV cell unchanged. **Fix:** apply shared spreadsheet-safe text handling to untrusted exported fields, including inputs, errors, and original uploaded columns, and review all CSV download callers rather than fixing one endpoint. Cover formula prefixes and leading control characters while preserving a documented machine-readable export contract. The reproduction verified raw output only; no dangerous spreadsheet formula was executed.

### B31 — P2 — Debug logs can include database credentials

**Bug:** enabling debug logging can expose credentials embedded in the database connection URL. **Root cause/evidence — source:** [storage/postgres.rs](backend/src/storage/postgres.rs):56 logs `db_url` directly. **Fix:** omit the URL or log only explicitly nonsecret connection metadata; redact userinfo and sensitive query parameters if a sanitized URL is necessary. Add a log-format assertion using a synthetic password and review retention/access for any existing debug logs. No real database credentials were retrieved or printed during this audit, and live debug-log exposure was not established.

### B32 — P2 — Guessed DKIM selectors are treated as a domain-wide verdict

**Bug:** a domain can be penalized for “missing DKIM” despite using an unguessed selector, or credited for an arbitrary/unusable TXT record. **Root cause/evidence — source:** [reputation/dns_records.rs](backend/src/reputation/dns_records.rs):132–154 tries a small fixed selector list and treats a TXT lookup as presence; [reputation/scorer.rs](backend/src/reputation/scorer.rs):22 deducts points when none are found. **Fix:** represent selector-specific detection and unknown coverage, parse key material including revoked/empty keys, and avoid a domain-wide absence penalty without authoritative evidence. Use selectors from an actual permitted message/signature when available, and test an uncommon valid selector plus unrelated TXT and revoked keys.

### B33 — P2 — SPF strictness heuristic is labeled validity

**Bug:** valid SPF records can be marked invalid and malformed ones marked valid. **Root cause/evidence — source:** `assess_spf` in [reputation/dns_records.rs](backend/src/reputation/dns_records.rs):79 accepts a terminal `all` or a redirect modifier without full syntax validation, so `v=spf1 a` is rejected by the heuristic while `v=spf1 nonsense -all` is accepted. **Fix:** use an SPF parser for syntactic validity, distinguish validity from policy strictness, and preserve DNS/evaluation errors separately; if only a heuristic is intended, name it accordingly and do not expose it as validation. Test default-neutral policies, invalid mechanisms, malformed redirect, multiple records, and modifier ordering.

### B34 — P2 — DNSBL query failure becomes “not listed”

**Bug:** a failed, refused, or skipped blacklist query can be reported as a clean result. **Root cause/evidence — source:** [reputation/dnsbl.rs](backend/src/reputation/dnsbl.rs):84–110 converts lookup errors through `.ok().is_some_and(...)`; no-MX/no-IPv4 paths also emit `listed=false`. The scorer cannot distinguish these from a completed negative lookup. **Fix:** return explicit query status and evidence, decode provider error answers separately, and keep unknown/skipped results from masquerading as negative evidence. Test timeout, refusal, provider error codes, no applicable address, and a genuine negative answer. Existing filtering that prevents special error addresses from becoming positive listings should remain; the remaining problem is loss of uncertainty.

### B35 — P2 — Legacy pruning deletes outside its transaction

**Bug:** a failed cleanup can delete child email results while retaining their parent jobs. **Root cause/evidence — source:** [bin/prune_db.rs](backend/src/bin/prune_db.rs):39 begins a transaction, but both DELETE statements execute on `&pool` at :50 and :65 instead of that transaction; committing the unused transaction does not make those deletes atomic. **Fix:** execute both statements through the same transaction and test failure between them with an isolated database. This affects the legacy pruning command, not a demonstrated live cleanup incident or every v1 retention path.

### B36 — P1 — List deletion is non-atomic and can race processing

**Bug:** a failed or concurrent delete can remove only part of a list's results/list/job structure or delete while work is being activated. **Root cause/evidence — source:** [lists/delete.rs](backend/src/http/v1/lists/delete.rs):24–64 reads status without a lock, rejects only `processing`, then performs three independent DELETE statements; an intermediate error leaves prior deletions committed, and an uploading/rerun transition can race the initial check. **Fix:** lock and revalidate the list/job lifecycle inside one transaction, define which states are deletable, and delete related records atomically with appropriate foreign-key behavior. Ensure active or already-published tasks cannot recreate detached results; test mid-delete failure and concurrent upload/rerun activation.

## Remediation order and acceptance criteria

1. **Restore and observe SMTP capability: B01–B03.** Verify actual provider permission, exit identity, greeting, and controlled SMTP behavior before declaring the network repaired. Route outage must remain distinct from recipient invalidity.
2. **Protect decisions and suppression: B04–B06, B23–B29.** Require the reproduced policy/catch-all/suppression cases to return correct evidence and conservative decisions. Verify the same behavior in API responses, stored results, list decisions, CSV exports, and generated contracts.
3. **Close network and export boundaries: B09–B10, B30–B31.** Test against isolated private destinations and controlled DNS changes, with no need to attack production. Confirm every shared caller uses the fix.
4. **Make work and quota recoverable: B11–B22, B35–B36.** Exercise concurrency and failures at transaction/publication/acknowledgement boundaries. A retry must reconcile the original operation and its charge, and parent status must reflect durable task state.
5. **Improve coverage and reputation semantics: B07–B08, B32–B34.** Add bounded MX failover, cleanup resilience, and explicit unknown/error states. Avoid claiming a reliable probability until outcomes support calibration.

For each implementation PR, add a regression that fails before the fix, run the focused Rust suite, and regenerate affected OpenAPI/SDK contracts when public response or request semantics change. For durable work, add isolated database/broker integration tests rather than only mocks of successful calls. Validate deployment identity and behavior separately after merge; local tests, a merged PR, and a working regional deployment are different milestones.

## Limits and additional observations

- **Exact direct-path cause remains open:** the Hetzner account restriction, provider firewall configuration, host firewall rules, and packet-level drop location were not confirmed. The observed failures are real; attributing all of them conclusively to Hetzner would exceed the evidence.
- **No network mutation or external mail effects:** no firewall rules, proxy subscriptions, DNS records, deployment settings, recipient-verification campaigns, or outbound messages were changed/sent. Production probes stopped at connectivity/authentication; classification tests used localhost.
- **Source-derived race/security findings need focused fault-injection tests:** they identify concrete code paths and triggers, but were not exploited against live tenants. The full external-provider/browser and complete database/broker integration suites were not run. Four existing core tests remained ignored.
- **International syntax support needs a product decision:** the local observations `δοκιμή@audit.example`, `user@bücher.de`, and `"a@b"@audit.example` all returned invalid syntax. This demonstrates current parser behavior, not that those particular mailboxes exist. Define supported syntax/IDNA/SMTPUTF8 explicitly, test it end to end, and distinguish unsupported forms from proven nonexistent addresses.
- **Already addressed behaviors were not reported as new bugs:** current core handling includes null MX and implicit MX; disposable-domain risk is not simply conflated with invalid syntax; known greylisting and some policy cases are recognized; API scope checks exist; malformed broker messages and publisher confirmation have handling. Those safeguards do not remove the narrower defects above.
- **No exhaustive correctness claim:** the review is broad and adversarial, but cannot prove the backend has no additional defects. External provider behavior, real-world accuracy, historical customer impact, replica lag, and production incidence of the source-derived failures remain unmeasured.
