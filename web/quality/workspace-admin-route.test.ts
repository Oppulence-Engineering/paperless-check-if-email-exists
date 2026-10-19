import { beforeEach, describe, expect, it, vi } from "vitest";
import { NextRequest } from "next/server";

const mocks = vi.hoisted(() => ({
	apply: vi.fn(),
	audit: vi.fn(),
	getPolicy: vi.fn(),
	getSession: vi.fn(),
	getState: vi.fn(),
}));

vi.mock("@/lib/auth/session", () => ({ getAuthorizedSession: mocks.getSession }));
vi.mock("@/lib/auth/identity-audit", () => ({ identityAudit: mocks.audit }));
vi.mock("@/lib/auth/workspace-admin", () => {
	class WorkspaceAdminError extends Error {
		constructor(
			readonly code: string,
			readonly status: number,
		) {
			super(code);
		}
	}
	const roles = (role: string) => role.split(",");
	return {
		WorkspaceAdminError,
		applyWorkspaceAdminAction: mocks.apply,
		getWorkspaceAdminPolicy: mocks.getPolicy,
		getWorkspaceAdminState: mocks.getState,
		isWorkspaceAdmin: (role: string) =>
			roles(role).some((value) => ["owner", "admin"].includes(value)),
		isWorkspaceOwner: (role: string) => roles(role).includes("owner"),
	};
});

import { GET, POST } from "@/app/api/auth/workspace-admin/route";

const session = {
	user: { id: "user-1", name: "Ada", email: "ada@example.com", emailVerified: true },
	session: {
		id: "session-1",
		userId: "user-1",
		activeOrganizationId: "org-1",
		createdAt: new Date(),
		updatedAt: new Date(),
		expiresAt: new Date(Date.now() + 60_000),
		stepUpVerifiedAt: new Date(),
		stepUpMethod: "passkey" as const,
		stepUpPurpose: "admin" as const,
	},
	membership: { id: "member-1", userId: "user-1", organizationId: "org-1", role: "owner" },
};

function post(body: unknown, origin = "https://ui.example.test") {
	return new NextRequest("https://ui.example.test/api/auth/workspace-admin", {
		method: "POST",
		headers: { origin, "content-type": "application/json" },
		body: JSON.stringify(body),
	});
}

describe("workspace administration route", () => {
	beforeEach(() => {
		vi.clearAllMocks();
		vi.stubEnv("BETTER_AUTH_URL", "https://ui.example.test");
		vi.stubEnv("TRUSTED_PUBLIC_ORIGINS", "https://ui.example.test");
		mocks.getSession.mockResolvedValue(session);
		mocks.getPolicy.mockResolvedValue({ requireAdminStepUp: true });
		mocks.apply.mockResolvedValue({ ok: true, action: "update_policy" });
		mocks.audit.mockResolvedValue(undefined);
	});

	it("returns validated tenant state for an administrator", async () => {
		mocks.getState.mockResolvedValue({
			organization: { id: "org-1" },
			members: [],
			invitations: [],
		});
		const response = await GET(new NextRequest("https://ui.example.test/api/auth/workspace-admin"));
		expect(response.status).toBe(200);
		expect(mocks.getState).toHaveBeenCalledWith("org-1");
		expect(response.headers.get("cache-control")).toBe("no-store");
	});

	it("rejects cross-origin mutations before touching auth state", async () => {
		const response = await POST(
			post({ action: "update_policy", requireAdminStepUp: false }, "https://evil.test"),
		);
		expect(response.status).toBe(403);
		expect(mocks.getSession).not.toHaveBeenCalled();
	});

	it("requires fresh passkey or TOTP verification by default", async () => {
		mocks.getSession.mockResolvedValue({
			...session,
			session: { ...session.session, stepUpVerifiedAt: null },
		});
		const response = await POST(post({ action: "update_policy", requireAdminStepUp: false }));
		expect(response.status).toBe(403);
		await expect(response.json()).resolves.toMatchObject({ code: "step_up_required" });
		expect(mocks.apply).not.toHaveBeenCalled();
	});

	it("honors an explicitly disabled persisted step-up policy", async () => {
		mocks.getPolicy.mockResolvedValue({ requireAdminStepUp: false });
		mocks.getSession.mockResolvedValue({
			...session,
			session: { ...session.session, stepUpVerifiedAt: null },
		});
		const response = await POST(post({ action: "update_policy", requireAdminStepUp: true }));
		expect(response.status).toBe(200);
		expect(mocks.apply).toHaveBeenCalledOnce();
		const calledAction: unknown = mocks.apply.mock.calls[0]?.[1];
		const calledHeaders: unknown = mocks.apply.mock.calls[0]?.[2];
		expect(calledAction).toEqual({ action: "update_policy", requireAdminStepUp: true });
		expect(calledHeaders).toBeInstanceOf(Headers);
	});

	it("reserves transfer, archive, and deletion for owners", async () => {
		mocks.getSession.mockResolvedValue({
			...session,
			membership: { ...session.membership, role: "admin" },
		});
		const response = await POST(post({ action: "archive_workspace" }));
		expect(response.status).toBe(403);
		await expect(response.json()).resolves.toMatchObject({ code: "owner_required" });
		expect(mocks.apply).not.toHaveBeenCalled();
	});

	it("rejects invalid discriminated actions", async () => {
		const response = await POST(
			post({ action: "change_member_role", memberId: "member-2", role: "owner" }),
		);
		expect(response.status).toBe(400);
		expect(mocks.apply).not.toHaveBeenCalled();
	});
});
