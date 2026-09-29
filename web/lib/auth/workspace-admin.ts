import "server-only";

import { and, asc, eq, sql } from "drizzle-orm";
import { z } from "zod";

import { auth } from "@/lib/auth/auth";
import {
	authDb,
	authInvitations,
	authMembers,
	authOrganizations,
	authSessions,
	authUsers,
} from "@/lib/auth/database";
import {
	WorkspaceAdminStateSchema,
	type AuthorizedSession,
	type WorkspaceAdminAction,
} from "@/lib/auth/schemas";

export class WorkspaceAdminError extends Error {
	constructor(
		readonly code: string,
		readonly status: number,
	) {
		super(code);
	}
}

const roles = (role: string) => role.split(",").map((value) => value.trim());
export const isWorkspaceAdmin = (role: string) =>
	roles(role).some((value) => value === "owner" || value === "admin");
export const isWorkspaceOwner = (role: string) => roles(role).includes("owner");
const InvitationRoleSchema = z.enum(["owner", "admin", "member"]);

export async function getWorkspaceAdminPolicy(organizationId: string) {
	const [organization] = await authDb
		.select({
			requireAdminStepUp: authOrganizations.requireAdminStepUp,
			archivedAt: authOrganizations.archivedAt,
		})
		.from(authOrganizations)
		.where(eq(authOrganizations.id, organizationId))
		.limit(1);
	if (!organization || organization.archivedAt) {
		throw new WorkspaceAdminError("workspace_unavailable", 404);
	}
	return { requireAdminStepUp: organization.requireAdminStepUp !== false };
}

export async function getWorkspaceAdminState(organizationId: string) {
	const [organizations, members, invitations] = await Promise.all([
		authDb
			.select({
				id: authOrganizations.id,
				name: authOrganizations.name,
				slug: authOrganizations.slug,
				logo: authOrganizations.logo,
				requireAdminStepUp: authOrganizations.requireAdminStepUp,
				archivedAt: authOrganizations.archivedAt,
			})
			.from(authOrganizations)
			.where(eq(authOrganizations.id, organizationId))
			.limit(1),
		authDb
			.select({
				id: authMembers.id,
				userId: authMembers.userId,
				name: authUsers.name,
				email: authUsers.email,
				role: authMembers.role,
				createdAt: authMembers.createdAt,
			})
			.from(authMembers)
			.innerJoin(authUsers, eq(authUsers.id, authMembers.userId))
			.where(eq(authMembers.organizationId, organizationId))
			.orderBy(asc(authMembers.createdAt)),
		authDb
			.select({
				id: authInvitations.id,
				email: authInvitations.email,
				role: authInvitations.role,
				status: authInvitations.status,
				expiresAt: authInvitations.expiresAt,
			})
			.from(authInvitations)
			.where(eq(authInvitations.organizationId, organizationId))
			.orderBy(asc(authInvitations.createdAt)),
	]);
	const organization = organizations[0];
	if (!organization || organization.archivedAt)
		throw new WorkspaceAdminError("workspace_unavailable", 404);
	return WorkspaceAdminStateSchema.parse({
		organization: {
			...organization,
			requireAdminStepUp: organization.requireAdminStepUp !== false,
		},
		members,
		invitations,
	});
}

async function scopedInvitation(organizationId: string, invitationId: string) {
	const [invitation] = await authDb
		.select({
			id: authInvitations.id,
			email: authInvitations.email,
			role: authInvitations.role,
			status: authInvitations.status,
		})
		.from(authInvitations)
		.where(
			and(eq(authInvitations.id, invitationId), eq(authInvitations.organizationId, organizationId)),
		)
		.limit(1);
	if (!invitation) throw new WorkspaceAdminError("invitation_not_found", 404);
	return invitation;
}

