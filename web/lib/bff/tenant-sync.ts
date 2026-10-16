import "server-only";

import { createHash } from "node:crypto";

import { backendApiURL } from "@/lib/auth/config";
import { MONTHLY_CHECKS } from "@/lib/billing/plans";
import { workspacePlan } from "@/lib/billing/workspace-plan";

/** RFC 4122 namespace for ISO OIDs. The backend derives tenant ids with it (backend/src/tenant/jwt.rs). */
const NAMESPACE_OID = "6ba7b812-9dad-11d1-80b4-00c04fd430c8";

/** The backend tenant id of a workspace: a UUID v5 of the organization id. */
export function tenantIdForOrganization(organizationId: string): string {
	const bytes = createHash("sha1")
		.update(Buffer.from(NAMESPACE_OID.replaceAll("-", ""), "hex"))
		.update(organizationId)
		.digest()
		.subarray(0, 16);
	bytes[6] = (bytes[6] & 0x0f) | 0x50;
	bytes[8] = (bytes[8] & 0x3f) | 0x80;
	const hex = bytes.toString("hex");
	return `${hex.slice(0, 8)}-${hex.slice(8, 12)}-${hex.slice(12, 16)}-${hex.slice(16, 20)}-${hex.slice(20)}`;
}

/**
 * Pushes the workspace plan to the backend tenant after a Stripe event.
 * API-key requests read the plan from the tenant row, so a canceled or
 * lapsed subscription must reach the backend without waiting for a browser
 * request. An Enterprise limit is set by contract, so it is left alone.
 * A thrown error fails the webhook, and Stripe sends it again.
 */
export async function syncTenantPlan(organizationId: string): Promise<void> {
	const secret = process.env.RCH__HEADER_SECRET?.trim();
	if (!secret) return;
	const { plan } = await workspacePlan(organizationId);
	const response = await fetch(
		backendApiURL(`/v1/admin/tenants/${tenantIdForOrganization(organizationId)}`),
		{
			method: "PUT",
			headers: { "content-type": "application/json", "x-reacher-secret": secret },
			body: JSON.stringify(
				plan === "enterprise"
					? { plan_tier: plan }
					: { plan_tier: plan, monthly_email_limit: MONTHLY_CHECKS[plan] },
			),
			cache: "no-store",
			signal: AbortSignal.timeout(10_000),
		},
	);
	// A workspace that never called the backend has no tenant yet; its first request sets the plan.
	if (!response.ok && response.status !== 404) {
		throw new Error(`Tenant plan sync failed with status ${String(response.status)}`);
	}
}
