/**
 * Facts and reference tables for the feature, use-case, integration, and
 * guide pages, keyed by page path. Every value matches the backend: reason
 * codes from backend/src/scoring, event mapping from
 * backend/src/http/v1/provider_outcomes.rs, routes from backend/openapi.json.
 */

export type Reference = {
	title: string;
	columns: string[];
	rows: string[][];
};

export type PageFacts = {
	/** Four short facts shown in the strip under the hero: [label, value]. */
	specs: [string, string][];
	reference?: Reference;
};

const RESULT_FIELDS: Reference = {
	title: "What a result contains",
	columns: ["Field", "Values", "Meaning"],
	rows: [
		["category", "valid · risky · invalid · unknown", "The verdict for the address."],
		["score", "0 to 100", "Higher is safer to send."],
		["safe_to_send", "true · false", "One flag for your send rules."],
		["reason_codes", "List of codes", "Each code names one thing the check found."],
		["is_reachable", "safe · risky · invalid · unknown", "The raw reachability reading."],
	],
};

const REASON_CODES: Reference = {
	title: "Reason codes",
	columns: ["Code", "What the check found"],
	rows: [
		["deliverable", "No problem was found."],
		["invalid_syntax", "The address format is not valid."],
		["possible_typo", "The domain looks like a typo of a known domain."],
		["no_mx", "The domain has no mail server (MX record)."],
		["smtp_unreachable", "The mail server could not be reached."],
		["smtp_error", "The mail server returned an error."],
		["smtp_undeliverable", "The mail server did not accept the mailbox."],
		["invalid_recipient", "The mailbox does not exist."],
		["disabled_mailbox", "The mailbox is disabled."],
		["full_inbox", "The mailbox is full."],
		["catch_all", "The domain accepts every address."],
		["disposable", "The domain gives out throwaway addresses."],
		["role_account", "The address belongs to a role, such as info@ or support@."],
		["spam_trap", "The domain is a known spam-trap domain."],
		["free_provider", "The address is at a free mail provider."],
		["unknown_deliverability", "The mailbox could not be confirmed."],
		["provider_rejected", "The mailbox provider rejected the address."],
	],
};

const LIST_ACTIONS: Reference = {
	title: "Export files",
	columns: ["File", "Rows in it"],
	rows: [
		["safe", "Rows to keep as they are."],
		["fixed", "Rows with a corrected address, such as a fixed domain typo."],
		["review", "Rows to look at before you send."],
		["drop", "Rows to suppress."],
		["all", "Every row, with its result and its action."],
	],
};

const OUTCOME_EVENTS: Reference = {
	title: "Delivery events",
	columns: ["Event", "Recorded in history", "Suppression"],
	rows: [
		["delivered", "Yes", "—"],
		["bounced", "Yes", "Added, reason bounce"],
		["complained", "Yes", "Added, reason complaint"],
		["unsubscribed", "Yes", "Added, reason unsubscribe"],
		["opened", "Yes", "—"],
		["clicked", "Yes", "—"],
	],
};

const SUPPRESSION_REASONS: Reference = {
	title: "Suppression reasons",
	columns: ["Reason", "How the address got on the list"],
	rows: [
		["bounce", "Your email platform reported a bounce."],
		["complaint", "The recipient marked a message as spam."],
		["unsubscribe", "The recipient unsubscribed."],
		["invalid", "A check found the address invalid."],
		["auto_invalid", "Added after a check, from your workspace settings."],
		["spam_trap", "The address is at a known spam-trap domain."],
		["manual", "Someone on your team added it, or imported a file."],
	],
};

const API_ROUTES: Reference = {
	title: "Endpoints you will use most",
	columns: ["Method", "Path", "What it does"],
	rows: [
		["POST", "/v1/check_email", "Check one address."],
		["POST", "/v1/lists", "Upload a list for checking."],
		["GET", "/v1/lists/{list_id}/quality", "Read the quality of a list."],
		[
			"POST",
			"/v1/lists/{list_id}/remediation-exports",
			"Export the files to keep, fix, review, or drop.",
		],
		["GET", "/v1/suppressions/check", "Ask if an address is suppressed."],
		["POST", "/v1/outcomes", "Send delivery events from any platform."],
		["GET", "/v1/emails/{email}/history", "Read every result for one address."],
	],
};

