import { NextRequest, NextResponse } from "next/server";

import { getAuthorizedSession } from "@/lib/auth/session";
import { BrowserSessionResponseSchema } from "@/lib/auth/schemas";

export async function GET(request: NextRequest) {
	const session = await getAuthorizedSession(request.headers);
	if (!session) {
		return NextResponse.json(BrowserSessionResponseSchema.parse({ authenticated: false }), {
			status: 401,
			headers: { "cache-control": "no-store" },
		});
	}

	return NextResponse.json(
		BrowserSessionResponseSchema.parse({
			authenticated: true,
			user: {
				id: session.user.id,
				name: session.user.name,
				email: session.user.email,
				emailVerified: session.user.emailVerified,
				sessionId: session.session.id,
				organizationId: session.membership.organizationId,
				role: session.membership.role,
				permissions: [],
			},
			expiresAt: Math.floor(session.session.expiresAt.getTime() / 1_000),
		}),
		{ headers: { "cache-control": "no-store" } },
	);
}
