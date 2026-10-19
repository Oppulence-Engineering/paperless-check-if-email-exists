import { describe, expect, it } from "vitest";

import {
	BackendErrorEnvelopeSchema,
	EntityTagSchema,
	IdempotencyKeySchema,
	UploadDescriptorSchema,
} from "./integration-contract.schema";

describe("backend integration schemas", () => {
	it("accepts the canonical public error envelope", () => {
		expect(
			BackendErrorEnvelopeSchema.parse({
				error: "temporary failure",
				code: "service_unavailable",
				requestId: "req-123",
				retryable: true,
			}),
		).toMatchObject({ code: "service_unavailable", retryable: true });
	});

	it("rejects unsafe idempotency keys and entity tags", () => {
		expect(IdempotencyKeySchema.safeParse("short").success).toBe(false);
		expect(IdempotencyKeySchema.safeParse("request-123").success).toBe(true);
		expect(EntityTagSchema.safeParse('W/"version-2"').success).toBe(true);
		expect(EntityTagSchema.safeParse("version-2").success).toBe(false);
	});

	it("validates short-lived upload descriptors", () => {
		expect(
			UploadDescriptorSchema.safeParse({
				uploadId: "upload-1",
				url: "https://uploads.example.test/object",
				method: "PUT",
				headers: { "content-type": "image/png" },
				expiresAt: "2027-01-01T00:00:00.000Z",
			}).success,
		).toBe(true);
		expect(
			UploadDescriptorSchema.safeParse({
				uploadId: "upload-1",
				url: "file:///private/object",
				method: "PUT",
				headers: {},
				expiresAt: "2027-01-01T00:00:00.000Z",
			}).success,
		).toBe(false);
		expect(
			UploadDescriptorSchema.safeParse({
				uploadId: "upload-1",
				url: "http://uploads.example.test/object",
				method: "PUT",
				headers: { authorization: "Bearer secret" },
				expiresAt: "2027-01-01T00:00:00.000Z",
			}).success,
		).toBe(false);
	});
});
