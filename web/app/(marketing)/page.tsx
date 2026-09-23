import type { Metadata, Viewport } from "next";
import { cacheLife } from "next/cache";

import { homepageFaqs } from "./catalog";
import { SimLandingPage } from "./sim-landing/landing-page";
import { JsonLd } from "./marketing-primitives";
import { faqJsonLd } from "./metadata";
import { ORGANIZATION_NAME, SITE_NAME, SITE_URL } from "./site";
import { createMetadata } from "@/lib/metadata";

const TITLE = "Check email addresses before you send";
const DESCRIPTION =
	"Check address syntax, mail servers, and mailbox signals. Review results and clean lists in one workspace.";

export const metadata: Metadata = createMetadata({
	title: TITLE,
	description: DESCRIPTION,
	alternates: { canonical: SITE_URL },
	openGraph: {
		title: TITLE,
		description: DESCRIPTION,
		url: SITE_URL,
		siteName: SITE_NAME,
		type: "website",
		locale: "en_US",
	},
	twitter: {
		card: "summary_large_image",
		title: TITLE,
		description: DESCRIPTION,
	},
});

export const viewport: Viewport = {
	themeColor: [
		{ media: "(prefers-color-scheme: light)", color: "#ffffff" },
		{ media: "(prefers-color-scheme: dark)", color: "#111111" },
	],
};

const jsonLd = {
	"@context": "https://schema.org",
	"@graph": [
		{
			"@type": "Organization",
			"@id": `${SITE_URL}/#organization`,
			name: ORGANIZATION_NAME,
			alternateName: SITE_NAME,
			url: SITE_URL,
			// Structured data wants a full-size logo; /icon.png is sized for a
			// browser tab, so the schema points at the full asset instead.
			logo: `${SITE_URL}/check-email-logo.svg`,
		},
		{
			"@type": "SoftwareApplication",
			name: SITE_NAME,
			applicationCategory: "BusinessApplication",
			operatingSystem: "Web",
			description: DESCRIPTION,
			url: SITE_URL,
			publisher: { "@id": `${SITE_URL}/#organization` },
		},
	],
};

export default async function Page() {
	"use cache";
	cacheLife("days");

	return (
		<>
			<script
				// Structured data for search engines; static content, no user input.
				dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
				type="application/ld+json"
			/>
			<JsonLd data={faqJsonLd(homepageFaqs)} />
			<SimLandingPage />
		</>
	);
}
