import { toNextJsHandler } from "better-auth/next-js";
import { z } from "zod";

import { auth } from "@/lib/auth/auth";
import { readBoundedBody } from "@/lib/api/routes/parse";

const handlers = toNextJsHandler(auth);
const MAX_AUTH_BODY_BYTES = 2 * 1024 * 1024;
const TOKEN_RESPONSE_PATHS = new Set([
	"/api/auth/email-otp/verify-email",
	"/api/auth/passkey/verify-authentication",
	"/api/auth/sign-in/email-otp",
	"/api/auth/two-factor/verify-backup-code",
	"/api/auth/two-factor/verify-otp",
	"/api/auth/two-factor/verify-totp",
]);
const JSONRecordSchema = z.looseObject({});

function record(value: unknown): Record<string, unknown> | null {
	const parsed = JSONRecordSchema.safeParse(value);
	return parsed.success ? parsed.data : null;
}

async function removeBrowserSessionTokens(request: Request, response: Response): Promise<Response> {
	if (!TOKEN_RESPONSE_PATHS.has(new URL(request.url).pathname) || !response.ok) return response;
	if (!response.headers.get("content-type")?.includes("application/json")) return response;

	const body = record(await response.clone().json());
	if (!body) return response;
	const sanitized = { ...body };
	delete sanitized.token;
	const session = record(sanitized.session);
	if (session) {
		const safeSession = { ...session };
		delete safeSession.token;
		sanitized.session = safeSession;
	}

	const headers = new Headers(response.headers);
	headers.delete("content-length");
	return Response.json(sanitized, {
		status: response.status,
		statusText: response.statusText,
		headers,
	});
}

export const GET = handlers.GET;

export async function POST(request: Request): Promise<Response> {
	const body = await readBoundedBody(request.clone(), MAX_AUTH_BODY_BYTES);
	if (!body.success) {
		return Response.json(
			{ error: "request body is too large", code: "payload_too_large" },
			{ status: 413 },
		);
	}
	return removeBrowserSessionTokens(request, await handlers.POST(request));
}
