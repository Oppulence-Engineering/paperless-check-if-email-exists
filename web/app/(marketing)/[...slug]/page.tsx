import type { Metadata } from "next";
import { cacheLife } from "next/cache";
import { notFound } from "next/navigation";
import { Suspense } from "react";

import { featureDetails, type FeatureDetail } from "../marketing-data";
import { getMarketingPage, marketingPaths } from "../marketing-data";
import { marketingMetadata } from "../metadata";
import { dedicatedMarketingPaths } from "../site";
import { getSeoLander } from "../seo-theme";
import { SimEditorialArticle } from "../sim-landing/subpages/sim-editorial-article";
import { SimFeatureMirrorPage } from "../sim-landing/subpages/sim-feature-mirror-page";
import { SimMarketingBulletPage } from "../sim-landing/subpages/sim-marketing-bullet-page";
import { SimSeoLanderPage } from "../sim-landing/subpages/sim-seo-lander-page";

type PageProps = {
	params: Promise<{ slug: string[] }>;
};

export function generateStaticParams() {
	const moved = new Set([
		...dedicatedMarketingPaths,
		"pricing",
		"blog",
		"customers",
		"product",
		"guides",
		"resources",
		"changelog",
		"answers",
	]);
	return marketingPaths
		.filter(
			(path) => !moved.has(path) && !path.startsWith("blog/") && !path.startsWith("customers/"),
		)
		.map((path) => ({
			slug: path.split("/"),
		}));
}

export async function generateMetadata(props: PageProps): Promise<Metadata> {
	const { slug } = await props.params;
	const page = getMarketingPage(slug.join("/"));

	if (!page) {
		return {
			title: "Page not found - Check If Email Exists",
		};
	}

	return marketingMetadata({
		title: page.title,
		description: page.description,
		path: `/${page.path}`,
	});
}

async function renderCatchAll(path: string) {
	"use cache";
	cacheLife("days");
	const page = getMarketingPage(path);

	if (!page) {
		notFound();
	}

	const oldLander = getSeoLander(page.path);
	if (oldLander) {
		const lander = {
			...oldLander,
			title: page.title,
			description: page.description,
			chips: ["Single checks", "CSV lists", "Self-hosted API"],
			definitionTitle: "How email verification works",
			definition: page.description,
			distinctions: [
				{ title: "Syntax", body: "Check whether the address has a valid format.", chip: "Address" },
				{
					title: "Mail servers",
					body: "Inspect DNS and MX records for the domain.",
					chip: "Domain",
				},
				{
					title: "Mailbox signals",
					body: "Review SMTP evidence and uncertain outcomes.",
					chip: "Reachability",
				},
			],
			for: [
				"Checking one address before sending.",
				"Cleaning a CSV list.",
				"Using the API in an existing workflow.",
			],
			notFor: [
				"Sending email or managing an inbox.",
				"Guaranteeing delivery for servers that hide mailbox status.",
			],
			faqs: [
				{
					question: "Does a check send a message?",
					answer: "No. Verification does not send an email to the recipient.",
				},
			],
			cluster: [
				{ label: "Email verification", href: "/product" },
				{ label: "Guides", href: "/guides" },
			],
		};
		return <SimSeoLanderPage lander={lander} page={page} />;
	}

	const details =
		featureDetails[page.path] ??
		(page.path === "lp/ai-help-center" ? featureDetails["ai-help-center"] : undefined);
	if (details) {
		const emailDetails: FeatureDetail = {
			summary: page.description,
			sections: [
				{
					title: "Check an address",
					body: "Review syntax, DNS, MX, and mailbox signals in one result.",
				},
				{
					title: "Review a list",
					body: "Upload a CSV and inspect each result before using the addresses.",
				},
			],
			workflow: [
				"Enter an address or upload a CSV.",
				"Run verification.",
				"Review the detailed result and any uncertainty.",
			],
			outcomes: ["A recorded result with the signals used to assess the address."],
			relatedPages: [
				{ label: "Product", href: "/product" },
				{ label: "Guides", href: "/guides" },
			],
		};
		return <SimFeatureMirrorPage details={emailDetails} page={page} />;
	}

	if (page.path.startsWith("legal/")) {
		return (
			<SimEditorialArticle
				crumbs={[
					{ name: "Home", path: "/" },
					{ name: page.eyebrow, path: `/${page.path}` },
				]}
				description={page.description}
				eyebrow={page.eyebrow}
				path={`/${page.path}`}
				title={page.title}
			>
				{page.bullets.map((bullet) => (
					<p key={bullet}>{bullet}</p>
				))}
				<p>
					This route is intentionally present for launch-readiness and should be reviewed by counsel
					before production use.
				</p>
			</SimEditorialArticle>
		);
	}

	return <SimMarketingBulletPage page={page} />;
}

export default function Page(props: PageProps) {
	return (
		<Suspense fallback={null}>
			<CatchAllFromParams params={props.params} />
		</Suspense>
	);
}

async function CatchAllFromParams({ params }: { params: Promise<{ slug: string[] }> }) {
	const { slug } = await params;
	return renderCatchAll(slug.join("/"));
}
