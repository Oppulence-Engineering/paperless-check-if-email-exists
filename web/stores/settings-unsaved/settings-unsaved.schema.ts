import { z } from "zod";

/**
 * @oppulence-gen kind=store
 * Runtime contract for Settings unsaved.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `settings-unsaved.lit.ts`.
 */
export const SettingsUnsavedStateSchema = z.object({
	/** Keys of the settings cards that hold edits the user has not saved. */
	dirtyKeys: z.array(z.string()),
});

export type SettingsUnsavedState = z.infer<typeof SettingsUnsavedStateSchema>;
