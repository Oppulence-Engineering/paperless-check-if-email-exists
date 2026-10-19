import type { Metadata } from "next";
import { cacheLife } from "next/cache";

import { EditorialBillingStatus } from "../../editorial/pages";
import { SITE_URL } from "../../site";

const TITLE = "Billing status — Verifier";
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
		<EditorialBillingStatus
			actions={[
				{ href: "/pricing", label: "See the plans" },
				{ href: "/app", label: "Go to the app", variant: "outline" },
			]}
			lede="Your plan is the same as before. Open the app to continue, or contact us about a plan."
			title="No changes were made"
		/>
	);
}
