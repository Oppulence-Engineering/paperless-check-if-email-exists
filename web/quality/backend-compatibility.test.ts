import { describe, expect, it } from "vitest";

import { checkBackendCompatibility } from "@/scripts/check-backend-compatibility";

const capabilities = {
	schemaVersion: 1,
	service: { name: "example-api", apiVersion: "1.2.3" },
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
		websocket: { enabled: true, connectionPath: "/realtime/connection" },
	},
	uploads: {
		enabled: true,
		descriptorPath: "/uploads/descriptor",
		maxBytes: 1_048_576,
		allowedContentTypes: ["image/png"],
	},
	health: { livenessPath: "/healthz", readinessPath: "/readyz", timeoutMs: 2_000 },
	pagination: { strategy: "cursor", defaultPageSize: 25, maxPageSize: 100 },
	contract: {
		errorMediaType: "application/problem+json",
		requiredOperationIds: ["listWidgets"],
	},
	features: {},
} as const;

const openapi = {
	openapi: "3.0.3",
	info: { title: "Example", version: "1.2.3" },
	paths: {
		"/v1/widgets": {
			get: {
				operationId: "listWidgets",
				responses: {
					"200": { content: { "application/json": {} } },
					"400": { content: { "application/problem+json": {} } },
				},
			},
		},
		"/v1/events": {
			get: {
				operationId: "streamEvents",
				responses: { "200": { content: { "text/event-stream": {} } } },
			},
		},
		"/v1/realtime/connection": {
			post: { operationId: "createRealtimeConnection", responses: { "200": {} } },
		},
		"/v1/uploads/descriptor": {
			post: { operationId: "createUploadDescriptor", responses: { "200": {} } },
		},
	},
	components: { schemas: { ErrorEnvelope: { type: "object" } } },
};

describe("backend compatibility", () => {
	it("accepts a capability manifest aligned with OpenAPI", () => {
		expect(checkBackendCompatibility(capabilities, openapi, "example-api")).toEqual([]);
	});

	it("reports version, transport, operation, and audience drift", () => {
		const issues = checkBackendCompatibility(
			{
				...capabilities,
				service: { ...capabilities.service, apiVersion: "2.0.0" },
				auth: { audience: "wrong-audience" },
				contract: {
					...capabilities.contract,
					requiredOperationIds: ["missingOperation"],
				},
			},
			{
				...openapi,
				paths: { "/v1/widgets": openapi.paths["/v1/widgets"] },
			},
			"example-api",
		);

		expect(issues.map((issue) => issue.code)).toEqual(
			expect.arrayContaining([
				"api_version_mismatch",
				"sse_contract_missing",
				"websocket_connection_path_missing",
				"upload_descriptor_path_missing",
				"required_operation_missing",
				"audience_mismatch",
			]),
		);
	});

	it("only accepts the error media type on error responses", () => {
		const issues = checkBackendCompatibility(capabilities, {
			...openapi,
			paths: {
				...openapi.paths,
				"/v1/widgets": {
					get: {
						...openapi.paths["/v1/widgets"].get,
						requestBody: { content: { "application/problem+json": {} } },
						responses: {
							"200": { content: { "application/problem+json": {} } },
							"400": { content: { "application/json": {} } },
						},
					},
				},
			},
		});

		expect(issues.map((issue) => issue.code)).toContain("error_media_type_missing");
	});

	it("only accepts SSE media types on successful responses", () => {
		const issues = checkBackendCompatibility(capabilities, {
			...openapi,
			paths: {
				...openapi.paths,
				"/v1/events": {
					get: {
						...openapi.paths["/v1/events"].get,
						requestBody: { content: { "text/event-stream": {} } },
						responses: { "200": { content: { "application/json": {} } } },
					},
				},
			},
		});

		expect(issues.map((issue) => issue.code)).toContain("sse_contract_missing");
	});
});
