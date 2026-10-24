import "server-only";

import { measurePhase, requestTimings, recordTiming } from "./request-timing";

import { randomUUID } from "node:crypto";

import type { NextRequest } from "next/server";
import { NextResponse } from "next/server";

import { BackendProxyPathSchema } from "@/lib/api/routes/schemas/proxy";
import { readBoundedBody } from "@/lib/api/routes/parse";
import { backendApiURL } from "@/lib/auth/config";
import { publicOrigin } from "@/lib/auth/origin";
import {
	deleteUserIdentity,
	getAuthorizedSession as resolveAuthorizedSession,
	mintBackendToken,
} from "@/lib/auth/session";
import type { AuthorizedSession } from "@/lib/auth/schemas";
import { hasValidStepUp } from "@/lib/auth/step-up";
import { isSameOriginBrowserRequest } from "@/lib/bff/same-origin-request";
import { loadBackendCapabilities } from "@/lib/backend/capabilities";
import { EntityTagSchema, IdempotencyKeySchema } from "@/lib/backend/integration-contract.schema";
import {
	acceptedRequestId,
	backendContextHeaders,
	requestSignal,
} from "@/lib/backend/integration-contract";

const FORWARDED_REQUEST_HEADERS = [
	"accept",
	"accept-language",
	"content-type",
	// POST /v1/pipelines/{id}/push requires it (backend/src/http/v1/pipelines.rs).
	"idempotency-key",
	"if-match",
	"if-none-match",
	"last-event-id",
	"range",
	"x-approval-token",
	"x-continuation-token",
	"x-idempotency-key",
	"x-request-id",
] as const;

const RESPONSE_HEADERS = [
	"accept-ranges",
	"cache-control",
	"content-disposition",
	"content-language",
	"content-range",
	"content-type",
	"etag",
	"expires",
	"last-modified",
	"pragma",
	"retry-after",
	"x-backend-session-id",
	"x-request-id",
] as const;

export function backendProxyHeaders(source: Headers): Headers {
	const forwarded = new Headers();
	for (const key of FORWARDED_REQUEST_HEADERS) {
		const value = source.get(key);
		if (value) forwarded.set(key, value);
	}
	return forwarded;
}

type AuthorizedSessionResult =
	{ ok: true; session: AuthorizedSession; token: string } | { ok: false; response: NextResponse };

function requiresAccountDeletionStepUp(request: NextRequest, path: string[]): boolean {
	return request.method === "DELETE" && path.length === 1 && path[0] === "me";
}

export async function getAuthorizedSession(
	request: NextRequest,
	path: string[] = [],
): Promise<AuthorizedSessionResult> {
	try {
		const session = await measurePhase("auth", () => resolveAuthorizedSession(request.headers));
		if (!session) {
			return {
				ok: false,
				response: NextResponse.json(
					{ error: "unauthenticated", code: "unauthorized" },
					{ status: 401 },
				),
			};
		}
		if (requiresAccountDeletionStepUp(request, path)) {
			if (
				!hasValidStepUp({
					verifiedAt: session.session.stepUpVerifiedAt,
					method: session.session.stepUpMethod,
					purpose: session.session.stepUpPurpose,
					requiredPurpose: "account-delete",
					allowedMethods: ["email-otp"],
				})
			) {
				return {
					ok: false,
					response: NextResponse.json(
						{ error: "recent identity verification required", code: "step_up_required" },
						{ status: 403 },
					),
				};
			}
		}
		return {
			ok: true,
			session,
			token: await measurePhase("token", () =>
				mintBackendToken(request.headers, session.membership.organizationId),
			),
		};
	} catch {
		return {
			ok: false,
			response: NextResponse.json(
				{ error: "authentication is temporarily unavailable", code: "session_unavailable" },
				{ status: 503 },
			),
		};
	}
}

export function backendProxyPath(path: string[]): string | null {
	const parsed = BackendProxyPathSchema.safeParse(path);
	if (!parsed.success) return null;
	return `/${parsed.data.map((part) => encodeURIComponent(part)).join("/")}`;
}

export async function proxyBackendAPI(request: NextRequest, path: string[]): Promise<NextResponse> {
	return requestTimings.run({}, async () => {
		const started = performance.now();
		const response = await proxyRequest(request, path);
		recordTiming("total", started);
		const timings = requestTimings.getStore() ?? {};
		response.headers.set("cache-control", "private, no-store");
		response.headers.set(
			"server-timing",
			Object.entries(timings)
				.map(([name, duration]) => `${name};dur=${duration.toFixed(1)}`)
				.join(", "),
		);
		const resources = new Set([
			"overview",
			"me",
			"lists",
			"jobs",
			"bulk",
			"alerts",
			"outcomes",
			"suppressions",
			"pipelines",
			"reverification",
			"research",
			"reputation",
			"emails",
			"query",
			"sources",
			"segments",
			"comments",
			"provider-endpoints",
			"score-policies",
			"outcome-policies",
			"check_email",
			"find_email",
		]);
		if (process.env.PERFORMANCE_TIMING_LOGS === "1")
			console.info(
				JSON.stringify({
					metric: "backend_request_timing",
					route: resources.has(path[1] ?? "") ? `/v1/${path[1]}` : "other",
					method: request.method,
					status: response.status,
					milliseconds: timings,
				}),
			);
		return response;
	});
}

