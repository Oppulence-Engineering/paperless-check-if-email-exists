import { setupWorker } from "msw/browser";

import { getV1Mock } from "@/lib/api/generated/client/v1/v1.msw";
import type { DevPersona } from "@/lib/dev/dev-prefs";
import { personaHandlers } from "@/lib/dev/msw-personas";

let worker: ReturnType<typeof setupWorker> | undefined;
let starting: Promise<void> | undefined;
let activePersona: DevPersona = "default";

function createWorker(persona: DevPersona) {
	return setupWorker(...getV1Mock(), ...personaHandlers(persona));
}

export function isMswRunning(): boolean {
	return Boolean(worker);
}

export async function startMswBrowser(persona: DevPersona = activePersona): Promise<void> {
	if (typeof window === "undefined") return;
	activePersona = persona;
	if (worker) return;
	if (starting) return starting;

	starting = (async () => {
		worker = createWorker(persona);
		await worker.start({
			onUnhandledRequest: "bypass",
			quiet: false,
			serviceWorker: { url: "/mockServiceWorker.js" },
		});
	})();

	try {
		await starting;
	} finally {
		starting = undefined;
	}
}

export async function restartMswBrowser(persona: DevPersona): Promise<void> {
	await stopMswBrowser();
	activePersona = persona;
	await startMswBrowser(persona);
}

export async function stopMswBrowser(): Promise<void> {
	if (!worker) return;
	await worker.stop();
	worker = undefined;
}
