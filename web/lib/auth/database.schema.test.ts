import { describe, expect, it } from "vitest";

import { DatabaseInputSchema } from "./database.schema";

describe("DatabaseInputSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = DatabaseInputSchema.safeParse({
			timeoutMs: 2_000,
		});
		expect(parsed.success).toBe(true);
	});

	it("provides a bounded default", () => {
		expect(DatabaseInputSchema.parse({}).timeoutMs).toBe(2_000);
	});
});
