import { cacheLife } from "next/cache";

import { EditorialSecurity } from "../editorial/pages";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Security and privacy",
	description: "How Verifier handles workspace access, backend credentials, and verification data.",
	path: "/security",
});

export default async function SecurityPage() {
	"use cache";
	cacheLife("days");
	return <EditorialSecurity />;
}
