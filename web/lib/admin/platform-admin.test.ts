import { afterEach, describe, expect, it } from "vitest";

import { isPlatformAdmin, platformAdminEmails } from "./platform-admin";

const saved = process.env.PLATFORM_ADMIN_EMAILS;

afterEach(() => {
	if (saved === undefined) delete process.env.PLATFORM_ADMIN_EMAILS;
	else process.env.PLATFORM_ADMIN_EMAILS = saved;
});

describe("isPlatformAdmin", () => {
	it("lets nobody in when the allowlist is unset", () => {
		delete process.env.PLATFORM_ADMIN_EMAILS;
		expect(platformAdminEmails()).toEqual([]);
		expect(isPlatformAdmin("ada@example.com")).toBe(false);
	});

	it("lets nobody in when the allowlist is blank", () => {
		process.env.PLATFORM_ADMIN_EMAILS = "  , ,";
		expect(isPlatformAdmin("ada@example.com")).toBe(false);
	});

	it("reads a comma-separated list and ignores spacing and case", () => {
		process.env.PLATFORM_ADMIN_EMAILS = " Ada@Example.com , grace@example.com ";
		expect(platformAdminEmails()).toEqual(["ada@example.com", "grace@example.com"]);
		expect(isPlatformAdmin("ADA@example.com")).toBe(true);
		expect(isPlatformAdmin("grace@example.com")).toBe(true);
	});

	it("refuses an address that is not on the list", () => {
		process.env.PLATFORM_ADMIN_EMAILS = "ada@example.com";
		expect(isPlatformAdmin("mallory@example.com")).toBe(false);
		expect(isPlatformAdmin("")).toBe(false);
		expect(isPlatformAdmin(null)).toBe(false);
		expect(isPlatformAdmin(undefined)).toBe(false);
	});

	it("does not match a lookalike address", () => {
		process.env.PLATFORM_ADMIN_EMAILS = "ada@example.com";
		expect(isPlatformAdmin("ada@example.com.evil.test")).toBe(false);
		expect(isPlatformAdmin("xada@example.com")).toBe(false);
	});
});
