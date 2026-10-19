import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Runtime contract for Data export.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `data-export.lit.ts`.
 */
export const DataExportSchema = z.object({
	exportedAt: z.iso.datetime(),
	/** Says plainly what this file is, and what it is not. */
	scope: z.string().min(1),
	identity: z.object({
		id: z.string(),
		name: z.string(),
		email: z.string(),
		emailVerified: z.boolean(),
		createdAt: z.string(),
	}),
	memberships: z.array(
		z.object({
			organizationId: z.string(),
			organizationName: z.string(),
			role: z.string(),
			joinedAt: z.string(),
		}),
	),
	sessions: z.array(
		z.object({
			id: z.string(),
			createdAt: z.string(),
			expiresAt: z.string(),
			ipAddress: z.string().nullable(),
			userAgent: z.string().nullable(),
		}),
	),
	identityEvents: z.array(
		z.object({
			action: z.string(),
			result: z.string(),
			organizationId: z.string().nullable(),
			createdAt: z.string(),
		}),
	),
});

export type DataExport = z.infer<typeof DataExportSchema>;
