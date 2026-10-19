import { describe, expect, it } from "vitest";

import { hasUnsavedSettings, useSettingsUnsavedStore } from "./store";
import { SettingsUnsavedStateSchema } from "./settings-unsaved.schema";

describe("useSettingsUnsavedStore", () => {
	it("starts from a Zod-validated snapshot", () => {
		const snapshot = () => ({ dirtyKeys: useSettingsUnsavedStore.getState().dirtyKeys });
		expect(typeof useSettingsUnsavedStore.getState().reset).toBe("function");
		expect(SettingsUnsavedStateSchema.safeParse(snapshot()).success).toBe(true);
		useSettingsUnsavedStore.getState().reset();
		expect(SettingsUnsavedStateSchema.safeParse(snapshot()).success).toBe(true);
	});

	it("tracks each card separately and clears only its own key", () => {
		const { reset, setDirty } = useSettingsUnsavedStore.getState();
		reset();
		expect(hasUnsavedSettings()).toBe(false);

		setDirty("profile", true);
		setDirty("defaults", true);
		expect(useSettingsUnsavedStore.getState().dirtyKeys).toEqual(["profile", "defaults"]);
		expect(hasUnsavedSettings()).toBe(true);

		setDirty("profile", false);
		expect(useSettingsUnsavedStore.getState().dirtyKeys).toEqual(["defaults"]);
		expect(hasUnsavedSettings()).toBe(true);

		setDirty("defaults", false);
		expect(hasUnsavedSettings()).toBe(false);
		reset();
	});

	it("ignores a repeated flag so the store does not churn", () => {
		const { reset, setDirty } = useSettingsUnsavedStore.getState();
		reset();
		setDirty("profile", true);
		const first = useSettingsUnsavedStore.getState().dirtyKeys;
		setDirty("profile", true);
		expect(useSettingsUnsavedStore.getState().dirtyKeys).toBe(first);
		reset();
	});
});
