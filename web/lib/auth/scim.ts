import "server-only";

import type { SCIMOptions, SCIMProjectedRoleGrant } from "@better-auth/scim";
import { and, eq } from "drizzle-orm";

import { authDb, scimGroupRoleMappings } from "@/lib/auth/database";
import { ScimInputSchema, type ScimInput } from "./scim.schema";

/**
 * @oppulence-gen kind=lib
 * scim is a server-safe auth helper.
 * Configures Better Auth 1.7 managed connections and organization projection.
 *
 * This module does not import React or fetch. Inputs are Zod-validated.
 * Owned by `scim.lit.ts`.
 */
export function projectedScimRole(
	grants: readonly Pick<SCIMProjectedRoleGrant, "role">[],
): "admin" | "member" {
	return grants.some((grant) => grant.role === "admin") ? "admin" : "member";
}

export function createScimOptions(input: ScimInput): SCIMOptions {
	const value = ScimInputSchema.parse(input);
	return {
		connections: [],
		managedConnections: {
			credentialHashSecret: value.credentialHashSecret,
			maxActiveCredentials: 5,
			lastUsedWriteIntervalSeconds: 300,
		},
		identity: {
			resolveUser: () => ({ action: "create" }),
			reconcileUser: async (state, context) => {
				await context.database.update({
					model: "user",
					where: [{ field: "id", value: state.userId }],
					update: {
						banned: !state.active,
						banReason: state.active ? null : "Deactivated by directory provisioning",
						banExpires: null,
					},
				});
				if (!state.active) {
					await context.database.deleteMany({
						model: "session",
						where: [{ field: "userId", value: state.userId }],
					});
				}
			},
		},
		projection: {
			roles: {
				map: async ({ connectionId, provisioningDomainId, source }) => {
					const group = source.externalId ?? source.id;
					const mappings = await authDb
						.select({ role: scimGroupRoleMappings.role })
						.from(scimGroupRoleMappings)
						.where(
							and(
								eq(scimGroupRoleMappings.connectionId, connectionId),
								eq(scimGroupRoleMappings.organizationId, provisioningDomainId),
								eq(scimGroupRoleMappings.groupExternalId, group),
							),
						)
						.limit(1);
					return mappings[0]?.role ? [mappings[0].role] : undefined;
				},
				exists: ({ provisioningDomainId, role }) =>
					Boolean(provisioningDomainId) && (role === "admin" || role === "member"),
			},
			reconcileUser: async (state, context) => {
				const current = await context.database.findOne<{ id: string; role: string }>({
					model: "member",
					where: [
						{ field: "organizationId", value: state.provisioningDomainId },
						{ field: "userId", value: state.userId },
					],
				});
				if (!state.active) return;
				const role = projectedScimRole(state.grants);
				if (!current) {
					await context.database.create({
						model: "member",
						data: {
							organizationId: state.provisioningDomainId,
							userId: state.userId,
							role,
							createdAt: new Date(),
						},
					});
				} else if (!current.role.split(",").includes("owner")) {
					await context.database.update({
						model: "member",
						where: [{ field: "id", value: current.id }],
						update: { role },
					});
				}
			},
		},
		compatibility: { microsoftEntra: { acceptLegacyGroupSchema: true } },
	};
}
