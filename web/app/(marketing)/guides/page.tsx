import { cacheLife } from "next/cache";

import { indexCopy } from "../catalog";
import { HubPage } from "../editorial/hub-page";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Guides",
	description: indexCopy.guides.description,
	path: "/guides",
});

export default async function GuidesIndexPage() {
	"use cache";
	cacheLife("days");

	return <HubPage kind="guide" />;
}
