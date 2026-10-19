import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Activity feed — Shows job events with filters and pages.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ActivityFeedLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("activity-feed"),
	domain: z.literal("insights"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ActivityFeedLit = ActivityFeedLitSchema.parse({
	kind: "component",
	name: "activity-feed",
	domain: "insights",
	owner: "feature",
	client: true,
	summary: "Shows job events with filters and pages.",
	schemas: ["components/features/insights/activity-feed/activity-feed.schema.ts"],
	files: [
		"components/features/insights/activity-feed/activity-feed.tsx",
		"components/features/insights/activity-feed/activity-feed.test.tsx",
		"components/features/insights/activity-feed/activity-feed.schema.ts",
		"components/features/insights/activity-feed/activity-feed.schema.test.ts",
		"components/features/insights/activity-feed/activity-feed.lit.ts",
		"components/features/insights/activity-feed/activity-feed.stories.tsx",
	],
});
