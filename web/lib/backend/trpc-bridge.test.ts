import { beforeEach, describe, expect, it, vi } from "vitest";
import { NextResponse } from "next/server";

import type { AuthorizedSession } from "@/lib/auth/schemas";
import { proxyBackendAPI } from "@/lib/auth/proxy";

import { backendTrpcRouter } from "./trpc-bridge";

vi.mock("@/lib/auth/proxy", () => ({ proxyBackendAPI: vi.fn() }));

const session = {
	user: { id: "user-1", name: "Ada", email: "ada@example.com", emailVerified: true },
	session: {
		id: "session-1",
		userId: "user-1",
		activeOrganizationId: "org-1",
		createdAt: new Date(),
		updatedAt: new Date(),
		expiresAt: new Date(Date.now() + 60_000),
	},
	membership: { id: "member-1", userId: "user-1", organizationId: "org-1", role: "owner" },
} satisfies AuthorizedSession;

const mockProxy = vi.mocked(proxyBackendAPI);

beforeEach(() => mockProxy.mockReset());

describe("backendTrpcRouter", () => {
	it("rejects anonymous backend calls", async () => {
		const caller = backendTrpcRouter.createCaller({
			request: new Request("http://app.example/api/trpc"),
			session: null,
		});
		await expect(caller.backend.read({ path: ["widgets"] })).rejects.toMatchObject({
			code: "UNAUTHORIZED",
		});
	});

	it("reuses the authenticated BFF for backend JSON", async () => {
		mockProxy.mockResolvedValueOnce(
			NextResponse.json(
				{ widgets: [] },
				{ status: 200, headers: { etag: '"v1"', "x-request-id": "request-1" } },
			),
		);
		const caller = backendTrpcRouter.createCaller({
			request: new Request("http://app.example/api/trpc", {
				headers: { cookie: "session=opaque" },
			}),
			session,
		});

		await expect(caller.backend.read({ path: ["widgets"] })).resolves.toEqual({
			status: 200,
			data: { widgets: [] },
			requestId: "request-1",
			etag: '"v1"',
		});
		expect(mockProxy.mock.calls[0]?.[1]).toEqual(["widgets"]);
	});
});
