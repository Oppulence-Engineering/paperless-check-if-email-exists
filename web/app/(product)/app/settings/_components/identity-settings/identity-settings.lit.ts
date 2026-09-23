import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Identity settings — Better Auth organization, authentication, enterprise identity, and audit controls.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const IdentitySettingsLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("identity-settings"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const IdentitySettingsLit = IdentitySettingsLitSchema.parse({
	kind: "component",
	name: "identity-settings",
	domain: "",
	owner: "route",
	client: true,
	summary: "Better Auth organization, authentication, enterprise identity, and audit controls.",
	schemas: ["app/(product)/app/settings/_components/identity-settings/identity-settings.schema.ts"],
	files: [
		"app/(product)/app/settings/_components/identity-settings/identity-settings.tsx",
		"app/(product)/app/settings/_components/identity-settings/identity-settings.test.tsx",
		"app/(product)/app/settings/_components/identity-settings/identity-settings.schema.ts",
		"app/(product)/app/settings/_components/identity-settings/identity-settings.schema.test.ts",
		"app/(product)/app/settings/_components/identity-settings/identity-settings.lit.ts",
		"app/(product)/app/settings/_components/identity-settings/identity-settings.stories.tsx",
	],
});
