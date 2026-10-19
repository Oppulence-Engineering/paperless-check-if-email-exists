import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Identity settings.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `identity-settings.lit.ts`.
 */
export const IdentitySettingsPropsSchema = z.object({
	organizationId: z.string().min(1),
	userId: z.string().min(1),
	organizationRole: z.string().min(1),
	scope: z
		.enum(["all", "workspace", "organization", "security", "enterprise", "branding"])
		.optional(),
});

export type IdentitySettingsPropsFields = z.infer<typeof IdentitySettingsPropsSchema>;

const NullableStringSchema = z.string().nullable().optional();

export const IdentityOrganizationSchema = z
	.object({
		id: z.string(),
		name: z.string(),
		slug: z.string(),
		logo: NullableStringSchema,
		requireSso: z.boolean().nullable().optional(),
		requireAdminStepUp: z.boolean().nullable().optional(),
		maxSessionAgeSeconds: z.number().nullable().optional(),
		idleTimeoutSeconds: z.number().nullable().optional(),
		brandName: NullableStringSchema,
		brandLogoUrl: NullableStringSchema,
		brandWordmarkUrl: NullableStringSchema,
		brandFaviconUrl: NullableStringSchema,
		brandPrimaryColor: NullableStringSchema,
		brandAccentColor: NullableStringSchema,
		supportEmail: NullableStringSchema,
		documentationUrl: NullableStringSchema,
		termsUrl: NullableStringSchema,
		privacyUrl: NullableStringSchema,
	})
	.loose();

export const IdentityOrganizationListSchema = z.array(IdentityOrganizationSchema);

export const IdentityFullOrganizationSchema = IdentityOrganizationSchema.extend({
	members: z.array(
		z.object({
			id: z.string(),
			userId: z.string(),
			role: z.string(),
			user: z.object({
				id: z.string(),
				name: z.string(),
				email: z.email(),
			}),
		}),
	),
	invitations: z.array(
		z.object({
			id: z.string(),
			email: z.email(),
			role: z.string().nullable().optional(),
			status: z.string(),
			expiresAt: z.coerce.date(),
		}),
	),
});

export const IdentityPasskeyListSchema = z.array(
	z.object({
		id: z.string(),
		name: z.string().nullable().optional(),
		deviceType: z.string(),
		backedUp: z.boolean(),
		createdAt: z.coerce.date(),
	}),
);

export const IdentitySSOProvidersSchema = z.object({
	providers: z.array(
		z.object({
			providerId: z.string(),
			type: z.string(),
			issuer: z.string(),
			domain: z.string(),
			organizationId: z.string().nullable(),
			domainVerified: z.boolean(),
			spMetadataUrl: z.string().optional(),
		}),
	),
});

export const IdentitySCIMProvidersSchema = z.object({
	providers: z.array(
		z.object({
			id: z.string(),
			connectionId: z.string(),
			providerId: z.string(),
			organizationId: z.string().nullable(),
			status: z.enum(["active", "decommissioning", "decommissioned"]),
		}),
	),
});

export const IdentitySCIMMappingsSchema = z.object({
	mappings: z.array(
		z.object({
			providerId: z.string(),
			group: z.string(),
			role: z.enum(["admin", "member"]),
		}),
	),
});

export const IdentitySCIMTokenSchema = z.object({
	scimToken: z.string().min(1),
});
export const IdentityDomainVerificationSchema = z.object({
	domainVerificationToken: z.string().min(1),
});
export const IdentityTOTPEnrollmentSchema = z.object({
	totpURI: z.string().min(1),
	backupCodes: z.array(z.string()),
});
export const IdentityActionResultSchema = z.looseObject({});

export const IdentitySessionListSchema = z.object({
	sessions: z.array(
		z.object({
			id: z.string(),
			createdAt: z.coerce.date(),
			updatedAt: z.coerce.date(),
			expiresAt: z.coerce.date(),
			ipAddress: NullableStringSchema,
			userAgent: NullableStringSchema,
			current: z.boolean(),
		}),
	),
});

export const IdentityAuditListSchema = z.object({
	events: z.array(
		z.object({
			id: z.string(),
			action: z.string(),
			targetId: NullableStringSchema,
			result: z.enum(["success", "failure"]),
			createdAt: z.coerce.date(),
		}),
	),
});
