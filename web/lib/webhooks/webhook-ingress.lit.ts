import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Webhook ingress — Signature, freshness and idempotency for inbound webhooks.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const WebhookIngressLitSchema = z.object({
	kind: z.literal("lib"),
	name: z.literal("webhook-ingress"),
	domain: z.literal("webhooks"),
	owner: z.literal("lib"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const WebhookIngressLit = WebhookIngressLitSchema.parse({
	kind: "lib",
	name: "webhook-ingress",
	domain: "webhooks",
	owner: "lib",
	client: false,
	summary: "Signature, freshness and idempotency for inbound webhooks.",
	schemas: ["lib/webhooks/webhook-ingress.schema.ts"],
	files: [
		"lib/webhooks/webhook-ingress.ts",
		"lib/webhooks/webhook-ingress.schema.ts",
		"lib/webhooks/webhook-ingress.schema.test.ts",
		"lib/webhooks/webhook-ingress.test.ts",
		"lib/webhooks/webhook-ingress.lit.ts",
	],
});
