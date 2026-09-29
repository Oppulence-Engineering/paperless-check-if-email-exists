import { z } from "zod";

export const HexColorSchema = z.string().regex(/^#[0-9a-fA-F]{6}$/);
export const BrandAssetURLSchema = z.string().refine((value) => {
	if (value.startsWith("/") && !value.startsWith("//")) return true;
	try {
		return new URL(value).protocol === "https:";
	} catch {
		return false;
	}
}, "Brand assets must use a same-origin path or HTTPS URL.");
export const PublicURLSchema = z.string().refine((value) => {
	if (value.startsWith("/") && !value.startsWith("//")) return true;
	try {
		return new URL(value).protocol === "https:";
	} catch {
		return false;
	}
}, "Public links must use a same-origin path or HTTPS URL.");

const WorkspaceLogoDataURLSchema = z
	.string()
	.max(350_000)
	.regex(/^data:image\/(?:jpeg|png|webp);base64,[A-Za-z0-9+/]+={0,2}$/);

/** A remote logo URL or a small, validated raster image uploaded by an administrator. */
export const WorkspaceLogoSchema = z.union([BrandAssetURLSchema, WorkspaceLogoDataURLSchema]);

export const BackendAPIErrorSchema = z
	.object({
		error: z.string().optional(),
		message: z.string().optional(),
		code: z.string().optional(),
		// Go problem details use `detail` / `title`; keep them in the envelope so
		// legacy 400s still match on the public sentence, not a generic status.
		detail: z.string().optional(),
		title: z.string().optional(),
	})
	.passthrough();

const ViewerResponseSchema = z.object({
	user: z.object({
		id: z.string(),
		email: z.string().email().optional().or(z.literal("")),
	}),
	billing: z
		.object({
			plan: z.string().nullable().optional(),
			status: z.string().nullable().optional(),
			trialExpiresAt: z.string().nullable().optional(),
			usage: z.unknown().optional(),
		})
		.passthrough(),
});

const AuthUserSchema = z.object({
	id: z.string().min(1),
	name: z.string(),
	email: z.string().email(),
	emailVerified: z.boolean(),
	image: z.string().nullable().optional(),
});

export const ActiveMembershipSchema = z.object({
	id: z.string().min(1),
	userId: z.string().min(1),
	organizationId: z.string().min(1),
	role: z.string().min(1),
});

/** Browser-safe workspace identity attached to the persistent product shell. */
export const WorkspaceSummarySchema = z.object({
	id: z.string().min(1),
	name: z.string().min(1),
	slug: z.string().min(1),
	role: z.string().min(1),
	logoUrl: WorkspaceLogoSchema.nullable(),
});

export const WorkspaceSummaryListSchema = z.array(WorkspaceSummarySchema);

export const WorkspaceSwitchRequestSchema = z.strictObject({
	organizationId: z.string().min(1),
});

export const WorkspaceSwitchResponseSchema = z.strictObject({
	activeOrganizationId: z.string().min(1),
});

export const WorkspaceCreateRequestSchema = z.strictObject({
	name: z.string().trim().min(2).max(80),
	slug: z
		.string()
		.trim()
		.min(2)
		.max(64)
		.regex(/^[a-z0-9]+(?:-[a-z0-9]+)*$/),
});

export const WorkspaceCreateResponseSchema = z.strictObject({
	activeOrganizationId: z.string().min(1),
	workspace: WorkspaceSummarySchema,
});

const WorkspaceRoleSchema = z.enum(["admin", "member"]);
const WorkspaceIdSchema = z.string().trim().min(1).max(255);

export const WorkspaceAdminActionSchema = z.discriminatedUnion("action", [
	z.strictObject({
		action: z.literal("update_workspace"),
		name: z.string().trim().min(2).max(80),
		slug: z
			.string()
			.trim()
			.min(2)
			.max(64)
			.regex(/^[a-z0-9]+(?:-[a-z0-9]+)*$/),
		logo: WorkspaceLogoSchema.nullable(),
	}),
	z.strictObject({ action: z.literal("update_policy"), requireAdminStepUp: z.boolean() }),
	z.strictObject({ action: z.literal("resend_invitation"), invitationId: WorkspaceIdSchema }),
	z.strictObject({ action: z.literal("revoke_invitation"), invitationId: WorkspaceIdSchema }),
	z.strictObject({
		action: z.literal("change_member_role"),
		memberId: WorkspaceIdSchema,
		role: WorkspaceRoleSchema,
	}),
	z.strictObject({ action: z.literal("remove_member"), memberId: WorkspaceIdSchema }),
	z.strictObject({ action: z.literal("transfer_ownership"), memberId: WorkspaceIdSchema }),
	z.strictObject({ action: z.literal("archive_workspace") }),
	z.strictObject({ action: z.literal("delete_workspace") }),
]);

export const WorkspaceAdminStateSchema = z.strictObject({
	organization: z.strictObject({
		id: WorkspaceIdSchema,
		name: z.string(),
		slug: z.string(),
		logo: WorkspaceLogoSchema.nullable(),
		requireAdminStepUp: z.boolean(),
	}),
	members: z.array(
		z.strictObject({
			id: WorkspaceIdSchema,
			userId: WorkspaceIdSchema,
			name: z.string(),
			email: z.string().email(),
			role: z.string(),
			createdAt: z.coerce.date(),
		}),
	),
	invitations: z.array(
		z.strictObject({
			id: WorkspaceIdSchema,
			email: z.string().email(),
			role: z.string().nullable(),
			status: z.string(),
			expiresAt: z.coerce.date(),
		}),
	),
});

export type WorkspaceAdminAction = z.infer<typeof WorkspaceAdminActionSchema>;

export type WorkspaceSummary = z.infer<typeof WorkspaceSummarySchema>;

export const AuthorizedSessionSchema = z.object({
	user: AuthUserSchema,
	session: z.object({
		id: z.string().min(1),
		userId: z.string().min(1),
		activeOrganizationId: z.string().min(1),
		createdAt: z.coerce.date(),
		updatedAt: z.coerce.date(),
		expiresAt: z.coerce.date(),
		stepUpVerifiedAt: z.coerce.date().nullable().optional(),
		stepUpMethod: z.enum(["email-otp", "passkey", "totp"]).nullable().optional(),
		stepUpPurpose: z.enum(["account-delete", "admin"]).nullable().optional(),
	}),
	membership: ActiveMembershipSchema,
});

export type AuthorizedSession = z.infer<typeof AuthorizedSessionSchema>;

export function isSsoSessionForOrganization(
	authenticationMethod: string | null,
	activeOrganizationId: string | null,
	ssoOrganizationId: string | null,
): boolean {
	return (
		authenticationMethod === "sso" &&
		Boolean(activeOrganizationId) &&
		ssoOrganizationId === activeOrganizationId
	);
}

export const RevokeIdentitySessionSchema = z.object({
	sessionId: z.string().min(1),
});

export const BrowserSessionResponseSchema = z.discriminatedUnion("authenticated", [
	z.object({
		authenticated: z.literal(false),
	}),
	z.object({
		authenticated: z.literal(true),
		user: z.object({
			id: z.string(),
			name: z.string(),
			email: z.string().email(),
			emailVerified: z.boolean(),
			sessionId: z.string(),
			organizationId: z.string(),
			role: z.string(),
			permissions: z.array(z.string()).default([]),
		}),
		billing: ViewerResponseSchema.shape.billing.optional(),
		expiresAt: z.number().int().positive(),
	}),
]);

export type BrowserSessionResponse = z.infer<typeof BrowserSessionResponseSchema>;
