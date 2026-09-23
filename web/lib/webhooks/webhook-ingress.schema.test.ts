import { describe, expect, it } from "vitest";

import { WebhookVerificationSchema } from "./webhook-ingress.schema";

describe("WebhookVerificationSchema", () => {
	it("parses the generated domain props", () => {
		expect(WebhookVerificationSchema.safeParse({ ok: true, eventId: "evt_1" }).success).toBe(true);
		expect(
			WebhookVerificationSchema.safeParse({ ok: false, reason: "stale", status: 400 }).success,
		).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		expect(WebhookVerificationSchema.safeParse({}).success).toBe(false);
		// A refusal must say why; an acceptance must say what it accepted.
		expect(WebhookVerificationSchema.safeParse({ ok: false }).success).toBe(false);
		expect(WebhookVerificationSchema.safeParse({ ok: true }).success).toBe(false);
		expect(
			WebhookVerificationSchema.safeParse({ ok: false, reason: "stale", status: 418 }).success,
		).toBe(false);
	});
});
