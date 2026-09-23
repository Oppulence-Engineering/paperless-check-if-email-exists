import type { CapabilityPage } from "../../../catalog";
import type { MarketingPage, PlatformPage } from "../../../marketing-data";
import { PlatformExplorer } from "./platform-explorer";
import { ProductHeroPreview } from "./product-hero-preview";
import { ProductModulePreview } from "./product-module-preview";
import type { ProductPreviewKey } from "./product-previews";
import type { SolutionsProductFeatureConfig, SolutionsProductPageConfig } from "./types";

function slugify(value: string) {
	return value
		.toLowerCase()
		.replace(/[^a-z0-9]+/g, "-")
		.replace(/^-|-$/g, "");
}

const CAPABILITY_HERO: Partial<Record<string, ProductPreviewKey>> = {
	"commitment-register": "register",
	"account-mission-control": "account",
	"attention-queue": "web",
	"governed-actions": "govern",
	"meeting-capture": "voice",
	"live-notes": "desktop",
	"local-vault": "desktop",
	agents: "desktop",
	"founder-operator": "web",
	"account-management": "account",
	meetings: "voice",
	research: "account",
	gmail: "register",
	calendar: "web",
	slack: "govern",
	hubspot: "account",
	mcp: "desktop",
};

/** Interactive showcase preview per capability slug. */
const CAPABILITY_SHOWCASE: Partial<Record<string, ProductPreviewKey | "register-interactive">> = {
	"commitment-register": "register-interactive",
	"governed-actions": "govern",
	"meeting-capture": "voice",
	"live-notes": "desktop",
	"attention-queue": "web",
	"account-mission-control": "account",
	gmail: "register",
	slack: "govern",
	hubspot: "account",
};

/** Two-up feature grid previews per slug — distinct pairs where possible. */
const CAPABILITY_GRID: Partial<Record<string, [ProductPreviewKey, ProductPreviewKey]>> = {
	"commitment-register": ["register", "account"],
	"governed-actions": ["govern", "register"],
	"meeting-capture": ["voice", "desktop"],
	"live-notes": ["desktop", "register"],
	"attention-queue": ["web", "account"],
	"account-mission-control": ["account", "web"],
	gmail: ["register", "govern"],
	calendar: ["web", "register"],
	slack: ["govern", "web"],
	hubspot: ["account", "register"],
	mcp: ["desktop", "govern"],
};

const PLATFORM_HERO: Record<string, ProductPreviewKey> = {
	web: "web",
	desktop: "desktop",
	"voice-app": "voice",
};

function modulePreview(
	product: ProductPreviewKey | "register-interactive",
	layout: "menu" | "stage" = "menu",
): SolutionsProductFeatureConfig["visual"] {
	return <ProductModulePreview layout={layout} product={product} />;
}

function featureBlock(
	id: string,
	label: string,
	title: string,
	description: string,
	product: ProductPreviewKey | "register-interactive",
	cta?: { label: string; href: string },
): SolutionsProductFeatureConfig {
	return {
		id,
		label,
		title,
		description,
		visual: modulePreview(product),
		cta,
	};
}

/** Map catalog pages to Sim `/knowledge` or `/tables` product layout with live previews. */
export function capabilityToSolutionsConfig(page: CapabilityPage): SolutionsProductPageConfig {
	const heroProduct = CAPABILITY_HERO[page.slug] ?? "register";
	const secondary = page.capabilities[0] ?? page.examples[0];
	const tertiary = page.capabilities[1] ?? page.examples[1] ?? page.capabilities[0];
	const [gridA, gridB] = CAPABILITY_GRID[page.slug] ?? [
		heroProduct === "register" ? "account" : "govern",
		heroProduct === "voice" ? "desktop" : "web",
	];

	const showcaseProduct = CAPABILITY_SHOWCASE[page.slug] ?? heroProduct;

	return {
		module: page.eyebrow,
		path: page.path,
		seoDescription: page.description,
		hero: {
			eyebrow: page.eyebrow,
			heading: page.title,
			description: page.lede,
			summary: `${page.description} ${page.lede}`.trim(),
			visual: <ProductHeroPreview product={heroProduct} />,
		},
		features: [
			{
				id: "showcase",
				label: page.howItWorks.title,
				title: page.problem.title,
				description: page.howItWorks.body,
				visual: modulePreview(showcaseProduct, "stage"),
				cta: page.related[0]
					? { label: page.related[0].label, href: page.related[0].href }
					: undefined,
			},
			secondary
				? featureBlock(
						slugify(secondary.title),
						page.capabilities[0] ? "Capability" : "Example",
						secondary.title,
						secondary.body,
						gridA,
						page.related[1]
							? { label: page.related[1].label, href: page.related[1].href }
							: undefined,
					)
				: featureBlock(
						"capability",
						"Capability",
						page.howItWorks.title,
						page.howItWorks.body,
						gridA,
					),
			tertiary
				? featureBlock(
						slugify(tertiary.title),
						page.capabilities[1] ? "Capability" : "Example",
						tertiary.title,
						tertiary.body,
						gridB,
						page.integrations?.[0]
							? { label: page.integrations[0].label, href: page.integrations[0].href }
							: undefined,
					)
				: featureBlock("example", "Example", page.problem.title, page.problem.body, "web"),
		],
	};
}

