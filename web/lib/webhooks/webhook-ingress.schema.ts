import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Runtime contract for Webhook ingress.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `webhook-ingress.lit.ts`.
 */
export const WebhookVerificationSchema = z.discriminatedUnion("ok", [
	z.object({ ok: z.literal(true), eventId: z.string().min(1) }),
	z.object({
		ok: z.literal(false),
		reason: z.enum(["missing_signature", "bad_signature", "stale", "replayed", "no_secret"]),
		status: z.union([z.literal(400), z.literal(401), z.literal(409)]),
	}),
]);

export type WebhookVerification = z.infer<typeof WebhookVerificationSchema>;
