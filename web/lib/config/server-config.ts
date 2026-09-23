import "server-only";

import { ServerConfigSchema, type ConfigProblem, type ServerConfig } from "./server-config.schema";

/**
 * @oppulence-gen kind=lib
 * serverConfig is a server-safe config helper.
 *
 * One typed read of the server environment, validated once and cached. Auth
 * keeps its own module (`lib/auth/config.ts`) because Better Auth needs it
 * before anything else boots; `assertServerConfig` checks both and is what
 * `instrumentation.ts` calls, so a misconfigured deployment fails at start
 * instead of on the first request that happens to need the value.
 *
 * `NEXT_PUBLIC_*` variables are deliberately absent: Next inlines those into
 * the browser bundle at build time, so they must be read where they are used.
 *
 * This module does not import React or fetch. Inputs are Zod-validated.
 * Owned by `server-config.lit.ts`.
 */

function text(name: string, fallback = ""): string {
	return process.env[name]?.trim() || fallback;
}

function list(name: string): string[] {
	return text(name)
		.split(",")
		.map((entry) => entry.trim())
		.filter(Boolean);
}

/** The shape as the environment gives it, before validation judges it. */
function rawConfig() {
	return {
		isProduction: process.env.NODE_ENV === "production",
		siteUrl: text("NEXT_PUBLIC_SITE_URL"),
		platformAdminEmails: list("PLATFORM_ADMIN_EMAILS").map((entry) => entry.toLowerCase()),
		brand: {
			name: text("BRAND_NAME", "Check If Email Exists"),
			logoUrl: text("BRAND_LOGO_URL"),
			wordmarkUrl: text("BRAND_WORDMARK_URL"),
			faviconUrl: text("BRAND_FAVICON_URL"),
			primaryColor: text("BRAND_PRIMARY_COLOR"),
			accentColor: text("BRAND_ACCENT_COLOR"),
			supportEmail: text("BRAND_SUPPORT_EMAIL"),
			documentationUrl: text("BRAND_DOCUMENTATION_URL"),
			privacyUrl: text("BRAND_PRIVACY_URL"),
			termsUrl: text("BRAND_TERMS_URL"),
		},
		realtime: {
			allowedOrigins: list("REALTIME_ALLOWED_ORIGINS"),
			backendConnectionPath: text("REALTIME_BACKEND_CONNECTION_PATH", "/v1/realtime/connection"),
			handshakeTimeoutMs: Number(text("REALTIME_HANDSHAKE_TIMEOUT_MS", "10000")),
		},
	};
}

let cached: ServerConfig | null = null;

/** Validated server configuration. Parsed once per process. */
export function serverConfig(): ServerConfig {
	cached ??= ServerConfigSchema.parse(rawConfig());
	return cached;
}

/** Only tests change the environment after the first read. */
export function resetServerConfigCache(): void {
	cached = null;
}

/** Variables a production deployment cannot boot without. */
const PRODUCTION_REQUIRED = [
	"BETTER_AUTH_URL",
	"BETTER_AUTH_SECRET",
	"DATABASE_URL",
	"BACKEND_API_URL",
	"BACKEND_JWT_AUDIENCE",
	"SCIM_CREDENTIAL_HASH_SECRET",
	"RESEND_API_KEY",
	"RESEND_FROM",
];

/**
 * Collects every configuration problem instead of throwing on the first, so a
 * deployment fixes one round of variables rather than one variable per deploy.
 */
export function serverConfigProblems(): ConfigProblem[] {
	const problems: ConfigProblem[] = [];
	const isProduction = process.env.NODE_ENV === "production";

	if (isProduction) {
		for (const variable of PRODUCTION_REQUIRED) {
			if (!process.env[variable]?.trim()) {
				problems.push({ variable, message: "is required in production" });
			}
		}
	}

	const parsed = ServerConfigSchema.safeParse(rawConfig());
	if (!parsed.success) {
		for (const issue of parsed.error.issues) {
			problems.push({ variable: issue.path.join("."), message: issue.message });
		}
	}

	return problems;
}

/** Throws with every problem listed. Called from instrumentation at boot. */
export function assertServerConfig(): void {
	const problems = serverConfigProblems();
	if (problems.length === 0) return;
	const lines = problems.map((problem) => `  - ${problem.variable} ${problem.message}`);
	throw new Error(`Invalid server configuration:\n${lines.join("\n")}`);
}