const WORKSPACE_OBJECTS: Reference = {
	title: "What belongs to a workspace",
	columns: ["Object", "Shared with the team"],
	rows: [
		["Lists and results", "Yes"],
		["Check history", "Yes"],
		["Suppression list", "Yes"],
		["API keys", "Yes, each key has its own scope"],
		["Delivery endpoints", "Yes"],
		["Usage", "Yes, one total for the plan"],
	],
};

const CATEGORIES: Reference = {
	title: "The four categories",
	columns: ["Category", "Meaning", "Usual action"],
	rows: [
		["valid", "The mailbox exists and accepts mail.", "Keep"],
		["risky", "It may deliver, but a signal lowers the score.", "Review"],
		["invalid", "The address will bounce.", "Suppress"],
		["unknown", "The mail server did not give a clear answer.", "Check again later"],
	],
};

function providerEvents(rows: [string, string][]): Reference {
	return {
		title: "Event mapping",
		columns: ["Event from the platform", "Recorded as"],
		rows: rows.map(([source, recorded]) => [source, recorded]),
	};
}

export const PAGE_FACTS: Record<string, PageFacts> = {
	"/features/single-checks": {
		specs: [
			["Readings", "Format, domain, mail server, mailbox"],
			["Result", "Category, score 0–100, reasons"],
			["Where", "App, API, and SDKs"],
			["Email sent", "None"],
		],
		reference: RESULT_FIELDS,
	},
	"/features/list-cleaning": {
		specs: [
			["Input", "CSV, one address per row"],
			["Duplicates", "Found and grouped"],
			["Output", "A result and an action per row"],
			["Export", "Safe, fixed, review, drop"],
		],
		reference: LIST_ACTIONS,
	},
	"/features/risk-scoring": {
		specs: [
			["Score", "0 to 100"],
			["Categories", "Valid, risky, invalid, unknown"],
			["Reason codes", "17"],
			["Flag", "safe_to_send"],
		],
		reference: REASON_CODES,
	},
	"/features/delivery-feedback": {
		specs: [
			["Platforms", "SendGrid, Mailgun, Postmark, Amazon SES"],
			["Any other platform", "POST /v1/outcomes"],
			["Suppressed", "Bounces, complaints, unsubscribes"],
			["Delivery", "A private webhook URL"],
		],
		reference: OUTCOME_EVENTS,
	},
	"/features/suppressions": {
		specs: [
			["Sources", "Events, checks, imports, your team"],
			["Reasons", "7"],
			["Lookup", "GET /v1/suppressions/check"],
			["Files", "CSV import and export"],
		],
		reference: SUPPRESSION_REASONS,
	},
	"/features/reverification": {
		specs: [
			["Trigger", "A result older than your limit"],
			["Limit", "Set in days, per workspace"],
			["Runs", "In batches, on a schedule"],
			["Status", "GET /v1/reverification/status"],
		],
	},
	"/features/api": {
		specs: [
			["Style", "REST, JSON"],
			["SDKs", "Go and TypeScript"],
			["Keys", "Scoped, per workspace"],
			["Plans", "Professional and Enterprise"],
		],
		reference: API_ROUTES,
	},
	"/features/workspace": {
		specs: [
			["Roles", "Owner, Admin, Member"],
			["Sign-in", "Each teammate has an account"],
			["Scope", "Everything belongs to the organization"],
			["Seats", "From 3 on Free"],
		],
		reference: WORKSPACE_OBJECTS,
	},
	"/use-cases/email-marketing": {
		specs: [
			["When", "Before each campaign"],
			["Input", "A CSV from your platform"],
			["Output", "A clean file to import back"],
			["After the send", "Bounces come back as suppressions"],
		],
		reference: LIST_ACTIONS,
	},
	"/use-cases/marketing-operations": {
		specs: [
			["Compare", "Quality by list source"],
			["Report", "GET /v1/sources/quality"],
			["Fix", "Typos corrected in the export"],
			["Keep clean", "Scheduled re-checks"],
		],
	},
	"/use-cases/signup-forms": {
		specs: [
			["Call", "POST /v1/check_email"],
			["Typo hint", "A suggested address"],
			["Decide with", "category and safe_to_send"],
			["Plans", "Professional and Enterprise"],
		],
		reference: RESULT_FIELDS,
	},
	"/use-cases/sending-platforms": {
		specs: [
			["Gate", "List quality before a send"],
			["Hold", "Low-quality uploads for review"],
			["Notify", "A webhook when a job is done"],
			["Plans", "Enterprise for volume"],
		],
	},
	"/integrations/sendgrid": {
		specs: [
			["Source", "SendGrid Event Webhook"],
			["Verification", "Signed webhook and timestamp"],
			["Suppressed", "Bounces, spam reports, unsubscribes"],
			["Setup", "One private URL"],
		],
		reference: providerEvents([
			["delivered", "delivered"],
			["bounce", "bounced"],
			["spamreport", "complained"],
			["unsubscribe", "unsubscribed"],
			["open", "opened"],
			["click", "clicked"],
		]),
	},
	"/integrations/mailgun": {
		specs: [
			["Source", "Mailgun webhooks"],
			["Verification", "Signing key and timestamp"],
			["Suppressed", "Bounces, complaints, unsubscribes"],
			["Setup", "One private URL"],
		],
		reference: providerEvents([
			["delivered", "delivered"],
			["failed (permanent)", "bounced"],
			["complained", "complained"],
			["unsubscribed", "unsubscribed"],
			["opened", "opened"],
			["clicked", "clicked"],
		]),
	},
	"/integrations/postmark": {
		specs: [
			["Source", "Postmark webhooks"],
			["Verification", "Private URL token and IP allow list"],
			["Suppressed", "Bounces, spam complaints, unsubscribes"],
			["Setup", "One private URL"],
		],
		reference: providerEvents([
			["Delivery", "delivered"],
			["Bounce", "bounced"],
			["SpamComplaint", "complained"],
			["SubscriptionChange (suppressed)", "unsubscribed"],
			["Open", "opened"],
			["Click", "clicked"],
		]),
	},
	"/integrations/amazon-ses": {
		specs: [
			["Source", "Amazon SNS notifications"],
			["Verification", "SNS signature and topic ARN"],
			["Suppressed", "Bounces, complaints"],
			["Setup", "One private URL"],
		],
		reference: providerEvents([
			["Delivery", "delivered"],
			["Bounce", "bounced"],
			["Complaint", "complained"],
		]),
	},
	"/integrations/csv": {
		specs: [
			["Input", "Any CSV with an email column"],
			["Your columns", "Kept in the export"],
			["Added", "Result and action per row"],
			["Works with", "HubSpot, Salesforce, any tool"],
		],
		reference: LIST_ACTIONS,
	},
	"/integrations/api": {
		specs: [
			["Style", "REST, JSON"],
			["SDKs", "Go and TypeScript"],
			["Webhooks", "task.completed"],
			["Plans", "Professional and Enterprise"],
		],
		reference: API_ROUTES,
	},
	"/guides/reading-results": {
		specs: [
			["Categories", "4"],
			["Score", "0 to 100"],
			["Reason codes", "17"],
			["Flag", "safe_to_send"],
		],
		reference: CATEGORIES,
	},
	"/guides/catch-all-domains": {
		specs: [
			["Category", "Risky"],
			["Reason code", "catch_all"],
			["Why", "The server accepts every address"],
			["Action", "Review before you send"],
		],
	},
	"/guides/list-hygiene": {
		specs: [
			["Check", "Before each send"],
			["Suppress", "Bounces and complaints"],
			["Re-check", "Results older than your limit"],
			["Watch", "List quality by source"],
		],
		reference: SUPPRESSION_REASONS,
	},
};

