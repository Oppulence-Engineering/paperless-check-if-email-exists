import { describe, expect, it } from "vitest";

import { ResultFiltersPropsSchema } from "./result-filters.schema";

describe("ResultFiltersPropsSchema", () => {
	it("parses the explorer filters", () => {
		expect(ResultFiltersPropsSchema.safeParse({ filters: { minScore: 10 } }).success).toBe(true);
	});

	it("rejects missing filters", () => {
		expect(ResultFiltersPropsSchema.safeParse({}).success).toBe(false);
	});
});
