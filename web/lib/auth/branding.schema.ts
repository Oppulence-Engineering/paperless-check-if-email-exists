import { z } from "zod";

import { BrandAssetURLSchema, HexColorSchema, PublicURLSchema } from "@/lib/auth/schemas";

const OptionalAssetSchema = BrandAssetURLSchema.optional();
const OptionalPublicURLSchema = PublicURLSchema.optional();

export const BrandingSchema = z.object({
	name: z.string().min(1).max(80),
	logoUrl: OptionalAssetSchema,
	wordmarkUrl: OptionalAssetSchema,
	faviconUrl: OptionalAssetSchema,
	primaryColor: HexColorSchema,
	accentColor: HexColorSchema,
	supportEmail: z.string().email().optional(),
	documentationUrl: OptionalPublicURLSchema,
	termsUrl: OptionalPublicURLSchema,
	privacyUrl: OptionalPublicURLSchema,
});

export const BrandingInputSchema = z.object({
	organizationId: z.string().min(1).optional(),
});

export type Branding = z.infer<typeof BrandingSchema>;
export type BrandingInput = z.infer<typeof BrandingInputSchema>;
