import { cacheLife } from "next/cache";

import { indexCopy } from "../catalog";
import { HubPage } from "../editorial/hub-page";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Features",
	description: indexCopy.features.description,
	path: "/features",
});

export default async function FeaturesIndexPage() {
	"use cache";
	cacheLife("days");

	return <HubPage kind="feature" />;
}
