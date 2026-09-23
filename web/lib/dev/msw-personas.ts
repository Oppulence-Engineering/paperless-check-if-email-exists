import { http, HttpResponse } from "msw";

import type { DevPersona } from "@/lib/dev/dev-prefs";

/** Extra MSW handlers layered on top of Orval fakes for dev personas. */
export function personaHandlers(persona: DevPersona) {
	switch (persona) {
		case "empty-workspace":
			return [http.get("/api/backend/v1/lists", () => HttpResponse.json({ lists: [], total: 0 }))];
		case "session-expired":
			return [
				http.get("/api/auth/session", () =>
					HttpResponse.json({ authenticated: false }, { status: 401 }),
				),
			];
		default:
			return [];
	}
}
