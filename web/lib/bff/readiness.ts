import "server-only";

export async function isBackendReady(
	apiBaseUrl: string,
	readinessPath = "/readyz",
	timeoutMs = 2_000,
): Promise<boolean> {
	try {
		const response = await fetch(new URL(readinessPath, apiBaseUrl), {
			cache: "no-store",
			signal: AbortSignal.timeout(timeoutMs),
		});
		return response.ok;
	} catch {
		return false;
	}
}
