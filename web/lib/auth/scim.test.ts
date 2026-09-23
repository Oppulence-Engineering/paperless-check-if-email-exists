import { describe, expect, it } from "vitest";

import { createScimOptions, projectedScimRole } from "./scim";

describe("scim", () => {
	it("requires an independent credential hash secret", () => {
		expect(() => createScimOptions({ credentialHashSecret: "short" })).toThrow();
	});

	it("projects the strongest supported organization role", () => {
		expect(projectedScimRole([])).toBe("member");
		expect(projectedScimRole([{ role: "member" }, { role: "admin" }])).toBe("admin");
	});
});
