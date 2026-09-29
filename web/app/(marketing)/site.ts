import { comparePages } from "./compare-catalog";
import type { LinkItem } from "./marketing-data";

/** Public origin used for canonical URLs, sitemap, and structured data. */
export const SITE_URL = process.env.NEXT_PUBLIC_SITE_URL ?? "http://localhost:3000";

export const SITE_NAME = "Check If Email Exists";

export const ORGANIZATION_NAME = "Oppulence Engineering";

export type NavLink = LinkItem & {
	href: string;
};

export type NavGroup = {
	label: string;
	href?: string;
	description?: string;
	items: NavLink[];
};

/**
 * Header groups for the public site. Every href here must resolve to a real
 * route. The list is intentionally smaller than the full footer map so the
 * bar stays scannable.
 */
export const headerNav: NavGroup[] = [
	{
		label: "Product",
		href: "/products",
		description: "Check one address, clean a list, or use the API.",
		items: [
			{
				label: "Email verification",
				href: "/product",
				description: "Syntax, mail servers, mailbox signals, and a clear result.",
			},
			{
				label: "Web workspace",
				href: "/web",
				description: "Checks, lists, and history in a shared workspace.",
			},
			{
				label: "Self-hosting",
				href: "/desktop",
				description: "Run the web application and Rust API in one image.",
			},
			{
				label: "Bulk lists",
				href: "/voice-app",
				description: "Upload CSV lists and review verification jobs.",
			},
			{
				label: "Run locally",
				href: "/download",
				description: "Build the open-source stack from this repository.",
			},
		],
	},
	{
		label: "Features",
		href: "/features",
		description: "Signals, list cleaning, and team workflows.",
		items: [
			{
				label: "Single checks",
				href: "/features/commitment-register",
				description: "Review address syntax, MX, and mailbox reachability.",
			},
			{
				label: "Workspace overview",
				href: "/features/account-mission-control",
				description: "Keep checks and lists under one organization.",
			},
			{
				label: "List cleaning",
				href: "/features/attention-queue",
				description: "Upload a CSV and inspect each result.",
			},
			{
				label: "API access",
				href: "/features/governed-actions",
				description: "Use scoped credentials and backend authorization.",
			},
			{
				label: "Syntax and MX",
				href: "/features/meeting-capture",
				description: "Catch malformed addresses and missing mail servers.",
			},
			{
				label: "Mailbox signals",
				href: "/features/live-notes",
				description: "See SMTP and delivery signals with each check.",
			},
			{
				label: "Data ownership",
				href: "/features/local-vault",
				description: "Store verification history in your own database.",
			},
			{
				label: "Pipelines",
				href: "/features/agents",
				description: "Schedule and monitor recurring verification work.",
			},
		],
	},
	{
		label: "Use cases",
		href: "/use-cases",
		items: [
			{
				label: "Solo senders",
				href: "/use-cases/founder-operator",
				description: "Check an address before an important message.",
			},
			{
				label: "Sales teams",
				href: "/use-cases/account-management",
				description: "Keep contact lists cleaner across a shared workspace.",
			},
			{
				label: "List hygiene",
				href: "/use-cases/meetings",
				description: "Review CSV addresses before a campaign.",
			},
			{
				label: "Research",
				href: "/use-cases/research",
				description: "Use verification evidence to prioritize follow-up.",
			},
		],
	},
];

export const headerUtilityLinks: NavLink[] = [
	{ label: "Pricing", href: "/pricing" },
	{ label: "Resources", href: "/resources" },
];

export const footerGroups: { title: string; items: NavLink[] }[] = [
	{
		title: "Product",
		items: [
			{ label: "Products", href: "/products" },
			{ label: "Email verification", href: "/product" },
			{ label: "Web workspace", href: "/web" },
			{ label: "Self-hosting", href: "/desktop" },
			{ label: "Bulk lists", href: "/voice-app" },
			{ label: "Pricing", href: "/pricing" },
			{ label: "Run locally", href: "/download" },
		],
	},
	{
		title: "Features",
		items: [
			{ label: "Single checks", href: "/features/commitment-register" },
			{ label: "Workspace overview", href: "/features/account-mission-control" },
			{ label: "List cleaning", href: "/features/attention-queue" },
			{ label: "API access", href: "/features/governed-actions" },
			{ label: "Syntax and MX", href: "/features/meeting-capture" },
			{ label: "Mailbox signals", href: "/features/live-notes" },
			{ label: "Data ownership", href: "/features/local-vault" },
			{ label: "Pipelines", href: "/features/agents" },
		],
	},
	{
		title: "Use cases",
		items: [
			{ label: "Solo senders", href: "/use-cases/founder-operator" },
			{ label: "Sales teams", href: "/use-cases/account-management" },
			{ label: "List hygiene", href: "/use-cases/meetings" },
			{ label: "Research", href: "/use-cases/research" },
		],
	},
	{
		title: "Resources",
		items: [
			{ label: "Resource hub", href: "/resources" },
			{ label: "Guides", href: "/guides" },
			{ label: "Changelog", href: "/changelog" },
			{ label: "Integrations", href: "/integrations" },
			{ label: "Security", href: "/security" },
			{ label: "Compare", href: "/compare" },
			{ label: "Answer hub", href: "/answers" },
			{ label: "Blog", href: "/blog" },
			{ label: "Customers", href: "/customers" },
		],
	},
	{
		title: "Compare",
		items: comparePages.map((page) => ({
			label:
				page.slug === "crm"
					? "CRM"
					: page.slug === "meeting-notes"
						? "Meeting notes"
						: page.slug === "inbox"
							? "Inbox"
							: "AI assistants",
			href: page.path,
		})),
	},
	{
		title: "Company",
		items: [
			{ label: "Contact", href: "/contact" },
			{ label: "Privacy", href: "/privacy" },
			{ label: "Terms", href: "/terms" },
			{ label: "Responsible disclosure", href: "/responsible-disclosure" },
			{
				label: "Source code",
				href: "https://github.com/Oppulence-Engineering/paperless-check-if-email-exists",
				external: true,
			},
		],
	},
];

/** Dedicated marketing routes that live outside the catch-all slug table. */
export const dedicatedMarketingPaths = [
	"product",
	"products",
	"web",
	"desktop",
	"voice-app",
	"download",
	"security",
	"changelog",
	"contact",
	"resources",
	"features",
	"features/commitment-register",
	"features/account-mission-control",
	"features/attention-queue",
	"features/governed-actions",
	"features/meeting-capture",
	"features/live-notes",
	"features/local-vault",
	"features/agents",
	"use-cases",
	"use-cases/founder-operator",
	"use-cases/account-management",
	"use-cases/meetings",
	"use-cases/research",
	"integrations",
	"integrations/gmail",
	"integrations/calendar",
	"integrations/slack",
	"integrations/hubspot",
	"integrations/mcp",
	"guides",
	"guides/desktop-vs-web",
	"guides/what-stays-on-device",
	"guides/approval-before-send",
	"guides/install-oppulence",
	"pricing",
	"blog",
	"customers",
	"compare",
	"answers",
	...comparePages.map((page) => page.path.slice(1)),
] as const;

export function absoluteUrl(path = ""): string {
	if (!path || path === "/") return SITE_URL;
	return `${SITE_URL}${path.startsWith("/") ? path : `/${path}`}`;
}
