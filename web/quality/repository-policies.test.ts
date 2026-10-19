import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { describe, expect, it } from "vitest";
import { parse } from "yaml";

import nextConfig from "../next.config";

const appRoot = path.dirname(path.dirname(fileURLToPath(import.meta.url)));

const allowedRootFiles = new Set([
	".dockerignore",
	".gitignore",
	".npmrc",
	".prettierignore",
	"AGENTS.md",
	"CLAUDE.md",
	"Dockerfile",
	"Dockerfile.dockerignore",
	"Makefile",
	"README.md",
	"compose.yaml",
	"components.json",
	"eslint.config.mjs",
	"global.d.ts",
	"next-env.d.ts",
	// Next discovers this at the root; it cannot move to config/.
	"instrumentation.ts",
	"next.config.ts",
	"package.json",
	"pnpm-lock.yaml",
	"playwright.config.ts",
	"postcss.config.mjs",
	"proxy.ts",
	"tsconfig.json",
	"vitest.config.ts",
]);

function isLocalRootFile(filename: string): boolean {
	return (
		filename === ".DS_Store" || filename.startsWith(".env") || filename.endsWith(".tsbuildinfo")
	);
}

// WEB021: a user sent to an external page (OAuth, hosted authorization) comes
// back to a screen that still shows the old state unless the flow refreshes on
// return. Composio connections shipped that way: the panel opened the consent
// page and never refetched, so a finished connection read "not connected"
// until a manual reload, and users reported the integration as broken.
const externalFlowRefreshExemptions = new Map<string, string>([
	[
		"components/features/settings/app-settings/app-settings.tsx",
		"opens a static documentation link; there is no state to refresh",
	],
]);

function sourceFilesBelow(root: string): string[] {
	if (!fs.existsSync(root)) return [];
	return fs.readdirSync(root, { withFileTypes: true }).flatMap((entry) => {
		const target = path.join(root, entry.name);
		if (entry.isDirectory()) return sourceFilesBelow(target);
		return /\.(ts|tsx)$/.test(entry.name) && !entry.name.includes(".test.") ? [target] : [];
	});
}

