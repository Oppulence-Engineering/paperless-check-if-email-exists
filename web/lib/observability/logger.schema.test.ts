import { describe, expect, it } from "vitest";

import { LogFieldsSchema, LogLevelSchema, LogRecordSchema } from "./logger.schema";

describe("LogRecordSchema", () => {
	it("parses the generated domain props", () => {
		expect(
			LogRecordSchema.safeParse({
				level: "info",
				message: "server started",
				time: new Date().toISOString(),
			}).success,
		).toBe(true);
		expect(LogLevelSchema.safeParse("debug").success).toBe(true);
		expect(LogFieldsSchema.safeParse({ count: 3, ok: true, note: null }).success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		expect(LogRecordSchema.safeParse({}).success).toBe(false);
		expect(LogLevelSchema.safeParse("trace").success).toBe(false);
		// Structured fields stay scalar; a nested object is how customer data leaks.
		expect(LogFieldsSchema.safeParse({ user: { email: "a@b.c" } }).success).toBe(false);
	});
});
