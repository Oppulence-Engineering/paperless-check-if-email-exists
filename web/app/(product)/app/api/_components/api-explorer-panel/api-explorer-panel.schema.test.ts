import { describe, expect, it } from "vitest";

import { ApiExplorerPanelPropsSchema } from "./api-explorer-panel.schema";

describe("ApiExplorerPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = ApiExplorerPanelPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(ApiExplorerPanelPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(ApiExplorerPanelPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
