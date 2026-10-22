// @vitest-environment jsdom
import { QueryClientProvider, QueryObserver, focusManager } from "@tanstack/react-query";
import { act, cleanup, render } from "@testing-library/react";
import { JobsApi } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";
import { createAppQueryClient } from "@/lib/query/get-query-client";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { v1ListListsKeys } from "@/hooks/queries/utils/v1-list-lists-keys";
import { useJobStatus } from "@/hooks/queries/use-job-status";

const clients: ReturnType<typeof createAppQueryClient>[] = [];
function client() {
	const value = createAppQueryClient();
	clients.push(value);
	return value;
}
afterEach(() => {
	cleanup();
	clients.forEach((c) => {
		c.clear();
	});
	clients.length = 0;
	focusManager.setFocused(undefined);
	vi.useRealTimers();
	vi.restoreAllMocks();
});

it("reuses reads for 30 seconds and invalidates derived safety after a mutation", async () => {
	vi.useFakeTimers();
	const cache = client(),
		read = vi.fn(() => Promise.resolve("ready"));
	await cache.query({ queryKey: ["overview"], queryFn: read });
	vi.advanceTimersByTime(29_000);
	await cache.query({ queryKey: ["overview"], queryFn: read });
	expect(read).toHaveBeenCalledTimes(1);
	for (const key of [
		["list-detail", 1, "quality"],
		["list-detail", 1, "plan"],
		["jobs", 2, "approval"],
		["source-quality"],
		["query-results"],
		["suppressions"],
		["suppression-check", "person@example.com"],
		["outcomes"],
		["v1-email-history"],
		["jobs", 2, "results"],
	])
		cache.setQueryData(key, {});
	await cache
		.getMutationCache()
		.build(cache, { mutationFn: () => Promise.resolve() })
		.execute(undefined);
	expect(
		cache
			.getQueryCache()
			.getAll()
			.every((q) => q.state.isInvalidated),
	).toBe(true);
	await cache.query({ queryKey: ["overview"], queryFn: read });
	expect(read).toHaveBeenCalledTimes(2);
});

it("retries transient reads once and never retries validation or mutation failures", async () => {
	const cache = client();
	const transient = vi
		.fn()
		.mockRejectedValueOnce(Object.assign(new Error("Unavailable"), { status: 503 }))
		.mockResolvedValue("ready");
	await expect(
		cache.query({ queryKey: ["read"], queryFn: transient, retryDelay: 0 }),
	).resolves.toBe("ready");
	expect(transient).toHaveBeenCalledTimes(2);
	const invalid = vi.fn().mockRejectedValue(Object.assign(new Error("Invalid"), { status: 400 }));
	await expect(cache.query({ queryKey: ["invalid"], queryFn: invalid })).rejects.toThrow("Invalid");
	expect(invalid).toHaveBeenCalledTimes(1);
	const mutation = vi.fn().mockRejectedValue(new Error("Unavailable"));
	await expect(
		cache.getMutationCache().build(cache, { mutationFn: mutation }).execute(undefined),
	).rejects.toThrow("Unavailable");
	expect(mutation).toHaveBeenCalledTimes(1);
});

function Status({ poll = false }: { poll?: boolean }) {
	useJobStatus(9, poll);
	return null;
}

it("shares status, pauses when hidden, refreshes on return, and stops at terminal state", async () => {
	vi.useFakeTimers();
	focusManager.setFocused(true);
	let state = "running";
	const read = vi.spyOn(JobsApi.prototype, "v1GetJobStatus").mockImplementation(() =>
		Promise.resolve({
			data: {
				job_id: 9,
				status: state,
				total_records: 2,
				created_at: "2026-10-21T00:00:00Z",
				task_summary: { completed: 0 },
			},
		} as never),
	);
	const cache = client();
	render(
		<QueryClientProvider client={cache}>
			<Status poll />
			<Status />
		</QueryClientProvider>,
	);
	await act(() => vi.advanceTimersByTimeAsync(10));
	expect(read).toHaveBeenCalledTimes(1);
	await act(() => vi.advanceTimersByTimeAsync(5_000));
	expect(read).toHaveBeenCalledTimes(2);
	act(() => {
		focusManager.setFocused(false);
	});
	await act(() => vi.advanceTimersByTimeAsync(15_000));
	expect(read).toHaveBeenCalledTimes(2);
	state = "cancelled";
	await act(async () => {
		focusManager.setFocused(true);
		await vi.advanceTimersByTimeAsync(10);
	});
	expect(read).toHaveBeenCalledTimes(3);
	await act(() => vi.advanceTimersByTimeAsync(15_000));
	expect(read).toHaveBeenCalledTimes(3);
});

it("aborts the SDK request when the final status observer unmounts", () => {
	let signal: AbortSignal | undefined;
	vi.spyOn(JobsApi.prototype, "v1GetJobStatus").mockImplementation((_args, options) => {
		signal = options?.signal as AbortSignal;
		return new Promise(() => {});
	});
	const view = render(
		<QueryClientProvider client={client()}>
			<Status poll />
		</QueryClientProvider>,
	);
	expect(signal?.aborted).toBe(false);
	view.unmount();
	expect(signal?.aborted).toBe(true);
});

it("refreshes an active resource only once when the page also invalidates it", async () => {
	const cache = client();
	const options = {
		queryKey: ["suppression-check", "person@example.com"],
		queryFn: vi.fn(() => Promise.resolve({ suppressed: true })),
	};
	const observer = new QueryObserver(cache, options);
	const unsubscribe = observer.subscribe(() => {});
	try {
		await cache.query(options);
		expect(options.queryFn).toHaveBeenCalledTimes(1);
		await cache
			.getMutationCache()
			.build(cache, {
				mutationFn: () => Promise.resolve(),
				onSuccess: () => cache.invalidateQueries({ queryKey: options.queryKey }),
			})
			.execute(undefined);
		expect(options.queryFn).toHaveBeenCalledTimes(2);
	} finally {
		unsubscribe();
	}
});

it("reuses equivalent filters and default pagination", async () => {
	const cache = client(),
		read = vi.fn(() => Promise.resolve([]));
	await cache.query({
		queryKey: workflowQueryKey("query-results", 0, { category: "valid", safeToSend: true }),
		queryFn: read,
	});
	await cache.query({
		queryKey: workflowQueryKey("query-results", 0, { safeToSend: true, category: "valid" }),
		queryFn: read,
	});
	expect(read).toHaveBeenCalledTimes(1);
	await cache.query({ queryKey: v1ListListsKeys.list(), queryFn: read });
	await cache.query({ queryKey: v1ListListsKeys.list({ offset: 0, limit: 50 }), queryFn: read });
	expect(read).toHaveBeenCalledTimes(2);
});
