import "server-only";

import path from "node:path";

import { z } from "zod";

const URLSchema = z.url().transform((value) => value.replace(/\/+$/, ""));

const RegistrationModeSchema = z.enum(["open", "invite-only", "disabled"]);

function isE2ELoopback(value: string): boolean {
	const hostname = new URL(value).hostname;
	return (
		process.env.AUTH_E2E_MODE === "1" &&
		(hostname === "localhost" || hostname === "127.0.0.1" || hostname === "[::1]")
	);
}

function isE2ELoopbackDatabase(appUrl: string, databaseUrl: string): boolean {
	if (!isE2ELoopback(appUrl)) return false;
	const hostname = new URL(databaseUrl).hostname;
	return hostname === "localhost" || hostname === "127.0.0.1" || hostname === "[::1]";
}

const RuntimeConfigSchema = z
	.object({
		backendApiUrl: URLSchema,
		backendJwtAudience: z.string().min(1),
		backendOpenApiPath: z.string().min(1),
		betterAuthUrl: URLSchema,
		betterAuthSecret: z.string().min(32),
		scimCredentialHashSecret: z.string().min(32),
		databaseUrl: z.string().min(1),
		resendApiKey: z.string(),
		resendFrom: z
			.string()
			.email()
			.or(z.string().regex(/^.+<[^<>]+@[^<>]+>$/)),
		registrationMode: RegistrationModeSchema,
		trustedOrigins: z.array(URLSchema),
		google: z.object({ clientId: z.string().min(1), clientSecret: z.string().min(1) }).optional(),
		microsoft: z
			.object({ clientId: z.string().min(1), clientSecret: z.string().min(1) })
			.optional(),
		isProduction: z.boolean(),
	})
	.superRefine((config, context) => {
		if (!config.isProduction) return;
		if (
			new URL(config.betterAuthUrl).protocol !== "https:" &&
			!isE2ELoopback(config.betterAuthUrl)
		) {
			context.addIssue({
				code: "custom",
				path: ["betterAuthUrl"],
				message: "BETTER_AUTH_URL must use HTTPS in production",
			});
		}
		if (
			config.trustedOrigins.some(
				(origin) => new URL(origin).protocol !== "https:" && !isE2ELoopback(origin),
			)
		) {
			context.addIssue({
				code: "custom",
				path: ["trustedOrigins"],
				message: "TRUSTED_PUBLIC_ORIGINS must use HTTPS in production",
			});
		}
		if (/change-me|replace-with|dev-only/i.test(config.betterAuthSecret)) {
			context.addIssue({
				code: "custom",
				path: ["betterAuthSecret"],
				message: "BETTER_AUTH_SECRET must not use an example value in production",
			});
		}
		if (/change-me|replace-with|dev-only/i.test(config.scimCredentialHashSecret)) {
			context.addIssue({
				code: "custom",
				path: ["scimCredentialHashSecret"],
				message: "SCIM_CREDENTIAL_HASH_SECRET must not use an example value in production",
			});
		}
		if (
			/postgres:postgres@/i.test(config.databaseUrl) &&
			!isE2ELoopbackDatabase(config.betterAuthUrl, config.databaseUrl)
		) {
			context.addIssue({
				code: "custom",
				path: ["databaseUrl"],
				message: "DATABASE_URL must not use the default development credentials in production",
			});
		}
		if (!config.resendApiKey) {
			context.addIssue({
				code: "custom",
				path: ["resendApiKey"],
				message: "RESEND_API_KEY is required in production",
			});
		}
	});

export type AuthRuntimeConfig = z.infer<typeof RuntimeConfigSchema>;

function optionalProvider(
	enabled: string | undefined,
	clientId: string | undefined,
	clientSecret: string | undefined,
) {
	if (enabled === "0" || (!clientId && !clientSecret)) return undefined;
	return { clientId: clientId ?? "", clientSecret: clientSecret ?? "" };
}

/** Validated server configuration shared by Better Auth and the backend BFF. */
export function getAuthRuntimeConfig(): AuthRuntimeConfig {
	const isProduction = process.env.NODE_ENV === "production";
	const betterAuthUrl =
		process.env.BETTER_AUTH_URL || (isProduction ? "" : "http://localhost:3000");

	return RuntimeConfigSchema.parse({
		backendApiUrl: process.env.BACKEND_API_URL || (isProduction ? "" : "http://localhost:8081"),
		backendJwtAudience:
			process.env.BACKEND_JWT_AUDIENCE || (isProduction ? "" : "check-if-email-exists-api"),
		backendOpenApiPath:
			process.env.BACKEND_OPENAPI_PATH ||
			path.resolve(process.cwd(), "config/contracts/backend.openapi.json"),
		betterAuthUrl,
		betterAuthSecret:
			process.env.BETTER_AUTH_SECRET ||
			(isProduction ? "" : "dev-only-better-auth-secret-change-me"),
		scimCredentialHashSecret:
			process.env.SCIM_CREDENTIAL_HASH_SECRET ||
			(isProduction ? "" : "dev-only-scim-credential-hash-secret-change-me"),
		databaseUrl:
			process.env.DATABASE_URL ||
			(isProduction ? "" : "postgres://postgres:postgres@localhost:25432/reacher"),
		resendApiKey: process.env.RESEND_API_KEY || "",
		resendFrom:
			process.env.RESEND_FROM ||
			(isProduction ? "" : "Check If Email Exists <noreply@example.com>"),
		registrationMode: process.env.REGISTRATION_MODE || "open",
		trustedOrigins: (process.env.TRUSTED_PUBLIC_ORIGINS || betterAuthUrl)
			.split(",")
			.map((origin) => origin.trim())
			.filter(Boolean),
		google: optionalProvider(
			process.env.AUTH_GOOGLE_ENABLED,
			process.env.GOOGLE_CLIENT_ID,
			process.env.GOOGLE_CLIENT_SECRET,
		),
		microsoft: optionalProvider(
			process.env.AUTH_MICROSOFT_ENABLED,
			process.env.MICROSOFT_CLIENT_ID,
			process.env.MICROSOFT_CLIENT_SECRET,
		),
		isProduction,
	});
}

/** Appends a validated API path without discarding a path prefix in BACKEND_API_URL. */
export function backendApiURL(pathname: string, searchParams?: URLSearchParams): URL {
	if (!pathname.startsWith("/") || pathname.startsWith("//")) {
		throw new Error("backendApiURL pathname must be an absolute path");
	}
	const url = new URL(`${getAuthRuntimeConfig().backendApiUrl}${pathname}`);
	if (searchParams) {
		for (const [key, value] of searchParams) url.searchParams.append(key, value);
	}
	return url;
}
