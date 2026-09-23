import { NextRequest } from "next/server";
import { afterEach, expect, it, vi } from "vitest";

import { POST } from "./route";

vi.mock("@/lib/auth/config", () => ({
	backendApiURL: (path: string) => new URL(path, "http://127.0.0.1:8080"),
}));
vi.mock("@/lib/backend/capabilities", () => ({
	loadBackendCapabilities: () => ({ http: { requestTimeoutMs: 1000 } }),
}));

afterEach(() => vi.restoreAllMocks());

it("forwards provider callbacks without an API key but protects tenant and onboarding routes", async () => {
	const upstream = vi
		.spyOn(globalThis, "fetch")
		.mockResolvedValue(new Response("received", { status: 202 }));
	const callback = await POST(
		new NextRequest("http://localhost:3000/v1/inbound/providers/postmark/id/token", {
			method: "POST",
			headers: { cookie: "session=private", "x-reacher-secret": "private" },
		}),
		{ params: Promise.resolve({ path: ["inbound", "providers", "postmark", "id", "token"] }) },
	);
	expect(callback.status).toBe(202);
	const forwarded = upstream.mock.calls[0]?.[0] as Request;
	expect(forwarded.url).toBe("http://127.0.0.1:8080/v1/inbound/providers/postmark/id/token");
	expect(forwarded.headers.get("cookie")).toBeNull();
	expect(forwarded.headers.get("x-reacher-secret")).toBeNull();

	const tenant = await POST(
		new NextRequest("http://localhost:3000/v1/check_email", { method: "POST" }),
		{
			params: Promise.resolve({ path: ["check_email"] }),
		},
	);
	expect(tenant.status).toBe(401);
	const onboard = await POST(
		new NextRequest("http://localhost:3000/v1/check-email-with-onboard", { method: "POST" }),
		{ params: Promise.resolve({ path: ["check-email-with-onboard"] }) },
	);
	expect(onboard.status).toBe(403);
	expect(upstream).toHaveBeenCalledTimes(1);
});
