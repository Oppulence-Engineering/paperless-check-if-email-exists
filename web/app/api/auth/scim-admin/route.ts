import { randomUUID } from "node:crypto";

import { NextRequest, NextResponse } from "next/server";
import { and, asc, eq } from "drizzle-orm";
import { z } from "zod";

import { parseJsonBody } from "@/lib/api/routes/parse";
import { auth } from "@/lib/auth/auth";
import { authDb, authOrganizations, scimGroupRoleMappings } from "@/lib/auth/database";
import { identityAudit } from "@/lib/auth/identity-audit";
import { publicOrigin } from "@/lib/auth/origin";
import { getAuthorizedSession } from "@/lib/auth/session";
import { hasValidStepUp } from "@/lib/auth/step-up";
import { isSameOriginBrowserRequest } from "@/lib/bff/same-origin-request";

const MAX_BODY_BYTES = 16 * 1024;
const CREDENTIAL_TTL_MS = 365 * 24 * 60 * 60 * 1_000;
const scopes = [
	"scim.users.read",
	"scim.users.write",
	"scim.groups.read",
	"scim.groups.write",
] as const;

const QuerySchema = z.object({ view: z.enum(["providers", "mappings"]) });
const MutationSchema = z.discriminatedUnion("action", [
	z.object({ action: z.literal("create"), providerId: z.string().trim().min(1).max(80) }),
	z.object({ action: z.literal("rotate"), connectionId: z.string().min(1) }),
	z.object({ action: z.literal("revoke"), connectionId: z.string().min(1) }),
	z.object({
		action: z.literal("mapping"),
		connectionId: z.string().min(1),
		group: z.string().trim().min(1).max(255),
		role: z.enum(["admin", "member"]).nullable(),
	}),
]);

async function requireAdministrator(request: NextRequest, mutation: boolean) {
	if (mutation && !isSameOriginBrowserRequest(request, publicOrigin(request))) {
		return { response: NextResponse.json({ error: "forbidden" }, { status: 403 }) } as const;
	}
	const session = await getAuthorizedSession(request.headers);
	if (!session) {
		return { response: NextResponse.json({ error: "unauthenticated" }, { status: 401 }) } as const;
	}
	if (!session.membership.role.split(",").some((role) => role === "owner" || role === "admin")) {
		return { response: NextResponse.json({ error: "forbidden" }, { status: 403 }) } as const;
	}
	if (mutation) {
		const policy = await authDb
			.select({ requireAdminStepUp: authOrganizations.requireAdminStepUp })
			.from(authOrganizations)
			.where(eq(authOrganizations.id, session.membership.organizationId))
			.limit(1);
		if (
			policy[0]?.requireAdminStepUp !== false &&
			!hasValidStepUp({
				verifiedAt: session.session.stepUpVerifiedAt,
				method: session.session.stepUpMethod ?? null,
				purpose: session.session.stepUpPurpose ?? null,
				requiredPurpose: "admin",
				allowedMethods: ["passkey", "totp"],
			})
		) {
			return {
				response: NextResponse.json(
					{ error: "recent identity verification required", code: "step_up_required" },
					{ status: 403 },
				),
			} as const;
		}
	}
	return { session } as const;
}

function displayLabel(creationRequestId: string, fallback: string) {
	const match = creationRequestId.match(/^label:([^:]+):/);
	if (!match) return fallback;
	try {
		return decodeURIComponent(match[1]);
	} catch {
		return fallback;
	}
}

export async function GET(request: NextRequest) {
	const authorized = await requireAdministrator(request, false);
	if ("response" in authorized) return authorized.response;
	const organizationId = authorized.session.membership.organizationId;
	const parsed = QuerySchema.safeParse({ view: request.nextUrl.searchParams.get("view") });
	if (!parsed.success) return NextResponse.json({ error: "invalid view" }, { status: 400 });

	if (parsed.data.view === "providers") {
		const result = await auth.api.listSCIMManagedConnections({
			body: { provisioningDomainId: organizationId },
		});
		return NextResponse.json(
			{
				providers: result.connections.map((connection) => ({
					id: connection.connectionId,
					connectionId: connection.connectionId,
					providerId: displayLabel(connection.creationRequestId, connection.connectionId),
					organizationId,
					status: connection.status,
				})),
			},
			{ headers: { "cache-control": "no-store" } },
		);
	}

	const mappings = await authDb
		.select({
			providerId: scimGroupRoleMappings.connectionId,
			group: scimGroupRoleMappings.groupExternalId,
			role: scimGroupRoleMappings.role,
		})
		.from(scimGroupRoleMappings)
		.where(eq(scimGroupRoleMappings.organizationId, organizationId))
		.orderBy(asc(scimGroupRoleMappings.connectionId), asc(scimGroupRoleMappings.groupExternalId));
	return NextResponse.json({ mappings }, { headers: { "cache-control": "no-store" } });
}

