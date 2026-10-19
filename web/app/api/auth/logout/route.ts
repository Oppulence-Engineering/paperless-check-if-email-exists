import { NextRequest, NextResponse } from "next/server";

import { parseSearchParams } from "@/lib/api/routes/parse";
import { LogoutQuerySchema } from "@/lib/api/routes/schemas/auth";
import { auth } from "@/lib/auth/auth";
import { publicOrigin, safeReturnTo } from "@/lib/auth/origin";
import { isSameOriginNavigation } from "@/lib/bff/same-origin-request";

async function logoutResponse(request: NextRequest) {
	if (!isSameOriginNavigation(request, publicOrigin(request))) {
		return NextResponse.json(
			{ error: "logout requires a same-origin request", code: "method_not_allowed" },
			{ status: 405, headers: { Allow: "GET, POST" } },
		);
	}

	const query = parseSearchParams(request.nextUrl.searchParams, LogoutQuerySchema);
	const target = new URL(
		safeReturnTo(query.success ? query.data.return_to || "/" : "/"),
		publicOrigin(request),
	);
	const signedOut = await auth.api.signOut({ headers: request.headers, asResponse: true });
	const responseHeaders = new Headers(signedOut.headers);
	responseHeaders.set("location", target.toString());
	return new Response(null, { status: 303, headers: responseHeaders });
}

export const GET = logoutResponse;
export const POST = logoutResponse;
