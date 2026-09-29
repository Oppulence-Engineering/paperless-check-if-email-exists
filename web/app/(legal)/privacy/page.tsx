import type { Metadata } from "next";
import { cacheLife } from "next/cache";

import { SimLegalDocumentPage } from "@/app/(marketing)/sim-landing/legal/sim-legal-document-page";
import type { LegalSection } from "@/components/legal/types";

export const metadata: Metadata = {
	title: "Privacy overview — Check If Email Exists",
	description:
		"This product-specific overview describes the data used by Check If Email Exists. A final privacy policy requires legal review before production publication.",
	robots: { index: false, follow: false },
};

const SECTIONS: LegalSection[] = [
	{
		heading: "Account and workspace data",
		body: [
			"The web application uses Better Auth for sign-in and stores account, organization, and membership data in PostgreSQL.",
		],
	},
	{
		heading: "Verification data",
		body: [
			"The application stores submitted addresses, results, list jobs, and history in the configured PostgreSQL database. Results may contain syntax, DNS, MX, and mailbox signals.",
		],
	},
	{
		heading: "Service infrastructure",
		body: [
			"The Rust API and worker process checks. RabbitMQ carries background jobs. Operators who self-host control these services and their retention settings.",
		],
	},
	{
		heading: "Questions",
		body: [
			"Contact privacy@oppulence.io with questions about this application. This overview does not replace a reviewed privacy policy.",
		],
	},
];

export default async function LegalPage() {
	"use cache";
	cacheLife("weeks");
	return (
		<SimLegalDocumentPage
			eyebrow="[legal]"
			contactEmail="privacy@oppulence.io"
			effective="Pending legal review"
			intro="This product-specific overview describes the data used by Check If Email Exists. A final privacy policy requires legal review before production publication."
			lastUpdated="September 22, 2026"
			related={[
				{ label: "Terms information", href: "/terms" },
				{ label: "Responsible disclosure", href: "/responsible-disclosure" },
			]}
			sections={SECTIONS}
			title="Privacy overview"
		/>
	);
}
