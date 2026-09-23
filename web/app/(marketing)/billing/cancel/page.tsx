import type { Metadata } from "next";
import { cacheLife } from "next/cache";

import { SimBillingStatusPage } from "../../sim-landing/subpages/sim-billing-status-page";
import { SITE_URL } from "../../site";

const TITLE = "Billing status — Check If Email Exists";
const DESCRIPTION = "Return to the email verification workspace.";

export const metadata: Metadata = {
	title: TITLE,
	description: DESCRIPTION,
	alternates: { canonical: `${SITE_URL}/billing/cancel` },
	robots: { index: false, follow: false },
	openGraph: {
		title: TITLE,
		description: DESCRIPTION,
		url: `${SITE_URL}/billing/cancel`,
	},
};

export default async function BillingCancelRoute() {
	"use cache";
	cacheLife("days");

	return (
		<SimBillingStatusPage
			actions={[
				{ href: "/pricing", label: "See the plans" },
				{ href: "/app", label: "Go to the app", variant: "outline" },
			]}
			description="This route is retained from the public template. Hosted checkout is not configured in this application."
			noteBody="Use the workspace or review the repository's deployment options."
			noteTitle="Next steps"
			title="Return to the workspace"
		/>
	);
}
