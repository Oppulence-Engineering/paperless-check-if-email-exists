import type { MarketingFaqItem } from "./marketing-faq";

export type CatalogLink = { label: string; href: string; description?: string };
export type CatalogSection = { title: string; body: string };

export type CapabilityPage = {
	kind: "feature" | "use-case" | "integration" | "guide";
	slug: string;
	path: string;
	eyebrow: string;
	title: string;
	description: string;
	lede: string;
	screenshot: string;
	screenshotAlt: string;
	problem: CatalogSection;
	howItWorks: CatalogSection;
	workflow: string[];
	capabilities: CatalogSection[];
	examples: CatalogSection[];
	integrations?: CatalogLink[];
	security?: string;
	faqs: MarketingFaqItem[];
	related: CatalogLink[];
};

export const homepageFaqs: MarketingFaqItem[] = [
	{
		question: "Does a check send an email?",
		answer:
			"No. A check reads the address format, the domain's DNS and MX records, and the answer of the mail server. Nothing is delivered to the recipient.",
	},
	{
		question: "What does a result tell me?",
		answer:
			"Each address gets a category (valid, risky, invalid, or unknown), a score from 0 to 100, a safe-to-send flag, and reason codes such as catch-all, disposable, role account, or full inbox.",
	},
	{
		question: "Is every result certain?",
		answer:
			"No. Some mail servers accept every address or hide whether a mailbox exists. We mark these results as risky or unknown and show the reason, so you can decide.",
	},
	{
		question: "Can I keep a list clean after the first check?",
		answer:
			"Yes. Schedule re-checks for older addresses, keep a suppression list, and connect your email platform so bounces and complaints are suppressed automatically.",
	},
	{
		question: "Can my team share a workspace?",
		answer:
			"Yes. Lists, results, history, suppressions, and API keys belong to your organization, and each teammate signs in with their own account.",
	},
];

const preview = "/marketing/email-check-preview.svg";
const previewAlt = "Sample email check result";

type PageContent = Omit<CapabilityPage, "kind" | "slug" | "path" | "screenshot" | "screenshotAlt">;

function page(kind: CapabilityPage["kind"], slug: string, content: PageContent): CapabilityPage {
	const prefix = kind === "use-case" ? "use-cases" : kind === "guide" ? "guides" : `${kind}s`;
	return {
		kind,
		slug,
		path: `/${prefix}/${slug}`,
		screenshot: preview,
		screenshotAlt: previewAlt,
		...content,
	};
}

const startFree: CatalogLink = { label: "Start free", href: "/sign-up" };
const seePricing: CatalogLink = { label: "See pricing", href: "/pricing" };

