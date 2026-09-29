import { z } from "zod";

/**
 * @oppulence-gen kind=store
 * Settings unsaved — Tracks which settings cards hold unsaved edits so navigation can warn before it discards them.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const SettingsUnsavedLitSchema = z.object({
	kind: z.literal("store"),
	name: z.literal("settings-unsaved"),
	domain: z.literal("settings"),
	owner: z.literal("store"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const SettingsUnsavedLit = SettingsUnsavedLitSchema.parse({
	kind: "store",
	name: "settings-unsaved",
	domain: "settings",
	owner: "store",
	client: false,
	summary:
		"Tracks which settings cards hold unsaved edits so navigation can warn before it discards them.",
	schemas: ["stores/settings-unsaved/settings-unsaved.schema.ts"],
	files: [
		"stores/settings-unsaved/store.ts",
		"stores/settings-unsaved/settings-unsaved.schema.ts",
		"stores/settings-unsaved/settings-unsaved.schema.test.ts",
		"stores/settings-unsaved/store.test.ts",
		"stores/settings-unsaved/settings-unsaved.lit.ts",
	],
});
