import { cacheLife } from "next/cache";

import { indexCopy } from "../catalog";
import { HubPage } from "../editorial/hub-page";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Integrations",
	description: indexCopy.integrations.description,
	path: "/integrations",
});

export default async function IntegrationsIndexPage() {
	"use cache";
	cacheLife("days");

	return <HubPage kind="integration" />;
}
