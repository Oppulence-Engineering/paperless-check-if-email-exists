import type { Metadata } from "next";
import { cacheLife } from "next/cache";

import { SimLegalDocumentPage } from "@/app/(marketing)/sim-landing/legal/sim-legal-document-page";
import type { LegalSection } from "@/components/legal/types";

export const metadata: Metadata = {
	title: "Responsible disclosure — Check If Email Exists",
	description:
		"Report security issues affecting Check If Email Exists or its deployment to the security contact below.",
	robots: { index: false, follow: false },
};

const SECTIONS: LegalSection[] = [
	{
		heading: "Report an issue",
		body: [
			"Send a concise description, affected component, reproduction steps, and impact to security@oppulence.io. Do not include live credentials or unrelated personal data.",
		],
	},
	{
		heading: "Research scope",
		body: [
			"Test only systems and data you own or are authorized to test. Stop if you encounter another person's data.",
		],
	},
	{
		heading: "Product components",
		body: [
			"Reports may concern the web application, Better Auth session flow, organization boundaries, Rust API, worker, or deployment configuration.",
		],
	},
	{
		heading: "Coordination",
		body: [
			"Allow time to investigate and fix a report before publishing details. This page does not promise a response deadline or a bounty.",
		],
	},
];

export default async function LegalPage() {
	"use cache";
	cacheLife("weeks");
	return (
		<SimLegalDocumentPage
			eyebrow="[legal]"
			contactEmail="security@oppulence.io"
			effective="Pending legal review"
			intro="Report security issues affecting Check If Email Exists or its deployment to the security contact below."
			lastUpdated="September 22, 2026"
			related={[
				{ label: "Privacy overview", href: "/privacy" },
				{ label: "Terms information", href: "/terms" },
			]}
			sections={SECTIONS}
			title="Responsible disclosure"
		/>
	);
}
