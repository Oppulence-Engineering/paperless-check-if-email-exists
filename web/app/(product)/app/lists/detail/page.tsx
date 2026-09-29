import { Suspense } from "react";
import { z } from "zod";
import { ListDetailPanel } from "./_components/list-detail-panel/list-detail-panel";
import ListDetailLoading from "./loading";

/**
 * @oppulence-gen kind=page
 * Authenticated List detail route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `detail.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "List detail - Oppulence",
	description: "Review quality, remediation, exports, and team comments for one list",
};

type ListDetailPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function ListDetailPage({ searchParams }: ListDetailPageProps) {
	return (
		<Suspense fallback={<ListDetailLoading />}>
			<ListDetailRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function ListDetailRouteContent({ searchParams }: ListDetailPageProps) {
	const raw = await searchParams;
	const id = z.coerce.number().int().positive().safeParse(raw.list);
	return <ListDetailPanel listId={id.success ? id.data : undefined} />;
}
