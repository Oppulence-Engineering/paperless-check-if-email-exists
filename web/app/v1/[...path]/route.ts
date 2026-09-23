import type { NextRequest } from "next/server";
import { NextResponse } from "next/server";

import { backendApiURL } from "@/lib/auth/config";
import { backendProxyPath } from "@/lib/auth/proxy";
import { loadBackendCapabilities } from "@/lib/backend/capabilities";
import { requestSignal } from "@/lib/backend/integration-contract";

type RouteContext = { params: Promise<{ path: string[] }> };

async function handle(request: NextRequest, context: RouteContext) {
	const { path } = await context.params;
	if (path[0] === "admin") {
		return NextResponse.json({ error: "Not found" }, { status: 404 });
	}
	if (path.length === 1 && path[0] === "check-email-with-onboard") {
		return NextResponse.json({ error: "Use app sign-up" }, { status: 403 });
	}
	const providerCallback =
		request.method === "POST" &&
		path.length === 5 &&
		path[0] === "inbound" &&
		path[1] === "providers";
	if (!providerCallback && !request.headers.get("authorization")?.startsWith("Bearer rch_live_")) {
		return NextResponse.json({ error: "API key required" }, { status: 401 });
	}

	const upstreamPath = backendProxyPath(["v1", ...path]);
	if (!upstreamPath) {
		return NextResponse.json({ error: "Invalid API path" }, { status: 400 });
	}

	try {
		const upstreamRequest = new Request(
			backendApiURL(upstreamPath, request.nextUrl.searchParams),
			request,
		);
		upstreamRequest.headers.delete("cookie");
		upstreamRequest.headers.delete("x-reacher-secret");
		return await fetch(upstreamRequest, {
			redirect: "manual",
			signal: requestSignal(request.signal, loadBackendCapabilities().http.requestTimeoutMs),
		});
	} catch {
		return NextResponse.json({ error: "Backend unavailable" }, { status: 503 });
	}
}

export const GET = handle;
export const POST = handle;
export const PUT = handle;
export const PATCH = handle;
export const DELETE = handle;
