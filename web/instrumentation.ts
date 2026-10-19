import { assertServerConfig } from "@/lib/config/server-config";
import { describeError, log, requestLogger } from "@/lib/observability/logger";

/**
 * Server boot hook. Next calls `register` once per server process, before it
 * serves anything, which is the only honest place to check configuration: a
 * deployment missing BETTER_AUTH_SECRET should fail to start, not fail on the
 * first sign-in.
 */
export function register(): void {
	if (process.env.NEXT_RUNTIME !== "nodejs") return;
	assertServerConfig();
	log.info("server started", {
		runtime: process.env.NEXT_RUNTIME ?? "nodejs",
		env: process.env.NODE_ENV ?? "development",
	});
}

/**
 * Every uncaught server error lands here with the request that caused it.
 * Point this at a monitoring service by forwarding `error` and `context`; the
 * request id is the same one the browser and the backend already carry.
 */
export function onRequestError(
	error: unknown,
	request: { path?: string; method?: string; headers?: Record<string, string | undefined> },
	context: { routerKind?: string; routePath?: string; renderSource?: string },
): void {
	const requestId = request.headers?.["x-request-id"];
	requestLogger(requestId).error("unhandled server error", {
		...describeError(error),
		method: request.method ?? null,
		path: request.path ?? null,
		routerKind: context.routerKind ?? null,
		routePath: context.routePath ?? null,
		renderSource: context.renderSource ?? null,
	});
}
