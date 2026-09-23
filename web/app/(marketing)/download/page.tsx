import { cacheLife } from "next/cache";

import { SimDownloadPage } from "../sim-landing/subpages/sim-download-page";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Run Check If Email Exists",
	description:
		"Run the open-source web application and Rust API locally or in a combined container.",
	path: "/download",
});

export default async function DownloadPage() {
	"use cache";
	cacheLife("hours");
	return <SimDownloadPage />;
}
