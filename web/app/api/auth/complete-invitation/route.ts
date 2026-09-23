import { NextRequest, NextResponse } from "next/server";
import { z } from "zod";

import { readBoundedBody } from "@/lib/api/routes/parse";
import { auth } from "@/lib/auth/auth";
import { publicOrigin, safeReturnTo } from "@/lib/auth/origin";
import { isSameOriginBrowserRequest } from "@/lib/bff/same-origin-request";

const QuerySchema = z.object({
	invitation: z.string().min(1),
	return_to: z.string().optional(),
});

export async function GET(request: NextRequest) {
	const query = QuerySchema.safeParse(Object.fromEntries(request.nextUrl.searchParams));
	if (!query.success) {
		return NextResponse.redirect(
			new URL("/sign-in?error=invalid_invitation", publicOrigin(request)),
		);
	}
	const confirmation = new URL("/app/invitations/accept", publicOrigin(request));
	confirmation.searchParams.set("invitation", query.data.invitation);
	confirmation.searchParams.set(
		"return_to",
		safeReturnTo(query.data.return_to || "/app/settings?settings=identity"),
	);
	return NextResponse.redirect(confirmation, 303);
}

export async function POST(request: NextRequest) {
	const origin = publicOrigin(request);
	if (!isSameOriginBrowserRequest(request, origin)) {
		return NextResponse.json({ error: "forbidden", code: "forbidden" }, { status: 403 });
	}
	const bounded = await readBoundedBody(request.clone(), 16 * 1024);
	if (!bounded.success) {
		return NextResponse.json({ error: "invalid invitation", code: "bad_request" }, { status: 400 });
	}
	const query = QuerySchema.safeParse(Object.fromEntries(await request.formData()));
	if (!query.success) {
		return NextResponse.redirect(new URL("/sign-in?error=invalid_invitation", origin), 303);
	}
	try {
		const { invitation } = await auth.api.acceptInvitation({
			headers: request.headers,
			body: { invitationId: query.data.invitation },
		});
		await auth.api.setActiveOrganization({
			headers: request.headers,
			body: { organizationId: invitation.organizationId },
		});
		return NextResponse.redirect(
			new URL(safeReturnTo(query.data.return_to || "/app/settings?settings=identity"), origin),
			303,
		);
	} catch {
		return NextResponse.redirect(new URL("/sign-in?error=invalid_invitation", origin), 303);
	}
}