/** "Without" and "With Verifier" rows for the first chapter. Row n of each list match. */
export type Compare = { without: string[]; with: string[] };

/** One block of a data sample: a request, a payload, a file, or a result. */
export type SamplePane = {
	label: string;
	lang: "json" | "csv" | "code" | "text";
	code: string;
};

/** Real data formats, shown in the second chapter. Values follow the backend rules. */
export type Sample = { title: string; caption: string; panes: SamplePane[] };

export const PAGE_COMPARE: Record<string, Compare> = {
	"/features/single-checks": {
		without: [
			"You find a bad address when it bounces.",
			"A typo such as gmial.com stays on the list.",
			"Role and catch-all addresses look like real people.",
		],
		with: [
			"Each address gets a verdict before the first send.",
			"A likely domain typo comes back as possible_typo.",
			"Reason codes name catch-all, role, and disposable addresses.",
		],
	},
	"/features/list-cleaning": {
		without: [
			"Duplicates and typos go out with the campaign.",
			"Each bad row costs a send and adds to the bounce rate.",
			"Someone fixes the file by hand in a spreadsheet.",
		],
		with: [
			"Duplicates are grouped, and typos get a corrected address.",
			"Each row gets one action: keep, fix, review, or drop.",
			"You import a clean file back into your platform.",
		],
	},
	"/features/risk-scoring": {
		without: [
			"Each tool uses different labels, so send rules differ by team.",
			"A yes-or-no answer hides why an address is risky.",
			"Catch-all and role addresses get the same treatment as people.",
		],
		with: [
			"One score from 0 to 100 and one of four categories.",
			"Reason codes tell why the score went down.",
			"One safe_to_send flag for your send rules.",
		],
	},
	"/features/delivery-feedback": {
		without: [
			"Bounces stay in the email platform.",
			"The same address comes back with the next import.",
			"Each platform needs its own cleanup step.",
		],
		with: [
			"Bounces, complaints, and unsubscribes come back to one workspace.",
			"Each of these events adds the address to the suppression list.",
			"One endpoint takes events from any platform.",
		],
	},
	"/features/suppressions": {
		without: [
			"Each tool keeps its own do-not-send list.",
			"A removed address comes back with the next import.",
			"Nobody knows why an address was blocked.",
		],
		with: [
			"One list for the workspace, from events, checks, and your team.",
			"One call tells you if an address is suppressed.",
			"Each entry keeps its reason and its history.",
		],
	},
	"/features/reverification": {
		without: [
			"Results get old while people change jobs.",
			"A list that was clean in spring bounces in autumn.",
			"Someone must remember to check the list again.",
		],
		with: [
			"You set the age limit in days for the workspace.",
			"Older results are checked again in batches.",
			"The status shows the last run and the next run.",
		],
	},
	"/features/api": {
		without: [
			"Signup forms accept any text with an @ sign.",
			"Checks run by hand in a separate tool.",
			"Each team writes its own HTTP client.",
		],
		with: [
			"Check the address in the request that creates the account.",
			"Upload lists and read results from your own code.",
			"Typed SDKs for Go and TypeScript.",
		],
	},
	"/features/workspace": {
		without: [
			"Lists and results sit in one person's account.",
			"API keys are shared in chat.",
			"Each teammate keeps a different suppression file.",
		],
		with: [
			"Lists, results, and suppressions belong to the workspace.",
			"Each API key has its own scopes and can be revoked.",
			"Everyone works from the same suppression list.",
		],
	},
	"/use-cases/email-marketing": {
		without: [
			"An old list goes out, and the bounce rate goes up.",
			"Mailbox providers move the next campaign to spam.",
			"Bounced addresses come back with the next import.",
		],
		with: [
			"The list is checked the day before the send.",
			"Only rows marked safe or fixed go to the platform.",
			"Bounces after the send become suppressions.",
		],
	},
	"/use-cases/marketing-operations": {
		without: [
			"Each list source looks the same in the CRM.",
			"Bad sources are found after the damage.",
			"Old records stay until someone cleans them.",
		],
		with: [
			"Quality is reported for each list source.",
			"A weak source is visible before the next campaign.",
			"Scheduled re-checks keep old records current.",
		],
	},
	"/use-cases/signup-forms": {
		without: [
			"Throwaway addresses create trial accounts.",
			"A mistyped address never gets the welcome email.",
			"Fake signups skew your funnel numbers.",
		],
		with: [
			"The form checks the address before it creates the account.",
			"A likely typo comes back as possible_typo.",
			"Disposable addresses get a reason code you can block on.",
		],
	},
	"/use-cases/sending-platforms": {
		without: [
			"A customer uploads a bad list, and your shared IPs pay for it.",
			"Your team finds the problem after the bounces.",
			"Every customer gets the same trust.",
		],
		with: [
			"Each upload gets a quality check before it is sent.",
			"A low-quality job is marked not ready to send.",
			"A webhook tells your system when the job is done.",
		],
	},
	"/integrations/sendgrid": {
		without: [
			"Bounces and spam reports stay in SendGrid.",
			"Imports from other tools bring the same addresses back.",
			"Suppression lists differ from tool to tool.",
		],
		with: [
			"Each SendGrid event is checked for a valid signature.",
			"Bounces, spam reports, and unsubscribes join one suppression list.",
			"Every event shows in the address history.",
		],
	},
	"/integrations/mailgun": {
		without: [
			"Permanent failures stay in the Mailgun logs.",
			"Imports from other tools bring the same addresses back.",
			"Suppression lists differ from tool to tool.",
		],
		with: [
			"Each Mailgun event is checked with your signing key.",
			"Permanent failures, complaints, and unsubscribes join one list.",
			"Every event shows in the address history.",
		],
	},
	"/integrations/postmark": {
		without: [
			"Bounces and spam complaints stay in Postmark.",
			"Imports from other tools bring the same addresses back.",
			"Suppression lists differ from tool to tool.",
		],
		with: [
			"Each Postmark event must carry your private URL token.",
			"Bounces, spam complaints, and unsubscribes join one list.",
			"Every event shows in the address history.",
		],
	},
	"/integrations/amazon-ses": {
		without: [
			"Bounce notifications stay in an SNS topic.",
			"Imports from other tools bring the same addresses back.",
			"Suppression lists differ from tool to tool.",
		],
		with: [
			"Each SNS message is checked for its signature and topic ARN.",
			"Bounces and complaints join one suppression list.",
			"Every event shows in the address history.",
		],
	},
	"/integrations/csv": {
		without: [
			"Each tool exports a different file layout.",
			"Checking a list means copying columns by hand.",
			"Results come back without your own columns.",
		],
		with: [
			"Any CSV with an email column works.",
			"Your columns stay in the file you download.",
			"A result and an action are added to each row.",
		],
	},
	"/integrations/api": {
		without: [
			"Checks run by hand in a separate tool.",
			"Your system polls for finished jobs.",
			"Each team writes its own HTTP client.",
		],
		with: [
			"Check addresses and lists from your own code.",
			"A task.completed webhook tells you when a job is done.",
			"Typed SDKs for Go and TypeScript.",
		],
	},
};

