# SMTP capability and outbound isolation

Production configuration enables `RCH__SMTP_HEALTH_ENABLED=true`. Set
`RCH__HELLO_NAME` to an owned public hostname and `RCH__FROM_EMAIL` to a valid
address on an owned domain. Startup rejects missing or local identities, including
provider overrides. The route monitor also requires the HELO hostname to resolve
exclusively to public addresses. This cannot prove ownership, matching reverse DNS,
provider permission or mailbox acceptance; verify those before deployment.

Every 60 seconds, each distinct configured SMTP route receives one bounded
20-second greeting probe to `gmail-smtp-in.l.google.com`. Probes send no MAIL,
RCPT or message data. The last probe becomes stale after 180 seconds. This is a
conservative service-wide gate: failure of any configured route pauses new
verification admission even if another provider might still work.

While capability is pending, stale or unavailable:

- New verification submissions return HTTP 503 before reserving quota.
- Queued verification remains recoverable; workers defer fresh SMTP work.
- Scheduled pipeline and reverification work waits without claiming new work.
- Stored results can finish required actions, and operation retrieval stays available
  on the backend.
- `/readyz` reports HTTP 503 and the cached SMTP status. The web readiness endpoint
  propagates it, so Kubernetes removes affected pods from service routing. This also
  removes ordinary web traffic until capability recovers.
- `/healthz` remains independent; a provider outage does not trigger a liveness restart.

Health requests never open SMTP connections. `/metrics` exposes
`reacher_smtp_monitor_enabled`, `reacher_smtp_capability_ready`, and
`reacher_smtp_capability_checked_timestamp_seconds`. The Helm
`smtpMonitoring.enabled` option adds a PodMonitor and PrometheusRule for unavailable
capability and missing metrics. Enable it only where the Prometheus Operator CRDs
exist, with `smtpMonitoring.labels` matching the installed discovery selectors.
Verify the scrape target, rule evaluation and alert receiver during deployment.
See [Prometheus alerting rules](https://prometheus.io/docs/prometheus/latest/configuration/alerting_rules/).

Set `RCH__SMTP_HEALTH_ENABLED=false` only for deliberate offline/development use.
This disables the monitor and admission guard and is reported as disabled.

The Helm chart enables egress isolation by default. It permits cluster DNS and
public destinations while excluding private, link-local, metadata, reserved and
transition ranges. Operator-managed private dependencies require exact `/32` or
`/128` addresses and explicit TCP ports in `networkPolicy.privateDestinations`.
Inventory database, broker and any private service destinations for each cluster
before rollout. DNS address rotation requires updating those entries.

NetworkPolicy enforcement depends on the cluster network implementation, and
policies combine additively. Verify enforcement with an isolated pod, inspect
other policies for broader grants, and check service address translation before
declaring the boundary effective. See
[Kubernetes NetworkPolicy](https://kubernetes.io/docs/concepts/services-networking/network-policies/).
Application-level destination validation and address pinning remain required.
