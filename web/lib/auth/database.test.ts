import { describe, expect, it } from "vitest";

import { DatabaseInputSchema } from "./database.schema";

describe("database", () => {
	it("rejects unsafe readiness timeouts", () => {
		expect(() => DatabaseInputSchema.parse({ timeoutMs: 0 })).toThrow();
	});
});
