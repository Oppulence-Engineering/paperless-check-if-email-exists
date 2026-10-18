"use client";

import "client-only";
import "@scalar/api-reference-react/style.css";

import dynamic from "next/dynamic";
import { useMemo, type ComponentProps, type ComponentPropsWithoutRef } from "react";
import type { ApiReferenceReact } from "@scalar/api-reference-react";

import { cn } from "@oppulence/ui/lib/utils";

import { type ApiReferencePropsFields } from "./api-reference.schema";

/**
 * @oppulence-gen kind=component
 * ApiReference is a client presentation component.
 * Shows an OpenAPI document with Scalar: a sidebar, search, models, code
 * samples, and a client that sends requests to this host.
 *
 * Data and callbacks arrive through validated props. This module does not fetch,
 * persist, or read cookies. Owned by `api-reference.lit.ts`.
 */
export type ApiReferenceProps = ApiReferencePropsFields & ComponentPropsWithoutRef<"section">;

type ScalarConfiguration = ComponentProps<typeof ApiReferenceReact>["configuration"];

// Scalar is a Vue app of several MB. It loads only in the browser, and only on
// the pages that show a reference.
const ScalarReference = dynamic(
	() => import("@scalar/api-reference-react").then((module) => module.ApiReferenceReact),
	{
		ssr: false,
		loading: () => <p className="px-6 py-10 text-sm text-[#858585]">Loading API reference…</p>,
	},
);

const BACKGROUND = { workspace: "#111111", public: "#090909" } as const;

/** Maps Scalar's theme variables to the dark palette of the app. */
function themeCss(background: string): string {
	return `
:root {
	--scalar-font: var(--font-geist-sans), ui-sans-serif, system-ui, sans-serif;
	--scalar-font-code: var(--font-geist-mono), ui-monospace, monospace;
	--scalar-radius: 6px;
	--scalar-radius-lg: 10px;
	--scalar-radius-xl: 14px;
}
.dark-mode {
	--scalar-background-1: ${background};
	--scalar-background-2: #151515;
	--scalar-background-3: #1a1a1a;
	--scalar-background-accent: rgb(255 255 255 / 0.06);
	--scalar-color-1: #ededed;
	--scalar-color-2: #a3a3a3;
	--scalar-color-3: #858585;
	--scalar-color-accent: #ffffff;
	--scalar-border-color: #212121;
	--scalar-button-1: #ffffff;
	--scalar-button-1-hover: rgb(255 255 255 / 0.9);
	--scalar-button-1-color: #090909;
	--scalar-sidebar-background-1: ${background};
	--scalar-sidebar-item-hover-background: rgb(255 255 255 / 0.04);
	--scalar-sidebar-item-active-background: rgb(255 255 255 / 0.06);
	--scalar-sidebar-search-background: rgb(255 255 255 / 0.03);
	--scalar-sidebar-search-border-color: rgb(255 255 255 / 0.08);
	--scalar-sidebar-border-color: #212121;
	--scalar-sidebar-color-1: #ededed;
	--scalar-sidebar-color-2: #a3a3a3;
	--scalar-sidebar-color-active: #ffffff;
	/* Scalar's red (#f00000) is below 4.5:1 on these surfaces. */
	--scalar-color-red: #f87171;
}
/* Lists of operations grow and wrap long paths, so they never scroll in a box
   that a keyboard cannot reach. */
.scalar-app .scalar-card-content:has(> .endpoints) {
	max-height: none;
}
.scalar-app .endpoints .endpoint,
.scalar-app .endpoints .endpoint * {
	white-space: normal;
	overflow-wrap: anywhere;
}`;
}

/**
 * Makes relative server URLs absolute on this host. Scalar shows them in the
 * request bar and in code samples, and a relative "/" would show as "//v1".
 */
export function withAbsoluteServers(
	document: ApiReferencePropsFields["document"],
	origin: string,
): ApiReferencePropsFields["document"] {
	const servers = document.servers as { url: string }[] | undefined;
	if (!servers) return document;
	return {
		...document,
		servers: servers.map((server) =>
			server.url.startsWith("/")
				? { ...server, url: new URL(server.url, origin).href.replace(/\/$/, "") }
				: server,
		),
	};
}

/**
 * The Scalar configuration for one page. Scalar must not load fonts, scripts,
 * or a proxy from its own servers: the CSP allows this host only.
 */
export function scalarConfiguration(
	document: ApiReferencePropsFields["document"],
	surface: ApiReferencePropsFields["surface"],
	origin?: string,
): ScalarConfiguration {
	return {
		content: origin ? withAbsoluteServers(document, origin) : document,
		theme: "none",
		customCss: themeCss(BACKGROUND[surface]),
		forceDarkModeState: "dark",
		hideDarkModeToggle: true,
		withDefaultFonts: false,
		hideClientButton: true,
		showDeveloperTools: "never",
		agent: { disabled: true },
		mcp: { disabled: true },
		telemetry: false,
		// API keys stay in memory. They never go to localStorage.
		persistAuth: false,
		showOperationId: true,
		orderSchemaPropertiesBy: "preserve",
		defaultRequestBodyView: "form",
		defaultHttpClient: { targetKey: "shell", clientKey: "curl" },
		documentDownloadType: "json",
		// The operationId is the last part of each anchor, so links stay stable.
		generateOperationSlug: ({ operationId, method, path }) =>
			operationId ?? `${method.toLowerCase()}${path}`,
	};
}

export function ApiReference({ className, document, surface, ...props }: ApiReferenceProps) {
	// Scalar renders in the browser only, so the origin is known when it reads this.
	const configuration = useMemo(
		() =>
			scalarConfiguration(
				document,
				surface,
				typeof window === "undefined" ? undefined : window.location.origin,
			),
		[document, surface],
	);
	return (
		<section data-slot="api-reference" className={cn("min-h-[60vh]", className)} {...props}>
			<ScalarReference configuration={configuration} />
		</section>
	);
}
