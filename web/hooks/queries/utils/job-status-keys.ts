/**
 * @oppulence-gen kind=hook
 * Query-key factory and staleTime for Job status.
 * Keep this module free of `"use client"` so server prefetch can import it.
 * Owned by `use-job-status.lit.ts`.
 */
export const jobStatusKeys = {
	all: ["jobs"] as const,
	list: (query?: Record<string, unknown>) => [...jobStatusKeys.all, "list", query ?? {}] as const,
	detail: (id?: string) => [...jobStatusKeys.all, Number(id ?? 0), "status"] as const,
};

export const JOB_STATUS_STALE_TIME = 60_000;
