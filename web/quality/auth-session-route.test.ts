import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { NextRequest } from "next/server";
import { z } from "zod";

const SessionResponseSchema = z.object({
	authenticated: z.boolean(),
	user: z
		.object({
			id: z.string(),
			organizationId: z.string(),
			role: z.string(),
		})
		.optional(),
});

const mocks = vi.hoisted(() => ({
	acceptInvitation: vi.fn(),
	getAuthorizedSession: vi.fn(),
	setActiveOrganization: vi.fn(),
}));
vi.mock("@/lib/auth/session", () => mocks);
vi.mock("@/lib/auth/auth", () => ({
	auth: {
		api: {
			acceptInvitation: mocks.acceptInvitation,
			setActiveOrganization: mocks.setActiveOrganization,
		},
	},
}));

import {
	GET as confirmInvitation,
	POST as completeInvitation,
} from "@/app/api/auth/complete-invitation/route";
import { GET } from "@/app/api/auth/session/route";

describe("browser session route", () => {
	beforeEach(() => {
		vi.clearAllMocks();
		vi.stubEnv("BETTER_AUTH_URL", "https://ui.example.test");
		vi.stubEnv("TRUSTED_PUBLIC_ORIGINS", "https://ui.example.test");
	});

	afterEach(() => vi.unstubAllEnvs());

	it("returns 401 for an unauthenticated request", async () => {
		mocks.getAuthorizedSession.mockResolvedValue(null);
		const response = await GET(new NextRequest("https://ui.example.test/api/auth/session"));
		expect(response.status).toBe(401);
		await expect(response.json()).resolves.toEqual({ authenticated: false });
	});

	it("returns neutral display data without a session token", async () => {
		mocks.getAuthorizedSession.mockResolvedValue({
			user: {
				id: "user-1",
				name: "Ada",
				email: "ada@example.com",
				emailVerified: true,
			},
			session: { id: "session-1", expiresAt: new Date("2030-01-01T00:00:00Z") },
			membership: { organizationId: "org-1", role: "owner" },
		});
		const response = await GET(new NextRequest("https://ui.example.test/api/auth/session"));
		const body = SessionResponseSchema.parse(await response.json());
		expect(response.status).toBe(200);
		expect(body).toMatchObject({
			authenticated: true,
			user: { id: "user-1", organizationId: "org-1", role: "owner" },
		});
		expect(JSON.stringify(body)).not.toContain("token");
	});

	it("requires confirmation before accepting an invitation", async () => {
		const request = new NextRequest(
			"https://ui.example.test/api/auth/complete-invitation?invitation=invite-1",
		);

		const response = await confirmInvitation(request);

		expect(mocks.acceptInvitation).not.toHaveBeenCalled();
		expect(response.status).toBe(303);
		expect(response.headers.get("location")).toBe(
			"https://ui.example.test/app/invitations/accept?invitation=invite-1&return_to=%2Fapp%2Fsettings%3Fsettings%3Didentity",
		);
	});

	it("activates the organization after a same-origin confirmed invitation", async () => {
		mocks.acceptInvitation.mockResolvedValue({
			invitation: { organizationId: "org-2" },
		});
		const request = new NextRequest("https://ui.example.test/api/auth/complete-invitation", {
			method: "POST",
			headers: {
				origin: "https://ui.example.test",
				"content-type": "application/x-www-form-urlencoded",
			},
			body: new URLSearchParams({ invitation: "invite-1" }),
		});

		const response = await completeInvitation(request);

		expect(mocks.setActiveOrganization).toHaveBeenCalledWith({
			headers: request.headers,
			body: { organizationId: "org-2" },
		});
		expect(response.headers.get("location")).toBe(
			"https://ui.example.test/app/settings?settings=identity",
		);
	});

	it("rejects cross-origin invitation acceptance", async () => {
		const response = await completeInvitation(
			new NextRequest("https://ui.example.test/api/auth/complete-invitation", {
				method: "POST",
				headers: { origin: "https://evil.example" },
				body: new URLSearchParams({ invitation: "invite-1" }),
			}),
		);

		expect(response.status).toBe(403);
		expect(mocks.acceptInvitation).not.toHaveBeenCalled();
	});
});
