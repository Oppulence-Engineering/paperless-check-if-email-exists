import { describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({ select: vi.fn() }));

vi.mock("@/lib/auth/database", () => ({
	authDb: { select: mocks.select },
	authUsers: { id: "id", name: "name", email: "email" },
	authMembers: { userId: "userId", organizationId: "organizationId", createdAt: "createdAt" },
	authOrganizations: { id: "id", name: "name" },
	authSessions: { userId: "userId", createdAt: "createdAt" },
	identityAuditEvents: { actorId: "actorId", createdAt: "createdAt" },
}));

import { exportUserData } from "./data-export";

/** Each select() call returns a chainable builder resolving to one fixture. */
function chain(rows: unknown[]) {
	const builder = {
		from: () => builder,
		innerJoin: () => builder,
		where: () => builder,
		orderBy: () => builder,
		limit: () => Promise.resolve(rows),
		then: (resolve: (value: unknown) => unknown) => Promise.resolve(rows).then(resolve),
	};
	return builder;
}

describe("exportUserData", () => {
	it("collects identity, membership, sessions and events into one document", async () => {
		const created = new Date("2026-01-01T00:00:00.000Z");
		mocks.select
			.mockReturnValueOnce(
				chain([
					{
						id: "user_1",
						name: "Ada",
						email: "ada@example.com",
						emailVerified: true,
						createdAt: created,
					},
				]),
			)
			.mockReturnValueOnce(
				chain([
					{
						organizationId: "org_1",
						organizationName: "Acme",
						role: "owner",
						joinedAt: created,
					},
				]),
			)
			.mockReturnValueOnce(
				chain([
					{
						id: "sess_1",
						createdAt: created,
						expiresAt: created,
						ipAddress: "203.0.113.4",
						userAgent: "Firefox",
					},
				]),
			)
			.mockReturnValueOnce(
				chain([
					{
						action: "account.export",
						result: "success",
						organizationId: "org_1",
						createdAt: created,
					},
				]),
			);

		const result = await exportUserData("user_1");

		expect(result.identity).toMatchObject({ id: "user_1", email: "ada@example.com" });
		expect(result.memberships[0].organizationName).toBe("Acme");
		expect(result.sessions[0].ipAddress).toBe("203.0.113.4");
		expect(result.identityEvents[0].action).toBe("account.export");
		// Dates leave as ISO strings so the file reads the same everywhere.
		expect(result.identity.createdAt).toBe("2026-01-01T00:00:00.000Z");
		expect(result.scope).toMatch(/attached backend/);
	});

	it("refuses to invent an export for a user that does not exist", async () => {
		mocks.select
			.mockReturnValueOnce(chain([]))
			.mockReturnValueOnce(chain([]))
			.mockReturnValueOnce(chain([]))
			.mockReturnValueOnce(chain([]));

		await expect(exportUserData("ghost")).rejects.toThrow(/unknown user/);
	});
});
