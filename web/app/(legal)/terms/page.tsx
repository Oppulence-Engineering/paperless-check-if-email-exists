import type { Metadata } from "next";
import { cacheLife } from "next/cache";

import { SimLegalDocumentPage } from "@/app/(marketing)/sim-landing/legal/sim-legal-document-page";
import type { LegalSection } from "@/components/legal/types";

export const metadata: Metadata = {
	title: "Terms information — Check If Email Exists",
	description:
		"This page describes the Check If Email Exists application while product terms are prepared for legal review.",
	robots: { index: false, follow: false },
};

const SECTIONS: LegalSection[] = [
	{
		heading: "Product scope",
		body: [
			"Check If Email Exists provides email verification through a web application, Rust API, worker, and SDKs. It checks signals without sending an email to the recipient.",
		],
	},
	{
		heading: "Result limits",
		body: [
			"Mail servers may hide mailbox status, accept all addresses, or reject verification probes. A result is evidence for review, not a guarantee of delivery.",
		],
	},
	{
		heading: "Self-hosting",
		body: [
			"The source is available in this repository. Review the repository licenses and deployment instructions before use. Self-hosted operators manage their own infrastructure and data.",
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
		<SimLegalDocumentPage
			eyebrow="[legal]"
			contactEmail="legal@oppulence.io"
			effective="Pending legal review"
			intro="This page describes the Check If Email Exists application while product terms are prepared for legal review."
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
