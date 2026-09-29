import type { Metadata } from "next";

const SITE_NAME = "Check If Email Exists";
const SITE_URL = "http://localhost:3000";

const DEFAULT_SOCIAL_IMAGE = {
	url: "/opengraph-image",
	width: 1200,
	height: 630,
	alt: "Check If Email Exists — Email verification",
} as const;
const DEFAULT_ICON = "/check-email-logo.svg";

/**
 * Canonical origin for metadata, sitemap, and structured data.
 *
 * Self-hosted deployments can override with `NEXT_PUBLIC_SITE_URL` for staging
 * previews without rewriting marketing copy.
 */
export const baseUrl = new URL(process.env.NEXT_PUBLIC_SITE_URL ?? SITE_URL);

/**
 * Shared metadata defaults modeled on the Better Auth docs app: every public
 * route gets `metadataBase`, Open Graph, Twitter cards, and icons unless it
 * explicitly overrides them.
 */
export function createMetadata(override: Metadata): Metadata {
	return {
		...override,
		metadataBase: baseUrl,
		openGraph: {
			title: override.title ?? undefined,
			description: override.description ?? undefined,
			url: baseUrl.href,
			siteName: SITE_NAME,
			type: "website",
			locale: "en_US",
			images: [DEFAULT_SOCIAL_IMAGE],
			...override.openGraph,
		},
		twitter: {
			card: "summary_large_image",
			title: override.title ?? undefined,
			description: override.description ?? undefined,
			images: [DEFAULT_SOCIAL_IMAGE],
			...override.twitter,
		},
		icons:
			override.icons ??
			({
				icon: [{ url: DEFAULT_ICON, sizes: "any" }],
				apple: DEFAULT_ICON,
			} satisfies Metadata["icons"]),
	};
}
