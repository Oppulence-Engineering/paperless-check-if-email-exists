import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Analytics — Explore verification results, activity, source quality, and domain reputation
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const AnalyticsLitSchema = z.object({
	kind: z.literal("page"),
	name: z.literal("analytics"),
	domain: z.literal("analytics"),
	owner: z.literal("page"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const AnalyticsLit = AnalyticsLitSchema.parse({
	kind: "page",
	name: "analytics",
	domain: "analytics",
	owner: "page",
	client: true,
	summary: "Explore verification results, activity, source quality, and domain reputation",
	schemas: ["app/(product)/app/analytics/search-params.ts"],
	files: [
		"app/(product)/app/analytics/analytics.lit.ts",
		"app/(product)/app/analytics/page.tsx",
		"app/(product)/app/analytics/loading.tsx",
		"app/(product)/app/analytics/error.tsx",
		"app/(product)/app/analytics/search-params.ts",
	],
});
