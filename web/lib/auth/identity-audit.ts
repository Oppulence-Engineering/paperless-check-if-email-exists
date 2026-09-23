import "server-only";

import { randomUUID } from "node:crypto";
import { and, desc, eq } from "drizzle-orm";

import { authDb, authMembers, authUsers, identityAuditEvents } from "./database";
import {
	IdentityAuditInputSchema,
	IdentityAuditListInputSchema,
	IdentityAuditResponseSchema,
	type IdentityAuditEvent,
	type IdentityAuditInput,
	type IdentityAuditListInput,
} from "./identity-audit.schema";

/**
 * @oppulence-gen kind=lib
 * identityAudit is a server-safe auth helper.
 * Records and lists organization-scoped identity activity.
 *
 * This module does not import React or fetch. Inputs are Zod-validated.
 * Owned by `identity-audit.lit.ts`.
 */
export async function identityAudit(input: IdentityAuditInput): Promise<void> {
	const value = IdentityAuditInputSchema.parse(input);
	await authDb.insert(identityAuditEvents).values({
		id: randomUUID(),
		actorId: value.actorId,
		organizationId: value.organizationId,
		action: value.action,
		targetId: value.targetId,
		reason: value.reason ?? null,
		result: value.result,
		requestId: value.requestId,
		createdAt: new Date(),
	});
}

/** Lists organization-scoped identity activity without exposing session data. */
export async function listIdentityAuditEvents(
	input: IdentityAuditListInput,
): Promise<IdentityAuditEvent[]> {
	const value = IdentityAuditListInputSchema.parse(input);
	const events = await authDb
		.select({
			id: identityAuditEvents.id,
			actorId: identityAuditEvents.actorId,
			actorName: authUsers.name,
			actorEmail: authUsers.email,
			organizationId: identityAuditEvents.organizationId,
			action: identityAuditEvents.action,
			targetId: identityAuditEvents.targetId,
			reason: identityAuditEvents.reason,
			result: identityAuditEvents.result,
			requestId: identityAuditEvents.requestId,
			createdAt: identityAuditEvents.createdAt,
		})
		.from(identityAuditEvents)
		.leftJoin(
			authMembers,
			and(
				eq(authMembers.userId, identityAuditEvents.actorId),
				eq(authMembers.organizationId, value.organizationId),
			),
		)
		.leftJoin(authUsers, eq(authUsers.id, authMembers.userId))
		.where(eq(identityAuditEvents.organizationId, value.organizationId))
		.orderBy(desc(identityAuditEvents.createdAt))
		.limit(value.limit);

	return IdentityAuditResponseSchema.parse({ events }).events;
}