async function proxyRequest(request: NextRequest, path: string[]): Promise<NextResponse> {
	const method = request.method.toUpperCase();
	const requestId = acceptedRequestId(request.headers.get("x-request-id"), randomUUID());
	const errorResponse = (status: number, error: string, code: string): NextResponse =>
		NextResponse.json(
			{ error, code },
			{ status, headers: { "cache-control": "no-store", "x-request-id": requestId } },
		);
	if (
		path[0] === "v0" ||
		(path[0] === "v1" && ["admin", "check-email-with-onboard", "inbound"].includes(path[1] ?? ""))
	) {
		return errorResponse(404, "not found", "not_found");
	}
	if (
		method !== "GET" &&
		method !== "HEAD" &&
		!isSameOriginBrowserRequest(request, publicOrigin(request))
	) {
		return errorResponse(403, "cross-origin request rejected", "forbidden");
	}

	const authorized = await getAuthorizedSession(request, path);
	if (!authorized.ok) {
		authorized.response.headers.set("cache-control", "no-store");
		authorized.response.headers.set("x-request-id", requestId);
		return authorized.response;
	}

	let capabilities;
	try {
		capabilities = loadBackendCapabilities();
	} catch {
		return errorResponse(503, "backend configuration is unavailable", "upstream_unavailable");
	}

	const upstreamPath = backendProxyPath(path);
	if (!upstreamPath) {
		return errorResponse(400, "invalid proxy path", "bad_request");
	}

	const headers = backendProxyHeaders(request.headers);
	const idempotencyKeys = [headers.get("x-idempotency-key"), headers.get("idempotency-key")];
	const entityTag = headers.get("if-match");
	if (idempotencyKeys.some((key) => key && !IdempotencyKeySchema.safeParse(key).success)) {
		return errorResponse(400, "invalid idempotency key", "invalid_idempotency_key");
	}
	if (entityTag && !EntityTagSchema.safeParse(entityTag).success) {
		return errorResponse(400, "invalid If-Match header", "bad_request");
	}
	const contextHeaders = backendContextHeaders({
		requestId,
		userId: authorized.session.user.id,
		organizationId: authorized.session.membership.organizationId,
		organizationRole: authorized.session.membership.role,
		sessionId: authorized.session.session.id,
	});
	contextHeaders.forEach((value, key) => headers.set(key, value));
	headers.set("authorization", `Bearer ${authorized.token}`);
	let body: ArrayBuffer | undefined;
	// The Rust bulk endpoint accepts 50 MiB and list multipart accepts 50 MB.
	const largeUpload =
		method === "POST" &&
		path[0] === "v1" &&
		path.length === 2 &&
		(path[1] === "bulk" || path[1] === "lists");
	if (method !== "GET" && method !== "HEAD") {
		try {
			const boundedBody = await readBoundedBody(
				request,
				largeUpload
					? Math.max(capabilities.http.maxRequestBodyBytes, 50 * 1024 * 1024)
					: capabilities.http.maxRequestBodyBytes,
			);
			if (!boundedBody.success) {
				return errorResponse(413, "request body is too large", "payload_too_large");
			}
			body = boundedBody.data;
		} catch {
			return errorResponse(400, "request body is invalid", "bad_request");
		}
	}

	let upstream: Response;
	try {
		upstream = await measurePhase("upstream", () =>
			fetch(backendApiURL(upstreamPath, request.nextUrl.searchParams), {
				method,
				headers,
				body,
				cache: "no-store",
				redirect: "manual",
				signal: requestSignal(
					request.signal,
					largeUpload
						? Math.max(capabilities.http.requestTimeoutMs, 120_000)
						: capabilities.http.requestTimeoutMs,
				),
			}),
		);
	} catch {
		return errorResponse(503, "backend is unreachable", "upstream_unavailable");
	}

	if (requiresAccountDeletionStepUp(request, path) && upstream.ok) {
		try {
			await deleteUserIdentity(authorized.session.user.id);
		} catch {
			return errorResponse(502, "identity deletion failed", "identity_deletion_failed");
		}
	}

	const responseHeaders = new Headers();
	for (const header of RESPONSE_HEADERS) {
		const value = upstream.headers.get(header);
		if (value) responseHeaders.set(header, value);
	}
	if (!responseHeaders.has("x-request-id")) responseHeaders.set("x-request-id", requestId);

	if (upstream.status >= 500) {
		await upstream.body?.cancel().catch(() => undefined);
		responseHeaders.set("cache-control", "no-store");
		responseHeaders.delete("content-type");
		return NextResponse.json(
			{ error: "backend request failed", code: "upstream_error" },
			{ status: upstream.status, headers: responseHeaders },
		);
	}

	return new NextResponse(upstream.body, {
		status: upstream.status,
		statusText: upstream.statusText,
		headers: responseHeaders,
	});
}
