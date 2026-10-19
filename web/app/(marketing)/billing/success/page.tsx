import type { Metadata } from "next";
import { cacheLife } from "next/cache";

import { EditorialBillingStatus } from "../../editorial/pages";
import { SITE_URL } from "../../site";

const TITLE = "Billing status — Verifier";
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
		<EditorialBillingStatus
			actions={[
				{ href: "/app/settings", label: "Open settings" },
				{ href: "/app", label: "Go to the app", variant: "outline" },
			]}
			lede="Open the app to see your plan and your usage."
			title="Review your workspace"
		/>
	);
}