/** Map `/web`, `/desktop`, and `/voice-app` to Sim product module pages. */
export function platformToSolutionsConfig(page: PlatformPage): SolutionsProductPageConfig {
	const heroProduct = PLATFORM_HERO[page.slug] ?? "web";
	const [first, second, third] = page.sections;

	const featureProducts: Record<string, [ProductPreviewKey, ProductPreviewKey]> = {
		web: ["web", "account"],
		desktop: ["desktop", "account"],
		"voice-app": ["voice", "desktop"],
	};
	const [gridA, gridB] = featureProducts[page.slug] ?? ["web", "govern"];

	return {
		module: page.name.replace(/^Oppulence /, ""),
		path: `/${page.slug}`,
		seoDescription: page.summary,
		hero: {
			eyebrow: page.eyebrow,
			heading: page.title,
			description: page.lede,
			summary: `${page.summary} ${page.lede}`.trim(),
			visual: <ProductHeroPreview product={heroProduct} />,
		},
		features: [
			first
				? {
						id: slugify(first.title),
						label: first.title,
						title: first.title,
						description: first.body,
						visual: modulePreview(heroProduct, "stage"),
					}
				: featureBlock("overview", page.summary, page.title, page.lede, heroProduct),
			second
				? featureBlock(slugify(second.title), second.title, second.title, second.body, gridA)
				: featureBlock(
						"detail",
						page.specs[0]?.term ?? page.name,
						page.specs[0]?.term ?? page.name,
						page.specs[0]?.detail ?? page.summary,
						gridA,
					),
			third
				? featureBlock(
						slugify(third.title),
						third.title,
						third.title,
						third.body,
						gridB,
						page.download
							? { label: "Download", href: "/download" }
							: { label: "Open dashboard", href: "/app" },
					)
				: featureBlock(
						"also",
						"Also available",
						page.specs[1]?.term ?? "Runs on",
						page.specs[1]?.detail ?? page.summary,
						gridB,
						{ label: "All products", href: "/products" },
					),
		],
	};
}

/** Map `/products` suite hub to sim.ai platform module rhythm with live previews. */
export function productsSuiteToSolutionsConfig(): SolutionsProductPageConfig {
	return {
		module: "Products",
		path: "/products",
		seoDescription: "Check one email address, clean CSV lists, or use the self-hosted Rust API.",
		hero: {
			eyebrow: "Products",
			heading: "One check, a whole list, or an API call.",
			description:
				"Use the browser for individual checks and CSV lists. Use the Rust API in your own workflow.",
			summary: "The web workspace, list jobs, and API report syntax, DNS, MX, and mailbox signals.",
			visual: <ProductHeroPreview product="overview" />,
		},
		features: [
			{
				id: "explore",
				label: "Workflows",
				title: "Verification where you need it.",
				description:
					"Switch between individual checks, the web workspace, self-hosting, and bulk lists.",
				visual: <PlatformExplorer />,
			},
			featureBlock(
				"web",
				"Web app",
				"Web workspace",
				"Check addresses and review lists and history in a browser.",
				"web",
				{ label: "Open web app", href: "/web" },
			),
			featureBlock(
				"desktop",
				"Self-hosting",
				"Your infrastructure",
				"Run the combined web application and Rust API with PostgreSQL and RabbitMQ.",
				"desktop",
				{ label: "Open desktop app", href: "/desktop" },
			),
		],
	};
}

/** Map `/product` to sim.ai `/platform` overview structure. */
export function productToSolutionsConfig(page: MarketingPage): SolutionsProductPageConfig {
	return {
		module: "Product",
		path: "/product",
		seoDescription: page.description,
		hero: {
			eyebrow: "Product",
			heading: page.title,
			description: page.description,
			summary: [page.description, ...page.bullets].join(" "),
			visual: <ProductHeroPreview product="overview" />,
		},
		features: [
			{
				id: "explore",
				label: "Explore the workspace",
				title: "Check, review, and keep history.",
				description: "Explore individual checks, CSV jobs, the web workspace, and the API.",
				visual: <PlatformExplorer />,
			},
			featureBlock(
				"register",
				"Verification details",
				"The signals behind each result.",
				"Review syntax, DNS, MX, and mailbox signals before using an address.",
				"register",
				{ label: "Explore single checks", href: "/features/commitment-register" },
			),
			featureBlock(
				"govern",
				"Bulk verification",
				"Clean a list before outreach.",
				"Upload a CSV and review the verification result for each address.",
				"govern",
				{ label: "Explore list cleaning", href: "/features/attention-queue" },
			),
		],
	};
}
