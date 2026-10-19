import "server-only";

import { createHmac, randomUUID, timingSafeEqual } from "node:crypto";

import { passkey } from "@better-auth/passkey";
import { scim as scimPlugin } from "@better-auth/scim";
import { sso } from "@better-auth/sso";
import { stripe } from "@better-auth/stripe";
import { betterAuth } from "better-auth";
import { APIError, createAuthMiddleware, getAuthoritativeSessionFromCtx } from "better-auth/api";
import { admin, emailOTP, jwt, organization, twoFactor } from "better-auth/plugins";
import { Resend } from "resend";
import Stripe from "stripe";
import { z } from "zod";

import { and, asc, eq, gt, ilike, isNull, sql } from "drizzle-orm";

import { getAuthRuntimeConfig } from "@/lib/auth/config";
import {
	authDatabase,
	authDb,
	authInvitations,
	authMembers,
	authOrganizations,
	authSessions,
	authSsoProviders,
	authUsers,
} from "@/lib/auth/database";
import { identityAudit } from "@/lib/auth/identity-audit";
import { createScimOptions, scimWorkspaceForToken } from "@/lib/auth/scim";
import { BrandAssetURLSchema, HexColorSchema, PublicURLSchema } from "@/lib/auth/schemas";
import { hasValidStepUp } from "@/lib/auth/step-up";
import { syncTenantPlan } from "@/lib/bff/tenant-sync";
import { workspacePlan } from "@/lib/billing/workspace-plan";
import { can, entitlementsFor, seatLimit } from "@/lib/entitlements/entitlements";

function defaultWorkspaceName(user: { name: string; email: string }): string {
	return user.name.trim() || user.email.split("@")[0] || "Workspace";
}

const config = getAuthRuntimeConfig();
const resend = config.resendApiKey ? new Resend(config.resendApiKey) : null;
const localE2EOtp =
	process.env.AUTH_E2E_MODE === "1" &&
	["localhost", "127.0.0.1"].includes(new URL(config.betterAuthUrl).hostname)
		? process.env.AUTH_E2E_OTP
		: undefined;
const DAY_SECONDS = 24 * 60 * 60;
export const backendTokenProof = createHmac("sha256", config.betterAuthSecret)
	.update("backend-token-request")
	.digest("base64url");

function validBackendTokenProof(candidate: string | null): boolean {
	if (!candidate) return false;
	const actual = Buffer.from(candidate);
	const expected = Buffer.from(backendTokenProof);
	return actual.length === expected.length && timingSafeEqual(actual, expected);
}

const adminMutationPaths = new Set([
	"/organization/update",
	"/organization/delete",
	"/organization/invite-member",
	"/organization/cancel-invitation",
	"/organization/remove-member",
	"/organization/update-member-role",
	"/sso/register",
	"/sso/update-provider",
	"/sso/delete-provider",
	"/sso/request-domain-verification",
	"/sso/verify-domain",
]);

const auditedPathPrefixes = [
	"/sign-in/",
	"/email-otp/",
	"/passkey/verify-",
	"/two-factor/verify-",
	"/organization/invite-",
	"/organization/accept-",
	"/organization/cancel-",
	"/organization/remove-",
	"/organization/update-",
	"/sso/",
	"/scim/",
	"/revoke-session",
	"/revoke-sessions",
];

const StepUpSessionSchema = z.object({
	id: z.string(),
	activeOrganizationId: z.string().optional(),
});
const EmailStepUpBodySchema = z.object({
	email: z.string().email(),
});

function authResultSucceeded(returned: unknown): boolean {
	if (returned instanceof APIError) return false;
	return !(returned instanceof Response) || returned.ok;
}

function defaultOrganizationSlug(name: string, userId: string): string {
	const prefix = name
		.toLowerCase()
		.replace(/[^a-z0-9]+/g, "-")
		.replace(/^-|-$/g, "")
		.slice(0, 40);
	return `${prefix || "workspace"}-${userId.slice(0, 8).toLowerCase()}`;
}

async function pendingInvitationExists(email: string): Promise<boolean> {
	const result = await authDb
		.select({ id: authInvitations.id })
		.from(authInvitations)
		.where(
			and(
				ilike(authInvitations.email, email),
				eq(authInvitations.status, "pending"),
				gt(authInvitations.expiresAt, new Date()),
			),
		)
		.limit(1);
	return result.length === 1;
}

