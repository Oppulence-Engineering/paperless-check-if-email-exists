import { z } from "zod";

import { TenantSummarySchema } from "@/lib/admin/platform-admin.schema";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Admin panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `admin-panel.lit.ts`.
 */
export const AdminPanelPropsSchema = z.object({
	tenants: z.array(TenantSummarySchema),
});

export type AdminPanelPropsFields = z.infer<typeof AdminPanelPropsSchema>;
