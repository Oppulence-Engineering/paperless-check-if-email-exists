import "server-only";

import { sql } from "drizzle-orm";
import { drizzle } from "drizzle-orm/node-postgres";
import { boolean, integer, pgSchema, primaryKey, text, timestamp, uuid } from "drizzle-orm/pg-core";
import { Kysely, PostgresDialect } from "kysely";
import { Pool } from "pg";

import { getAuthRuntimeConfig } from "@/lib/auth/config";
import { DatabaseInputSchema, type DatabaseInput } from "./database.schema";

const globalDatabase = globalThis as typeof globalThis & {
	betterAuthPool?: Pool;
	betterAuthDatabase?: Kysely<unknown>;
};

const schema = pgSchema("auth");
const date = (name: string) => timestamp(name, { withTimezone: true, mode: "date" });

export const authUsers = schema.table("user", {
	id: text("id").primaryKey(),
	name: text("name").notNull(),
	email: text("email").notNull(),
	emailVerified: boolean("emailVerified").notNull(),
	createdAt: date("createdAt").notNull(),
	updatedAt: date("updatedAt").notNull(),
	legacyWorkosUserId: text("legacyWorkosUserId"),
});

export const authOrganizations = schema.table("organization", {
	id: text("id").primaryKey(),
	name: text("name").notNull(),
	slug: text("slug").notNull(),
	logo: text("logo"),
	createdAt: date("createdAt").notNull(),
	legacyWorkosOrganizationId: text("legacyWorkosOrganizationId"),
	requireSso: boolean("requireSso"),
	requireAdminStepUp: boolean("requireAdminStepUp"),
	maxSessionAgeSeconds: integer("maxSessionAgeSeconds"),
	idleTimeoutSeconds: integer("idleTimeoutSeconds"),
	brandName: text("brandName"),
	brandLogoUrl: text("brandLogoUrl"),
	brandWordmarkUrl: text("brandWordmarkUrl"),
	brandFaviconUrl: text("brandFaviconUrl"),
	brandPrimaryColor: text("brandPrimaryColor"),
	brandAccentColor: text("brandAccentColor"),
	supportEmail: text("supportEmail"),
	documentationUrl: text("documentationUrl"),
	termsUrl: text("termsUrl"),
	privacyUrl: text("privacyUrl"),
	archivedAt: date("archivedAt"),
	archivedBy: text("archivedBy"),
});

export const authMembers = schema.table("member", {
	id: text("id").primaryKey(),
	organizationId: text("organizationId").notNull(),
	userId: text("userId").notNull(),
	role: text("role").notNull(),
	createdAt: date("createdAt").notNull(),
});

export const authInvitations = schema.table("invitation", {
	id: text("id").primaryKey(),
	organizationId: text("organizationId").notNull(),
	email: text("email").notNull(),
	role: text("role"),
	status: text("status").notNull(),
	expiresAt: date("expiresAt").notNull(),
	createdAt: date("createdAt").notNull(),
});

export const authSessions = schema.table("session", {
	id: text("id").primaryKey(),
	userId: text("userId").notNull(),
	createdAt: date("createdAt").notNull(),
	updatedAt: date("updatedAt").notNull(),
	expiresAt: date("expiresAt").notNull(),
	ipAddress: text("ipAddress"),
	userAgent: text("userAgent"),
	activeOrganizationId: text("activeOrganizationId"),
	authenticationMethod: text("authenticationMethod"),
	stepUpVerifiedAt: date("stepUpVerifiedAt"),
	stepUpMethod: text("stepUpMethod"),
	stepUpPurpose: text("stepUpPurpose"),
	ssoProviderId: text("ssoProviderId"),
	ssoOrganizationId: text("ssoOrganizationId"),
});

export const authSsoProviders = schema.table("ssoProvider", {
	id: text("id").primaryKey(),
	providerId: text("providerId").notNull(),
	organizationId: text("organizationId"),
});

export const identityAuditEvents = schema.table("identity_audit_event", {
	id: uuid("id").primaryKey(),
	actorId: text("actorId"),
	organizationId: text("organizationId"),
	action: text("action").notNull(),
	targetId: text("targetId"),
	result: text("result").notNull(),
	requestId: text("requestId"),
	createdAt: date("createdAt").notNull(),
});

export const scimGroupRoleMappings = schema.table(
	"scim_group_role_mapping",
	{
		connectionId: text("connectionId").notNull(),
		organizationId: text("organizationId").notNull(),
		groupExternalId: text("groupExternalId").notNull(),
		role: text("role").notNull(),
	},
	(table) => [primaryKey({ columns: [table.connectionId, table.groupExternalId] })],
);

const authPool =
	globalDatabase.betterAuthPool ??
	new Pool({
		connectionString: getAuthRuntimeConfig().databaseUrl,
		options:
			"-c search_path=auth,pg_catalog -c statement_timeout=15000 -c idle_in_transaction_session_timeout=15000",
		max: 10,
	});

export const authDatabase =
	globalDatabase.betterAuthDatabase ??
	new Kysely<unknown>({ dialect: new PostgresDialect({ pool: authPool }) });

export const authDb = drizzle(authPool, {
	schema: {
		authInvitations,
		authMembers,
		authOrganizations,
		authSessions,
		authSsoProviders,
		authUsers,
		identityAuditEvents,
		scimGroupRoleMappings,
	},
});

if (process.env.NODE_ENV !== "production") {
	globalDatabase.betterAuthPool = authPool;
	globalDatabase.betterAuthDatabase = authDatabase;
}

/**
 * @oppulence-gen kind=lib
 * database is a server-safe auth helper.
 * Checks connectivity to the Better Auth database.
 *
 * This module does not import React or fetch. Inputs are Zod-validated.
 * Owned by `database.lit.ts`.
 */
export async function database(input: DatabaseInput = { timeoutMs: 2_000 }): Promise<boolean> {
	const { timeoutMs } = DatabaseInputSchema.parse(input);
	const timeout = AbortSignal.timeout(timeoutMs);
	await Promise.race([
		authDb.execute(sql`select 1`),
		new Promise<never>((_, reject) => {
			timeout.addEventListener("abort", () => reject(timeout.reason), {
				once: true,
			});
		}),
	]);
	return true;
}
