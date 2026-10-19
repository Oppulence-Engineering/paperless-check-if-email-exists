import { describe, expect, it } from "vitest";

import { createMemorySeenStore, signWebhook, verifyWebhook } from "./webhook-ingress";

const secret = "whsec_example";
const body = JSON.stringify({ type: "invoice.paid" });
const now = () => 1_700_000_000_000;
const timestamp = String(now());

function signed(overrides: Partial<Parameters<typeof verifyWebhook>[0]> = {}) {
	return verifyWebhook({
		secret,
		body,
		signature: signWebhook(secret, timestamp, body),
		timestamp,
		eventId: "evt_1",
		now,
		...overrides,
	});
}

describe("verifyWebhook", () => {
	it("accepts a request signed with the shared secret", async () => {
		await expect(signed()).resolves.toEqual({ ok: true, eventId: "evt_1" });
	});

	it("refuses a body that changed after signing", async () => {
		await expect(signed({ body: JSON.stringify({ type: "invoice.voided" }) })).resolves.toEqual({
			ok: false,
			reason: "bad_signature",
			status: 401,
		});
	});

	it("refuses a signature made with another secret", async () => {
		await expect(
			signed({ signature: signWebhook("whsec_other", timestamp, body) }),
		).resolves.toMatchObject({ reason: "bad_signature" });
	});

	it("refuses a captured request replayed tomorrow", async () => {
		const yesterday = String(now() - 24 * 60 * 60 * 1_000);
		await expect(
			signed({ timestamp: yesterday, signature: signWebhook(secret, yesterday, body) }),
		).resolves.toMatchObject({ reason: "stale", status: 400 });
	});

	it("refuses a timestamp that is not a number", async () => {
		await expect(signed({ timestamp: "yesterday" })).resolves.toMatchObject({ reason: "stale" });
	});

	it("refuses a request missing any part of the contract", async () => {
		await expect(signed({ signature: null })).resolves.toMatchObject({
			reason: "missing_signature",
			status: 400,
		});
		await expect(signed({ eventId: null })).resolves.toMatchObject({
			reason: "missing_signature",
		});
		await expect(signed({ timestamp: null })).resolves.toMatchObject({
			reason: "missing_signature",
		});
	});

	it("refuses everything when the deployment configured no secret", async () => {
		await expect(signed({ secret: "" })).resolves.toMatchObject({
			reason: "no_secret",
			status: 401,
		});
	});

	it("runs an event once, however many times the provider sends it", async () => {
		const store = createMemorySeenStore(now);
		const idempotent = { seen: store.seen, remember: store.remember };
		await expect(signed(idempotent)).resolves.toMatchObject({ ok: true });
		await expect(signed(idempotent)).resolves.toMatchObject({ reason: "replayed", status: 409 });
	});

	it("lets a different event through the same store", async () => {
		const store = createMemorySeenStore(now);
		const idempotent = { seen: store.seen, remember: store.remember };
		await signed(idempotent);
		await expect(signed({ ...idempotent, eventId: "evt_2" })).resolves.toMatchObject({ ok: true });
	});

	it("forgets an event once it can no longer be replayed", () => {
		let clock = now();
		const store = createMemorySeenStore(() => clock);
		store.remember("evt_1", 1_000);
		expect(store.seen("evt_1")).toBe(true);
		clock += 2_000;
		expect(store.seen("evt_1")).toBe(false);
	});
});
