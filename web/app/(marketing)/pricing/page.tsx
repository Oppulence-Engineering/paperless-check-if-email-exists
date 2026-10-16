import { cacheLife } from "next/cache";

import { SimPricingPage } from "../sim-landing/subpages/sim-pricing-page";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Pricing",
	description:
		"Start free. Starter is $49 a month and Professional is $149 a month. Every plan includes scores, suppressions, and delivery feedback.",
	path: "/pricing",
});

export default async function PricingRoutePage() {
	"use cache";
	cacheLife("days");
	return <SimPricingPage />;
}