export async function applyWorkspaceAdminAction(
	session: AuthorizedSession,
	action: WorkspaceAdminAction,
	headers: Headers,
) {
	const organizationId = session.membership.organizationId;
	const actorId = session.user.id;

	if (action.action === "update_workspace") {
		await auth.api.updateOrganization({
			headers,
			body: { organizationId, data: { name: action.name, slug: action.slug, logo: action.logo } },
		});
	} else if (action.action === "update_policy") {
		await authDb
			.update(authOrganizations)
			.set({ requireAdminStepUp: action.requireAdminStepUp })
			.where(eq(authOrganizations.id, organizationId));
	} else if (action.action === "resend_invitation") {
		const invitation = await scopedInvitation(organizationId, action.invitationId);
		if (invitation.status !== "pending")
			throw new WorkspaceAdminError("invitation_not_pending", 409);
		await auth.api.createInvitation({
			headers,
			body: {
				organizationId,
				email: invitation.email,
				role: InvitationRoleSchema.parse(invitation.role ?? "member"),
				resend: true,
			},
		});
	} else if (action.action === "revoke_invitation") {
		await scopedInvitation(organizationId, action.invitationId);
		await auth.api.cancelInvitation({ headers, body: { invitationId: action.invitationId } });
	} else if (action.action === "change_member_role" || action.action === "remove_member") {
		await authDb.transaction(async (tx) => {
			await tx.execute(sql`select pg_advisory_xact_lock(hashtext(${organizationId}))`);
			const [target] = await tx
				.select({ id: authMembers.id, role: authMembers.role, userId: authMembers.userId })
				.from(authMembers)
				.where(
					and(eq(authMembers.id, action.memberId), eq(authMembers.organizationId, organizationId)),
				)
				.limit(1);
			if (!target) throw new WorkspaceAdminError("member_not_found", 404);
			if (target.userId === actorId) throw new WorkspaceAdminError("self_member_mutation", 409);
			if (isWorkspaceOwner(target.role))
				throw new WorkspaceAdminError("owner_mutation_requires_transfer", 409);
			if (action.action === "remove_member") {
				await tx.delete(authMembers).where(eq(authMembers.id, target.id));
				await tx
					.update(authSessions)
					.set({ activeOrganizationId: null })
					.where(
						and(
							eq(authSessions.userId, target.userId),
							eq(authSessions.activeOrganizationId, organizationId),
						),
					);
			} else {
				await tx
					.update(authMembers)
					.set({ role: action.role })
					.where(eq(authMembers.id, target.id));
			}
		});
	} else if (action.action === "transfer_ownership") {
		await authDb.transaction(async (tx) => {
			await tx.execute(sql`select pg_advisory_xact_lock(hashtext(${organizationId}))`);
			const [target] = await tx
				.select({ id: authMembers.id, role: authMembers.role, userId: authMembers.userId })
				.from(authMembers)
				.where(
					and(eq(authMembers.id, action.memberId), eq(authMembers.organizationId, organizationId)),
				)
				.limit(1);
			if (!target) throw new WorkspaceAdminError("member_not_found", 404);
			if (target.userId === actorId)
				throw new WorkspaceAdminError("ownership_transfer_invalid", 409);
			if (isWorkspaceOwner(target.role))
				throw new WorkspaceAdminError("ownership_transfer_invalid", 409);
			await tx.update(authMembers).set({ role: "owner" }).where(eq(authMembers.id, target.id));
			await tx
				.update(authMembers)
				.set({ role: "admin" })
				.where(
					and(eq(authMembers.userId, actorId), eq(authMembers.organizationId, organizationId)),
				);
		});
	} else if (action.action === "archive_workspace") {
		await authDb.transaction(async (tx) => {
			await tx.execute(sql`select pg_advisory_xact_lock(hashtext(${organizationId}))`);
			await tx
				.update(authOrganizations)
				.set({ archivedAt: new Date(), archivedBy: actorId })
				.where(eq(authOrganizations.id, organizationId));
			await tx
				.update(authSessions)
				.set({ activeOrganizationId: null })
				.where(eq(authSessions.activeOrganizationId, organizationId));
		});
	} else {
		await auth.api.deleteOrganization({ headers, body: { organizationId } });
	}

	return { ok: true as const, action: action.action };
}