export const featurePages: CapabilityPage[] = [
	page("feature", "single-checks", {
		eyebrow: "Single checks",
		title: "Check one address before you send",
		description:
			"Get a clear answer for one address: valid, risky, invalid, or unknown, with a score and the reasons behind it.",
		lede: "Type an address and get a result in seconds. No email is sent to the recipient.",
		problem: {
			title: "A typo can cost you the conversation",
			body: "An address can look right and still bounce. The domain may have no mail server, the mailbox may be gone, or the address may be a throwaway.",
		},
		howItWorks: {
			title: "Every check reads the full path to the inbox",
			body: "We check the address format, the domain and its mail servers, and how the mail server answers for that mailbox. Then we score the result and explain it.",
		},
		workflow: [
			"Enter an address.",
			"See the category, the score, and the safe-to-send flag.",
			"Open the details to see each signal and reason code.",
		],
		capabilities: [
			{
				title: "Reasons you can act on",
				body: "Reason codes name the problem: no mail server, disabled mailbox, full inbox, catch-all domain, disposable address, role account, or a likely typo.",
			},
			{
				title: "History for every address",
				body: "See earlier results for the same address, so you know if it changed.",
			},
		],
		examples: [
			{
				title: "Before an important email",
				body: "Check a new contact before you send a proposal or an invoice.",
			},
		],
		faqs: [
			{
				question: "Does a check send an email?",
				answer: "No. The check stops before any message is delivered.",
			},
			{
				question: "What if the result is unknown?",
				answer:
					"Some mail servers do not say if a mailbox exists. Unknown means we could not confirm it. Treat it with care, or check it again later.",
			},
		],
		related: [
			{ label: "Scores and reasons", href: "/features/risk-scoring" },
			{ label: "Clean a whole list", href: "/features/list-cleaning" },
			startFree,
		],
	}),
	page("feature", "list-cleaning", {
		eyebrow: "List cleaning",
		title: "Clean a list before your next campaign",
		description:
			"Upload a CSV and get a result for every address, a quality score for the list, and clean files to import back.",
		lede: "Find the addresses that will bounce before your email platform does.",
		problem: {
			title: "Old lists hurt your sender reputation",
			body: "People change jobs, mailboxes close, and typos pile up. When too many messages bounce, mailbox providers send the rest to spam.",
		},
		howItWorks: {
			title: "Upload, check, and export",
			body: "Upload a CSV, and we check every address. You get a list quality score, the result for each row, and a fix plan you can export.",
		},
		workflow: [
			"Upload a CSV from your CRM or email platform.",
			"Review the quality score and the results by category.",
			"Export the rows to keep, fix, review, or suppress.",
		],
		capabilities: [
			{
				title: "A fix plan, not only a label",
				body: "Each row gets a recommendation: keep it, fix it, review it, or suppress it. Duplicates are flagged too.",
			},
			{
				title: "List quality score",
				body: "See the share of valid, risky, invalid, and unknown addresses, so you can compare lists and sources.",
			},
		],
		examples: [
			{
				title: "Before a launch email",
				body: "Clean the full list the day before a big send.",
			},
			{
				title: "After an event",
				body: "Check badge-scan and signup exports before they enter your nurture flow.",
			},
		],
		faqs: [
			{
				question: "What file format do you need?",
				answer: "A CSV with one address in each row. Other columns are kept in the export.",
			},
			{
				question: "Can I see why a row was removed?",
				answer: "Yes. Each row keeps its category, score, and reason codes.",
			},
		],
		related: [
			{ label: "Suppression list", href: "/features/suppressions" },
			{ label: "Keep lists clean over time", href: "/features/reverification" },
			seePricing,
		],
	}),
	page("feature", "risk-scoring", {
		eyebrow: "Scores and reasons",
		title: "Know why an address is risky",
		description:
			"Each result has a score from 0 to 100, a category, a safe-to-send flag, and the reason codes behind them.",
		lede: "A single label hides too much. We show the score and every reason, so your team can set its own rules.",
		problem: {
			title: "Risky is not one thing",
			body: "A catch-all company domain, a disposable address, and a full inbox are different risks. A single label treats them the same.",
		},
		howItWorks: {
			title: "One score, many signals",
			body: "We combine format, domain, mail server, and mailbox signals into one score. Reason codes show what moved it.",
		},
		workflow: [
			"Run a check or a list.",
			"Read the score, the category, and the safe-to-send flag.",
			"Use the reason codes to build your own rules.",
		],
		capabilities: [
			{
				title: "Catch-all domains graded",
				body: "Domains that accept every address get a severity tier, so a clean free-provider address is not treated like a risky corporate one.",
			},
			{
				title: "Policies that match the send",
				body: "Set the policy for your workspace in Settings: growth, deliverability, or signup protection. The same signals give a decision that fits.",
			},
		],
		examples: [
			{
				title: "Segment by risk",
				body: "Send to safe addresses now, and hold risky ones for a smaller send.",
			},
		],
		faqs: [
			{
				question: "What is a catch-all domain?",
				answer:
					"A domain that accepts mail for any address. A check cannot confirm the mailbox, so we grade the risk instead of guessing.",
			},
		],
		related: [
			{ label: "Catch-all domains explained", href: "/guides/catch-all-domains" },
			{ label: "Reading results", href: "/guides/reading-results" },
			startFree,
		],
	}),
	page("feature", "delivery-feedback", {
		eyebrow: "Delivery feedback",
		title: "Learn from what actually bounced",
		description:
			"Connect SendGrid, Mailgun, Postmark, or Amazon SES. Bounces, complaints, and unsubscribes come back and update your suppression list.",
		lede: "A check tells you what should happen. Delivery feedback tells you what did happen.",
		problem: {
			title: "Bounces repeat when nobody closes the loop",
			body: "When a bounce stays inside the email platform, the same bad address comes back in the next import.",
		},
		howItWorks: {
			title: "Your email platform reports back",
			body: "Add a webhook in your email platform that points to your workspace. Each delivery event is matched to the address and recorded.",
		},
		workflow: [
			"Create a delivery endpoint in your workspace.",
			"Paste its private URL into your email platform's webhook settings.",
			"Bounces, complaints, and unsubscribes update your suppressions.",
		],
		capabilities: [
			{
				title: "Signed and private",
				body: "Each endpoint has its own private URL. SendGrid, Mailgun, and Amazon SES events are signature-checked.",
			},
			{
				title: "Any other platform",
				body: "Send delivery events from any tool through the outcomes API on Professional and Enterprise. On every plan, you can add an event in the app.",
			},
		],
		examples: [
			{
				title: "Stop repeat bounces",
				body: "An address that bounced last month is suppressed before the next campaign.",
			},
		],
		integrations: [
			{ label: "SendGrid", href: "/integrations/sendgrid" },
			{ label: "Mailgun", href: "/integrations/mailgun" },
			{ label: "Postmark", href: "/integrations/postmark" },
			{ label: "Amazon SES", href: "/integrations/amazon-ses" },
		],
		faqs: [
			{
				question: "Which events are suppressed?",
				answer: "Bounces, spam complaints, and unsubscribes.",
			},
		],
		related: [
			{ label: "Suppression list", href: "/features/suppressions" },
			{ label: "All integrations", href: "/integrations" },
			startFree,
		],
	}),
	page("feature", "suppressions", {
		eyebrow: "Suppression list",
		title: "Never send to a known bad address again",
		description:
			"Keep one suppression list for your workspace. Import it, export it, and check it before each send.",
		lede: "Bounces, complaints, unsubscribes, and invalid results all end up in one place.",
		problem: {
			title: "Suppressions get lost between tools",
			body: "Each email platform keeps its own list. When you switch tools or import a new file, old bounces come back.",
		},
		howItWorks: {
			title: "One list, many sources",
			body: "Add suppressions by hand, by import, from invalid results, or from delivery feedback. Each entry keeps its reason and its history.",
		},
		workflow: [
			"Import your current suppression list.",
			"Let invalid results and delivery feedback add new entries.",
			"Check addresses against the list before each send, or export it to your email platform.",
		],
		capabilities: [
			{
				title: "Reasons and history",
				body: "Each entry records why it was suppressed: bounce, complaint, unsubscribe, spam trap, invalid, or manual.",
			},
			{
				title: "Check before you send",
				body: "Check an address against the list in the app or through the API.",
			},
		],
		examples: [
			{
				title: "Switching email platforms",
				body: "Bring your suppressions with you when you move to a new platform.",
			},
		],
		faqs: [
			{
				question: "Can I remove an entry?",
				answer: "Yes. Removing an entry is recorded in its history.",
			},
		],
		related: [
			{ label: "Delivery feedback", href: "/features/delivery-feedback" },
			{ label: "List cleaning", href: "/features/list-cleaning" },
			startFree,
		],
	}),
	page("feature", "reverification", {
		eyebrow: "Ongoing re-checks",
		title: "Keep lists clean as they age",
		description:
			"Schedule re-checks for addresses that have not been checked recently, and verify new contacts as they arrive.",
		lede: "A list that was clean in January is not clean in June. Re-checks catch the change before you send.",
		problem: {
			title: "Every list decays",
			body: "People leave companies, and mailboxes close. A list you cleaned once slowly fills with addresses that will bounce.",
		},
		howItWorks: {
			title: "Re-check on a schedule",
			body: "Choose how old a result can be. Addresses older than that are checked again automatically.",
		},
		workflow: [
			"Set how many days a result stays fresh.",
			"We re-check stale addresses in batches.",
			"Review what changed, and suppress what went bad.",
		],
		capabilities: [
			{
				title: "Pipelines for new contacts",
				body: "Push new contacts to a pipeline through the API, and each one is checked before it reaches your email platform. The API comes with Professional and Enterprise.",
			},
			{
				title: "Results delivered to you",
				body: "Get results by webhook when a run finishes.",
			},
		],
		examples: [
			{
				title: "Monthly hygiene",
				body: "Re-check every address older than 30 days before your monthly newsletter.",
			},
		],
		faqs: [
			{
				question: "Do re-checks use my monthly checks?",
				answer: "Yes. Each re-checked address counts as one check.",
			},
		],
		related: [
			{ label: "List hygiene guide", href: "/guides/list-hygiene" },
			{ label: "API and SDKs", href: "/features/api" },
			seePricing,
		],
	}),
	page("feature", "api", {
		eyebrow: "API and SDKs",
		title: "Add checks to your product",
		description:
			"Check addresses in real time at signup, run list jobs, and get results by webhook. Use the REST API with Go and TypeScript SDKs.",
		lede: "Everything in the app is also available through the API.",
		problem: {
			title: "Bad addresses enter through your own forms",
			body: "Typos and throwaway addresses at signup become bounces, fake accounts, and support tickets later.",
		},
		howItWorks: {
			title: "One call per address, one job per list",
			body: "Call the check endpoint from your signup flow, or send a list as a job and get the results by webhook.",
		},
		workflow: [
			"Create an API key with the scopes you need.",
			"Call the API from your backend.",
			"Act on the category, the score, and the reason codes.",
		],
		capabilities: [
			{
				title: "Scoped API keys",
				body: "Give each key only the access it needs, and revoke it at any time.",
			},
			{
				title: "Safe retries",
				body: "Idempotency keys stop a retried request from creating a second job.",
			},
		],
		examples: [
			{
				title: "Signup protection",
				body: "Ask the user to fix a likely typo before the account is created.",
			},
		],
		faqs: [
			{
				question: "Which plans include the API?",
				answer: "The Professional and Enterprise plans.",
			},
		],
		related: [
			{ label: "Signup forms", href: "/use-cases/signup-forms" },
			{ label: "REST API and SDKs", href: "/integrations/api" },
			seePricing,
		],
	}),
	page("feature", "workspace", {
		eyebrow: "Team workspace",
		title: "Work on lists as a team",
		description:
			"Share lists, results, history, and suppressions in one organization. Invite teammates and manage API keys in one place.",
		lede: "Everyone sees the same results, so nobody cleans the same list twice.",
		problem: {
			title: "Checks get lost in spreadsheets",
			body: "When each person cleans lists on their own, results are repeated, lost, or out of date.",
		},
		howItWorks: {
			title: "One organization, shared work",
			body: "Lists, jobs, history, suppressions, and settings belong to your organization. Each teammate signs in with their own account.",
		},
		workflow: [
			"Create your organization.",
			"Invite your teammates.",
			"Work on lists, suppressions, and settings together.",
		],
		capabilities: [
			{
				title: "Comments on jobs",
				body: "Leave notes on a list job, so the next person knows what was decided.",
			},
			{
				title: "Provisioning for larger teams",
				body: "Enterprise plans support SCIM, so accounts follow your identity provider.",
			},
		],
		examples: [
			{
				title: "Marketing and operations together",
				body: "Operations cleans the export, and marketing sends from the clean file.",
			},
		],
		faqs: [
			{
				question: "How many people can join?",
				answer:
					"Free includes 3 seats, Starter 10, Professional 50, and Enterprise has no seat limit.",
			},
		],
		related: [{ label: "API and SDKs", href: "/features/api" }, seePricing, startFree],
	}),
];

