import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";

import { safeReturnTo } from "@/lib/auth/origin";
import { backendProxyHeaders, backendProxyPath } from "@/lib/auth/proxy";
import { isSsoSessionForOrganization } from "@/lib/auth/schemas";

const appRoot = dirname(dirname(fileURLToPath(import.meta.url)));

describe("authentication and BFF boundaries", () => {
	it.each(["https://evil.example/path", "//evil.example/path", "javascript:alert(1)"])(
		"rejects unsafe return target %s",
		(target) => {
			expect(safeReturnTo(target)).toBe("/app");
		},
	);

	it("preserves safe relative paths including query and fragment", () => {
		expect(safeReturnTo("/app/agents?view=active#agent-1")).toBe("/app/agents?view=active#agent-1");
	});

	it("preserves backend paths without forcing a version prefix", () => {
		expect(backendProxyPath(["agents", "a/b"])).toBe("/agents/a%2Fb");
	});

	it.each([
		["..", "admin"],
		["%2e%2e", "admin"],
		["agents", "a%2F.."],
	])("rejects path traversal segments", (...path) => {
		expect(backendProxyPath(path)).toBeNull();
	});

	it("forwards only approved request headers", () => {
		const forwarded = backendProxyHeaders(
			new Headers({
				Authorization: "browser-token",
				Cookie: "session=secret",
				"X-Approval-Token": "signed",
				"X-Request-Id": "request-1",
			}),
		);
		expect(forwarded.get("x-approval-token")).toBe("signed");
		expect(forwarded.get("x-request-id")).toBe("request-1");
		expect(forwarded.has("authorization")).toBe(false);
		expect(forwarded.has("cookie")).toBe(false);
	});

	it("binds an SSO-authenticated session to the active organization", () => {
		expect(isSsoSessionForOrganization("sso", "org-1", "org-1")).toBe(true);
		expect(isSsoSessionForOrganization("sso", "org-2", "org-1")).toBe(false);
		expect(isSsoSessionForOrganization("email-otp", "org-1", "org-1")).toBe(false);
	});

	it("uses the Better Auth 1.7 managed SCIM catalog", () => {
		const runtime = readFileSync(resolve(appRoot, "lib/auth/auth.ts"), "utf8");
		const cli = readFileSync(resolve(appRoot, "config/auth/better-auth.cli.ts"), "utf8");
		const adminRoute = readFileSync(resolve(appRoot, "app/api/auth/scim-admin/route.ts"), "utf8");

		expect(runtime).toContain("createScimOptions");
		expect(cli).toContain("managedConnections:");
		expect(adminRoute).toContain("isSameOriginBrowserRequest");
		expect(adminRoute).toContain("hasValidStepUp");
		expect(adminRoute).toContain('"cache-control": "no-store"');
		expect(adminRoute).toContain("decommissionSCIMManagedConnection");
		expect(runtime).not.toContain("providerOwnership:");
		expect(cli).not.toContain("providerOwnership:");
	});
});
