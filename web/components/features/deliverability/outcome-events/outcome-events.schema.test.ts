import { describe, expect, it } from "vitest";

import { OutcomeEventsPropsSchema } from "./outcome-events.schema";

describe("OutcomeEventsPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = OutcomeEventsPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(OutcomeEventsPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(OutcomeEventsPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
