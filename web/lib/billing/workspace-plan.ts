import "server-only";

import { and, desc, eq, inArray } from "drizzle-orm";

import { authDb, authSubscriptions } from "@/lib/auth/database";
import { planFromValue } from "@/lib/entitlements/entitlements";
import type { Plan } from "@/lib/entitlements/entitlements.schema";

/**
 * Stripe statuses that keep a paid plan. Any other status (past_due,
 * canceled, unpaid, incomplete) puts the workspace back on Free, which matches
 * the entitlements policy in lib/entitlements.
 */
const PAID_STATUSES = ["active", "trialing"];

export type WorkspacePlan = { plan: Plan; status: string | null };

/**
 * The plan a workspace is on: its paid Stripe subscription, or Free. An
 * Enterprise workspace is a subscription row with plan "enterprise" that an
 * operator adds by hand after a contract; it has no Stripe ids.
 */
export async function workspacePlan(organizationId: string): Promise<WorkspacePlan> {
	const rows = await authDb
		.select({ plan: authSubscriptions.plan, status: authSubscriptions.status })
		.from(authSubscriptions)
		.where(
			and(
				eq(authSubscriptions.referenceId, organizationId),
				inArray(authSubscriptions.status, PAID_STATUSES),
			),
		)
		.orderBy(desc(authSubscriptions.periodEnd))
		.limit(1);
	const row = rows[0];
	return row
		? { plan: planFromValue(row.plan), status: row.status }
		: { plan: "free", status: null };
}
