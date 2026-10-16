import type { Metadata } from "next";
import { cacheLife } from "next/cache";

import { EditorialLegalDocument } from "@/app/(marketing)/editorial/legal";
import type { LegalSection } from "@/components/legal/types";

export const metadata: Metadata = {
	title: "Terms information — Verifier",
	description:
		"This page describes the Verifier application while product terms are prepared for legal review.",
	robots: { index: false, follow: false },
};

const SECTIONS: LegalSection[] = [
	{
		heading: "Product scope",
		body: [
			"Verifier provides email verification through a web application, an API, and SDKs. It checks signals without sending an email to the recipient.",
		],
	},
	{
		heading: "Result limits",
		body: [
			"Mail servers may hide mailbox status, accept all addresses, or reject verification probes. A result is evidence for review, not a guarantee of delivery.",
		],
	},
	{
		heading: "Questions",
		body: [
			"Contact legal@oppulence.io for contractual questions. Final service terms require legal review before production publication.",
		],
	},
];

export default async function LegalPage() {
	"use cache";
	cacheLife("weeks");
	return (
		<EditorialLegalDocument
			eyebrow="[legal]"
			contactEmail="legal@oppulence.io"
			effective="Pending legal review"
			intro="This page describes the Verifier application while product terms are prepared for legal review."
			lastUpdated="September 22, 2026"
			related={[
				{ label: "Privacy overview", href: "/privacy" },
				{ label: "Responsible disclosure", href: "/responsible-disclosure" },
			]}
			sections={SECTIONS}
			title="Terms information"
		/>
	);
}
