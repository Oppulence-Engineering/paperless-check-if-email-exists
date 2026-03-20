# Customer Feature Backlog

This sheet captures high-value features that would be useful to customers after the current capability set. It is intentionally product-oriented: each item focuses on customer outcomes, not only internal implementation work.

Priority guide:

- `Critical`: likely to move conversion, retention, or day-to-day customer value immediately
- `High`: strongly useful and likely to reduce churn or expand usage
- `Medium`: helpful and strategic, but less urgent than core workflow improvements

The table below is sorted by overall priority rank, with `#1` as the highest-priority item.

## Completed

| # | Area | Feature | PR | Shipped |
|---|---|---|---|---|
| 1 | Verification | `safe_to_send` recommendation flag | [#15](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/pull/15) | Yes |
| 2 | Verification | Expanded reason-code taxonomy | [#16](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/pull/16) | Yes |
| 3 | Verification | Scheduled re-verification | [#19](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/pull/19) | Yes |
| 4 | Verification | Spam-trap / honeypot detection | [#17](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/pull/17) | Yes |
| 5 | List Cleaning | Workspace suppression list management | [#18](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/pull/18) | Yes |
| 6 | List Cleaning | Smart deduplication and canonicalization | [#20](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/pull/20) | Yes |
| 22 | Developer / API | Partial retry endpoint for failed bulk rows | [#21](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/pull/21) | Yes |
| 27 | Verification | Result freshness / confidence decay | [#21](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/pull/21) | Yes |
| 9 | Finder / Enrichment | Finder confidence explanation | [#22](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/pull/22) | Yes |
| 15 | Automation | Conditional actions based on score and category | [#22](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/pull/22) | Yes |
| 21 | Developer / API | Sandbox mode with deterministic mock results | [#22](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/pull/22) | Yes |
| 30 | Verification | Accept-all severity tiers | [#23](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/pull/23) | Yes |
| 31 | Verification | Domain typo correction suggestions | [#23](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/pull/23) | Yes |
| 32 | Verification | Alias and plus-address normalization | [#23](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/pull/23) | Yes |

## Remaining

| # | Area | Feature | Why customers would care | Priority |
|---|---|---|---|---|
| 7 | List Cleaning | One-click suppression sync back to ESPs and CRMs | Saves customers from manually pushing bad records into downstream systems | Critical |
| 8 | Finder / Enrichment | Contact waterfall search strategy | Increases match rates by chaining multiple candidate-generation and verification steps | Critical |
| ~~9~~ | ~~Finder / Enrichment~~ | ~~Finder confidence explanation~~ | ~~Shipped in PR #22~~ | ~~Done~~ |
| 10 | Integrations | Native HubSpot sync | Reduces manual export/import steps for CRM-driven teams | Critical |
| 11 | Integrations | Native Salesforce sync | Makes enterprise sales and marketing workflows much easier to operationalize | Critical |
| 12 | Integrations | Zapier connector | Expands automation reach for non-technical customers | Critical |
| 13 | Integrations | Google Sheets sync | Gives spreadsheet-heavy teams a native workflow instead of CSV round-trips | Critical |
| 14 | Automation | Scheduled list-cleaning pipelines | Lets customers keep recurring imports clean without manual uploads every time | Critical |
| ~~15~~ | ~~Automation~~ | ~~Conditional actions based on score and category~~ | ~~Shipped in PR #22~~ | ~~Done~~ |
| 16 | Verification | Provider-specific syntax validation | Prevents false positives by applying Gmail, Outlook, Yahoo, and other provider rules more precisely | Critical |
| 17 | Verification | Bounce-risk prediction model | Gives customers a forward-looking risk estimate beyond raw SMTP checks | Critical |
| 18 | Analytics / Reporting | Campaign outcome feedback loop | Lets customers feed bounce and engagement outcomes back into verification decisions | Critical |
| 19 | Analytics / Reporting | Deliverability trends dashboard | Helps customers monitor quality changes over time instead of treating each job in isolation | Critical |
| 20 | Finder / Enrichment | Account-based bulk people finder | Lets teams discover multiple likely contacts for a target company in one workflow | Critical |
| ~~21~~ | ~~Developer / API~~ | ~~Sandbox mode with deterministic mock results~~ | ~~Shipped in PR #22~~ | ~~Done~~ |
| ~~22~~ | ~~Developer / API~~ | ~~Partial retry endpoint for failed bulk rows~~ | ~~Shipped in PR #21~~ | ~~Done~~ |
| 23 | Team / Admin | SSO / SAML / SCIM | Required by many larger customers before they can adopt a new vendor | Critical |
| 24 | Team / Admin | Role-based access control | Prevents accidental access to lists, exports, and admin actions | Critical |
| 25 | Team / Admin | Retention policy controls | Lets customers define how long contact and verification data should be stored | Critical |
| 26 | Security / Compliance | Consent and provenance tracking | Lets customers retain where contact data came from and whether it can be used | Critical |
| ~~27~~ | ~~Verification~~ | ~~Result freshness / confidence decay~~ | ~~Shipped in PR #21~~ | ~~Done~~ |
| 28 | Verification | Historical verification timeline | Lets customers see how an address changed across checks over time | High |
| 29 | Verification | Mailbox status change alerts | Notifies teams when previously valid contacts become risky or invalid | High |
| ~~30~~ | ~~Verification~~ | ~~Accept-all severity tiers~~ | ~~Shipped in PR #23~~ | ~~Done~~ |
| ~~31~~ | ~~Verification~~ | ~~Domain typo correction suggestions~~ | ~~Shipped in PR #23~~ | ~~Done~~ |
| ~~32~~ | ~~Verification~~ | ~~Alias and plus-address normalization~~ | ~~Shipped in PR #23~~ | ~~Done~~ |
| 33 | Verification | Pre-send audience approval checklist | Gives teams a simple go or no-go summary before launching a campaign | High |
| 34 | Analytics / Reporting | Custom score-threshold policies | Lets teams define their own send, review, or suppress rules by workflow | High |
| 35 | List Cleaning | Saved segmentation rules | Makes it easy to repeatedly pull only valid, only risky, or only role-based subsets | High |
| 36 | List Cleaning | Data repair suggestions for bad rows | Helps customers fix malformed emails and broken CSV inputs quickly | High |
| 37 | List Cleaning | CSV and CRM column mapping wizard | Makes imports and exports much easier for non-technical users | High |
| 38 | List Cleaning | Export only changed or newly invalid rows | Reduces downstream churn for customers who only need incremental updates | High |
| 39 | List Cleaning | Custom export templates | Lets customers tailor output columns for ESPs, CRMs, and internal systems | High |
| 40 | List Cleaning | Multi-file upload jobs | Lets customers clean many CSVs in one run instead of one file at a time | High |
| 41 | List Cleaning | Resume interrupted uploads | Saves time on large imports and poor connections | High |
| 42 | List Cleaning | Automatic remediation workflows for bad rows | Fixes common formatting and parsing issues without manual cleanup | High |
| 43 | List Cleaning | Side-by-side original and cleaned data views | Helps teams review what changed and build trust in the cleaning process | High |
| 44 | List Cleaning | List-to-list diffing | Shows what improved, worsened, or changed between two cleanings | High |
| 45 | Finder / Enrichment | Company identification from domain | Turns a plain domain into usable company context for sales and ops teams | High |
| 46 | Finder / Enrichment | Basic firmographic enrichment | Adds company size, industry, and location to verified contacts | High |
| 47 | Finder / Enrichment | Job title enrichment | Makes results more actionable for prospecting and routing | High |
| 48 | Finder / Enrichment | Department-based finder presets | Supports searches for sales, support, finance, legal, and recruiting addresses | High |
| 49 | Finder / Enrichment | People search by company | Supports contact discovery when users start with a company, not a person | High |
| 50 | Finder / Enrichment | Team / role mailbox finder | Helps users locate functional inboxes like billing@ or partnerships@ | High |
| 51 | Finder / Enrichment | Alternative contact suggestions | Gives users fallback options when a primary contact is invalid or unavailable | High |
| 52 | Finder / Enrichment | Seniority and department enrichment | Helps customers prioritize and route verified contacts more effectively | High |
| 53 | Finder / Enrichment | LinkedIn and company-page matching | Gives sales teams more confidence that a found contact belongs to the right person and company | High |
| 54 | Integrations | Native Outreach sync | Helps SDR teams keep sequences clean and effective | High |
| 55 | Integrations | Native Salesloft sync | Supports sales-engagement teams with less CSV handling | High |
| 56 | Integrations | Native Marketo sync | Helps lifecycle marketing teams clean and enrich records where they already work | High |
| 57 | Integrations | Native Mailchimp sync | Gives smaller teams a direct path from verification to campaign execution | High |
| 58 | Integrations | Native Klaviyo sync | Helps ecommerce and lifecycle teams operationalize list quality quickly | High |
| 59 | Integrations | Native Pipedrive sync | Adds value for SMB sales teams with lightweight CRM workflows | High |
| 60 | Integrations | Native Close sync | Helps agencies and outbound teams keep contact data clean | High |
| 61 | Integrations | Make connector | Enables richer no-code automation for ops and growth teams | High |
| 62 | Integrations | n8n templates / connector | Supports self-hosted and technical automation users | High |
| 63 | Integrations | Reverse ETL and warehouse destinations | Lets customers push verified data back into broader analytics and data-stack workflows | High |
| 64 | Integrations | Snowflake and BigQuery export targets | Makes the platform more useful inside enterprise data environments | High |
| 65 | Automation | Webhook subscriptions by event type | Lets customers react differently to job completion, failures, and list uploads | High |
| 66 | Automation | Auto re-verify before sync or webhook delivery | Prevents stale results from being pushed into customer systems | High |
| 67 | Automation | Approval workflows before export or sync | Gives operators a governance checkpoint before risky or large-scale downstream actions | High |
| 68 | Automation | Retry dashboard for failed automations | Helps customers recover from integration failures without support intervention | High |
| 69 | Team / Admin | Audit log explorer and export | Helps with investigations, governance, and customer trust | High |
| 70 | Team / Admin | Endpoint- and action-level API key scopes | Makes integrations safer and easier to govern across teams | High |
| 71 | Team / Admin | Usage alerts and forecast dashboards | Helps customers avoid quota surprises and plan spend or capacity | High |
| 72 | Team / Admin | Shared saved views and filters | Makes it easier for teams to collaborate around the same operational slices of data | High |
| 73 | Team / Admin | Projects, folders, or workspaces for jobs and lists | Helps larger customers organize verification work by business unit or campaign | High |
| 74 | Team / Admin | Team-level chargeback and usage allocation | Helps agencies and multi-team companies understand who consumed which credits | High |
| 75 | Team / Admin | Regional data residency options | Important for compliance-sensitive customers and procurement reviews | High |
| 76 | Team / Admin | IP allowlists and network restrictions | Adds an enterprise control many security-conscious customers expect | High |
| 77 | Security / Compliance | Data-processing region lock | Gives customers a hard guarantee that processing stays in an approved geography | High |
| 78 | Security / Compliance | Customer-managed encryption keys | Gives security-sensitive buyers more control over stored data protection | High |
| 79 | Security / Compliance | Sensitive-field masking in UI and exports | Helps customers minimize exposure of contact data to the wrong users | High |
| 80 | Analytics / Reporting | Acquisition-source quality benchmarks | Shows which lead sources, vendors, or imports produce the worst data so customers can spend smarter | High |
| 81 | Analytics / Reporting | Provider and ISP breakdown reporting | Makes it easier to understand Gmail, Outlook, Yahoo, and corporate-domain risk patterns separately | High |
| 82 | Verification | Internationalized email support | Expands usability for customers with global audiences and non-ASCII addresses | High |
| 83 | Reliability / Insight | Cost-per-job and cost-per-source forecasting | Gives customers a better handle on budget efficiency and vendor quality | High |
| 84 | Reliability / Insight | Self-serve replay for failed jobs | Lets customers recover from transient failures without rebuilding entire workflows | High |
| 85 | Developer / API | API request and response explorer | Makes onboarding faster for developers integrating the platform directly | High |
| 86 | Developer / API | Historical query API for jobs, results, and changes | Gives advanced customers a better way to build internal dashboards and automations | High |
| 87 | Developer / API | Per-endpoint webhook signing secrets and rotation | Gives customers tighter security control over downstream automation endpoints | High |
| 88 | Team / Admin | Comments and annotations on jobs and lists | Gives teams a lightweight collaboration layer for reviews and handoffs | Medium |
| 89 | Reliability / Insight | Provider status dashboard and outage awareness | Helps customers interpret verification anomalies during provider or DNS incidents | Medium |
| 90 | Reliability / Insight | Historical provider-incident overlays | Helps customers separate their own data issues from temporary provider-side incidents | Medium |
| 91 | Reliability / Insight | Verification latency analytics | Helps customers understand throughput, bottlenecks, and SLA behavior over time | Medium |
| 92 | Reliability / Insight | SLA dashboard and service-credit reporting | Helps enterprise customers track uptime expectations and account health | Medium |
| 93 | Automation | Slack and Microsoft Teams alerts | Keeps sales and ops teams informed without checking the dashboard constantly | Medium |
| 94 | Verification | Inbox placement readiness signal | Gives marketers an extra pre-send quality cue beyond pure deliverability checks | Medium |
| 95 | List Cleaning | List quality benchmark reports | Gives teams a simple before/after quality score for uploads | Medium |
| 96 | List Cleaning | Cross-workspace duplicate detection | Avoids sending to the same contact from multiple pipelines or teams | Medium |
| 97 | Finder / Enrichment | Social profile enrichment | Helps teams confirm identity and personalize outreach | Medium |
| 98 | Finder / Enrichment | Technographic enrichment | Helps B2B customers segment and prioritize accounts by software stack | Medium |
| 99 | Finder / Enrichment | Enrichment source attribution | Shows where appended company and contact fields came from, which improves trust and governance | Medium |
| 100 | Security / Compliance | Legal hold support | Helps customers preserve relevant records for audits, disputes, or investigations | Medium |

## Suggested Build Order

If the goal is maximum customer value, the strongest next sequence would be:

1. ~~`safe_to_send`, richer reason codes, and spam-trap / honeypot detection~~ — **Done** (PRs #15-#17)
2. ~~scheduled re-verification, suppression lists, and smart deduplication~~ — **Done** (PRs #18-#20)
3. HubSpot, Salesforce, Zapier, and Google Sheets integrations
4. finder confidence explanations plus contact waterfall improvements
5. RBAC, SSO, retention controls, and audit log search

## Notes

- This is a backlog, not a promised delivery sequence.
- Several features can share infrastructure. For example, scheduled re-verification, change alerts, and freshness scoring all benefit from the same historical result model.
- Integrations are especially high leverage because they reduce manual handling and make the platform “stickier” inside customer workflows.
