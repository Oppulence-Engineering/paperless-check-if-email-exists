import { cacheLife } from "next/cache";

import { SimSecurityPage } from "../sim-landing/subpages/sim-security-page";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Security and privacy",
	description:
		"How Check If Email Exists handles workspace access, backend credentials, and verification data.",
	path: "/security",
});

export default async function SecurityPage() {
	"use cache";
	cacheLife("days");
	return <SimSecurityPage />;
}
