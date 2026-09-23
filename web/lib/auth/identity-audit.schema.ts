import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Runtime contract for Identity audit.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `identity-audit.lit.ts`.
 */
export const IdentityAuditInputSchema = z.object({
	actorId: z.string().min(1).nullable(),
	organizationId: z.string().min(1).nullable(),
	action: z.string().min(1).max(120),
	targetId: z.string().min(1).max(255).nullable(),
	result: z.enum(["success", "failure"]),
	requestId: z.string().min(1).max(255).nullable(),
});

export type IdentityAuditInput = z.infer<typeof IdentityAuditInputSchema>;

export const IdentityAuditListInputSchema = z.object({
	organizationId: z.string().min(1),
	limit: z.number().int().min(1).max(500).default(100),
});

const IdentityAuditEventSchema = z.object({
	id: z.string().uuid(),
	actorId: z.string().nullable(),
	actorName: z.string().nullable(),
	actorEmail: z.string().email().nullable(),
	organizationId: z.string().nullable(),
	action: z.string().min(1),
	targetId: z.string().nullable(),
	result: z.enum(["success", "failure"]),
	requestId: z.string().nullable(),
	createdAt: z.coerce.date(),
});

export const IdentityAuditResponseSchema = z.object({
	events: z.array(IdentityAuditEventSchema),
});

export type IdentityAuditListInput = z.infer<typeof IdentityAuditListInputSchema>;
export type IdentityAuditEvent = z.infer<typeof IdentityAuditEventSchema>;
