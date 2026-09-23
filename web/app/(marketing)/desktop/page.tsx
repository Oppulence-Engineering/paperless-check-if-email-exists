import { cacheLife } from "next/cache";
import { notFound } from "next/navigation";

import { SimPlatformPage } from "../sim-landing/subpages/sim-platform-page";
import { getPlatformPage } from "../marketing-data";
import { marketingMetadata } from "../metadata";

const page = getPlatformPage("desktop");

export const metadata = marketingMetadata({
	title: "Self-hosting",
	description: page?.lede ?? "Run the web application and Rust API on your infrastructure.",
	path: "/desktop",
});

export default async function DesktopPage() {
	"use cache";
	cacheLife("days");
	if (!page) notFound();
	return <SimPlatformPage page={page} />;
}
