import bundleAnalyzer from "@next/bundle-analyzer";
import { createMDX } from "fumadocs-mdx/next";
import type { NextConfig } from "next";
import path from "path";
import uiPackage from "./vendor/oppulence/ui/package.json" with { type: "json" };
import { isDevelopment } from "./lib/environment";

const repoRoot = __dirname;

const withMDX = createMDX({ configPath: "config/fumadocs/source.config.ts" });

type Bundler = "webpack" | "turbopack";

/** Normalize alias targets for Turbopack (repo-root-relative, forward slashes). */
function toTurbopackAliasTarget(...segments: string[]): string {
	return path
		.join(...segments)
		.split(path.sep)
		.join("/");
}

/**
 * @oppulence/ui is vendored and file:-linked; pin its runtime deps to this app's node_modules.
 * Webpack accepts absolute paths; Turbopack prepends `./` to alias targets, so absolute
 * paths become invalid server-relative imports — use repo-root-relative paths instead.
 */
function uiPackageResolveAlias(bundler: Bundler): Record<string, string> {
	const appNodeModulesAbs = path.join(__dirname, "node_modules");
	const uiSrcAbs = path.join(repoRoot, "vendor/oppulence/ui/src");
	const runtimeDeps = Object.keys(uiPackage.dependencies ?? {});

	const depPath = (dep: string) =>
		bundler === "turbopack"
			? toTurbopackAliasTarget("node_modules", dep)
			: path.join(appNodeModulesAbs, dep);

	const uiInternal = (suffix: string) =>
		bundler === "turbopack"
			? toTurbopackAliasTarget("vendor/oppulence/ui/src", suffix)
			: path.join(uiSrcAbs, suffix);

	const aliases = Object.fromEntries(runtimeDeps.map((dep) => [dep, depPath(dep)]));

	return {
		...aliases,
		react: depPath("react"),
		"react-dom": depPath("react-dom"),
		"#lib/utils": uiInternal("lib/utils.ts"),
		"#lib/icons": uiInternal("lib/icons.tsx"),
		"#components": uiInternal("components"),
	};
}

const withBundleAnalyzer = bundleAnalyzer({
	enabled: process.env.ANALYZE === "true",
});

// react-grab loads from unpkg in dev and talks to a local MCP server (Cursor 5567,
// Claude 4567, Gemini 5568, OpenCode 6567). Keep these origins dev-only.
const reactGrabDev = {
	script: "https://unpkg.com",
	connect: [
		"http://localhost:4567",
		"http://localhost:5567",
		"http://localhost:5568",
		"http://localhost:6567",
	],
};

const developmentBuild = isDevelopment();

const contentSecurityPolicy = [
	"default-src 'self'",
	`script-src 'self' 'unsafe-inline'${developmentBuild ? ` ${reactGrabDev.script} 'unsafe-eval'` : ""}`,
	"style-src 'self' 'unsafe-inline'",
	"img-src 'self' data: blob: https:",
	"font-src 'self' data:",
	`connect-src 'self'${developmentBuild ? ` ${reactGrabDev.connect.join(" ")} ws://localhost:* ws://127.0.0.1:*` : ""}`,
	"frame-src 'self'",
	"frame-ancestors 'self'",
	"base-uri 'self'",
	"form-action 'self'",
	"object-src 'none'",
].join("; ");

const securityHeaders = [
	{ key: "Content-Security-Policy", value: contentSecurityPolicy },
	{
		key: "Strict-Transport-Security",
		value: "max-age=63072000; includeSubDomains; preload",
	},
	{ key: "X-Content-Type-Options", value: "nosniff" },
	{ key: "Referrer-Policy", value: "strict-origin-when-cross-origin" },
	{
		key: "Permissions-Policy",
		value: "camera=(), geolocation=(), microphone=(self), payment=(), usb=()",
	},
] satisfies Array<{ key: string; value: string }>;

const nextConfig: NextConfig = {
	// Instant Navigations (Next 16.3): prefetchable loading shells + partial
	// prefetching for instant page transitions.
	cacheComponents: true,
	partialPrefetching: true,
	output: "standalone",
	outputFileTracingRoot: __dirname,
	transpilePackages: [
		"@oppulence/ui",
		"@sim/emcn",
		"@sim/workflow-renderer",
		"@sim/workflow-types",
		"@sim/utils",
	],
	// Cursor's embedded browser uses the loopback IP. Without this development
	// exception Next blocks client chunks, leaving the app before hydration on
	// the server-rendered "Checking session" fallback indefinitely.
	allowedDevOrigins: ["127.0.0.1"],
	async headers() {
		return [{ source: "/:path*", headers: securityHeaders }];
	},
	webpack(config) {
		config.resolve.alias = {
			...config.resolve.alias,
			...uiPackageResolveAlias("webpack"),
		};
		return config;
	},
	turbopack: {
		// Relationship contracts are shared with the desktop from the repository
		// package boundary, so Turbopack must be allowed to trace that package.
		root: repoRoot,
		resolveAlias: uiPackageResolveAlias("turbopack"),
	},
};

export default withBundleAnalyzer(withMDX(nextConfig));