export const useCasePages: CapabilityPage[] = [
	page("use-case", "email-marketing", {
		eyebrow: "Email marketing",
		title: "Protect your sender reputation",
		description:
			"Check lists before each campaign and keep your bounce rate low, so your mail reaches the inbox.",
		lede: "For lifecycle and newsletter teams that send to lists they already have.",
		problem: {
			title: "Bounces send your next campaign to spam",
			body: "Mailbox providers watch bounce and complaint rates. When they rise, your mail goes to spam, and your email platform may pause your account.",
		},
		howItWorks: {
			title: "Clean before, learn after",
			body: "Clean the list before you send. Connect your email platform, so bounces and complaints are suppressed after the send.",
		},
		workflow: [
			"Upload the campaign list.",
			"Send only to addresses that are safe to send.",
			"Let delivery feedback suppress what bounced.",
		],
		capabilities: [
			{
				title: "Safe-to-send in one column",
				body: "Export a file with one clear yes or no for each address.",
			},
			{
				title: "Fewer repeat bounces",
				body: "Bounced addresses are suppressed before they come back in the next import.",
			},
			{
				title: "Domain reputation",
				body: "Check a domain against 8 blocklists, such as Spamhaus ZEN, and read its SPF and DMARC records.",
			},
		],
		examples: [
			{
				title: "Re-engagement campaigns",
				body: "Check old subscribers before you try to win them back. These lists bounce the most.",
			},
		],
		faqs: [
			{
				question: "Do I still need my email platform's bounce handling?",
				answer: "Yes. We add a check before the send and one shared suppression list across tools.",
			},
		],
		related: [
			{ label: "List cleaning", href: "/features/list-cleaning" },
			{ label: "Delivery feedback", href: "/features/delivery-feedback" },
			startFree,
		],
	}),
	page("use-case", "marketing-operations", {
		eyebrow: "Marketing operations",
		title: "Clean CRM and event exports",
		description:
			"Check contact exports before they enter your email platform, and send clean files back to your CRM.",
		lede: "For the team that owns the data between the CRM and the email platform.",
		problem: {
			title: "Bad data spreads between tools",
			body: "An invalid address imported once is copied into every tool that syncs with it.",
		},
		howItWorks: {
			title: "A clean file in, a clean file out",
			body: "Upload the export, review the fix plan, and import the clean file back. Keep the list quality score to compare sources.",
		},
		workflow: [
			"Export contacts from your CRM or event tool.",
			"Upload the CSV and review the fix plan.",
			"Import the clean file and the suppressions back.",
		],
		capabilities: [
			{
				title: "Compare sources",
				body: "Use the list quality score to see which forms, events, or vendors bring in bad addresses.",
			},
			{
				title: "Re-checks on a schedule",
				body: "Keep the database fresh without a manual project every quarter.",
			},
			{
				title: "Result explorer",
				body: "Filter every result by category and by the safe-to-send flag in Analytics.",
			},
		],
		examples: [
			{
				title: "Trade show leads",
				body: "Check badge-scan exports before they reach sales and marketing.",
			},
		],
		faqs: [
			{
				question: "Do you connect to my CRM directly?",
				answer: "Use CSV export and import today, or build a sync with the API.",
			},
		],
		related: [
			{ label: "CSV import and export", href: "/integrations/csv" },
			{ label: "Ongoing re-checks", href: "/features/reverification" },
			startFree,
		],
	}),
	page("use-case", "signup-forms", {
		eyebrow: "Signup forms",
		title: "Stop bad addresses at signup",
		description:
			"Check each address in real time, and catch typos, throwaway addresses, and dead domains before they enter your list.",
		lede: "For product and engineering teams that own signup and transactional email.",
		problem: {
			title: "A bad signup costs more than a bounce",
			body: "A mistyped address means a lost user and a password reset that never arrives. A throwaway address means a fake account.",
		},
		howItWorks: {
			title: "One API call in your signup flow",
			body: "Call the check endpoint when the user submits the form. Use the result to accept the address, ask for a fix, or block it.",
		},
		workflow: [
			"Call the API when the user submits the form.",
			"Ask the user to fix a likely typo.",
			"Block disposable addresses if your policy says so.",
		],
		capabilities: [
			{
				title: "Signup protection policy",
				body: "Set signup protection as your workspace policy, and each result comes with a clear decision.",
			},
			{
				title: "Typo hints",
				body: "Catch common domain typos before the account is created.",
			},
		],
		examples: [
			{
				title: "Free trials",
				body: "Reduce fake trial accounts made with throwaway addresses.",
			},
		],
		faqs: [
			{
				question: "Will a slow mail server slow my signup form?",
				answer:
					"Set a timeout in your form. If the check does not finish in time, accept the address and check it again later.",
			},
		],
		related: [
			{ label: "API and SDKs", href: "/features/api" },
			{ label: "REST API and SDKs", href: "/integrations/api" },
			seePricing,
		],
	}),
	page("use-case", "sending-platforms", {
		eyebrow: "Platforms and agencies",
		title: "Check lists for the senders you serve",
		description:
			"Verify customer lists through the API, keep each client's work apart, and use delivery feedback to stop repeat bounces.",
		lede: "For platforms and agencies that send email for their customers.",
		problem: {
			title: "One bad list can hurt every sender",
			body: "When customers share sending infrastructure, one dirty list can damage the reputation of all of them.",
		},
		howItWorks: {
			title: "Check every list before it is sent",
			body: "Run each uploaded list as a job, apply your policy, and send only addresses that are safe to send.",
		},
		workflow: [
			"Send each customer list to the API as a job.",
			"Get the results by webhook.",
			"Block or warn on lists with a low quality score.",
		],
		capabilities: [
			{
				title: "List quality before the send",
				body: "Use the list quality score to stop a risky list before it reaches your sending servers.",
			},
			{
				title: "Delivery feedback at scale",
				body: "Feed bounces and complaints back, so the next upload is cleaner.",
			},
		],
		examples: [
			{
				title: "Customer imports",
				body: "Check a new customer's first import before you send any mail for them.",
			},
		],
		faqs: [
			{
				question: "Do you offer volume pricing?",
				answer: "Yes. Contact us about the Enterprise plan.",
			},
		],
		related: [
			{ label: "API and SDKs", href: "/features/api" },
			{ label: "Delivery feedback", href: "/features/delivery-feedback" },
			{ label: "Contact us", href: "/contact" },
		],
	}),
];

