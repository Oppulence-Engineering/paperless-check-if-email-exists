import "server-only";

import { createHmac, timingSafeEqual } from "node:crypto";

import { WebhookVerificationSchema, type WebhookVerification } from "./webhook-ingress.schema";

/**
 * @oppulence-gen kind=lib
 * webhookIngress is a server-safe webhooks helper.
 *
 * The three things every inbound webhook needs and most implementations skip:
 * a signature compared in constant time, a timestamp so a captured request
 * cannot be replayed tomorrow, and an id so a provider's retry does not run
 * the work twice.
 *
 * The default seen-store is per process, which is honest rather than correct:
 * two replicas do not share it. Pass a store backed by your database or Redis
 * before you rely on exactly-once.
 *
 * This module does not import React or fetch. Inputs are Zod-validated.
 * Owned by `webhook-ingress.lit.ts`.
 */

const FIVE_MINUTES = 5 * 60 * 1_000;

/** Per-process memory. Correct for one replica, a lie for several. */
export function createMemorySeenStore(now: () => number = Date.now) {
	const entries = new Map<string, number>();
	return {
		seen(eventId: string): boolean {
			const expires = entries.get(eventId);
			if (expires === undefined) return false;
			if (expires <= now()) {
				entries.delete(eventId);
				return false;
			}
			return true;
		},
		remember(eventId: string, ttlMs: number): void {
			entries.set(eventId, now() + ttlMs);
		},
	};
}

function constantTimeEquals(a: string, b: string): boolean {
	const left = Buffer.from(a);
	const right = Buffer.from(b);
	if (left.length !== right.length) return false;
	return timingSafeEqual(left, right);
}

/** `v1=<hex>` over `<timestamp>.<body>`, the shape most providers settled on. */
export function signWebhook(secret: string, timestamp: string, body: string): string {
	return `v1=${createHmac("sha256", secret).update(`${timestamp}.${body}`).digest("hex")}`;
}

export async function verifyWebhook(input: {
	secret: string;
	body: string;
	signature: string | null;
	timestamp: string | null;
	eventId: string | null;
	/** Idempotency, as two calls rather than an object to implement. */
	seen?: (eventId: string) => Promise<boolean> | boolean;
	remember?: (eventId: string, ttlMs: number) => Promise<void> | void;
	toleranceMs?: number;
	now?: () => number;
}): Promise<WebhookVerification> {
	const now = input.now ?? Date.now;
	const tolerance = input.toleranceMs ?? FIVE_MINUTES;

	if (!input.secret) {
		return WebhookVerificationSchema.parse({ ok: false, reason: "no_secret", status: 401 });
	}
	if (!input.signature || !input.timestamp || !input.eventId) {
		return WebhookVerificationSchema.parse({
			ok: false,
			reason: "missing_signature",
			status: 400,
		});
	}

	const sentAt = Number(input.timestamp);
	if (!Number.isFinite(sentAt) || Math.abs(now() - sentAt) > tolerance) {
		return WebhookVerificationSchema.parse({ ok: false, reason: "stale", status: 400 });
	}

	const expected = signWebhook(input.secret, input.timestamp, input.body);
	if (!constantTimeEquals(expected, input.signature)) {
		return WebhookVerificationSchema.parse({ ok: false, reason: "bad_signature", status: 401 });
	}

	if (input.seen && (await input.seen(input.eventId))) {
		// A provider retrying is normal; doing the work twice is not.
		return WebhookVerificationSchema.parse({ ok: false, reason: "replayed", status: 409 });
	}
	await input.remember?.(input.eventId, tolerance * 2);

	return WebhookVerificationSchema.parse({ ok: true, eventId: input.eventId });
}
