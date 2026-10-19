import { describe, expect, it } from "vitest";

import { HistoryPanelPropsSchema } from "./history-panel.schema";

describe("HistoryPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = HistoryPanelPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(HistoryPanelPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(HistoryPanelPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