type EspSetup = {
	slug: string;
	name: string;
	/** The suppressed events, in the platform's own words. */
	events: string;
	description: string;
	howItWorks: string;
	/** Setup steps in Verifier and in the platform, with the platform's own menu names. */
	workflow: string[];
	verification: { question: string; answer: string };
};

function espPage(setup: EspSetup): CapabilityPage {
	const { name, events } = setup;
	return page("integration", setup.slug, {
		eyebrow: name,
		title: `Send ${name} events back to your suppression list`,
		description: setup.description,
		lede: `Close the loop between ${name} and your list checks.`,
		problem: {
			title: `Delivery problems stay inside ${name}`,
			body: "When an event is only recorded in the email platform, the address comes back with the next import.",
		},
		howItWorks: { title: "A private URL for each endpoint", body: setup.howItWorks },
		workflow: setup.workflow,
		capabilities: [
			{
				title: "One suppression list",
				body: `Suppressions from ${name} sit next to invalid results and manual entries.`,
			},
			{
				title: "Event history",
				body: "See the delivery events for each address in its history.",
			},
		],
		examples: [
			{
				title: "Stop repeat problems",
				body: "A suppressed address goes to the drop file when you clean your next list.",
			},
			{
				title: "More than one sending tool",
				body: `A ${name} bounce also stops the address in lists from your CRM and other tools.`,
			},
		],
		faqs: [
			{
				question: "Which events are recorded?",
				answer: `The event mapping on this page shows what is recorded. ${events} go on the suppression list.`,
			},
			setup.verification,
		],
		related: [
			{ label: "Delivery feedback", href: "/features/delivery-feedback" },
			{ label: "Suppression list", href: "/features/suppressions" },
			startFree,
		],
	});
}

