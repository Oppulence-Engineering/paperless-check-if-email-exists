import { describe, expect, it } from "vitest";

import { IntegrationStackPropsSchema } from "./integration-stack.schema";

describe("IntegrationStackPropsSchema", () => {
	it("parses the empty props", () => {
		expect(IntegrationStackPropsSchema.safeParse({}).success).toBe(true);
	});

	it("rejects props that are not an object", () => {
		expect(IntegrationStackPropsSchema.safeParse(null).success).toBe(false);
	});
});
