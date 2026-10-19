import "server-only";

import { desc, eq } from "drizzle-orm";

import {
	authDb,
	authMembers,
	authOrganizations,
	authSessions,
	authUsers,
	identityAuditEvents,
} from "@/lib/auth/database";

import { DataExportSchema, type DataExport } from "./data-export.schema";

/**
 * @oppulence-gen kind=lib
 * exportUserData is a server-safe auth helper.
 *
 * Everything this application holds about one person: who they are, which
 * workspaces they belong to, the sessions they opened, and the identity
 * actions they took. Product data lives in the attached backend, so the
 * export says what it covers rather than implying it holds everything.
 *
 * This module does not import React or fetch. Inputs are Zod-validated.
 * Owned by `data-export.lit.ts`.
 */

const MAX_EVENTS = 1_000;

function asIso(value: Date | string | null): string {
	if (value === null) return "";
	return value instanceof Date ? value.toISOString() : new Date(value).toISOString();
}

export async function exportUserData(userId: string): Promise<DataExport> {
	const [identity, memberships, sessions, events] = await Promise.all([
		authDb.select().from(authUsers).where(eq(authUsers.id, userId)).limit(1),
		authDb
			.select({
				organizationId: authMembers.organizationId,
				organizationName: authOrganizations.name,
				role: authMembers.role,
				joinedAt: authMembers.createdAt,
			})
			.from(authMembers)
			.innerJoin(authOrganizations, eq(authOrganizations.id, authMembers.organizationId))
			.where(eq(authMembers.userId, userId)),
		authDb
			.select({
				id: authSessions.id,
				createdAt: authSessions.createdAt,
				expiresAt: authSessions.expiresAt,
				ipAddress: authSessions.ipAddress,
				userAgent: authSessions.userAgent,
			})
			.from(authSessions)
			.where(eq(authSessions.userId, userId))
			.orderBy(desc(authSessions.createdAt)),
		authDb
			.select({
				action: identityAuditEvents.action,
				result: identityAuditEvents.result,
				organizationId: identityAuditEvents.organizationId,
				createdAt: identityAuditEvents.createdAt,
			})
			.from(identityAuditEvents)
			.where(eq(identityAuditEvents.actorId, userId))
			.orderBy(desc(identityAuditEvents.createdAt))
			.limit(MAX_EVENTS),
	]);

	const user = identity[0];
	if (!user) throw new Error("Cannot export data for an unknown user");

	return DataExportSchema.parse({
		exportedAt: new Date().toISOString(),
		scope:
			"Identity, workspace membership, sessions and identity events held by this application. Product data lives in the attached backend and is exported from there.",
		identity: {
			id: user.id,
			name: user.name,
			email: user.email,
			emailVerified: user.emailVerified,
			createdAt: asIso(user.createdAt),
		},
		memberships: memberships.map((membership) => ({
			...membership,
			joinedAt: asIso(membership.joinedAt),
		})),
		sessions: sessions.map((session) => ({
			id: session.id,
			createdAt: asIso(session.createdAt),
			expiresAt: asIso(session.expiresAt),
			ipAddress: session.ipAddress,
			userAgent: session.userAgent,
		})),
		identityEvents: events.map((event) => ({
			...event,
			createdAt: asIso(event.createdAt),
		})),
	});
}
