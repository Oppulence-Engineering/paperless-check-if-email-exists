import { cacheLife } from "next/cache";
import { notFound } from "next/navigation";

import { SimPlatformPage } from "../sim-landing/subpages/sim-platform-page";
import { getPlatformPage } from "../marketing-data";
import { marketingMetadata } from "../metadata";

const page = getPlatformPage("self-hosting");

export const metadata = marketingMetadata({
	title: "Self-hosting",
	description:
		page?.lede ?? "Run the web workspace and verification service on your infrastructure.",
	path: "/self-hosting",
});

export default async function SelfHostingPage() {
	"use cache";
	cacheLife("days");
	if (!page) notFound();
	return <SimPlatformPage page={page} />;
}
