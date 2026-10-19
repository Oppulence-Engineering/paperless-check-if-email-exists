import { allCapabilityPages, indexCopy, type CapabilityPage } from "../catalog";
import { PAGE_FACTS } from "./capability-content";
import { CatalogHub } from "./catalog-hub";

const HUB_COPY: Record<CapabilityPage["kind"], { title: string; lede: string }> = {
	feature: { title: "Features", lede: indexCopy.features.description },
	"use-case": { title: "Use cases", lede: indexCopy.useCases.description },
	integration: { title: "Integrations", lede: indexCopy.integrations.description },
	guide: { title: "Guides", lede: indexCopy.guides.description },
};

/** Server wrapper: builds the hub items and the category counts from the catalog. */
export function HubPage({ kind }: { kind: CapabilityPage["kind"] }) {
	const counts = { feature: 0, "use-case": 0, integration: 0, guide: 0 };
	for (const page of allCapabilityPages) counts[page.kind] += 1;
	const items = allCapabilityPages
		.filter((page) => page.kind === kind)
		.map((page) => ({
			kind: page.kind,
			slug: page.slug,
			href: page.path,
			title: page.eyebrow,
			body: page.description,
			spec: PAGE_FACTS[page.path]?.specs[0],
		}));
	return <CatalogHub counts={counts} items={items} kind={kind} {...HUB_COPY[kind]} />;
}