const SDK_SAMPLE: Sample = {
	title: "One check from your code",
	caption: "Generated SDKs · same call in each language",
	panes: [
		{
			label: "TypeScript",
			lang: "code",
			code: `import { Configuration, V1Api } from "@oppulence/reacher-sdk";

const api = new V1Api(
  new Configuration({
    basePath: process.env.VERIFIER_API_URL,
    apiKey: \`Bearer \${process.env.VERIFIER_API_KEY}\`,
  }),
);

const { data } = await api.v1CheckEmail({
  checkEmailRequest: { to_email: "maya@brightlane.co" },
});

if (!data.score.safe_to_send) {
  // Ask the person to check the address.
}`,
		},
		{
			label: "Go",
			lang: "code",
			code: `cfg := reacher.NewConfiguration()
cfg.Servers = reacher.ServerConfigurations{{URL: os.Getenv("VERIFIER_API_URL")}}
cfg.AddDefaultHeader("Authorization", "Bearer "+os.Getenv("VERIFIER_API_KEY"))
client := reacher.NewAPIClient(cfg)

out, _, err := client.V1API.V1CheckEmail(ctx).
	CheckEmailRequest(*reacher.NewCheckEmailRequest("maya@brightlane.co")).
	Execute()`,
		},
	],
};

const LIST_SAMPLE: Sample = {
	title: "Rows in, actions out",
	caption: "Export columns trimmed · your own columns stay in the file",
	panes: [
		{
			label: "newsletter.csv",
			lang: "csv",
			code: `email,first_name
ada@acme.io,Ada
sam@gmial.com,Sam
info@northwind-labs.com,
old@closed-shop.com,Lee`,
		},
		{
			label: "Export · all rows",
			lang: "csv",
			code: `original_email,effective_email,remediation_classification
ada@acme.io,ada@acme.io,safe
sam@gmial.com,sam@gmail.com,fixed
info@northwind-labs.com,info@northwind-labs.com,review
old@closed-shop.com,old@closed-shop.com,drop`,
		},
	],
};