export const integrationPages: CapabilityPage[] = [
	espPage({
		slug: "sendgrid",
		name: "SendGrid",
		events: "Bounces, spam reports, and unsubscribes",
		description:
			"Point the SendGrid Event Webhook at your workspace. Bounces, spam reports, and unsubscribes update your suppression list.",
		howItWorks:
			"Turn on the signed Event Webhook in SendGrid. Verifier checks each request with your SendGrid verification key and a timestamp window.",
		workflow: [
			"In SendGrid, open Settings > Mail Settings > Event Webhooks, and turn on signature verification. Copy the verification key.",
			"In Verifier, open Outcomes, and add a SendGrid endpoint with that key. Copy its private URL.",
			"In SendGrid, paste the URL as the Post URL. Select the bounce, spam report, and unsubscribe events.",
		],
		verification: {
			question: "How do you know an event came from SendGrid?",
			answer:
				"SendGrid signs each request. Verifier checks the signature with the verification key that you added to the endpoint. A request with a bad signature or an old timestamp is rejected.",
		},
	}),
	espPage({
		slug: "mailgun",
		name: "Mailgun",
		events: "Permanent failures, complaints, and unsubscribes",
		description:
			"Point your Mailgun webhooks at your workspace. Permanent failures, complaints, and unsubscribes update your suppression list.",
		howItWorks:
			"Mailgun signs each webhook with your signing key. Verifier checks the signature and the timestamp before it records the event.",
		workflow: [
			"In Mailgun, open Sending > Webhooks, and copy the HTTP webhook signing key.",
			"In Verifier, open Outcomes, and add a Mailgun endpoint with that key. Copy its private URL.",
			"In Mailgun, add the URL for the Permanent Failure, Spam Complaints, and Unsubscribes events.",
		],
		verification: {
			question: "Is a temporary failure suppressed?",
			answer:
				"No. Only a failure with severity permanent is recorded as a bounce. A temporary failure is not added to the suppression list.",
		},
	}),
	espPage({
		slug: "postmark",
		name: "Postmark",
		events: "Bounces, spam complaints, and unsubscribes",
		description:
			"Point a Postmark webhook at your workspace. Bounces, spam complaints, and unsubscribes update your suppression list.",
		howItWorks:
			"Postmark does not sign webhooks. The private URL carries a secret token, and you can limit the endpoint to a list of IP addresses through the API.",
		workflow: [
			"In Verifier, open Outcomes, and add a Postmark endpoint. Copy its private URL.",
			"In Postmark, open your server, then the message stream, then Webhooks > Add webhook.",
			"Paste the URL. Select Bounce, Spam Complaint, and Subscription Change.",
		],
		verification: {
			question: "Which subscription changes are suppressed?",
			answer:
				"A SubscriptionChange event with SuppressSending set to true is recorded as an unsubscribe. A change that turns sending back on is not.",
		},
	}),
	espPage({
		slug: "amazon-ses",
		name: "Amazon SES",
		events: "Bounces and complaints",
		description:
			"Subscribe your workspace to the Amazon SNS topic for your SES notifications. Bounces and complaints update your suppression list.",
		howItWorks:
			"Amazon SES sends bounce and complaint notifications to an SNS topic. Verifier checks the SNS signature and accepts only the topic ARNs that you set.",
		workflow: [
			"In Amazon SNS, create a topic, and copy its topic ARN.",
			"In Verifier, open Outcomes, and add an Amazon SES endpoint with that topic ARN. Copy its private URL.",
			"In Amazon SNS, add an HTTPS subscription to the URL. Verifier confirms the subscription.",
			"In Amazon SES, open your verified identity, then Notifications. Send bounce and complaint feedback to the topic.",
		],
		verification: {
			question: "Do I need to confirm the SNS subscription?",
			answer:
				"No. Verifier checks the signature of the confirmation message from Amazon SNS and confirms the subscription.",
		},
	}),
	page("integration", "csv", {
		eyebrow: "CSV import and export",
		title: "Bring lists from any tool",
		description:
			"Export contacts from your CRM or email platform as CSV, clean them, and import the clean file back.",
		lede: "Works with any tool that can export and import a CSV.",
		problem: {
			title: "Every tool exports differently",
			body: "Contacts live in CRMs, email platforms, event tools, and spreadsheets.",
		},
		howItWorks: {
			title: "Upload, clean, export",
			body: "Upload a CSV with an email column. Your other columns are kept, and each row gets its result and recommendation.",
		},
		workflow: [
			"Export a CSV from your tool.",
			"Upload it and review the results.",
			"Export the clean file and import it back.",
		],
		capabilities: [
			{
				title: "Your columns stay",
				body: "Names, owners, and custom fields stay in the export.",
			},
			{
				title: "Split files",
				body: "Export the rows to keep, fix, review, or suppress as separate files.",
			},
		],
		examples: [
			{ title: "HubSpot or Salesforce exports", body: "Clean a contact export before a campaign." },
		],
		faqs: [
			{
				question: "Is there a size limit?",
				answer: "Each address in the file counts as one check in your plan.",
			},
		],
		related: [
			{ label: "List cleaning", href: "/features/list-cleaning" },
			{ label: "Marketing operations", href: "/use-cases/marketing-operations" },
			startFree,
		],
	}),
	page("integration", "api", {
		eyebrow: "REST API and SDKs",
		title: "Build your own integration",
		description:
			"Use the REST API with Go and TypeScript SDKs to check addresses, run list jobs, and receive webhooks.",
		lede: "Connect checks to any system you own.",
		problem: {
			title: "Your data lives in your own systems",
			body: "Some teams need checks inside their own product, warehouse, or sync job.",
		},
		howItWorks: {
			title: "Documented endpoints and generated clients",
			body: "Call the API with a scoped key. Use the Go or TypeScript SDK, or any HTTP client.",
		},
		workflow: [
			"Create an API key.",
			"Install the Go or TypeScript SDK.",
			"Check addresses, run jobs, and handle webhooks.",
		],
		capabilities: [
			{ title: "Webhooks", body: "Get results when a job finishes." },
			{
				title: "Delivery events in",
				body: "Send delivery events from any platform through the outcomes API.",
			},
		],
		examples: [
			{ title: "Nightly sync", body: "Check new CRM contacts every night with a scheduled job." },
		],
		faqs: [
			{
				question: "Which plans include the API?",
				answer: "The Professional and Enterprise plans.",
			},
		],
		related: [
			{ label: "API and SDKs", href: "/features/api" },
			{ label: "Signup forms", href: "/use-cases/signup-forms" },
			seePricing,
		],
	}),
];

