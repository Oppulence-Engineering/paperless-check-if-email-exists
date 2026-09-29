import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Developer settings — Tenant API key management for developer settings
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const DeveloperSettingsLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("developer-settings"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const DeveloperSettingsLit = DeveloperSettingsLitSchema.parse({
	kind: "component",
	name: "developer-settings",
	domain: "",
	owner: "route",
	client: true,
	summary: "Tenant API key management for developer settings",
	schemas: [
		"app/(product)/app/settings/_components/developer-settings/developer-settings.schema.ts",
	],
	files: [
		"app/(product)/app/settings/_components/developer-settings/developer-settings.tsx",
		"app/(product)/app/settings/_components/developer-settings/developer-settings.test.tsx",
		"app/(product)/app/settings/_components/developer-settings/developer-settings.schema.ts",
		"app/(product)/app/settings/_components/developer-settings/developer-settings.schema.test.ts",
		"app/(product)/app/settings/_components/developer-settings/developer-settings.lit.ts",
		"app/(product)/app/settings/_components/developer-settings/developer-settings.stories.tsx",
	],
});
