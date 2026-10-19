import { randomUUID } from "node:crypto";

import type { NextRequest } from "next/server";
import { NextResponse } from "next/server";

import { isPlatformAdmin } from "@/lib/admin/platform-admin";
import { readBoundedBody } from "@/lib/api/routes/parse";
import { backendApiURL } from "@/lib/auth/config";
import { identityAudit } from "@/lib/auth/identity-audit";
import { publicOrigin } from "@/lib/auth/origin";
import { backendProxyHeaders, backendProxyPath } from "@/lib/auth/proxy";
import { getAuthorizedSession } from "@/lib/auth/session";
import { hasValidStepUp } from "@/lib/auth/step-up";
import { isSameOriginBrowserRequest } from "@/lib/bff/same-origin-request";
import { loadBackendCapabilities } from "@/lib/backend/capabilities";
import { acceptedRequestId, requestSignal } from "@/lib/backend/integration-contract";
import { adminOperationFor } from "@/lib/developer-portal/operations";

type Context = { params: Promise<{ path: string[] }> };

async function handle(request: NextRequest, context: Context) {
	const { path } = await context.params;
	const operation = adminOperationFor(request.method, path);
	if (!operation) return NextResponse.json({ error: "not_found" }, { status: 404 });
	const mutation = request.method !== "GET";
	if (mutation && !isSameOriginBrowserRequest(request, publicOrigin(request))) {
		return NextResponse.json({ error: "forbidden" }, { status: 403 });
	}
	let session;
	try {
		session = await getAuthorizedSession(request.headers);
	} catch {
		return NextResponse.json({ error: "authentication_unavailable" }, { status: 503 });
	}
	if (!session) return NextResponse.json({ error: "unauthorized" }, { status: 401 });
	const requestId = acceptedRequestId(request.headers.get("x-request-id"), randomUUID());
	const granted = isPlatformAdmin(session.user.email);
	if (!granted) {
		await identityAudit({
			actorId: session.user.id,
			organizationId: null,
			action: `platform.backend.${operation.id}`,
			targetId: null,
			result: "failure",
			requestId,
		});
		return NextResponse.json({ error: "not_found" }, { status: 404 });
	}
	const secret = process.env.RCH__HEADER_SECRET;
	if (!secret?.trim())
		return NextResponse.json({ error: "operator_api_unavailable" }, { status: 503 });
	const reason = mutation ? request.headers.get("x-admin-reason")?.trim() : undefined;
	if (mutation && (!reason || reason.length < 10 || reason.length > 255)) {
		return NextResponse.json({ error: "reason_required" }, { status: 400 });
	}
	if (
		mutation &&
		!hasValidStepUp({
			verifiedAt: session.session.stepUpVerifiedAt,
			method: session.session.stepUpMethod,
			purpose: session.session.stepUpPurpose,
			requiredPurpose: "admin",
			allowedMethods: ["totp", "passkey"],
		})
	) {
		return NextResponse.json({ error: "step_up_required" }, { status: 403 });
	}
	const upstreamPath = backendProxyPath(path);
	if (!upstreamPath) return NextResponse.json({ error: "invalid_path" }, { status: 400 });
	const capabilities = loadBackendCapabilities();
	const boundedBody = await readBoundedBody(request, capabilities.http.maxRequestBodyBytes);
	if (!boundedBody.success)
		return NextResponse.json({ error: "payload_too_large" }, { status: 413 });
	// The append-only audit write must succeed before the privileged request leaves this process.
	await identityAudit({
		actorId: session.user.id,
		organizationId: null,
		action: `platform.backend.${operation.id}`,
		targetId: upstreamPath,
		reason: reason ?? null,
		result: "success",
		requestId,
	});
	const headers = backendProxyHeaders(request.headers);
	headers.set("x-reacher-secret", secret);
	headers.set("x-request-id", requestId);
	let upstream: Response;
	try {
		upstream = await fetch(backendApiURL(upstreamPath, request.nextUrl.searchParams), {
			method: request.method,
			headers,
			body: mutation ? boundedBody.data : undefined,
			cache: "no-store",
			redirect: "manual",
			signal: requestSignal(request.signal, capabilities.http.requestTimeoutMs),
		});
	} catch {
		return NextResponse.json({ error: "backend_unavailable" }, { status: 503 });
	}
	if (upstream.status >= 500)
		return NextResponse.json({ error: "upstream_error" }, { status: 502 });
	const responseHeaders = new Headers({ "cache-control": "no-store", "x-request-id": requestId });
	for (const key of ["content-type", "content-disposition", "etag", "retry-after"]) {
		const value = upstream.headers.get(key);
		if (value) responseHeaders.set(key, value);
	}
	return new Response(upstream.body, { status: upstream.status, headers: responseHeaders });
}

export const GET = handle;
export const POST = handle;
export const PUT = handle;
export const PATCH = handle;
export const DELETE = handle;
