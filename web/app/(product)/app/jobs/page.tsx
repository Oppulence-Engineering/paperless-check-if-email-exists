import { Suspense } from "react";
import { JobsPanel } from "./_components/jobs-panel/jobs-panel";
import { JobsSearchParamsSchema } from "./search-params";
import JobsLoading from "./loading";

/**
 * @oppulence-gen kind=page
 * Authenticated Jobs route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `jobs.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "Jobs - Oppulence",
	description: "Create bulk jobs and inspect progress, results, approvals, and failures",
};

type JobsPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function JobsPage({ searchParams }: JobsPageProps) {
	return (
		<Suspense fallback={<JobsLoading />}>
			<JobsRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function JobsRouteContent({ searchParams }: JobsPageProps) {
	const raw = await searchParams;
	const parsed = JobsSearchParamsSchema.safeParse({
		job: Array.isArray(raw.job) ? raw.job[0] : raw.job,
	});
	return <JobsPanel initialJobId={parsed.success ? parsed.data.job : undefined} />;
}
