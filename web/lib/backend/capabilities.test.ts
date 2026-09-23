import fs from "node:fs";
import os from "node:os";
import path from "node:path";

import { describe, expect, it } from "vitest";

import { loadBackendCapabilities, publicBackendCapabilities } from "./capabilities";

const manifest = {
	schemaVersion: 1,
	service: { name: "example-api", apiVersion: "1.0.0" },
	auth: { audience: "private-audience" },
	http: {
		basePath: "/v1",
		openapiPath: "config/contracts/backend.openapi.json",
		requestTimeoutMs: 15_000,
		maxRequestBodyBytes: 1_048_576,
	},
	transports: {
		rest: { enabled: true },
		sse: { enabled: true },
		websocket: { enabled: false, connectionPath: null },
	},
	uploads: {
		enabled: false,
		descriptorPath: null,
		maxBytes: 1_048_576,
		allowedContentTypes: [],
	},
	health: { livenessPath: "/healthz", readinessPath: "/readyz", timeoutMs: 2_000 },
	pagination: { strategy: "cursor", defaultPageSize: 25, maxPageSize: 100 },
	contract: { errorMediaType: "application/problem+json", requiredOperationIds: [] },
	features: { reports: true },
} as const;

describe("backend capabilities", () => {
	it("loads and validates an explicit manifest", () => {
		const directory = fs.mkdtempSync(path.join(os.tmpdir(), "backend-capabilities-"));
		const file = path.join(directory, "capabilities.json");
		fs.writeFileSync(file, JSON.stringify(manifest));
		try {
			expect(loadBackendCapabilities(file)).toEqual(manifest);
		} finally {
			fs.rmSync(directory, { recursive: true, force: true });
		}
	});

	it("projects only browser-safe capability fields", () => {
		const projected = publicBackendCapabilities(manifest);
		expect(projected.features).toEqual({ reports: true });
		expect(projected.transports).toEqual({ rest: true, sse: true, websocket: false });
		expect(projected).not.toHaveProperty("auth");
		expect(projected).not.toHaveProperty("health");
		expect(projected).not.toHaveProperty("http");
		expect(projected).not.toHaveProperty("contract");
	});
});