export async function POST(request: NextRequest) {
	const authorized = await requireAdministrator(request, true);
	if ("response" in authorized) return authorized.response;
	const body = await parseJsonBody(request, MutationSchema, MAX_BODY_BYTES);
	if (!body.success) return NextResponse.json({ error: "invalid request" }, { status: 400 });

	const { session } = authorized;
	const organizationId = session.membership.organizationId;
	const actorId = session.user.id;
	const expiresAt = new Date(Date.now() + CREDENTIAL_TTL_MS);
	let targetId = "connectionId" in body.data ? body.data.connectionId : body.data.providerId;

	try {
		if (body.data.action === "create") {
			const result = await auth.api.createSCIMManagedConnection({
				body: {
					creationRequestId: `label:${encodeURIComponent(body.data.providerId)}:${randomUUID()}`,
					provisioningDomainId: organizationId,
					actorId,
					scopes,
					expiresAt,
				},
			});
			targetId = result.connection.connectionId;
			await identityAudit({
				actorId,
				organizationId,
				action: "scim.connection.create",
				targetId,
				result: "success",
				requestId: request.headers.get("x-request-id"),
			});
			return NextResponse.json(
				{ scimToken: result.token },
				{ headers: { "cache-control": "no-store" } },
			);
		}

		if (body.data.action === "rotate") {
			const previous = await auth.api.getSCIMManagedConnection({
				body: { connectionId: body.data.connectionId, provisioningDomainId: organizationId },
			});
			const rotated = await auth.api.rotateSCIMManagedCredential({
				body: {
					connectionId: body.data.connectionId,
					provisioningDomainId: organizationId,
					actorId,
					scopes,
					expiresAt,
				},
			});
			for (const credential of previous.credentials.filter(
				(credential) => credential.status === "active",
			)) {
				await auth.api.revokeSCIMManagedCredential({
					body: {
						connectionId: body.data.connectionId,
						provisioningDomainId: organizationId,
						credentialId: credential.credentialId,
						actorId,
					},
				});
			}
			await identityAudit({
				actorId,
				organizationId,
				action: "scim.credential.rotate",
				targetId,
				result: "success",
				requestId: request.headers.get("x-request-id"),
			});
			return NextResponse.json(
				{ scimToken: rotated.token },
				{ headers: { "cache-control": "no-store" } },
			);
		}

		if (body.data.action === "revoke") {
			await auth.api.decommissionSCIMManagedConnection({
				body: {
					connectionId: body.data.connectionId,
					provisioningDomainId: organizationId,
					actorId,
				},
			});
			await identityAudit({
				actorId,
				organizationId,
				action: "scim.connection.decommission",
				targetId,
				result: "success",
				requestId: request.headers.get("x-request-id"),
			});
			return NextResponse.json({ success: true });
		}

		await auth.api.getSCIMManagedConnection({
			body: { connectionId: body.data.connectionId, provisioningDomainId: organizationId },
		});
		if (body.data.role) {
			await authDb
				.insert(scimGroupRoleMappings)
				.values({
					connectionId: body.data.connectionId,
					organizationId,
					groupExternalId: body.data.group,
					role: body.data.role,
				})
				.onConflictDoUpdate({
					target: [scimGroupRoleMappings.connectionId, scimGroupRoleMappings.groupExternalId],
					set: { role: body.data.role, organizationId },
				});
		} else {
			await authDb
				.delete(scimGroupRoleMappings)
				.where(
					and(
						eq(scimGroupRoleMappings.connectionId, body.data.connectionId),
						eq(scimGroupRoleMappings.organizationId, organizationId),
						eq(scimGroupRoleMappings.groupExternalId, body.data.group),
					),
				);
		}
		await identityAudit({
			actorId,
			organizationId,
			action: "scim.role-mapping.update",
			targetId: `${body.data.connectionId}:${body.data.group}`,
			result: "success",
			requestId: request.headers.get("x-request-id"),
		});
		return NextResponse.json({ success: true });
	} catch {
		await identityAudit({
			actorId,
			organizationId,
			action: `scim.${body.data.action}`,
			targetId,
			result: "failure",
			requestId: request.headers.get("x-request-id"),
		});
		return NextResponse.json({ error: "SCIM operation failed" }, { status: 400 });
	}
}
