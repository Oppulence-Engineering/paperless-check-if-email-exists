import { cacheLife } from "next/cache";
import { notFound } from "next/navigation";

import { SimPlatformPage } from "../sim-landing/subpages/sim-platform-page";
import { getPlatformPage } from "../marketing-data";
import { marketingMetadata } from "../metadata";

const page = getPlatformPage("web");

export const metadata = marketingMetadata({
	title: "Web workspace",
	description: page?.lede ?? "Check email addresses and review lists in a browser.",
	path: "/web",
});

export default async function WebPage() {
	"use cache";
	cacheLife("days");
	if (!page) notFound();
	return <SimPlatformPage page={page} />;
}
