import "server-only";

import type { NextRequest } from "next/server";

/**
 * Public origin of the current request, used to build OAuth redirect URIs.
 *
 * Behind the ingress, `request.nextUrl.origin` reflects the container bind
 * address (e.g. https://0.0.0.0:8080). Resolution order, most authoritative first:
 *   1. BETTER_AUTH_URL — a configured canonical origin. Not
 *      spoofable, always correct; set this in deployed environments.
 *   2. request.nextUrl.origin — direct/local development requests.
 */
export function publicOrigin(request: NextRequest): string {
	const configured = process.env.BETTER_AUTH_URL;
	if (configured) {
		return configured.replace(/\/+$/, "");
	}
	if (process.env.NODE_ENV === "production") throw new Error("BETTER_AUTH_URL is required");
	return request.nextUrl.origin;
}

export { safeReturnTo } from "./return-to";