export const guidePages: CapabilityPage[] = [
	page("guide", "reading-results", {
		eyebrow: "Reading results",
		title: "What valid, risky, invalid, and unknown mean",
		description: "Learn what each category means, and when to keep, review, or remove an address.",
		lede: "Four categories, one score, and the reasons behind them.",
		problem: {
			title: "A result is a decision input",
			body: "A check gives you evidence about an address. Your policy decides what to do with it.",
		},
		howItWorks: {
			title: "The four categories",
			body: "Valid: the mail server accepts the mailbox. Risky: it may deliver, but something is wrong, such as a catch-all domain or a full inbox. Invalid: it will bounce. Unknown: the server did not answer clearly.",
		},
		workflow: [
			"Send to valid addresses.",
			"Review risky addresses, or send to them in a smaller batch.",
			"Remove invalid addresses, and check unknown ones again later.",
		],
		capabilities: [
			{
				title: "The score",
				body: "A number from 0 to 100. Higher is safer. Use it to sort and to set thresholds.",
			},
			{
				title: "Safe to send",
				body: "A yes or no for each address that combines the category and the most serious risks.",
			},
		],
		examples: [
			{
				title: "Role addresses",
				body: "Addresses such as info@ or sales@ often reach a team, not a person. They are flagged so you can decide.",
			},
		],
		faqs: [
			{
				question: "Why is a valid address not safe to send?",
				answer:
					"A valid address can still be disposable, a role account, or on a spam-trap domain. These lower the score and can turn safe to send off.",
			},
		],
		related: [
			{ label: "Scores and reasons", href: "/features/risk-scoring" },
			{ label: "Catch-all domains", href: "/guides/catch-all-domains" },
			startFree,
		],
	}),
	page("guide", "catch-all-domains", {
		eyebrow: "Catch-all domains",
		title: "Why some domains accept every address",
		description:
			"A catch-all domain accepts mail for any address, so a check cannot confirm the mailbox. Learn how severity tiers help you decide.",
		lede: "Many company domains accept every address. Here is how to handle them.",
		problem: {
			title: "Accepted does not mean delivered",
			body: "A catch-all mail server says yes to every address, then may bounce the message later.",
		},
		howItWorks: {
			title: "Severity tiers instead of guesses",
			body: "We detect catch-all domains and grade them. A clean address on a free-provider domain gets a low tier. A company domain, or any address with another warning sign, gets a high tier.",
		},
		workflow: [
			"Send to low-tier catch-all addresses with normal care.",
			"Hold high-tier catch-all addresses for a smaller, separate send.",
			"Let delivery feedback show which ones bounced.",
		],
		capabilities: [
			{
				title: "Graded, not hidden",
				body: "Catch-all addresses are marked risky, with the tier in the result.",
			},
			{
				title: "Feedback improves the next send",
				body: "Bounces from catch-all domains are suppressed for next time.",
			},
		],
		examples: [
			{
				title: "B2B lists",
				body: "Company lists often have many catch-all domains. Send to them in smaller batches.",
			},
		],
		faqs: [
			{
				question: "Can any tool confirm a catch-all mailbox?",
				answer:
					"Not with a mailbox check alone, because the server accepts every address. Grading and delivery feedback are the safe approach.",
			},
		],
		related: [
			{ label: "Scores and reasons", href: "/features/risk-scoring" },
			{ label: "Delivery feedback", href: "/features/delivery-feedback" },
			startFree,
		],
	}),
	page("guide", "list-hygiene", {
		eyebrow: "List hygiene",
		title: "Keep a list healthy between campaigns",
		description: "Re-check old addresses, suppress bounces, and track list quality over time.",
		lede: "A short routine that keeps bounce rates low all year.",
		problem: {
			title: "Cleaning once is not enough",
			body: "Lists decay every month. A single clean-up only resets the clock.",
		},
		howItWorks: {
			title: "A simple routine",
			body: "Check new contacts as they arrive, re-check old ones on a schedule, and suppress every bounce and complaint.",
		},
		workflow: [
			"Check every new contact before its first email.",
			"Re-check addresses older than 30 to 90 days.",
			"Connect delivery feedback, and keep one suppression list.",
		],
		capabilities: [
			{
				title: "Track list quality",
				body: "Compare the list quality score from month to month.",
			},
			{
				title: "Find the bad sources",
				body: "If one form or vendor brings in most invalid addresses, fix the source.",
			},
		],
		examples: [
			{
				title: "Before a big send",
				body: "Re-check the full list a day before a launch or a holiday campaign.",
			},
		],
		faqs: [
			{
				question: "How often should I re-check?",
				answer:
					"Every 30 to 90 days for active lists, and always before you send to addresses you have not mailed in months.",
			},
		],
		related: [
			{ label: "Ongoing re-checks", href: "/features/reverification" },
			{ label: "Suppression list", href: "/features/suppressions" },
			startFree,
		],
	}),
];

export const allCapabilityPages = [
	...featurePages,
	...useCasePages,
	...integrationPages,
	...guidePages,
];

export function getCapabilityPage(kind: CapabilityPage["kind"], slug: string) {
	return allCapabilityPages.find((page) => page.kind === kind && page.slug === slug);
}

export const indexCopy = {
	features: {
		title: "Everything you need to send to real inboxes.",
		description:
			"Check addresses, clean lists, suppress bounces, and keep lists clean as they age.",
	},
	useCases: {
		title: "Built for teams that send email.",
		description:
			"Email marketing, marketing operations, signup forms, and the platforms that send for others.",
	},
	integrations: {
		title: "Connect your email platform.",
		description:
			"Send delivery events from SendGrid, Mailgun, Postmark, or Amazon SES, and move lists by CSV or API.",
	},
	guides: {
		title: "Guides to cleaner lists.",
		description: "Read results with confidence, handle catch-all domains, and keep lists healthy.",
	},
	resources: {
		title: "Resources for cleaner sending.",
		description: "Guides, integrations, security, and product updates.",
	},
} as const;
