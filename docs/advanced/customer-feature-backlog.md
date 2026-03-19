# Customer Feature Backlog

This sheet captures high-value features that would be useful to customers after the current capability set. It is intentionally product-oriented: each item focuses on customer outcomes, not only internal implementation work.

Priority guide:

- `Critical`: likely to move conversion, retention, or day-to-day customer value immediately
- `High`: strongly useful and likely to reduce churn or expand usage
- `Medium`: helpful and strategic, but less urgent than core workflow improvements

| # | Area | Feature | Why customers would care | Priority |
|---|---|---|---|---|
| 1 | Verification | Provider-specific syntax validation | Prevents false positives by applying Gmail, Outlook, Yahoo, and other provider rules more precisely | Critical |
| 2 | Verification | Expanded reason-code taxonomy | Gives customers exact machine-readable causes for invalid, risky, and unknown outcomes | Critical |
| 3 | Verification | `safe_to_send` recommendation flag | Simplifies decision-making for marketers and ops teams without forcing them to interpret raw fields | Critical |
| 4 | Verification | Historical verification timeline | Lets customers see how an address changed across checks over time | High |
| 5 | Verification | Scheduled re-verification | Keeps lists fresh automatically before campaigns or outbound sequences | Critical |
| 6 | Verification | Result freshness / confidence decay | Helps customers understand when an old verification result should no longer be trusted | High |
| 7 | Verification | Mailbox status change alerts | Notifies teams when previously valid contacts become risky or invalid | High |
| 8 | Verification | Spam-trap / honeypot detection | Protects sender reputation and reduces deliverability risk | Critical |
| 9 | Verification | Accept-all severity tiers | Distinguishes between mild catch-all risk and high-risk accept-all domains | High |
| 10 | Verification | Bounce-risk prediction model | Gives customers a forward-looking risk estimate beyond raw SMTP checks | Critical |
| 11 | List Cleaning | Smart deduplication and canonicalization | Removes duplicate records across aliases and formatting differences | Critical |
| 12 | List Cleaning | Workspace suppression list management | Prevents teams from repeatedly emailing bounced, blocked, or unsubscribed contacts | Critical |
| 13 | List Cleaning | Custom export templates | Lets customers tailor output columns for ESPs, CRMs, and internal systems | High |
| 14 | List Cleaning | Saved segmentation rules | Makes it easy to repeatedly pull only valid, only risky, or only role-based subsets | High |
| 15 | List Cleaning | List-to-list diffing | Shows what improved, worsened, or changed between two cleanings | High |
| 16 | List Cleaning | Cross-workspace duplicate detection | Avoids sending to the same contact from multiple pipelines or teams | Medium |
| 17 | List Cleaning | Multi-file upload jobs | Lets customers clean many CSVs in one run instead of one file at a time | High |
| 18 | List Cleaning | Resume interrupted uploads | Saves time on large imports and poor connections | High |
| 19 | List Cleaning | Data repair suggestions for bad rows | Helps customers fix malformed emails and broken CSV inputs quickly | High |
| 20 | List Cleaning | List quality benchmark reports | Gives teams a simple before/after quality score for uploads | Medium |
| 21 | Finder / Enrichment | Company identification from domain | Turns a plain domain into usable company context for sales and ops teams | High |
| 22 | Finder / Enrichment | Basic firmographic enrichment | Adds company size, industry, and location to verified contacts | High |
| 23 | Finder / Enrichment | Job title enrichment | Makes results more actionable for prospecting and routing | High |
| 24 | Finder / Enrichment | Social profile enrichment | Helps teams confirm identity and personalize outreach | Medium |
| 25 | Finder / Enrichment | Technographic enrichment | Helps B2B customers segment and prioritize accounts by software stack | Medium |
| 26 | Finder / Enrichment | Finder confidence explanation | Shows why one candidate won, which improves trust in results | Critical |
| 27 | Finder / Enrichment | Department-based finder presets | Supports searches for sales, support, finance, legal, and recruiting addresses | High |
| 28 | Finder / Enrichment | Team / role mailbox finder | Helps users locate functional inboxes like billing@ or partnerships@ | High |
| 29 | Finder / Enrichment | People search by company | Supports contact discovery when users start with a company, not a person | High |
| 30 | Finder / Enrichment | Contact waterfall search strategy | Increases match rates by chaining multiple candidate-generation and verification steps | Critical |
| 31 | Integrations | Native HubSpot sync | Reduces manual export/import steps for CRM-driven teams | Critical |
| 32 | Integrations | Native Salesforce sync | Makes enterprise sales and marketing workflows much easier to operationalize | Critical |
| 33 | Integrations | Native Outreach sync | Helps SDR teams keep sequences clean and effective | High |
| 34 | Integrations | Native Salesloft sync | Supports sales-engagement teams with less CSV handling | High |
| 35 | Integrations | Native Pipedrive sync | Adds value for SMB sales teams with lightweight CRM workflows | High |
| 36 | Integrations | Native Close sync | Helps agencies and outbound teams keep contact data clean | High |
| 37 | Integrations | Zapier connector | Expands automation reach for non-technical customers | Critical |
| 38 | Integrations | Make connector | Enables richer no-code automation for ops and growth teams | High |
| 39 | Integrations | n8n templates / connector | Supports self-hosted and technical automation users | High |
| 40 | Integrations | Google Sheets sync | Gives spreadsheet-heavy teams a native workflow instead of CSV round-trips | Critical |
| 41 | Automation | Webhook subscriptions by event type | Lets customers react differently to job completion, failures, and list uploads | High |
| 42 | Automation | Slack and Microsoft Teams alerts | Keeps sales and ops teams informed without checking the dashboard constantly | Medium |
| 43 | Team / Admin | Role-based access control | Prevents accidental access to lists, exports, and admin actions | Critical |
| 44 | Team / Admin | SSO / SAML / SCIM | Required by many larger customers before they can adopt a new vendor | Critical |
| 45 | Team / Admin | Audit log explorer and export | Helps with investigations, governance, and customer trust | High |
| 46 | Team / Admin | Retention policy controls | Lets customers define how long contact and verification data should be stored | Critical |
| 47 | Team / Admin | Regional data residency options | Important for compliance-sensitive customers and procurement reviews | High |
| 48 | Team / Admin | Usage alerts and forecast dashboards | Helps customers avoid quota surprises and plan spend or capacity | High |
| 49 | Team / Admin | Endpoint- and action-level API key scopes | Makes integrations safer and easier to govern across teams | High |
| 50 | Reliability / Insight | Provider status dashboard and outage awareness | Helps customers interpret verification anomalies during provider or DNS incidents | Medium |

## Suggested Build Order

If the goal is maximum customer value, the strongest next sequence would be:

1. `safe_to_send`, richer reason codes, and spam-trap / honeypot detection
2. scheduled re-verification, suppression lists, and smart deduplication
3. HubSpot, Salesforce, Zapier, and Google Sheets integrations
4. finder confidence explanations plus contact waterfall improvements
5. RBAC, SSO, retention controls, and audit log search

## Notes

- This is a backlog, not a promised delivery sequence.
- Several features can share infrastructure. For example, scheduled re-verification, change alerts, and freshness scoring all benefit from the same historical result model.
- Integrations are especially high leverage because they reduce manual handling and make the platform “stickier” inside customer workflows.
