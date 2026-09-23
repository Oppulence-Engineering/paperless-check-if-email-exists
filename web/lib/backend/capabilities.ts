import "server-only";

import fs from "node:fs";
import path from "node:path";

import {
	CapabilitiesSchema,
	PublicCapabilitiesSchema,
	type Capabilities,
	type PublicCapabilities,
} from "./capabilities.schema";

let cachedPath: string | null = null;
let cachedCapabilities: Capabilities | null = null;

function backendCapabilitiesPath(override?: string): string {
	const configured = override || process.env.BACKEND_CAPABILITIES_PATH;
	return configured
		? path.resolve(/* turbopackIgnore: true */ process.cwd(), configured)
		: path.resolve(process.cwd(), "config/contracts/backend.capabilities.json");
}

/**
 * @oppulence-gen kind=lib
 * Loads the deployment-selected backend capability manifest once per process.
 * Validated backend capability manifest with a safe browser-facing projection.
 *
 * The manifest contains no secrets, but server-only fields such as the JWT
 * audience and backend health paths never enter the browser projection. Owned
 * by `capabilities.lit.ts`.
 */
export function loadBackendCapabilities(override?: string): Capabilities {
	const manifestPath = backendCapabilitiesPath(override);
	if (!override && cachedCapabilities && cachedPath === manifestPath) return cachedCapabilities;

	const parsed = CapabilitiesSchema.parse(JSON.parse(fs.readFileSync(manifestPath, "utf8")));
	if (!override) {
		cachedPath = manifestPath;
		cachedCapabilities = parsed;
	}
	return parsed;
}

export function publicBackendCapabilities(manifest: Capabilities): PublicCapabilities {
	return PublicCapabilitiesSchema.parse({
		schemaVersion: manifest.schemaVersion,
		service: manifest.service,
		transports: {
			rest: manifest.transports.rest.enabled,
			sse: manifest.transports.sse.enabled,
			websocket: manifest.transports.websocket.enabled,
		},
		uploads: {
			enabled: manifest.uploads.enabled,
			maxBytes: manifest.uploads.maxBytes,
			allowedContentTypes: manifest.uploads.allowedContentTypes,
		},
		pagination: manifest.pagination,
		features: manifest.features,
	});
}
