import { describe, expect, it } from "vitest";

import { ListExportPropsSchema } from "./list-export.schema";

describe("ListExportPropsSchema", () => {
	it("parses a positive listId", () => {
		expect(ListExportPropsSchema.safeParse({ listId: 3 }).success).toBe(true);
	});

	it("rejects a missing listId", () => {
		expect(ListExportPropsSchema.safeParse({}).success).toBe(false);
	});
});
