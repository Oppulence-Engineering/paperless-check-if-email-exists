import { describe, expect, it } from "vitest";

import { ListComparePropsSchema } from "./list-compare.schema";

describe("ListComparePropsSchema", () => {
	it("parses a positive listId", () => {
		expect(ListComparePropsSchema.safeParse({ listId: 3 }).success).toBe(true);
	});

	it("rejects a missing listId", () => {
		expect(ListComparePropsSchema.safeParse({}).success).toBe(false);
	});
});
