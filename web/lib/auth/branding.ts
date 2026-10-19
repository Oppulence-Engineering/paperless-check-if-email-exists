import "server-only";

import { eq } from "drizzle-orm";

import { authDb, authOrganizations } from "@/lib/auth/database";
import { serverConfig } from "@/lib/config/server-config";
import {
	BrandingInputSchema,
	BrandingSchema,
	type Branding,
	type BrandingInput,
} from "./branding.schema";

function deploymentBranding(): Branding {
	// Deployment branding comes from the one validated config read; the
	// fallbacks below are this template's own assets.
	const brand = serverConfig().brand;
	return BrandingSchema.parse({
		name: brand.name,
		logoUrl: brand.logoUrl || "/check-email-logo.svg",
		wordmarkUrl: brand.wordmarkUrl || undefined,
		faviconUrl: brand.faviconUrl || "/check-email-logo.svg",
		primaryColor: brand.primaryColor || "#111111",
		accentColor: brand.accentColor || "#F97316",
		supportEmail: brand.supportEmail || undefined,
		documentationUrl: brand.documentationUrl || undefined,
		termsUrl: brand.termsUrl || "/terms",
		privacyUrl: brand.privacyUrl || "/privacy",
	});
}

/** Returns deployment branding with validated organization overrides. */
export async function branding(input: BrandingInput = {}): Promise<Branding> {
	const { organizationId } = BrandingInputSchema.parse(input);
	const deployment = deploymentBranding();
	if (!organizationId) return deployment;
	const result = await authDb
		.select({
			brandName: authOrganizations.brandName,
			brandLogoUrl: authOrganizations.brandLogoUrl,
			brandWordmarkUrl: authOrganizations.brandWordmarkUrl,
			brandFaviconUrl: authOrganizations.brandFaviconUrl,
			brandPrimaryColor: authOrganizations.brandPrimaryColor,
			brandAccentColor: authOrganizations.brandAccentColor,
			supportEmail: authOrganizations.supportEmail,
			documentationUrl: authOrganizations.documentationUrl,
			termsUrl: authOrganizations.termsUrl,
			privacyUrl: authOrganizations.privacyUrl,
		})
		.from(authOrganizations)
		.where(eq(authOrganizations.id, organizationId))
		.limit(1);
	const organization = result[0];
	if (!organization) return deployment;
	return BrandingSchema.parse({
		name: organization.brandName || deployment.name,
		logoUrl: organization.brandLogoUrl || deployment.logoUrl,
		wordmarkUrl: organization.brandWordmarkUrl || deployment.wordmarkUrl,
		faviconUrl: organization.brandFaviconUrl || deployment.faviconUrl,
		primaryColor: organization.brandPrimaryColor || deployment.primaryColor,
		accentColor: organization.brandAccentColor || deployment.accentColor,
		supportEmail: organization.supportEmail || deployment.supportEmail,
		documentationUrl: organization.documentationUrl || deployment.documentationUrl,
		termsUrl: organization.termsUrl || deployment.termsUrl,
		privacyUrl: organization.privacyUrl || deployment.privacyUrl,
	});
}
