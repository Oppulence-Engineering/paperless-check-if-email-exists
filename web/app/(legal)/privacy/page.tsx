import type { Metadata } from "next";
import { cacheLife } from "next/cache";

import { EditorialLegalDocument } from "@/app/(marketing)/editorial/legal";
import type { LegalSection } from "@/components/legal/types";

export const metadata: Metadata = {
	title: "Privacy overview — Verifier",
	description:
		"This product-specific overview describes the data used by Verifier. A final privacy policy requires legal review before production publication.",
	robots: { index: false, follow: false },
};

const SECTIONS: LegalSection[] = [
	{
		heading: "Account and workspace data",
		body: [
			"We store account, organization, and membership data in our database to sign you in and manage your workspace.",
		],
	},
	{
		heading: "Verification data",
		body: [
			"We store the addresses you submit, their results, list jobs, and history in our database. Results may contain format, domain, mail server, and mailbox signals.",
		],
	},
	{
		heading: "Processing and retention",
		body: [
			"Our servers run the checks and the background jobs. You choose how many days results are kept.",
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
		<EditorialLegalDocument
			eyebrow="[legal]"
			contactEmail="privacy@oppulence.io"
			effective="Pending legal review"
			intro="This product-specific overview describes the data used by Verifier. A final privacy policy requires legal review before production publication."
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
