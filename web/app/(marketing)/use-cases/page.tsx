import { cacheLife } from "next/cache";

import { indexCopy } from "../catalog";
import { HubPage } from "../editorial/hub-page";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Use cases",
	description: indexCopy.useCases.description,
	path: "/use-cases",
});

export default async function UseCasesIndexPage() {
	"use cache";
	cacheLife("days");

	return <HubPage kind="use-case" />;
}
