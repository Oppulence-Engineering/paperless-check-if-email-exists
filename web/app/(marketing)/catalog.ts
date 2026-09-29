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
			"No. Verification inspects address syntax, DNS and MX records, and available SMTP signals without sending a message to the recipient.",
	},
	{
		question: "Can I clean a whole list?",
		answer: "Yes. Upload a CSV, run a verification job, and review the result for each address.",
	},
	{
		question: "Is every result certain?",
		answer:
			"No. Some mail servers hide mailbox status or accept all addresses. Review the detailed signals and treat unknown results as unknown.",
	},
	{
		question: "Can I run it on my own infrastructure?",
		answer:
			"Yes. This repository includes the Rust API, worker, and web application. The combined container uses external PostgreSQL and RabbitMQ services.",
	},
	{
		question: "Can my team share a workspace?",
		answer: "Yes. Organizations scope checks, lists, history, and backend access.",
	},
];

const preview = "/marketing/email-check-preview.svg";
const featureSpecs = [
	[
		"commitment-register",
		"Single email checks",
		"Check an address before you send",
		"Inspect syntax, domain, mail servers, and mailbox signals for one address.",
	],
	[
		"account-mission-control",
		"Workspace overview",
		"Keep verification work together",
		"Review checks, lists, and history under one organization.",
	],
	[
		"attention-queue",
		"List cleaning",
		"Verify a CSV list",
		"Upload addresses and review each result in a bulk verification job.",
	],
	[
		"governed-actions",
		"API access",
		"Use verification in your workflow",
		"Call the Rust API through the authenticated web application or your own integration.",
	],
	[
		"meeting-capture",
		"Syntax and MX",
		"Find address and mail server problems",
		"See malformed addresses, DNS records, and whether the domain accepts mail.",
	],
	[
		"live-notes",
		"Mailbox signals",
		"Review the evidence behind a result",
		"Inspect SMTP and reachability signals with the limits of mailbox detection in mind.",
	],
	[
		"local-vault",
		"Data ownership",
		"Keep verification data in your database",
		"Run the open-source stack with PostgreSQL and control where history is stored.",
	],
	[
		"agents",
		"Pipelines",
		"Repeat verification work",
		"Schedule re-verification and monitor list jobs through the backend.",
	],
] as const;
const useCaseSpecs = [
	[
		"founder-operator",
		"Solo senders",
		"Check a contact before an important email",
		"Inspect one address and review its signals before sending.",
	],
	[
		"account-management",
		"Sales teams",
		"Keep shared contact lists cleaner",
		"Verify addresses before they enter a team outreach workflow.",
	],
	[
		"meetings",
		"List hygiene",
		"Review a list before a campaign",
		"Upload a CSV and inspect invalid, risky, and unknown results.",
	],
	[
		"research",
		"Research",
		"Use evidence to review an address",
		"Look at domain, mail server, and mailbox signals together.",
	],
] as const;
const integrationSpecs = [
	[
		"gmail",
		"Gmail addresses",
		"Check addresses used with Gmail",
		"Export contacts to CSV or check an address individually. Direct Gmail account connection is not part of this app.",
	],
	[
		"calendar",
		"Calendar contacts",
		"Verify addresses from your calendar workflow",
		"Export addresses to CSV and check them here. Calendar account sync is not included.",
	],
	[
		"slack",
		"Slack contacts",
		"Check addresses from your team workflow",
		"Use a CSV export or individual check. This app does not connect to Slack workspaces.",
	],
	[
		"hubspot",
		"HubSpot contacts",
		"Clean a HubSpot contact export",
		"Upload a contact CSV to review address quality. Direct HubSpot sync is not included.",
	],
	[
		"mcp",
		"Developer tools",
		"Connect through the API",
		"Build your own integration against the documented Rust API and generated SDKs.",
	],
] as const;
const guideSpecs = [
	[
		"desktop-vs-web",
		"Web or API",
		"Choose how to run a check",
		"Use the web workspace for manual checks and lists, or the API for application workflows.",
	],
	[
		"what-stays-on-device",
		"Data and hosting",
		"Understand where results live",
		"A self-hosted deployment stores account and verification data in its PostgreSQL database.",
	],
	[
		"approval-before-send",
		"Review before sending",
		"Use verification as a decision input",
		"A result helps you review an address; it never sends a message for you.",
	],
	[
		"install-oppulence",
		"Run locally",
		"Start the full application",
		"Use make dev for local development or build the combined production container.",
	],
] as const;

function pages(
	kind: CapabilityPage["kind"],
	specs: readonly (readonly [string, string, string, string])[],
): CapabilityPage[] {
	const prefix = kind === "use-case" ? "use-cases" : kind === "guide" ? "guides" : `${kind}s`;
	return specs.map(([slug, eyebrow, title, description]) => ({
		kind,
		slug,
		path: `/${prefix}/${slug}`,
		eyebrow,
		title,
		description,
		lede: description,
		screenshot: preview,
		screenshotAlt: "Illustrative email verification workspace",
		problem: {
			title: "Know what the address signals say",
			body: "An address can look valid while its domain or mailbox cannot receive mail. Verification gives you more evidence before you use it.",
		},
		howItWorks: { title: "Check, then review", body: description },
		workflow: [
			"Enter an address or upload a CSV list.",
			"Run the verification check.",
			"Review the result and its underlying signals.",
		],
		capabilities: [
			{ title: "Address and domain", body: "Inspect syntax, DNS, and MX records." },
			{ title: "Mailbox signals", body: "Review reachability details and uncertain outcomes." },
		],
		examples: [
			{ title: "One address or a list", body: "Use the web workspace or the documented API." },
		],
		security: "Organization access controls scope verification data and API requests.",
		faqs: [
			{
				question: "Does verification guarantee delivery?",
				answer:
					"No. Mail servers can hide mailbox status, reject probes, or accept all addresses. Review uncertain results before using them.",
			},
		],
		related: [
			{ label: "Check an email", href: "/sign-up" },
			{ label: "All features", href: "/features" },
		],
	}));
}

export const featurePages = pages("feature", featureSpecs);
export const useCasePages = pages("use-case", useCaseSpecs);
export const integrationPages = pages("integration", integrationSpecs);
export const guidePages = pages("guide", guideSpecs);
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
		title: "Email verification, from one address to a list.",
		description: "Inspect syntax, DNS, MX, mailbox signals, history, and jobs.",
	},
	useCases: {
		title: "Check addresses before they matter.",
		description: "Use a single check or clean a CSV list before outreach.",
	},
	integrations: {
		title: "Bring addresses into your workflow.",
		description:
			"Import CSV data or use the API and SDKs. Direct third-party account connections are not included.",
	},
	guides: {
		title: "Guides to running verification.",
		description: "Learn how results work and how to run the application.",
	},
	resources: {
		title: "Resources for email verification.",
		description: "Browse guides, documentation, security information, and product updates.",
	},
} as const;
