import { describe, expect, it } from "vitest";

import { CapabilitiesSchema } from "./capabilities.schema";

const manifest = {
	schemaVersion: 1,
	service: { name: "example-api", apiVersion: "1.0.0" },
	auth: { audience: "example-api" },
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
	features: {},
} as const;

describe("CapabilitiesSchema", () => {
	it("parses a deployable backend manifest", () => {
		expect(CapabilitiesSchema.parse(manifest)).toEqual(manifest);
	});

	it("requires endpoints for enabled optional transports", () => {
		expect(
			CapabilitiesSchema.safeParse({
				...manifest,
				transports: {
					...manifest.transports,
					websocket: { enabled: true, connectionPath: null },
				},
			}).success,
		).toBe(false);
		expect(
			CapabilitiesSchema.safeParse({
				...manifest,
				uploads: { ...manifest.uploads, enabled: true, descriptorPath: null },
			}).success,
		).toBe(false);
	});

	it("rejects unsafe contract paths and invalid pagination bounds", () => {
		expect(
			CapabilitiesSchema.safeParse({
				...manifest,
				http: { ...manifest.http, openapiPath: "../backend.openapi.json" },
			}).success,
		).toBe(false);
		expect(
			CapabilitiesSchema.safeParse({
				...manifest,
				pagination: { strategy: "cursor", defaultPageSize: 101, maxPageSize: 100 },
			}).success,
		).toBe(false);
	});
});
