import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Verification settings — Tenant verification settings backed by the Rust API
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const VerificationSettingsLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("verification-settings"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const VerificationSettingsLit = VerificationSettingsLitSchema.parse({
	kind: "component",
	name: "verification-settings",
	domain: "",
	owner: "route",
	client: true,
	summary: "Tenant verification settings backed by the Rust API",
	schemas: [
		"app/(product)/app/settings/_components/verification-settings/verification-settings.schema.ts",
	],
	files: [
		"app/(product)/app/settings/_components/verification-settings/verification-settings.tsx",
		"app/(product)/app/settings/_components/verification-settings/verification-settings.test.tsx",
		"app/(product)/app/settings/_components/verification-settings/verification-settings.schema.ts",
		"app/(product)/app/settings/_components/verification-settings/verification-settings.schema.test.ts",
		"app/(product)/app/settings/_components/verification-settings/verification-settings.lit.ts",
		"app/(product)/app/settings/_components/verification-settings/verification-settings.stories.tsx",
	],
});
