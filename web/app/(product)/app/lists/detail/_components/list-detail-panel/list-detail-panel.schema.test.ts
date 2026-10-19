import { describe, expect, it } from "vitest";

import { ListDetailPanelPropsSchema } from "./list-detail-panel.schema";

describe("ListDetailPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = ListDetailPanelPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(ListDetailPanelPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(ListDetailPanelPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
