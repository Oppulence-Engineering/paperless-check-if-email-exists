import { describe, expect, it } from "vitest";

import { SettingsUnsavedStateSchema } from "./settings-unsaved.schema";

describe("SettingsUnsavedStateSchema", () => {
	it("parses the generated domain props", () => {
		expect(SettingsUnsavedStateSchema.safeParse({ dirtyKeys: [] }).success).toBe(true);
		expect(SettingsUnsavedStateSchema.safeParse({ dirtyKeys: ["profile"] }).success).toBe(true);
	});

	it("rejects a payload that is not a list of keys", () => {
		expect(SettingsUnsavedStateSchema.safeParse({}).success).toBe(false);
		expect(SettingsUnsavedStateSchema.safeParse({ dirtyKeys: "profile" }).success).toBe(false);
	});
});
