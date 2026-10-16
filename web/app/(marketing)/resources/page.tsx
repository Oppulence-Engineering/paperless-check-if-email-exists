import { cacheLife } from "next/cache";

import { indexCopy } from "../catalog";
import { EditorialResources } from "../editorial/pages";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Resources",
	description: indexCopy.resources.description,
	path: "/resources",
});

export default async function ResourcesPage() {
	"use cache";
	cacheLife("days");

	return <EditorialResources />;
}
