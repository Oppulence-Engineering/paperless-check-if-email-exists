import "server-only";
import { AsyncLocalStorage } from "node:async_hooks";
import { type RequestTimingInput } from "./request-timing.schema";

/** @oppulence-gen kind=lib Owned by `request-timing.lit.ts`. */
export const requestTimings = new AsyncLocalStorage<
	Partial<Record<RequestTimingInput["phase"], number>>
>();

export function recordTiming(
	phase: RequestTimingInput["phase"],
	started: number,
	timings = requestTimings.getStore(),
) {
	if (timings) timings[phase] = (timings[phase] ?? 0) + performance.now() - started;
}

export async function measurePhase<T>(
	phase: RequestTimingInput["phase"],
	work: () => Promise<T>,
): Promise<T> {
	const started = performance.now();
	try {
		return await work();
	} finally {
		recordTiming(phase, started);
	}
}
