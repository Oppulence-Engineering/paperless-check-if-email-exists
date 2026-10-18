import { describe, expect, it } from "vitest";

import { DomainHealthPropsSchema } from "./domain-health.schema";

describe("DomainHealthPropsSchema", () => {
	it("parses a domain", () => {
		expect(DomainHealthPropsSchema.safeParse({ domain: "example.com" }).success).toBe(true);
	});

	it("rejects a missing or empty domain", () => {
		expect(DomainHealthPropsSchema.safeParse({}).success).toBe(false);
		expect(DomainHealthPropsSchema.safeParse({ domain: " " }).success).toBe(false);
	});
});
