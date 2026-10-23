# Verify product focus

Updated 2026-10-23. Verify is a PlaybookMedia product built on the Paperless application and separate Go research backend. This replaces earlier sender-only and low-cost discovery/parity positioning.

The authoritative business documents are [company.md](../../company.md) and [vision.md](../../vision.md), derived from the [October 2026 thesis](verify-business-thesis-2026-10.md). The [codebase audit](../audits/2026-10-23-verify-north-star.md) records what exists; the [RFC index](../rfcs/README.md) is the single build order.

Start with US B2B outbound agencies managing approximately 3–20 client lists. Test a paid, fixed-scope $249 audit of customer-owned records: assess company, person, employment, and role; selectively repair; verify; review evidence; deliver Ready, Needs Review, and Excluded records with original IDs and corrections. Prices and future $149–$299 recurring plans are experiments, not changes to subscription entitlements.

The north star is total cost per correctly matched, usable, role-relevant contact, including failed research and human review. Require five paid audits, three repeat payers, positive contribution margin, and measured quality or cleanup-time benefit before broader investment. These are future gates, not achieved results.

Reuse Go for research/evidence and Rust for verification, tenant authority, suppression, policies, and audit delivery. Keep credentials server-side; managed checks use MillionVerifier without SMTP fallback. Keep identity, employment, role, and mailbox claims distinct. Ordinary Parallel terms do not establish permission for a resale database or transfer of Verify customer records to the internal finance graph.

The connected source supports a ten-company / three-candidate / thirty-verification pilot with approved/review CSV modes. Durable human decisions, complete source-ID exports, dollar budgets, agency-scale execution, CRM writeback, and recurring maintenance remain the work in RFCs 0027–0033. Broad discovery, newsletter/sender tooling, and MCP distribution wait for demand.