async function ensureDefaultOrganization(userId: string) {
	await authDb.transaction(async (tx) => {
		await tx.execute(sql`select pg_advisory_xact_lock(hashtext(${userId}))`);
		const existing = await tx
			.select({ organizationId: authMembers.organizationId })
			.from(authMembers)
			.where(eq(authMembers.userId, userId))
			.limit(1);
		if (existing.length) return;
		const users = await tx
			.select({
				id: authUsers.id,
				name: authUsers.name,
				email: authUsers.email,
			})
			.from(authUsers)
			.where(eq(authUsers.id, userId))
			.limit(1);
		const user = users[0];
		if (!user) throw new Error("Cannot create an organization for an unknown user");
		const organizationId = randomUUID();
		const name = defaultWorkspaceName(user);
		await tx.insert(authOrganizations).values({
			id: organizationId,
			name,
			slug: defaultOrganizationSlug(name, userId),
			createdAt: new Date(),
		});
		await tx.insert(authMembers).values({
			id: randomUUID(),
			organizationId,
			userId,
			role: "owner",
			createdAt: new Date(),
		});
	});
}

async function activeMembership(userId: string, organizationId?: string) {
	const conditions = [eq(authMembers.userId, userId)];
	if (organizationId) conditions.push(eq(authMembers.organizationId, organizationId));
	const result = await authDb
		.select({
			organizationId: authMembers.organizationId,
			role: authMembers.role,
			organizationName: authOrganizations.name,
			requireAdminStepUp: authOrganizations.requireAdminStepUp,
			legacyWorkosOrganizationId: authOrganizations.legacyWorkosOrganizationId,
			archivedAt: authOrganizations.archivedAt,
		})
		.from(authMembers)
		.innerJoin(authOrganizations, eq(authOrganizations.id, authMembers.organizationId))
		.where(and(...conditions, isNull(authOrganizations.archivedAt)))
		.orderBy(asc(authMembers.createdAt))
		.limit(1);
	return result[0] ?? null;
}

/** Roles that may buy a plan or open the billing portal for a workspace. */
const BILLING_ROLES = new Set(["owner", "admin"]);

/**
 * Stripe billing for Starter and Professional: one subscription per
 * workspace, bought by its owners and admins. Off when Stripe is not
 * configured; Enterprise is set by hand after a contract.
 */
function billingPlugins() {
	if (!config.stripe) return [];
	return [
		stripe({
			stripeClient: new Stripe(config.stripe.secretKey),
			stripeWebhookSecret: config.stripe.webhookSecret,
			organization: { enabled: true },
			subscription: {
				enabled: true,
				plans: [
					{ name: "starter", priceId: config.stripe.starterPriceId },
					{ name: "professional", priceId: config.stripe.professionalPriceId },
				],
				authorizeReference: async ({ user, referenceId, action }) => {
					const membership = await activeMembership(user.id, referenceId);
					if (!membership) return false;
					return action === "list-subscription" || BILLING_ROLES.has(membership.role);
				},
				// API keys read the plan from the backend tenant, so every billing change is pushed there.
				onSubscriptionComplete: ({ subscription }) => syncTenantPlan(subscription.referenceId),
				onSubscriptionUpdate: ({ subscription }) => syncTenantPlan(subscription.referenceId),
				onSubscriptionCancel: ({ subscription }) => syncTenantPlan(subscription.referenceId),
				onSubscriptionDeleted: ({ subscription }) => syncTenantPlan(subscription.referenceId),
			},
		}),
	];
}

async function sessionStepUpGrant(sessionId: string) {
	const [session] = await authDb
		.select({
			verifiedAt: authSessions.stepUpVerifiedAt,
			method: authSessions.stepUpMethod,
			purpose: authSessions.stepUpPurpose,
		})
		.from(authSessions)
		.where(eq(authSessions.id, sessionId))
		.limit(1);
	return session ?? null;
}

function ssoProviderIdFromPath(path: string): string | null {
	const match = path.match(/^\/sso\/(?:callback|saml2\/(?:callback|sp\/acs))\/([^/]+)$/);
	if (!match?.[1]) return null;
	try {
		return decodeURIComponent(match[1]);
	} catch {
		return null;
	}
}

async function ssoProviderForPath(path: string) {
	const providerId = ssoProviderIdFromPath(path);
	if (!providerId) return null;
	const [provider] = await authDb
		.select({
			providerId: authSsoProviders.providerId,
			organizationId: authSsoProviders.organizationId,
		})
		.from(authSsoProviders)
		.where(eq(authSsoProviders.providerId, providerId))
		.limit(1);
	return provider ?? null;
}

