import "server-only";

import { count, desc, eq, isNull } from "drizzle-orm";

import { authDb, authMembers, authOrganizations } from "@/lib/auth/database";
import { identityAudit } from "@/lib/auth/identity-audit";

import { TenantSummarySchema, type TenantSummary } from "./platform-admin.schema";

/**
 * @oppulence-gen kind=lib
 * platformAdmin is a server-safe admin helper.
 *
 * Support needs to see which tenants exist without joining each one. The
 * allowlist is an environment variable rather than a role in the database, so
 * nobody can grant themselves platform access through the product, and every
 * look is written to the identity audit trail.
 *
 * This module does not import React or fetch. Inputs are Zod-validated.
 * Owned by `platform-admin.lit.ts`.
 */

/** Emails allowed to see across tenants. Empty means nobody, including in development. */
export function platformAdminEmails(): string[] {
	return (process.env.PLATFORM_ADMIN_EMAILS ?? "")
		.split(",")
		.map((entry) => entry.trim().toLowerCase())
		.filter(Boolean);
}

export function isPlatformAdmin(email: string | null | undefined): boolean {
	const normalized = email?.trim().toLowerCase();
	if (!normalized) return false;
	return platformAdminEmails().includes(normalized);
}

/**
 * Live tenants, newest first. Counts members but reads no tenant content:
 * seeing that a workspace exists is a different power from reading its data.
 */
export async function listActiveTenants(limit = 200): Promise<TenantSummary[]> {
	const rows = await authDb
		.select({
			id: authOrganizations.id,
			name: authOrganizations.name,
			slug: authOrganizations.slug,
			createdAt: authOrganizations.createdAt,
			archivedAt: authOrganizations.archivedAt,
			memberCount: count(authMembers.id),
		})
		.from(authOrganizations)
		.leftJoin(authMembers, eq(authMembers.organizationId, authOrganizations.id))
		.where(isNull(authOrganizations.archivedAt))
		.groupBy(authOrganizations.id)
		.orderBy(desc(authOrganizations.createdAt))
		.limit(limit);

	return rows.map((row) => TenantSummarySchema.parse(row));
}

/** Support access is a privilege, so it leaves a record whether or not it is used. */
export async function recordPlatformAdminAccess(input: {
	actorId: string;
	action: string;
	targetId?: string | null;
	granted: boolean;
}): Promise<void> {
	await identityAudit({
		actorId: input.actorId,
		organizationId: null,
		action: `platform.${input.action}`,
		targetId: input.targetId ?? null,
		result: input.granted ? "success" : "failure",
		requestId: null,
	});
}
