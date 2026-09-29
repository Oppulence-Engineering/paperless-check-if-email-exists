import { SettingsDashboardRoute } from "./_components/settings-dashboard-route/settings-dashboard-route";
import { requireSession } from "@/lib/auth/session";

export default async function SettingsPage({
	searchParams,
}: {
	searchParams: Promise<{ settings?: string | string[] }>;
}) {
	const [session, params] = await Promise.all([requireSession("/app/settings"), searchParams]);
	return (
		<SettingsDashboardRoute
			organizationId={session.membership.organizationId}
			organizationRole={session.membership.role}
			section={typeof params.settings === "string" ? params.settings : "overview"}
			userEmail={session.user.email}
			userId={session.user.id}
			userName={session.user.name}
		/>
	);
}
