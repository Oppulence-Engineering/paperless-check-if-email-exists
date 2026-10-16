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
		href: "/product",
		description: "Check addresses, clean lists, and keep them clean.",
		items: [
			{
				label: "Email verification",
				href: "/product",
				description: "Know which addresses are safe to send to.",
			},
			{
				label: "Web workspace",
				href: "/web",
				description: "Lists, results, and history for your team.",
			},
			{
				label: "Integrations",
				href: "/integrations",
				description: "Delivery feedback from your email platform.",
			},
		],
	},
	{
		label: "Features",
		href: "/features",
		description: "From one address to every list you send.",
		items: [
			{
				label: "Single checks",
				href: "/features/single-checks",
				description: "A clear answer for one address.",
			},
			{
				label: "List cleaning",
				href: "/features/list-cleaning",
				description: "Clean a CSV before your next campaign.",
			},
			{
				label: "Scores and reasons",
				href: "/features/risk-scoring",
				description: "See why an address is risky.",
			},
			{
				label: "Delivery feedback",
				href: "/features/delivery-feedback",
				description: "Suppress what actually bounced.",
			},
			{
				label: "Suppression list",
				href: "/features/suppressions",
				description: "One list of addresses to never send to.",
			},
			{
				label: "Ongoing re-checks",
				href: "/features/reverification",
				description: "Keep lists clean as they age.",
			},
			{
				label: "API and SDKs",
				href: "/features/api",
				description: "Checks inside your own product.",
			},
			{
				label: "Team workspace",
				href: "/features/workspace",
				description: "Shared lists, history, and settings.",
			},
		],
	},
	{
		label: "Use cases",
		href: "/use-cases",
		items: [
			{
				label: "Email marketing",
				href: "/use-cases/email-marketing",
				description: "Protect your sender reputation.",
			},
			{
				label: "Marketing operations",
				href: "/use-cases/marketing-operations",
				description: "Clean CRM and event exports.",
			},
			{
				label: "Signup forms",
				href: "/use-cases/signup-forms",
				description: "Stop bad addresses at signup.",
			},
			{
				label: "Platforms and agencies",
				href: "/use-cases/sending-platforms",
				description: "Check lists for the senders you serve.",
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
			{ label: "Email verification", href: "/product" },
			{ label: "Web workspace", href: "/web" },
			{ label: "Pricing", href: "/pricing" },
			{ label: "Integrations", href: "/integrations" },
			{ label: "Changelog", href: "/changelog" },
		],
	},
	{
		title: "Features",
		items: headerNav[1].items.map(({ label, href }) => ({ label, href })),
	},
	{
		title: "Use cases",
		items: headerNav[2].items.map(({ label, href }) => ({ label, href })),
	},
	{
		title: "Resources",
		items: [
			{ label: "Resource hub", href: "/resources" },
			{ label: "Guides", href: "/guides" },
			{ label: "Blog", href: "/blog" },
			{ label: "Security", href: "/security" },
			{ label: "Self-hosting", href: "/self-hosting" },
		],
	},
	{
		title: "Company",
		items: [
			{ label: "Contact", href: "/contact" },
			{ label: "Privacy", href: "/privacy" },
			{ label: "Terms", href: "/terms" },
			{ label: "Responsible disclosure", href: "/responsible-disclosure" },
		],
	},
];

export function absoluteUrl(path = ""): string {
	if (!path || path === "/") return SITE_URL;
	return `${SITE_URL}${path.startsWith("/") ? path : `/${path}`}`;
}
