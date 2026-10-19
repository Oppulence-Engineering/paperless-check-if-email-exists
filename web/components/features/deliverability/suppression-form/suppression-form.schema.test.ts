import { describe, expect, it } from "vitest";

import { SuppressionFormPropsSchema } from "./suppression-form.schema";

describe("SuppressionFormPropsSchema", () => {
	it("parses each mode", () => {
		expect(SuppressionFormPropsSchema.safeParse({ mode: "add" }).success).toBe(true);
		expect(SuppressionFormPropsSchema.safeParse({ mode: "import" }).success).toBe(true);
	});

	it("rejects a missing or unknown mode", () => {
		expect(SuppressionFormPropsSchema.safeParse({}).success).toBe(false);
		expect(SuppressionFormPropsSchema.safeParse({ mode: "delete" }).success).toBe(false);
	});
});
