import { beforeEach, describe, expect, it, vi } from "vitest";
import { NextRequest } from "next/server";

const mocks = vi.hoisted(() => ({
	getAuthorizedSession: vi.fn(),
	post: vi.fn(() => Promise.resolve(Response.json({ ok: true }))),
}));

vi.mock("better-auth/next-js", () => ({
	toNextJsHandler: () => ({ GET: vi.fn(), POST: mocks.post }),
}));
vi.mock("@/lib/auth/auth", () => ({ auth: {} }));
vi.mock("@/lib/auth/session", () => ({ getAuthorizedSession: mocks.getAuthorizedSession }));
vi.mock("@/lib/auth/database", () => ({ authDb: {}, authSessions: {} }));
vi.mock("@/lib/auth/identity-audit", () => ({ identityAudit: vi.fn() }));

import { POST } from "@/app/api/auth/[...all]/route";
import { DELETE } from "@/app/api/auth/sessions/route";
import { readBoundedBody } from "@/lib/api/routes/parse";

describe("Better Auth route boundary", () => {
	beforeEach(() => vi.clearAllMocks());

	it("rejects oversized authentication requests before parsing them", async () => {
		const response = await POST(
			new Request("https://app.example.test/api/auth/sign-in/email-otp", {
				method: "POST",
				headers: { "Content-Length": String(2 * 1024 * 1024 + 1) },
				body: "{}",
			}),
		);

		expect(response.status).toBe(413);
		expect(mocks.post).not.toHaveBeenCalled();
	});

	it("rejects a streamed body that exceeds the limit without a content-length header", async () => {
		const stream = new ReadableStream({
			start(controller) {
				controller.enqueue(new Uint8Array([1, 2, 3]));
				controller.enqueue(new Uint8Array([4, 5, 6]));
				controller.close();
			},
		});

		await expect(
			readBoundedBody(
				new Request("https://app.example.test", { body: stream, method: "POST", duplex: "half" }),
				5,
			),
		).resolves.toEqual({ success: false });
	});

	it("passes bounded requests to Better Auth", async () => {
		const request = new Request("https://app.example.test/api/auth/sign-in/email-otp", {
			method: "POST",
			body: "{}",
		});

		expect((await POST(request)).status).toBe(200);
		expect(mocks.post).toHaveBeenCalledWith(request);
	});

	it("keeps session credentials out of browser authentication responses", async () => {
		mocks.post.mockResolvedValueOnce(
			Response.json(
				{
					token: "top-level-secret",
					session: { id: "session-1", token: "nested-secret" },
					user: { id: "user-1" },
				},
				{ headers: { "set-cookie": "session=signed; HttpOnly; Path=/" } },
			),
		);
		const response = await POST(
			new Request("https://app.example.test/api/auth/passkey/verify-authentication", {
				method: "POST",
				body: "{}",
			}),
		);

		await expect(response.json()).resolves.toEqual({
			session: { id: "session-1" },
			user: { id: "user-1" },
		});
		expect(response.headers.get("set-cookie")).toContain("HttpOnly");
	});

	it("rejects cross-origin session revocation before reading the session", async () => {
		const response = await DELETE(
			new NextRequest("https://app.example.test/api/auth/sessions", {
				method: "DELETE",
				headers: { Origin: "https://evil.example" },
				body: JSON.stringify({ sessionId: "session-1" }),
			}),
		);

		expect(response.status).toBe(403);
		expect(mocks.getAuthorizedSession).not.toHaveBeenCalled();
	});
});