async function sendEmail(input: { to: string; subject: string; text: string }) {
	if (!resend) throw new Error("RESEND_API_KEY is required to send authentication email");
	const { error } = await resend.emails.send({
		from: config.resendFrom,
		...input,
	});
	// Resend's error name and message hold no secrets and name the cause, such as an unverified sender domain.
	if (error)
		throw new Error(`Authentication email delivery failed: ${error.name}: ${error.message}`);
}

function bodyIdentifier(body: unknown): string | null {
	const parsed = z.record(z.string(), z.unknown()).safeParse(body);
	if (!parsed.success) return null;
	for (const key of ["providerId", "userId", "memberId", "invitationId", "organizationId"]) {
		const value = parsed.data[key];
		if (typeof value === "string" && value) return value;
	}
	return null;
}

function auditAction(path: string): string {
	return path.replace(/^\//, "").replaceAll("/", ".");
}

export const auth = betterAuth({
	appName: process.env.BRAND_NAME || "Oppulence",
	baseURL: config.betterAuthUrl,
	basePath: "/api/auth",
	secret: config.betterAuthSecret,
	trustedOrigins: config.trustedOrigins,
	database: { db: authDatabase, type: "postgres", transaction: true },
	advanced: {
		database: { validateSchema: process.env.APP_BUILD_PHASE !== "1" },
	},
	session: {
		expiresIn: 30 * DAY_SECONDS,
		updateAge: DAY_SECONDS,
		freshAge: 15 * 60,
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
	account: {
		encryptOAuthTokens: true,
		accountLinking: {
			enabled: true,
			requireLocalEmailVerified: true,
			trustedProviders: ["google", "microsoft"],
		},
	},
	socialProviders: {
		...(config.google
			? {
					google: {
						...config.google,
						disableImplicitSignUp: false,
					},
				}
			: {}),
		...(config.microsoft
			? {
					microsoft: {
						...config.microsoft,
						tenantId: "common",
						disableImplicitSignUp: false,
					},
				}
			: {}),
	},
	rateLimit: {
		enabled: !localE2EOtp,
		storage: "database",
		window: 60,
		max: 100,
	},
	telemetry: { enabled: false },
	databaseHooks: {
		user: {
			create: {
				before: async (user, context) => {
					const trustedProvisioner = context?.path.startsWith("/scim/");
					if (trustedProvisioner || config.registrationMode === "open") return;
					if (
						config.registrationMode === "invite-only" &&
						(await pendingInvitationExists(user.email))
					) {
						return;
					}
					throw new APIError("FORBIDDEN", {
						message: "Registration is not available.",
					});
				},
			},
		},
		session: {
			create: {
				before: async (session, context) => {
					const path = context?.path ?? "server";
					const ssoProvider = await ssoProviderForPath(path);
					let membership = await activeMembership(
						session.userId,
						ssoProvider?.organizationId ?? undefined,
					);
					if (!membership && config.registrationMode === "open") {
						await ensureDefaultOrganization(session.userId);
						membership = await activeMembership(
							session.userId,
							ssoProvider?.organizationId ?? undefined,
						);
					}
					const authenticationMethod = ssoProvider
						? "sso"
						: path.includes("passkey")
							? "passkey"
							: path.includes("email-otp")
								? "email-otp"
								: path.includes("google")
									? "google"
									: path.includes("microsoft")
										? "microsoft"
										: "other";
					return {
						data: {
							...session,
							authenticationMethod,
							...(membership ? { activeOrganizationId: membership.organizationId } : {}),
							...(ssoProvider?.organizationId
								? {
										ssoProviderId: ssoProvider.providerId,
										ssoOrganizationId: ssoProvider.organizationId,
										activeOrganizationId: ssoProvider.organizationId,
									}
								: {}),
						},
					};
				},
			},
		},
	},
	hooks: {
		before: createAuthMiddleware(async (ctx) => {
			if (
				ctx.path === "/token" &&
				!validBackendTokenProof(ctx.headers?.get("x-backend-token-request") ?? null)
			) {
				throw new APIError("NOT_FOUND");
			}
			// SCIM is sold on Enterprise only: a credential stops working when the plan ends.
			if (ctx.path.startsWith("/scim/v2/")) {
				const organizationId = await scimWorkspaceForToken(
					ctx.headers?.get("authorization") ?? null,
				);
				if (
					organizationId &&
					!can(entitlementsFor(await workspacePlan(organizationId)), "enterprise_identity")
				) {
					throw new APIError("FORBIDDEN", {
						message: "SCIM provisioning comes with the Enterprise plan.",
					});
				}
			}
			if (!ctx.request || !adminMutationPaths.has(ctx.path)) return;
			const authSession = await getAuthoritativeSessionFromCtx(ctx);
			if (!authSession) throw new APIError("UNAUTHORIZED");
			const session = StepUpSessionSchema.parse(authSession.session);
			const body = z.record(z.string(), z.unknown()).safeParse(ctx.body);
			const requestedOrganizationId = body.success ? body.data.organizationId : undefined;
			const organizationId =
				typeof requestedOrganizationId === "string"
					? requestedOrganizationId
					: session.activeOrganizationId;
			if (!organizationId) throw new APIError("FORBIDDEN");
			const membership = await activeMembership(authSession.user.id, organizationId);
			if (
				!membership ||
				!membership.role.split(",").some((role) => ["owner", "admin"].includes(role))
			) {
				throw new APIError("FORBIDDEN");
			}
			if (membership.requireAdminStepUp === false) return;
			const stepUp = await sessionStepUpGrant(session.id);
			if (
				!hasValidStepUp({
					verifiedAt: stepUp?.verifiedAt,
					method: stepUp?.method === "passkey" || stepUp?.method === "totp" ? stepUp.method : null,
					purpose: stepUp?.purpose === "admin" ? stepUp.purpose : null,
					requiredPurpose: "admin",
					allowedMethods: ["passkey", "totp"],
				})
			) {
				await identityAudit({
					actorId: authSession.user.id,
					organizationId,
					action: `${auditAction(ctx.path)}.step-up-required`,
					targetId: bodyIdentifier(ctx.body),
					result: "failure",
					requestId: ctx.headers?.get("x-request-id") ?? null,
				});
				throw new APIError("FORBIDDEN", {
					message: "Passkey or TOTP verification is required.",
				});
			}
		}),
		after: createAuthMiddleware(async (ctx) => {
			const successfulStepUp =
				authResultSucceeded(ctx.context.returned) &&
				[
					"/email-otp/verify-email",
					"/passkey/verify-authentication",
					"/two-factor/verify-totp",
				].includes(ctx.path);
			if (successfulStepUp) {
				const priorSession = ctx.request ? await getAuthoritativeSessionFromCtx(ctx) : null;
				const newSession = ctx.context.newSession;
				if (ctx.path === "/email-otp/verify-email" && priorSession) {
					const body = EmailStepUpBodySchema.safeParse(ctx.body);
					if (
						body.success &&
						body.data.email.toLowerCase() === priorSession.user.email.toLowerCase()
					) {
						await authDb
							.update(authSessions)
							.set({
								stepUpVerifiedAt: new Date(),
								stepUpMethod: "email-otp",
								stepUpPurpose: "account-delete",
							})
							.where(eq(authSessions.id, priorSession.session.id));
					}
				} else if (ctx.path === "/two-factor/verify-totp") {
					const verifiedSession = newSession ?? priorSession;
					if (verifiedSession) {
						await authDb
							.update(authSessions)
							.set({
								stepUpVerifiedAt: new Date(),
								stepUpMethod: "totp",
								stepUpPurpose: "admin",
							})
							.where(eq(authSessions.id, verifiedSession.session.id));
					}
				} else if (
					ctx.path === "/passkey/verify-authentication" &&
					priorSession &&
					newSession &&
					priorSession.user.id === newSession.user.id
				) {
					await authDb.transaction(async (tx) => {
						await tx
							.update(authSessions)
							.set({
								activeOrganizationId: priorSession.session.activeOrganizationId,
								createdAt: priorSession.session.createdAt,
								expiresAt: priorSession.session.expiresAt,
								stepUpVerifiedAt: new Date(),
								stepUpMethod: "passkey",
								stepUpPurpose: "admin",
							})
							.where(eq(authSessions.id, newSession.session.id));
						if (priorSession.session.id !== newSession.session.id) {
							await tx.delete(authSessions).where(eq(authSessions.id, priorSession.session.id));
						}
					});
				}
			}

			if (!auditedPathPrefixes.some((prefix) => ctx.path.startsWith(prefix))) return;
			try {
				const current =
					ctx.context.newSession ??
					(ctx.request ? await getAuthoritativeSessionFromCtx(ctx) : null);
				const body = z.record(z.string(), z.unknown()).safeParse(ctx.body);
				const bodyOrganizationId = body.success ? body.data.organizationId : undefined;
				const sessionOrganizationId = current?.session
					? StepUpSessionSchema.safeParse(current.session).data?.activeOrganizationId
					: undefined;
				await identityAudit({
					actorId: current?.user.id ?? null,
					organizationId:
						typeof bodyOrganizationId === "string"
							? bodyOrganizationId
							: (sessionOrganizationId ?? null),
					action: auditAction(ctx.path),
					targetId: bodyIdentifier(ctx.body),
					result: authResultSucceeded(ctx.context.returned) ? "success" : "failure",
					requestId: ctx.headers?.get("x-request-id") ?? null,
				});
			} catch {
				ctx.context.logger.error("Identity audit write failed");
			}
		}),
	},
	plugins: [
		admin(),
		organization({
			creatorRole: "owner",
			// Seats follow the workspace's plan; an uncapped plan has no limit.
			membershipLimit: async (_user, workspace) =>
				seatLimit((await workspacePlan(workspace.id)).plan) ?? Number.MAX_SAFE_INTEGER,
			requireEmailVerificationOnInvitation: true,
			sendInvitationEmail: async ({ id, email, organization: invitedOrganization }) => {
				const url = new URL("/sign-in", config.betterAuthUrl);
				url.searchParams.set("invitation", id);
				url.searchParams.set("return_to", "/app/settings?settings=identity");
				await sendEmail({
					to: email,
					subject: `Join ${invitedOrganization.name}`,
					text: `You were invited to join ${invitedOrganization.name}. Sign in to accept: ${url.toString()}`,
				});
			},
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
						brandName: {
							type: "string",
							required: false,
							validator: { input: z.string().max(80) },
						},
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
						supportEmail: {
							type: "string",
							required: false,
							validator: { input: z.string().email() },
						},
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
		emailOTP({
			disableSignUp: false,
			storeOTP: "hashed",
			expiresIn: 10 * 60,
			allowedAttempts: 5,
			// The plugin spreads these options over its defaults, so an undefined
			// generateOTP would replace the default generator and break every code.
			...(localE2EOtp ? { generateOTP: () => localE2EOtp } : {}),
			sendVerificationOTP: async ({ email, otp, type }) => {
				if (localE2EOtp) return;
				const stepUp = type === "email-verification";
				await sendEmail({
					to: email,
					subject: `Your ${process.env.BRAND_NAME || "Verifier"} ${stepUp ? "verification" : "sign-in"} code`,
					text: `Your ${stepUp ? "verification" : "sign-in"} code is ${otp}. It expires in 10 minutes.`,
				});
			},
		}),
		passkey({
			authentication: {
				afterVerification: async ({ ctx, verification }) => {
					const current = ctx.request ? await getAuthoritativeSessionFromCtx(ctx) : null;
					if (current && !verification.authenticationInfo.userVerified) {
						throw new APIError("FORBIDDEN", {
							message: "Passkey user verification is required for administrator step-up.",
						});
					}
				},
			},
		}),
		twoFactor({
			issuer: process.env.BRAND_NAME || "Verifier",
			allowPasswordless: true,
			twoFactorCookieMaxAge: 15 * 60,
		}),
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
		scimPlugin(
			createScimOptions({
				credentialHashSecret: config.scimCredentialHashSecret,
			}),
		),
		jwt({
			disableSettingJwtHeader: true,
			jwks: {
				keyPairConfig: { alg: "EdDSA", crv: "Ed25519" },
				rotationInterval: 30 * DAY_SECONDS,
				gracePeriod: 30 * DAY_SECONDS,
			},
			jwt: {
				issuer: config.betterAuthUrl,
				audience: config.backendJwtAudience,
				expirationTime: "5m",
				definePayload: async ({ user, session }) => {
					const membership = await activeMembership(user.id, session.activeOrganizationId);
					if (!membership)
						throw new APIError("FORBIDDEN", {
							message: "Active membership required.",
						});
					const legacyUser = await authDb
						.select({ legacyWorkosUserId: authUsers.legacyWorkosUserId })
						.from(authUsers)
						.where(eq(authUsers.id, user.id))
						.limit(1);
					const { plan } = await workspacePlan(membership.organizationId);
					return {
						org_id: membership.organizationId,
						org_role: membership.role,
						org_name: membership.organizationName,
						// The backend sets the workspace's tier and monthly allowance from this claim.
						plan,
						session_id: session.id,
						email: user.email,
						email_verified: user.emailVerified,
						...(legacyUser[0]?.legacyWorkosUserId
							? {
									"https://oppulence.io/legacy/workos_user_id": legacyUser[0].legacyWorkosUserId,
								}
							: {}),
						...(membership.legacyWorkosOrganizationId
							? {
									"https://oppulence.io/legacy/workos_organization_id":
										membership.legacyWorkosOrganizationId,
								}
							: {}),
					};
				},
			},
		}),
		...billingPlugins(),
	],
});
