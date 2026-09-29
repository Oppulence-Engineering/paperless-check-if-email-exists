import { cacheLife } from "next/cache";
import { notFound } from "next/navigation";

import { SimPlatformPage } from "../sim-landing/subpages/sim-platform-page";
import { getPlatformPage } from "../marketing-data";
import { marketingMetadata } from "../metadata";

const page = getPlatformPage("voice-app");

export const metadata = marketingMetadata({
	title: "Bulk lists",
	description: page?.lede ?? "Upload and verify CSV lists.",
	path: "/voice-app",
});

export default async function BulkListsPage() {
	"use cache";
	cacheLife("days");
	if (!page) notFound();
	return <SimPlatformPage page={page} />;
}
