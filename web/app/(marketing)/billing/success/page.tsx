import type { Metadata } from "next";
import { cacheLife } from "next/cache";

import { SimBillingStatusPage } from "../../sim-landing/subpages/sim-billing-status-page";
import { SITE_URL } from "../../site";

const TITLE = "Billing status — Check If Email Exists";
const DESCRIPTION = "Open your workspace to review your current account status.";

export const metadata: Metadata = {
	title: TITLE,
	description: DESCRIPTION,
	alternates: { canonical: `${SITE_URL}/billing/success` },
	robots: { index: false, follow: false },
	openGraph: {
		title: TITLE,
		description: DESCRIPTION,
		url: `${SITE_URL}/billing/success`,
	},
};

export default async function BillingSuccessRoute() {
	"use cache";
	cacheLife("days");

	return (
		<SimBillingStatusPage
			actions={[
				{ href: "/app/settings", label: "Open settings" },
				{ href: "/app", label: "Go to the app", variant: "outline" },
			]}
			description="This route is retained from the public template. The application does not configure hosted billing. Open your workspace to review its current status."
			noteBody="Hosted billing must be configured before this route can confirm a payment."
			noteTitle="Billing is not configured"
			title="Review your workspace"
		/>
	);
}
