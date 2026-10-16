export type LinkItem = {
	label: string;
	href: string;
	description?: string;
	external?: boolean;
};

export type MarketingPage = {
	path: string;
	eyebrow: string;
	title: string;
	description: string;
	category: "feature" | "product" | "tool" | "blog" | "customer" | "legal" | "landing";
	bullets: string[];
	proof: string[];
	ctaLabel?: string;
	ctaHref?: string;
};

/** Content for `/product`. */
export const productPage: MarketingPage = {
	path: "product",
	eyebrow: "Email verification",
	title: "Know which addresses are safe to send to",
	description:
		"Check one address or a whole list. Each address gets a category, a score, and the reasons, and bounces from your email platform keep your suppression list current.",
	category: "product",
	bullets: [
		"Check one address or upload a CSV list.",
		"See the category, the score, the safe-to-send flag, and the reason codes.",
		"Suppress bounces and complaints from SendGrid, Mailgun, Postmark, or Amazon SES.",
	],
	proof: [
		"No email is sent during a check.",
		"Some mail servers hide whether a mailbox exists. We say so in the result.",
		"Your lists and results stay in your workspace.",
	],
	ctaLabel: "Start free",
	ctaHref: "/sign-up",
};

/**
 * Public plans (set 2026-10-16). Onboarding is self-serve: Starter and
 * Professional start on the free plan through sign-up until self-serve billing
 * exists, and only Enterprise links to Contact for custom terms. Check allowances are per month; seats follow the plan
 * entitlements in lib/entitlements.
 */
export const pricingPlans = [
	{
		name: "Free",
		price: "$0",
		period: "/month",
		note: "100 checks a month",
		description: "Try checks and clean a small list.",
		ctaLabel: "Start free",
		ctaHref: "/sign-up",
	},
	{
		name: "Starter",
		price: "$49",
		period: "/month",
		note: "4,000 checks a month",
		description: "For teams that clean lists before each campaign.",
		recommended: true,
		ctaLabel: "Start free",
		ctaHref: "/sign-up",
	},
	{
		name: "Professional",
		price: "$149",
		period: "/month",
		note: "12,000 checks a month",
		description: "For teams that check at signup and send at volume.",
		ctaLabel: "Start free",
		ctaHref: "/sign-up",
	},
	{
		name: "Enterprise",
		price: "Custom",
		period: "",
		note: "Volume pricing",
		description: "For high volume, account provisioning, and custom terms.",
		ctaLabel: "Contact us",
		ctaHref: "/contact",
	},
];

/**
 * The three ways people actually run Oppulence. Each gets its own page at
 * /web, /desktop, and /voice-app so the nav can point at a product rather than
 * a feature essay. Desktop and Voice both ship an installer, so their pages
 * carry the download chooser.
 */
export type PlatformPage = {
	slug: string;
	name: string;
	eyebrow: string;
	title: string;
	lede: string;
	/** Shown as a one-line answer to "what is this, concretely?" */
	summary: string;
	screenshot: string;
	screenshotAlt: string;
	/** Whether the page shows the desktop installer picker. */
	download: boolean;
	sections: {
		title: string;
		body: string;
		bullets: string[];
		screenshot: string;
		alt: string;
	}[];
	specs: { term: string; detail: string }[];
	/** Call to action for the last feature block. Defaults to opening the app. */
	cta?: { label: string; href: string };
};

export const platformPages: PlatformPage[] = [
	{
		slug: "web",
		name: "Web workspace",
		eyebrow: "Web workspace",
		title: "Your team's workspace for clean lists",
		lede: "Check addresses, clean lists, and review history in your browser. There is nothing to install.",
		summary: "Check addresses and clean lists in your browser.",
		screenshot: "/marketing/email-check-preview.svg",
		screenshotAlt: "Sample email check result",
		download: false,
		sections: [
			{
				title: "Check an address",
				body: "Get the category, the score, and the reasons in seconds. No email is sent.",
				bullets: ["Category and score", "Reason codes", "History for each address"],
				screenshot: "/marketing/email-check-preview.svg",
				alt: "Sample email check result",
			},
			{
				title: "Clean a list",
				body: "Upload a CSV, review the fix plan, and export clean files for your email platform.",
				bullets: ["CSV upload", "List quality score", "Keep, fix, review, or suppress"],
				screenshot: "/marketing/email-check-preview.svg",
				alt: "Sample list results",
			},
			{
				title: "Keep it clean",
				body: "Suppress bounces from your email platform, and re-check old addresses on a schedule.",
				bullets: ["Delivery feedback", "Suppression list", "Scheduled re-checks"],
				screenshot: "/marketing/email-check-preview.svg",
				alt: "Sample suppression list",
			},
		],
		specs: [
			{ term: "Team", detail: "Organizations, invitations, and per-user sign-in." },
			{ term: "API", detail: "REST API with Go and TypeScript SDKs on Professional plans." },
		],
		cta: { label: "Start free", href: "/sign-up" },
	},
];

export function getPlatformPage(slug: string) {
	return platformPages.find((page) => page.slug === slug);
}
