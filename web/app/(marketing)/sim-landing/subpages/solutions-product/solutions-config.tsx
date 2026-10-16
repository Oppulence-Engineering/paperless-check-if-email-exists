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
	const heroProduct: ProductPreviewKey = "overview";
	const secondary = page.capabilities[0] ?? page.examples[0];
	const tertiary = page.capabilities[1] ?? page.examples[1] ?? page.capabilities[0];
	const [gridA, gridB]: [ProductPreviewKey, ProductPreviewKey] = ["web", "account"];
	const showcaseProduct = heroProduct;

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

/** Map `/web` and `/self-hosting` to Sim product module pages. */
export function platformToSolutionsConfig(page: PlatformPage): SolutionsProductPageConfig {
	const heroProduct: ProductPreviewKey = "web";
	const [first, second, third] = page.sections;
	const [gridA, gridB]: [ProductPreviewKey, ProductPreviewKey] = ["web", "account"];

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
						page.cta ?? { label: "Open the app", href: "/app" },
					)
				: featureBlock(
						"also",
						"Also available",
						page.specs[1]?.term ?? "Runs on",
						page.specs[1]?.detail ?? page.summary,
						gridB,
						{ label: "Email verification", href: "/product" },
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
				label: "One product, four ways in",
				title: "Check one address, clean a list, or call the API.",
				description:
					"Use the web workspace for checks and lists, or the API inside your own product.",
				visual: <PlatformExplorer />,
			},
			featureBlock(
				"scores",
				"Scores and reasons",
				"Know why an address is risky.",
				"Each result has a category, a score from 0 to 100, a safe-to-send flag, and reason codes.",
				"register",
				{ label: "Explore scores and reasons", href: "/features/risk-scoring" },
			),
			featureBlock(
				"feedback",
				"Delivery feedback",
				"Suppress what actually bounced.",
				"Connect SendGrid, Mailgun, Postmark, or Amazon SES, and keep one suppression list.",
				"govern",
				{ label: "Explore delivery feedback", href: "/features/delivery-feedback" },
			),
		],
	};
}
