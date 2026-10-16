import { SettingsDashboardRoute } from "./_components/settings-dashboard-route/settings-dashboard-route";
import { requireSession } from "@/lib/auth/session";
import { PAID_PLANS, type PaidPlan } from "@/lib/billing/plans";

export default async function SettingsPage({
	searchParams,
}: {
	searchParams: Promise<{ settings?: string | string[]; plan?: string | string[] }>;
}) {
	const [session, params] = await Promise.all([requireSession("/app/settings"), searchParams]);
	return (
		<SettingsDashboardRoute
			chosenPlan={PAID_PLANS.find((plan): plan is PaidPlan => plan === params.plan)}
			organizationId={session.membership.organizationId}
			organizationRole={session.membership.role}
			section={typeof params.settings === "string" ? params.settings : "overview"}
			userEmail={session.user.email}
			userId={session.user.id}
			userName={session.user.name}
		/>
	);
}
