import { NextRequest, NextResponse } from "next/server";
import { and, desc, eq } from "drizzle-orm";

import { parseJsonBody } from "@/lib/api/routes/parse";
import { authDb, authSessions } from "@/lib/auth/database";
import { identityAudit } from "@/lib/auth/identity-audit";
import { publicOrigin } from "@/lib/auth/origin";
import { getAuthorizedSession } from "@/lib/auth/session";
import { RevokeIdentitySessionSchema } from "@/lib/auth/schemas";
import { isSameOriginBrowserRequest } from "@/lib/bff/same-origin-request";

const MAX_REVOKE_BODY_BYTES = 16 * 1024;

export async function GET(request: NextRequest) {
	const current = await getAuthorizedSession(request.headers);
	if (!current) {
		return NextResponse.json({ error: "unauthenticated", code: "unauthorized" }, { status: 401 });
	}
	const sessions = await authDb
		.select({
			id: authSessions.id,
			createdAt: authSessions.createdAt,
			updatedAt: authSessions.updatedAt,
			expiresAt: authSessions.expiresAt,
			ipAddress: authSessions.ipAddress,
			userAgent: authSessions.userAgent,
		})
		.from(authSessions)
		.where(eq(authSessions.userId, current.user.id))
		.orderBy(desc(authSessions.updatedAt));
	return NextResponse.json(
		{
			sessions: sessions.map((session) => ({
				...session,
				current: session.id === current.session.id,
			})),
		},
		{ headers: { "cache-control": "no-store" } },
	);
}

export async function DELETE(request: NextRequest) {
	if (!isSameOriginBrowserRequest(request, publicOrigin(request))) {
		return NextResponse.json({ error: "forbidden", code: "forbidden" }, { status: 403 });
	}
	const current = await getAuthorizedSession(request.headers);
	if (!current) {
		return NextResponse.json({ error: "unauthenticated", code: "unauthorized" }, { status: 401 });
	}
	const body = await parseJsonBody(request, RevokeIdentitySessionSchema, MAX_REVOKE_BODY_BYTES);
	if (!body.success) {
		return NextResponse.json({ error: "invalid session", code: "bad_request" }, { status: 400 });
	}
	const revoked = await authDb
		.delete(authSessions)
		.where(and(eq(authSessions.id, body.data.sessionId), eq(authSessions.userId, current.user.id)))
		.returning({ id: authSessions.id });
	await identityAudit({
		actorId: current.user.id,
		organizationId: current.membership.organizationId,
		action: "session.revoke",
		targetId: body.data.sessionId,
		result: revoked.length === 1 ? "success" : "failure",
		requestId: request.headers.get("x-request-id"),
	});
	if (revoked.length !== 1) {
		return NextResponse.json({ error: "session not found", code: "not_found" }, { status: 404 });
	}
	return NextResponse.json({ status: true });
}
