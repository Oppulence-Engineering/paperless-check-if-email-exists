import { cacheLife } from "next/cache";

import { SimPricingPage } from "../sim-landing/subpages/sim-pricing-page";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Pricing",
	description: "Ways to run the email verification web workspace, bulk jobs, and self-hosted API.",
	path: "/pricing",
});

export default async function PricingRoutePage() {
	"use cache";
	cacheLife("days");
	return <SimPricingPage />;
}
