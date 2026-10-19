import "server-only";

import { headers } from "next/headers";
import { redirect } from "next/navigation";
import { connection } from "next/server";
import { and, asc, eq, isNull } from "drizzle-orm";
import { z } from "zod";

import { auth, backendTokenProof } from "@/lib/auth/auth";
import { authDb, authMembers, authOrganizations, authSessions } from "@/lib/auth/database";
import { safeReturnTo } from "@/lib/auth/origin";
import {
	ActiveMembershipSchema,
	AuthorizedSessionSchema,
	isSsoSessionForOrganization,
	WorkspaceSummaryListSchema,
	type AuthorizedSession,
	type WorkspaceSummary,
} from "@/lib/auth/schemas";

const BackendTokenClaimsSchema = z.object({ org_id: z.string().min(1) });

function backendTokenOrganizationId(token: string): string | null {
	const payload = token.split(".")[1];
	if (!payload) return null;
	try {
		return BackendTokenClaimsSchema.parse(
			JSON.parse(Buffer.from(payload, "base64url").toString("utf8")),
		).org_id;
	} catch {
		return null;
	}
}

async function sessionPolicy(sessionId: string) {
	const result = await authDb
		.select({
			authenticationMethod: authSessions.authenticationMethod,
			createdAt: authSessions.createdAt,
			updatedAt: authSessions.updatedAt,
			stepUpVerifiedAt: authSessions.stepUpVerifiedAt,
			stepUpMethod: authSessions.stepUpMethod,
			stepUpPurpose: authSessions.stepUpPurpose,
			activeOrganizationId: authSessions.activeOrganizationId,
			ssoOrganizationId: authSessions.ssoOrganizationId,
			maxSessionAgeSeconds: authOrganizations.maxSessionAgeSeconds,
			idleTimeoutSeconds: authOrganizations.idleTimeoutSeconds,
			requireSso: authOrganizations.requireSso,
			archivedAt: authOrganizations.archivedAt,
		})
		.from(authSessions)
		.innerJoin(authOrganizations, eq(authOrganizations.id, authSessions.activeOrganizationId))
		.where(eq(authSessions.id, sessionId))
		.limit(1);
	const policy = result[0];
	if (!policy) return null;
	if (policy.archivedAt) return null;
	const now = Date.now();
	const maximumAgeMs = (policy.maxSessionAgeSeconds ?? 30 * 24 * 60 * 60) * 1_000;
	if (now - policy.createdAt.getTime() > maximumAgeMs) return null;
	if (
		policy.idleTimeoutSeconds &&
		now - policy.updatedAt.getTime() > policy.idleTimeoutSeconds * 1_000
	) {
		return null;
	}
	if (
		policy.requireSso === true &&
		!isSsoSessionForOrganization(
			policy.authenticationMethod,
			policy.activeOrganizationId,
			policy.ssoOrganizationId,
		)
	)
		return null;
	return policy;
}

/** Resolves and validates a Better Auth session and its active membership. */
export async function getAuthorizedSession(
	requestHeaders: Headers,
): Promise<AuthorizedSession | null> {
	const session = await auth.api.getSession({ headers: requestHeaders });
	if (!session?.session.activeOrganizationId) return null;

	const member = await auth.api.getActiveMember({ headers: requestHeaders });
	const membership = ActiveMembershipSchema.safeParse(member);
	if (
		!membership.success ||
		membership.data.userId !== session.user.id ||
		membership.data.organizationId !== session.session.activeOrganizationId
	)
		return null;
	const policy = await sessionPolicy(session.session.id);
	if (!policy) return null;

	return AuthorizedSessionSchema.parse({
		user: session.user,
		session: {
			...session.session,
			stepUpVerifiedAt: policy.stepUpVerifiedAt,
			stepUpMethod: policy.stepUpMethod,
			stepUpPurpose: policy.stepUpPurpose,
		},
		membership: membership.data,
	});
}

/** Resolves the current Server Component request without exposing its session token. */
export async function getOptionalSession(): Promise<AuthorizedSession | null> {
	await connection();
	return getAuthorizedSession(await headers());
}

export async function requireSession(returnTo = "/app"): Promise<AuthorizedSession> {
	const session = await getOptionalSession();
	if (!session) {
		const target = safeReturnTo(returnTo);
		redirect(`/sign-in?return_to=${encodeURIComponent(target)}`);
	}
	return session;
}

/** Lists every organization the user can enter without exposing auth credentials. */
export async function listUserWorkspaces(userId: string): Promise<WorkspaceSummary[]> {
	const workspaces = await authDb
		.select({
			id: authOrganizations.id,
			name: authOrganizations.name,
			slug: authOrganizations.slug,
			role: authMembers.role,
			logoUrl: authOrganizations.logo,
		})
		.from(authMembers)
		.innerJoin(authOrganizations, eq(authOrganizations.id, authMembers.organizationId))
		.where(and(eq(authMembers.userId, userId), isNull(authOrganizations.archivedAt)))
		.orderBy(asc(authOrganizations.name));

	return WorkspaceSummaryListSchema.parse(workspaces);
}

/** Mints the short-lived JWKS-verifiable token used only for server-to-backend calls. */
export async function mintBackendToken(
	requestHeaders: Headers,
	expectedOrganizationId?: string,
): Promise<string> {
	const internalHeaders = new Headers(requestHeaders);
	internalHeaders.set("x-backend-token-request", backendTokenProof);
	const token = (await auth.api.getToken({ headers: internalHeaders })).token;
	if (expectedOrganizationId && backendTokenOrganizationId(token) !== expectedOrganizationId) {
		throw new Error("Backend token tenant changed during authorization");
	}
	return token;
}

/** Removes every Better Auth session before deleting the identity record. */
export async function deleteUserIdentity(userId: string): Promise<void> {
	const context = await auth.$context;
	await context.internalAdapter.deleteUserSessions(userId);
	await context.internalAdapter.deleteUser(userId);
}
