import "server-only";

import { getAuthRuntimeConfig } from "@/lib/auth/config";
import { serverConfigProblems } from "@/lib/config/server-config";
import { loadBackendCapabilities } from "@/lib/backend/capabilities";
import { database } from "@/lib/auth/database";
import { isBackendReady } from "@/lib/bff/readiness";
import { NextResponse } from "next/server";

function notReady() {
	return NextResponse.json(
		{ status: "not_ready" },
		{
			status: 503,
			headers: { "Cache-Control": "no-store" },
		},
	);
}

export async function GET() {
	try {
		const config = getAuthRuntimeConfig();
		if (serverConfigProblems().length > 0) return notReady();
		const capabilities = loadBackendCapabilities();
		if (
			!config.resendApiKey ||
			!(await database()) ||
			!(await isBackendReady(
				config.backendApiUrl,
				capabilities.health.readinessPath,
				capabilities.health.timeoutMs,
			))
		) {
			return notReady();
		}
	} catch {
		// Keep configuration details and upstream errors out of the public response.
		return notReady();
	}

	return NextResponse.json({ status: "ready" }, { headers: { "Cache-Control": "no-store" } });
}
