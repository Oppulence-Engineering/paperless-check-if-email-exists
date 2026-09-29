import { cacheLife } from "next/cache";

import { comparePages } from "../compare-catalog";
import { SimCompareHub } from "../sim-landing/subpages/sim-compare-hub";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Compare email verification",
	description: "How email verification fits beside a CRM, inbox, meeting notes, and AI assistants.",
	path: "/compare",
});

export default async function CompareIndexPage() {
	"use cache";
	cacheLife("days");

	return (
		<SimCompareHub
			description="Check an address here, then use other products for their own workflow. Import addresses by CSV or use the API."
			heading="Where email verification fits."
			pages={comparePages}
		/>
	);
}
