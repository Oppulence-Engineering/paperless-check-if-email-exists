import { NextRequest, NextResponse } from "next/server";

import { listIdentityAuditEvents } from "@/lib/auth/identity-audit";
import { IdentityAuditResponseSchema } from "@/lib/auth/identity-audit.schema";
import { getAuthorizedSession } from "@/lib/auth/session";

export async function GET(request: NextRequest) {
	const session = await getAuthorizedSession(request.headers);
	if (!session) {
		return NextResponse.json({ error: "unauthenticated", code: "unauthorized" }, { status: 401 });
	}
	if (!session.membership.role.split(",").some((role) => role === "owner" || role === "admin")) {
		return NextResponse.json({ error: "forbidden", code: "forbidden" }, { status: 403 });
	}

	const events = await listIdentityAuditEvents({
		organizationId: session.membership.organizationId,
		limit: 100,
	});
	return NextResponse.json(IdentityAuditResponseSchema.parse({ events }), {
		headers: { "cache-control": "no-store" },
	});
}
