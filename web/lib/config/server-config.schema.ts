import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Runtime contract for Server config.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `server-config.lit.ts`.
 */

const OptionalUrlSchema = z.union([z.url(), z.literal("")]).optional();

const BrandConfigSchema = z.object({
	name: z.string().min(1),
	logoUrl: OptionalUrlSchema,
	wordmarkUrl: OptionalUrlSchema,
	faviconUrl: OptionalUrlSchema,
	primaryColor: z.string().optional(),
	accentColor: z.string().optional(),
	supportEmail: z.union([z.email(), z.literal("")]).optional(),
	documentationUrl: OptionalUrlSchema,
	privacyUrl: OptionalUrlSchema,
	termsUrl: OptionalUrlSchema,
});

const RealtimeConfigSchema = z.object({
	allowedOrigins: z.array(z.string().min(1)),
	backendConnectionPath: z.string().startsWith("/"),
	handshakeTimeoutMs: z.number().int().positive().max(60_000),
});

export const ServerConfigSchema = z.object({
	/** Emails allowed to see across tenants. Empty means nobody. */
	platformAdminEmails: z.array(z.email()),
	/** "production" turns every optional-with-a-dev-default into a requirement. */
	isProduction: z.boolean(),
	siteUrl: z.union([z.url(), z.literal("")]),
	brand: BrandConfigSchema,
	realtime: RealtimeConfigSchema,
});

export type ServerConfig = z.infer<typeof ServerConfigSchema>;

export const ConfigProblemSchema = z.object({
	variable: z.string().min(1),
	message: z.string().min(1),
});

export type ConfigProblem = z.infer<typeof ConfigProblemSchema>;