const RISKY_RESULT = `{
  "is_reachable": "risky",
  "score": {
    "score": 65,
    "category": "risky",
    "safe_to_send": false,
    "reason_codes": ["catch_all", "role_account"]
  }
}`;

/** A provider event in, the response, and the suppression it creates. */
function providerSample(name: string, provider: string, payload: string, verifies: string): Sample {
	return {
		title: `One bounce from ${name}`,
		caption: `${verifies} · fields trimmed`,
		panes: [
			{ label: `Sent by ${name}`, lang: "json", code: payload },
			{
				label: `Verifier · POST /v1/inbound/providers/${provider}/{endpoint}/{token}`,
				lang: "json",
				code: `{
  "provider": "${provider}",
  "accepted": 1,
  "duplicates": 0,
  "rejected": 0
}`,
			},
			{
				label: "GET /v1/suppressions/check?email=jane@acme.io",
				lang: "json",
				code: `{
  "suppressed": true,
  "reason": "bounce",
  "source": "${provider}"
}`,
			},
		],
	};
}

export const PAGE_SAMPLES: Record<string, Sample> = {
	"/features/single-checks": {
		title: "A risky address",
		caption: "Response trimmed to the score",
		panes: [
			{
				label: "POST /v1/check_email",
				lang: "json",
				code: `{
  "to_email": "info@northwind-labs.com"
}`,
			},
			{ label: "Response", lang: "json", code: RISKY_RESULT },
		],
	},
	"/features/list-cleaning": LIST_SAMPLE,
	"/features/risk-scoring": {
		title: "Two addresses, one scale",
		caption: "Valid 80–100 · risky 50–79 · unknown 1–49 · invalid 0",
		panes: [
			{
				label: "maya@brightlane.co",
				lang: "json",
				code: `{
  "score": 100,
  "category": "valid",
  "safe_to_send": true,
  "reason_codes": ["deliverable"]
}`,
			},
			{
				label: "info@northwind-labs.com · catch-all domain, role address",
				lang: "json",
				code: `{
  "score": 65,
  "category": "risky",
  "safe_to_send": false,
  "reason_codes": ["catch_all", "role_account"]
}`,
			},
		],
	},
	"/features/delivery-feedback": {
		title: "Events from any platform",
		caption: "Bounces, complaints, and unsubscribes are suppressed",
		panes: [
			{
				label: "POST /v1/outcomes",
				lang: "json",
				code: `{
  "provider": "custom",
  "outcomes": [
    { "email": "jane@acme.io", "event_type": "bounced" },
    { "email": "lee@acme.io", "event_type": "delivered" }
  ]
}`,
			},
			{
				label: "Response",
				lang: "json",
				code: `{
  "ingested": 2,
  "auto_suppressed": 1,
  "ignored": 0
}`,
			},
		],
	},
	"/features/suppressions": {
		title: "Ask before you send",
		caption: "Fields trimmed",
		panes: [
			{
				label: "Request",
				lang: "text",
				code: "GET /v1/suppressions/check?email=jane@acme.io",
			},
			{
				label: "Response",
				lang: "json",
				code: `{
  "suppressed": true,
  "reason": "bounce",
  "source": "sendgrid",
  "created_at": "2026-10-16T09:12:44Z"
}`,
			},
		],
	},
	"/features/reverification": {
		title: "Turn on re-checks",
		caption: "Status fields trimmed",
		panes: [
			{
				label: "PUT /v1/reverification",
				lang: "json",
				code: `{
  "enabled": true,
  "staleness_days": 30,
  "batch_size": 100
}`,
			},
			{
				label: "GET /v1/reverification/status",
				lang: "json",
				code: `{
  "enabled": true,
  "staleness_days": 30,
  "batch_size": 100,
  "next_run_at": "2026-10-16T10:00:00Z"
}`,
			},
		],
	},
	"/features/api": SDK_SAMPLE,
	"/use-cases/email-marketing": LIST_SAMPLE,
	"/use-cases/signup-forms": SDK_SAMPLE,
	"/use-cases/sending-platforms": {
		title: "A job that is not ready",
		caption: "1,300 of 2,000 rows are safe to send · fields trimmed",
		panes: [
			{
				label: "Request",
				lang: "text",
				code: "GET /v1/jobs/4182/approval",
			},
			{
				label: "Response",
				lang: "json",
				code: `{
  "job_id": 4182,
  "total_records": 2000,
  "safe_to_send_count": 1300,
  "safe_to_send_pct": 65.0,
  "ready_to_send": false,
  "recommendation": "List quality is moderate. Remove invalid and risky addresses before sending."
}`,
			},
		],
	},
	"/integrations/sendgrid": providerSample(
		"SendGrid",
		"sendgrid",
		`[
  {
    "email": "jane@acme.io",
    "event": "bounce",
    "type": "bounce",
    "reason": "550 5.1.1 User unknown",
    "timestamp": 1760605964,
    "sg_event_id": "ZGVsaXZlcmVkLTA..."
  }
]`,
		"Signed with your SendGrid verification key",
	),
	"/integrations/mailgun": providerSample(
		"Mailgun",
		"mailgun",
		`{
  "signature": {
    "timestamp": "1760605964",
    "token": "a8ce0edb2dd8...",
    "signature": "d2271d12299f..."
  },
  "event-data": {
    "event": "failed",
    "severity": "permanent",
    "recipient": "jane@acme.io"
  }
}`,
		"Signed with your Mailgun signing key",
	),
	"/integrations/postmark": providerSample(
		"Postmark",
		"postmark",
		`{
  "RecordType": "Bounce",
  "Type": "HardBounce",
  "Email": "jane@acme.io",
  "BouncedAt": "2026-10-16T09:12:44Z",
  "MessageID": "883953f4-6105-42a2..."
}`,
		"Private URL token, with an optional IP allow list",
	),
	"/integrations/amazon-ses": providerSample(
		"Amazon SES",
		"ses",
		`{
  "Type": "Notification",
  "TopicArn": "arn:aws:sns:us-east-1:123456789012:ses-events",
  "MessageId": "f3c1a2b4-...",
  "Message": "{\\"notificationType\\":\\"Bounce\\",\\"mail\\":{\\"destination\\":[\\"jane@acme.io\\"]}}",
  "Signature": "EXAMPLEpH+..."
}`,
		"SNS signature and your topic ARN",
	),
	"/integrations/csv": {
		title: "Your columns stay",
		caption: "Result columns trimmed",
		panes: [
			{
				label: "contacts.csv · exported from HubSpot",
				lang: "csv",
				code: `Email,First Name,Lifecycle Stage
ada@acme.io,Ada,customer
old@closed-shop.com,Lee,lead`,
			},
			{
				label: "Download · results added",
				lang: "csv",
				code: `Email,First Name,Lifecycle Stage,is_reachable,score,category,safe_to_send
ada@acme.io,Ada,customer,safe,100,valid,true
old@closed-shop.com,Lee,lead,invalid,0,invalid,false`,
			},
		],
	},
	"/integrations/api": SDK_SAMPLE,
};
