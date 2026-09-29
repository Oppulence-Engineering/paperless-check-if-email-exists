import "server-only";

import { initTRPC, TRPCError } from "@trpc/server";
import { NextRequest } from "next/server";

import { BackendAPIErrorSchema, type AuthorizedSession } from "@/lib/auth/schemas";
import { getAuthorizedSession } from "@/lib/auth/session";
import { proxyBackendAPI } from "@/lib/auth/proxy";
import { loadBackendCapabilities, publicBackendCapabilities } from "@/lib/backend/capabilities";

import {
	TrpcBackendReadInputSchema,
	TrpcBackendResponseSchema,
	TrpcBackendWriteInputSchema,
	type TrpcBackendReadInput,
	type TrpcBackendWriteInput,
} from "./trpc-bridge.schema";

/**
 * @oppulence-gen kind=lib
 * Authenticated tRPC router for JSON calls through the existing backend BFF.
 *
 * Backend JWTs remain server-only because every procedure reuses proxyBackendAPI.
 * Owned by `trpc-bridge.lit.ts`.
 */
export async function createTrpcBridgeContext(request: Request) {
	return { request, session: await getAuthorizedSession(request.headers) };
}

const trpc = initTRPC.context<{ request: Request; session: AuthorizedSession | null }>().create();
const authenticatedProcedure = trpc.procedure.use(({ ctx, next }) => {
	if (!ctx.session) throw new TRPCError({ code: "UNAUTHORIZED" });
	return next({ ctx: { ...ctx, session: ctx.session } });
});

function backendRequest(
	context: Awaited<ReturnType<typeof createTrpcBridgeContext>>,
	input: TrpcBackendReadInput | TrpcBackendWriteInput,
): NextRequest {
	const url = new URL(
		`/api/backend/${input.path.map((segment) => encodeURIComponent(segment)).join("/")}`,
		context.request.url,
	);
	for (const [key, value] of Object.entries(input.query ?? {})) {
		for (const item of Array.isArray(value) ? value : [value]) url.searchParams.append(key, item);
	}

	const method = "method" in input ? input.method : "GET";
	const headers = new Headers(context.request.headers);
	headers.set("accept", "application/json");
	if (input.requestId) headers.set("x-request-id", input.requestId);
	if ("idempotencyKey" in input && input.idempotencyKey) {
		headers.set("x-idempotency-key", input.idempotencyKey);
	}
	if ("ifMatch" in input && input.ifMatch) headers.set("if-match", input.ifMatch);
	if ("body" in input && input.body !== undefined) headers.set("content-type", "application/json");

	return new NextRequest(url, {
		method,
		headers,
		body: "body" in input && input.body !== undefined ? JSON.stringify(input.body) : undefined,
	});
}

function trpcCode(status: number): ConstructorParameters<typeof TRPCError>[0]["code"] {
	if (status === 401) return "UNAUTHORIZED";
	if (status === 403) return "FORBIDDEN";
	if (status === 404) return "NOT_FOUND";
	if (status === 409) return "CONFLICT";
	if (status === 412) return "PRECONDITION_FAILED";
	if (status === 413) return "PAYLOAD_TOO_LARGE";
	if (status === 429) return "TOO_MANY_REQUESTS";
	if (status === 501) return "NOT_IMPLEMENTED";
	if (status === 502) return "BAD_GATEWAY";
	if (status === 503) return "SERVICE_UNAVAILABLE";
	if (status >= 500) return "INTERNAL_SERVER_ERROR";
	return "BAD_REQUEST";
}

async function proxyJson(
	context: Awaited<ReturnType<typeof createTrpcBridgeContext>>,
	input: TrpcBackendReadInput | TrpcBackendWriteInput,
) {
	const response = await proxyBackendAPI(backendRequest(context, input), input.path);
	const text = await response.text();
	let data: unknown = null;
	if (text) {
		try {
			data = JSON.parse(text);
		} catch {
			throw new TRPCError({ code: "BAD_GATEWAY", message: "Backend returned malformed JSON" });
		}
	}
	if (!response.ok) {
		const error = BackendAPIErrorSchema.safeParse(data);
		throw new TRPCError({
			code: trpcCode(response.status),
			message: error.success
				? error.data.error || error.data.message || error.data.detail || "Backend request failed"
				: "Backend request failed",
		});
	}
	return TrpcBackendResponseSchema.parse({
		status: response.status,
		data,
		requestId: response.headers.get("x-request-id"),
		etag: response.headers.get("etag"),
	});
}

const backendRouter = trpc.router({
	capabilities: authenticatedProcedure.query(() =>
		publicBackendCapabilities(loadBackendCapabilities()),
	),
	read: authenticatedProcedure
		.input(TrpcBackendReadInputSchema)
		.query(({ ctx, input }) => proxyJson(ctx, input)),
	write: authenticatedProcedure
		.input(TrpcBackendWriteInputSchema)
		.mutation(({ ctx, input }) => proxyJson(ctx, input)),
});

export const backendTrpcRouter = trpc.router({ backend: backendRouter });

export type BackendTrpcRouter = typeof backendTrpcRouter;
