import { getSessionCookie } from "better-auth/cookies";
import { NextRequest, NextResponse } from "next/server";

import { safeReturnTo } from "@/lib/auth/origin";

/**
 * Reject anonymous product requests before App Router starts streaming.
 *
 * The protected layout performs the same validation as a defense-in-depth
 * boundary. Keeping this early check in proxy.ts guarantees direct browser and
 * API requests receive a real HTTP redirect instead of an in-stream redirect.
 */
export function proxy(request: NextRequest) {
	const publicOrigin = process.env.BETTER_AUTH_URL || request.nextUrl.origin;
	const forwardedProtocol = request.headers.get("x-forwarded-proto")?.split(",")[0]?.trim();
	const e2eLoopback =
		process.env.AUTH_E2E_MODE === "1" &&
		["localhost", "127.0.0.1"].includes(new URL(publicOrigin).hostname);
	if (forwardedProtocol === "http" && !e2eLoopback) {
		const secureURL = new URL(`${request.nextUrl.pathname}${request.nextUrl.search}`, publicOrigin);
		secureURL.protocol = "https:";
		return NextResponse.redirect(secureURL, 308);
	}

	if (!request.nextUrl.pathname.startsWith("/app")) {
		return NextResponse.next();
	}

	// This is an optimistic presence check. The product layout performs the
	// authoritative database-backed session and membership check.
	if (!getSessionCookie(request)) {
		const login = new URL("/sign-in", publicOrigin);
		login.searchParams.set(
			"return_to",
			safeReturnTo(`${request.nextUrl.pathname}${request.nextUrl.search}`),
		);
		return NextResponse.redirect(login);
	}

	return NextResponse.next();
}

export const config = {
	matcher: [
		"/app/:path*",
		"/sign-in",
		"/sign-up",
		{
			source: "/:path*",
			has: [{ type: "header", key: "x-forwarded-proto", value: "http" }],
		},
	],
};
