import { passkey } from "@better-auth/passkey";
import { scim } from "@better-auth/scim";
import { sso } from "@better-auth/sso";
import { betterAuth } from "better-auth";
import { admin, emailOTP, jwt, organization, twoFactor } from "better-auth/plugins";
import { Kysely, PostgresDialect } from "kysely";
import { Pool } from "pg";
import { z } from "zod";

import { BrandAssetURLSchema, HexColorSchema, PublicURLSchema } from "../../lib/auth/schemas";

const DAY_SECONDS = 24 * 60 * 60;
const pool = new Pool({
	connectionString: process.env.DATABASE_URL || "postgres:///postgres",
	options: "-c search_path=auth,pg_catalog",
});

/** Schema-only mirror used by the Better Auth CLI. Runtime policy stays in lib/auth/auth.ts. */
export const auth = betterAuth({
	baseURL: "http://localhost:3000",
	secret: process.env.BETTER_AUTH_SECRET || "schema-generation-secret-change-me",
	database: {
		db: new Kysely<unknown>({ dialect: new PostgresDialect({ pool }) }),
		type: "postgres",
		transaction: true,
	},
	rateLimit: {
		enabled: true,
		storage: "database",
		window: 60,
		max: 100,
	},
	session: {
		additionalFields: {
			authenticationMethod: {
				type: "string",
				required: false,
				input: false,
				returned: false,
			},
			stepUpVerifiedAt: {
				type: "date",
				required: false,
				input: false,
				returned: false,
			},
			stepUpMethod: {
				type: "string",
				required: false,
				input: false,
				returned: false,
			},
			stepUpPurpose: {
				type: "string",
				required: false,
				input: false,
				returned: false,
			},
			ssoProviderId: {
				type: "string",
				required: false,
				input: false,
				returned: false,
			},
			ssoOrganizationId: {
				type: "string",
				required: false,
				input: false,
				returned: false,
			},
		},
	},
	user: {
		additionalFields: {
			legacyWorkosUserId: {
				type: "string",
				required: false,
				input: false,
				returned: false,
				unique: true,
			},
		},
	},
	plugins: [
		admin(),
		organization({
			schema: {
				organization: {
					additionalFields: {
						legacyWorkosOrganizationId: {
							type: "string",
							required: false,
							input: false,
							returned: false,
							unique: true,
						},
						requireSso: {
							type: "boolean",
							required: false,
							defaultValue: false,
						},
						requireAdminStepUp: {
							type: "boolean",
							required: false,
							defaultValue: true,
						},
						maxSessionAgeSeconds: {
							type: "number",
							required: false,
							defaultValue: 30 * DAY_SECONDS,
							validator: {
								input: z
									.number()
									.int()
									.min(900)
									.max(365 * DAY_SECONDS),
							},
						},
						idleTimeoutSeconds: {
							type: "number",
							required: false,
							validator: {
								input: z
									.number()
									.int()
									.min(300)
									.max(30 * DAY_SECONDS),
							},
						},
						brandName: { type: "string", required: false },
						brandLogoUrl: {
							type: "string",
							required: false,
							validator: { input: BrandAssetURLSchema },
						},
						brandWordmarkUrl: {
							type: "string",
							required: false,
							validator: { input: BrandAssetURLSchema },
						},
						brandFaviconUrl: {
							type: "string",
							required: false,
							validator: { input: BrandAssetURLSchema },
						},
						brandPrimaryColor: {
							type: "string",
							required: false,
							validator: { input: HexColorSchema },
						},
						brandAccentColor: {
							type: "string",
							required: false,
							validator: { input: HexColorSchema },
						},
						supportEmail: { type: "string", required: false },
						documentationUrl: {
							type: "string",
							required: false,
							validator: { input: PublicURLSchema },
						},
						termsUrl: {
							type: "string",
							required: false,
							validator: { input: PublicURLSchema },
						},
						privacyUrl: {
							type: "string",
							required: false,
							validator: { input: PublicURLSchema },
						},
						archivedAt: {
							type: "date",
							required: false,
							input: false,
							returned: false,
						},
						archivedBy: {
							type: "string",
							required: false,
							input: false,
							returned: false,
						},
					},
				},
			},
		}),
		emailOTP({ sendVerificationOTP: async () => undefined }),
		passkey(),
		twoFactor(),
		sso({
			domainVerification: { enabled: true },
			disableImplicitSignUp: false,
			organizationProvisioning: { disabled: false, defaultRole: "member" },
			saml: {
				enableInResponseToValidation: true,
				allowIdpInitiated: false,
				requireTimestamps: true,
				clockSkew: 2 * 60 * 1_000,
			},
		}),
		scim({
			connections: [],
			managedConnections: {
				credentialHashSecret:
					process.env.SCIM_CREDENTIAL_HASH_SECRET ||
					"schema-only-scim-credential-hash-secret-change-me",
			},
			identity: { resolveUser: () => ({ action: "create" }) },
			projection: {
				roles: { map: () => undefined, exists: () => true },
				reconcileUser: () => undefined,
			},
		}),
		jwt(),
	],
});
