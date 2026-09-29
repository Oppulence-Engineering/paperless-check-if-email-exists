import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Settings dashboard route.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `settings-dashboard-route.lit.ts`.
 */
export const SettingsDashboardRoutePropsSchema = z.object({
	section: z.string(),
	organizationId: z.string().min(1),
	organizationRole: z.string().min(1),
	userId: z.string().min(1),
	userName: z.string(),
	userEmail: z.email(),
});

export type SettingsDashboardRoutePropsFields = z.infer<typeof SettingsDashboardRoutePropsSchema>;