describe("repository architecture policies", () => {
	it("keeps the contributor-facing application root intentional", () => {
		const unexpectedFiles = fs
			.readdirSync(appRoot, { withFileTypes: true })
			.filter((entry) => entry.isFile())
			.map((entry) => entry.name)
			.filter((filename) => !allowedRootFiles.has(filename) && !isLocalRootFile(filename));

		expect(
			unexpectedFiles,
			"Move repository-owned configuration to config/, automation to scripts/, and engineering records to docs/",
		).toEqual([]);
	});

	it("uses pnpm as the only package manager", () => {
		const packageJson = JSON.parse(fs.readFileSync(path.join(appRoot, "package.json"), "utf8")) as {
			packageManager?: string;
		};

		expect(packageJson.packageManager).toMatch(/^pnpm@/);
		expect(fs.existsSync(path.join(appRoot, "pnpm-lock.yaml"))).toBe(true);
		expect(fs.existsSync(path.join(appRoot, "package-lock.json"))).toBe(false);
	});

	it("keeps local environment files out of every Docker build context", () => {
		const dockerignore = fs.readFileSync(path.join(appRoot, "../.dockerignore"), "utf8");
		expect(dockerignore).toContain("**/.env\n");
		expect(dockerignore).toContain("**/.env.*");
	});

	it("WEB015 configures the complete security header boundary", async () => {
		expect(typeof nextConfig).toBe("object");
		const configuredHeaders = await nextConfig.headers?.();
		const headers = new Map(
			configuredHeaders?.flatMap((entry) =>
				entry.headers.map((header) => [header.key, header.value]),
			) ?? [],
		);

		for (const required of [
			"Content-Security-Policy",
			"Strict-Transport-Security",
			"X-Content-Type-Options",
			"Referrer-Policy",
			"Permissions-Policy",
		]) {
			expect(headers.has(required), `missing ${required}`).toBe(true);
		}
		expect(headers.get("Content-Security-Policy")).toContain("frame-ancestors");
		expect(headers.get("Content-Security-Policy")).toContain("object-src 'none'");
	});

	it("WEB016 gives the protected product root loading and error boundaries", () => {
		const productRoot = path.join(appRoot, "app/(product)/app");
		for (const filename of ["layout.tsx", "page.tsx", "loading.tsx", "error.tsx"]) {
			expect(fs.existsSync(path.join(productRoot, filename)), `missing ${filename}`).toBe(true);
		}
		expect(fs.readFileSync(path.join(productRoot, "layout.tsx"), "utf8")).toMatch(
			/requireSession\s*\(/,
		);
		for (const route of ["check", "lists", "history", "settings"]) {
			expect(
				fs.existsSync(path.join(productRoot, route, "page.tsx")),
				`missing /app/${route}`,
			).toBe(true);
		}
		expect(fs.readFileSync(path.join(appRoot, "proxy.ts"), "utf8")).toContain('"/app/:path*"');
	});

	it("WEB017 verifies the vendored packages the application depends on", () => {
		// The application vendors its `file:` dependencies, so a change inside
		// vendor/ is a change to the app and must run the same gates.
		const workflowPath = path.join(appRoot, "../.github/workflows/pr.yml");
		expect(fs.existsSync(workflowPath), "CI workflow is missing").toBe(true);

		const workflow = parse(fs.readFileSync(workflowPath, "utf8")) as {
			on?: { push?: { paths?: string[] }; pull_request?: unknown };
		};
		const filtered = workflow.on?.push?.paths ?? [];
		for (const vendored of ["vendor/oppulence/ui", "vendor/sim"]) {
			const covered = filtered.length === 0 || filtered.some((entry) => entry.startsWith(vendored));
			expect(covered, `CI does not run when ${vendored} changes`).toBe(true);
		}
	});

	it("WEB018 keeps the source OpenAPI document and generator configuration in-repo", () => {
		expect(fs.existsSync(path.join(appRoot, "config/contracts/backend.openapi.json"))).toBe(true);
		expect(fs.existsSync(path.join(appRoot, "config/contracts/orval.config.ts"))).toBe(true);
	});

	it("WEB021 refreshes state when the user returns from an external page", () => {
		const offenders: string[] = [];
		for (const root of ["app", "components", "lib"]) {
			for (const filename of sourceFilesBelow(path.join(appRoot, root))) {
				const source = fs.readFileSync(filename, "utf8");
				if (!source.includes("window.open(")) continue;

				const relative = path.relative(appRoot, filename).replaceAll(path.sep, "/");
				if (externalFlowRefreshExemptions.has(relative)) continue;

				const refreshesOnReturn =
					source.includes("visibilitychange") || source.includes('addEventListener("focus"');
				if (!refreshesOnReturn) offenders.push(relative);
			}
		}

		expect(
			offenders,
			"Sending a user to an external page must refresh on return: listen for visibilitychange or focus, or record an exemption in externalFlowRefreshExemptions with the reason",
		).toEqual([]);
	});

	it("scans dependencies on every push, not only before a release", () => {
		const workflow = fs.readFileSync(path.join(appRoot, "../.github/workflows/pr.yml"), "utf8");
		// The scanner version is pinned in package.json; CI only has to run it.
		expect(workflow).toContain("pnpm security:osv");
		expect(workflow).toContain("pnpm security:gitleaks");
		expect(workflow).toContain("pnpm security:semgrep");

		const scanner = JSON.parse(fs.readFileSync(path.join(appRoot, "package.json"), "utf8")) as {
			scripts: Record<string, string>;
		};
		expect(scanner.scripts["security:osv"]).toContain("osv-scanner@v2.5.1");
	});
});
