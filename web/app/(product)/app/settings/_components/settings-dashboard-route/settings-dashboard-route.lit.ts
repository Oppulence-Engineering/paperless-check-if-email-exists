import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Settings dashboard route — Template-style grouped settings navigation for the email verification workspace
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const SettingsDashboardRouteLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("settings-dashboard-route"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const SettingsDashboardRouteLit = SettingsDashboardRouteLitSchema.parse({
	kind: "component",
	name: "settings-dashboard-route",
	domain: "",
	owner: "route",
	client: true,
	summary: "Template-style grouped settings navigation for the email verification workspace",
	schemas: [
		"app/(product)/app/settings/_components/settings-dashboard-route/settings-dashboard-route.schema.ts",
	],
	files: [
		"app/(product)/app/settings/_components/settings-dashboard-route/settings-dashboard-route.tsx",
		"app/(product)/app/settings/_components/settings-dashboard-route/settings-dashboard-route.test.tsx",
		"app/(product)/app/settings/_components/settings-dashboard-route/settings-dashboard-route.schema.ts",
		"app/(product)/app/settings/_components/settings-dashboard-route/settings-dashboard-route.schema.test.ts",
		"app/(product)/app/settings/_components/settings-dashboard-route/settings-dashboard-route.lit.ts",
		"app/(product)/app/settings/_components/settings-dashboard-route/settings-dashboard-route.stories.tsx",
	],
});
