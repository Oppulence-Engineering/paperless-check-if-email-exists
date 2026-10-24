import { V1GetJobStatus200Response } from "@/lib/api/generated/zod/jobs/jobs";
import { JobStatusSchema } from "@/lib/api/schemas/jobs";
import { Configuration, JobsApi } from "@oppulence/reacher-sdk";
import { type RequestJsonFn } from "@/lib/api/request-json";

/**
 * @oppulence-gen kind=hook
 * Same-origin BFF read for Job status. Validates with the Orval schema
 * JobStatusSchema. Do not cast the parsed result. Owned by `use-job-status.lit.ts`.
 */
export function jobStatusPath(job: string): string {
	return `/v1/jobs/${encodeURIComponent(job)}`;
}

export async function loadJobStatus(request: RequestJsonFn, job: string, signal?: AbortSignal) {
	return request({
		path: jobStatusPath(job),
		schema: V1GetJobStatus200Response.pipe(JobStatusSchema),
		signal,
	});
}

const sdk = new JobsApi(new Configuration({ basePath: "/api/backend" }));
export async function fetchJobStatus(job: string, signal?: AbortSignal) {
	const jobId = Number(job);
	if (!Number.isSafeInteger(jobId) || jobId <= 0) throw new Error("Select a valid job.");
	return V1GetJobStatus200Response.pipe(JobStatusSchema).parse(
		(await sdk.v1GetJobStatus({ jobId }, { signal })).data,
	);
}
