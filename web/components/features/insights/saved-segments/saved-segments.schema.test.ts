import { describe, expect, it } from "vitest";

import { SavedSegmentsPropsSchema } from "./saved-segments.schema";

describe("SavedSegmentsPropsSchema", () => {
	it("parses the explorer filters", () => {
		expect(SavedSegmentsPropsSchema.safeParse({ filters: { category: "valid" } }).success).toBe(
			true,
		);
	});

	it("rejects missing filters", () => {
		expect(SavedSegmentsPropsSchema.safeParse({}).success).toBe(false);
	});
});
