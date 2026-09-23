import { readFileSync } from "node:fs";

import { describe, expect, it } from "vitest";

import { BFF_FETCH_PREFIXES, isBffRequest, resolveRequestUrl } from "@/lib/dev/bff-fetch-log";
import { defaultDevPrefs, getDevPrefs, setDevPrefs } from "@/lib/dev/dev-prefs";
import { personaHandlers } from "@/lib/dev/msw-personas";
import { fullRouteCatalog, productRouteCatalog } from "@/lib/dev/route-catalog";
import { readDevEnvFromProcess } from "@/lib/dev/validate-env";

describe("dev toolkit modules", () => {
	it("detects same-origin BFF paths", () => {
		expect(isBffRequest("/api/backend/revenue-actions")).toBe(true);
		expect(isBffRequest("/api/auth/session")).toBe(true);
		expect(isBffRequest("https://oppulence.io/api/support/chat")).toBe(true);
		expect(isBffRequest("/api/changelog")).toBe(false);
		expect(BFF_FETCH_PREFIXES.length).toBeGreaterThan(0);
	});

	it("resolves fetch input URLs", () => {
		expect(resolveRequestUrl("/api/auth/session")).toBe("/api/auth/session");
		expect(resolveRequestUrl(new URL("https://oppulence.io/api/auth/session"))).toBe(
			"https://oppulence.io/api/auth/session",
		);
	});

	it("persists dev prefs when localStorage is available", () => {
		const storage = new Map<string, string>();
		const original = globalThis.localStorage;
		Object.defineProperty(globalThis, "localStorage", {
			configurable: true,
			value: {
				getItem: (key: string) => storage.get(key) ?? null,
				setItem: (key: string, value: string) => {
					storage.set(key, value);
				},
			},
		});

		setDevPrefs({ ...defaultDevPrefs, mswEnabled: true });
		expect(getDevPrefs().mswEnabled).toBe(true);

		Object.defineProperty(globalThis, "localStorage", {
			configurable: true,
			value: original,
		});
	});

	it("wires tier 1–4 dev providers and scripts", () => {
		const appProviders = readFileSync(
			new URL("../components/providers/app-providers.tsx", import.meta.url),
			"utf8",
		);
		const queryProvider = readFileSync(
			new URL("../components/providers/query-provider.tsx", import.meta.url),
			"utf8",
		);
		const toolkit = readFileSync(
			new URL("../components/dev/dev-toolkit.tsx", import.meta.url),
			"utf8",
		);
		const pkg = readFileSync(new URL("../package.json", import.meta.url), "utf8");

		expect(appProviders).toContain("DevProviders");
		expect(queryProvider).toContain("QueryDevtoolsPanel");
		expect(toolkit).toContain("ToolsTab");
		expect(toolkit).toContain("LoAF");
		expect(pkg).toContain("dev:doctor");
		expect(pkg).toContain("dev:stack");
		expect(pkg).toContain("test:e2e:smoke");
		expect(
			readFileSync(new URL("../public/mockServiceWorker.js", import.meta.url), "utf8"),
		).toContain("Mock Service Worker");
	});

	it("validates dev env with the Better Auth development fallback", () => {
		const report = readDevEnvFromProcess({
			BACKEND_API_URL: "http://127.0.0.1:8081",
		});
		expect(report.ok).toBe(true);
		expect(report.warnings.some((warning) => warning.includes("development Better Auth"))).toBe(
			true,
		);
	});

	it("lists product routes with email verification BFF endpoints", () => {
		const check = productRouteCatalog().find((route) => route.path === "/app/check");
		expect(check?.bffEndpoints).toContain("/api/backend/v1/check_email");
		expect(fullRouteCatalog().length).toBeGreaterThan(productRouteCatalog().length);
	});

	it("returns persona handlers for empty workspace", () => {
		expect(personaHandlers("empty-workspace").length).toBeGreaterThan(0);
		expect(personaHandlers("default")).toEqual([]);
	});
});
