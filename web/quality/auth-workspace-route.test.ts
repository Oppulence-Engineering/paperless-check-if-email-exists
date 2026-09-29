import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { NextRequest } from "next/server";

const mocks = vi.hoisted(() => ({
	createOrganization: vi.fn(),
	getAuthorizedSession: vi.fn(),
	identityAudit: vi.fn(),
	setActiveOrganization: vi.fn(),
}));

vi.mock("@/lib/auth/session", () => ({ getAuthorizedSession: mocks.getAuthorizedSession }));
vi.mock("@/lib/auth/identity-audit", () => ({ identityAudit: mocks.identityAudit }));
vi.mock("@/lib/auth/auth", () => ({
	auth: {
		api: {
			createOrganization: mocks.createOrganization,
			setActiveOrganization: mocks.setActiveOrganization,
		},
	},
}));

import { POST, PUT } from "@/app/api/auth/workspace/route";

describe("active workspace route", () => {
	beforeEach(() => {
		vi.clearAllMocks();
		vi.stubEnv("BETTER_AUTH_URL", "https://ui.example.test");
		mocks.getAuthorizedSession.mockResolvedValue({
			user: { id: "user-1" },
			membership: { organizationId: "workspace-1", role: "owner" },
		});
		mocks.identityAudit.mockResolvedValue(undefined);
		mocks.createOrganization.mockResolvedValue({
			id: "workspace-2",
			name: "Juniper Health",
			slug: "juniper-health",
			logo: null,
		});
	});

	afterEach(() => vi.unstubAllEnvs());

	it("switches only through a same-origin request and records the tenant event", async () => {
		const request = new NextRequest("https://ui.example.test/api/auth/workspace", {
			method: "POST",
			headers: { origin: "https://ui.example.test", "content-type": "application/json" },
			body: JSON.stringify({ organizationId: "workspace-2" }),
		});

		const response = await POST(request);

		expect(response.status).toBe(200);
		await expect(response.json()).resolves.toEqual({ activeOrganizationId: "workspace-2" });
		expect(mocks.setActiveOrganization).toHaveBeenCalledWith({
			headers: request.headers,
			body: { organizationId: "workspace-2" },
		});
		expect(mocks.identityAudit).toHaveBeenCalledWith(
			expect.objectContaining({
				action: "organization.switch",
				organizationId: "workspace-2",
				result: "success",
			}),
		);
	});

	it("keeps a successful tenant switch successful when audit persistence is unavailable", async () => {
		mocks.identityAudit.mockRejectedValueOnce(new Error("audit unavailable"));
		const response = await POST(
			new NextRequest("https://ui.example.test/api/auth/workspace", {
				method: "POST",
				headers: { origin: "https://ui.example.test", "content-type": "application/json" },
				body: JSON.stringify({ organizationId: "workspace-2" }),
			}),
		);

		expect(response.status).toBe(200);
		await expect(response.json()).resolves.toEqual({ activeOrganizationId: "workspace-2" });
		expect(mocks.setActiveOrganization).toHaveBeenCalledOnce();
	});

	it("rejects cross-origin workspace mutations", async () => {
		const response = await POST(
			new NextRequest("https://ui.example.test/api/auth/workspace", {
				method: "POST",
				headers: { origin: "https://outside.example", "content-type": "application/json" },
				body: JSON.stringify({ organizationId: "workspace-2" }),
			}),
		);

		expect(response.status).toBe(403);
		expect(mocks.setActiveOrganization).not.toHaveBeenCalled();
	});

	it("creates and activates a new workspace through the same-origin boundary", async () => {
		const request = new NextRequest("https://ui.example.test/api/auth/workspace", {
			method: "PUT",
			headers: { origin: "https://ui.example.test", "content-type": "application/json" },
			body: JSON.stringify({ name: "Juniper Health", slug: "juniper-health" }),
		});

		const response = await PUT(request);

		expect(response.status).toBe(200);
		await expect(response.json()).resolves.toEqual({
			activeOrganizationId: "workspace-2",
			workspace: {
				id: "workspace-2",
				name: "Juniper Health",
				slug: "juniper-health",
				role: "owner",
				logoUrl: null,
			},
		});
		expect(mocks.createOrganization).toHaveBeenCalledWith({
			headers: request.headers,
			body: { name: "Juniper Health", slug: "juniper-health" },
		});
		expect(mocks.identityAudit).toHaveBeenCalledWith(
			expect.objectContaining({
				action: "organization.create",
				organizationId: "workspace-2",
				result: "success",
			}),
		);
	});

	it("returns the created workspace when audit persistence is temporarily unavailable", async () => {
		mocks.identityAudit.mockRejectedValueOnce(new Error("audit unavailable"));
		const response = await PUT(
			new NextRequest("https://ui.example.test/api/auth/workspace", {
				method: "PUT",
				headers: { origin: "https://ui.example.test", "content-type": "application/json" },
				body: JSON.stringify({ name: "Juniper Health", slug: "juniper-health" }),
			}),
		);

		expect(response.status).toBe(200);
		await expect(response.json()).resolves.toMatchObject({
			activeOrganizationId: "workspace-2",
		});
	});

	it("rejects invalid workspace slugs before calling Better Auth", async () => {
		const response = await PUT(
			new NextRequest("https://ui.example.test/api/auth/workspace", {
				method: "PUT",
				headers: { origin: "https://ui.example.test", "content-type": "application/json" },
				body: JSON.stringify({ name: "Juniper Health", slug: "Juniper Health" }),
			}),
		);

		expect(response.status).toBe(400);
		expect(mocks.createOrganization).not.toHaveBeenCalled();
	});
});
