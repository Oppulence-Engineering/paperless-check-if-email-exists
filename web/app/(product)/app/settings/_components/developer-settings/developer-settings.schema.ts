import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Developer settings.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `developer-settings.lit.ts`.
 */
export const DeveloperSettingsPropsSchema = z.object({
	organizationId: z.string().min(1),
	organizationRole: z.string().min(1),
});

export type DeveloperSettingsPropsFields = z.infer<typeof DeveloperSettingsPropsSchema>;

export const ApiKeySchema = z.object({
	id: z.uuid(),
	tenant_id: z.uuid(),
	key_prefix: z.string(),
	name: z.string(),
	scopes: z.array(z.string()),
	status: z.string(),
	last_used_at: z.string().nullable(),
	expires_at: z.string().nullable(),
	created_at: z.string(),
});

export const ApiKeyListSchema = z.object({ api_keys: z.array(ApiKeySchema) });
export const CreatedApiKeySchema = ApiKeySchema.omit({ last_used_at: true }).extend({
	key: z.string().min(1),
});

export const ApiKeyScopeSchema = z.enum([
	"verify",
	"bulk",
	"find",
	"lists",
	"suppressions",
	"reputation",
	"settings",
	"pipelines.read",
	"pipelines.write",
	"pipelines.trigger",
]);

export const CreateApiKeySchema = z.object({
	name: z.string().trim().min(1).max(80),
	scopes: z.array(ApiKeyScopeSchema).min(1),
	expires_at: z.iso.datetime(),
});
