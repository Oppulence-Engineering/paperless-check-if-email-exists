import { describe, expect, it } from "vitest";

import { TrpcBackendReadInputSchema, TrpcBackendWriteInputSchema } from "./trpc-bridge.schema";

describe("tRPC backend bridge schemas", () => {
	it("accepts safe backend reads and writes", () => {
		expect(TrpcBackendReadInputSchema.safeParse({ path: ["widgets"] }).success).toBe(true);
		expect(
			TrpcBackendWriteInputSchema.safeParse({
				path: ["widgets"],
				method: "POST",
				body: { name: "Widget" },
				idempotencyKey: "create-widget-123",
			}).success,
		).toBe(true);
	});

	it("rejects traversal and unsafe write methods", () => {
		expect(TrpcBackendReadInputSchema.safeParse({ path: ["..", "secrets"] }).success).toBe(false);
		expect(
			TrpcBackendWriteInputSchema.safeParse({ path: ["widgets"], method: "GET" }).success,
		).toBe(false);
	});
});
