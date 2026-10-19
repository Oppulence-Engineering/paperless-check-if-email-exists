import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

import { z } from "zod";

import { CapabilitiesSchema } from "@/lib/backend/capabilities.schema";

const OpenAPIOperationSchema = z.looseObject({
	operationId: z.string().min(1).optional(),
	responses: z.record(z.string(), z.unknown()).default({}),
});

const OpenAPIPathItemSchema = z.looseObject({
	get: OpenAPIOperationSchema.optional(),
	post: OpenAPIOperationSchema.optional(),
	put: OpenAPIOperationSchema.optional(),
	patch: OpenAPIOperationSchema.optional(),
	delete: OpenAPIOperationSchema.optional(),
	options: OpenAPIOperationSchema.optional(),
	head: OpenAPIOperationSchema.optional(),
});

const HTTP_METHODS = ["get", "post", "put", "patch", "delete", "options", "head"] as const;

export const BackendOpenAPISchema = z.looseObject({
	openapi: z.string().min(1),
	info: z.looseObject({ version: z.string().min(1) }),
	paths: z.record(z.string(), OpenAPIPathItemSchema),
	components: z.looseObject({
		schemas: z.record(z.string(), z.unknown()).default({}),
	}),
});

export const CompatibilityIssueSchema = z.strictObject({
	code: z.string().min(1),
	message: z.string().min(1),
});

export type CompatibilityIssue = z.infer<typeof CompatibilityIssueSchema>;

function operationIds(openapi: z.infer<typeof BackendOpenAPISchema>): Set<string> {
	const ids = new Set<string>();
	for (const item of Object.values(openapi.paths)) {
		for (const method of HTTP_METHODS) {
			const id = item[method]?.operationId;
			if (id) ids.add(id);
		}
	}
	return ids;
}

function contractPath(basePath: string, endpoint: string): string {
	if (endpoint === basePath || endpoint.startsWith(`${basePath}/`)) return endpoint;
	return `${basePath.replace(/\/$/, "")}/${endpoint.replace(/^\//, "")}`;
}

function isRecord(value: unknown): value is Record<string, unknown> {
	return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function responseHasMediaType(response: unknown, mediaType: string): boolean {
	if (!isRecord(response) || !isRecord(response.content)) return false;
	return mediaType in response.content;
}

function hasResponseMediaType(
	openapi: z.infer<typeof BackendOpenAPISchema>,
	mediaType: string,
	acceptsStatus: (status: string) => boolean,
): boolean {
	for (const item of Object.values(openapi.paths)) {
		for (const method of HTTP_METHODS) {
			const operation = item[method];
			if (!operation) continue;
			for (const [status, response] of Object.entries(operation.responses)) {
				if (acceptsStatus(status) && responseHasMediaType(response, mediaType)) return true;
			}
		}
	}
	return false;
}

/** Pure compatibility check used by CI, local development, and tests. */
export function checkBackendCompatibility(
	capabilityInput: unknown,
	openapiInput: unknown,
	expectedAudience?: string,
): CompatibilityIssue[] {
	const capabilities = CapabilitiesSchema.parse(capabilityInput);
	const openapi = BackendOpenAPISchema.parse(openapiInput);
	const issues: CompatibilityIssue[] = [];

	if (openapi.info.version !== capabilities.service.apiVersion) {
		issues.push({
			code: "api_version_mismatch",
			message: `capability apiVersion ${capabilities.service.apiVersion} does not match OpenAPI ${openapi.info.version}`,
		});
	}

	if (!Object.keys(openapi.paths).some((entry) => entry.startsWith(capabilities.http.basePath))) {
		issues.push({
			code: "base_path_missing",
			message: `OpenAPI has no paths under ${capabilities.http.basePath}`,
		});
	}

	if (!("ErrorEnvelope" in openapi.components.schemas)) {
		issues.push({
			code: "error_envelope_missing",
			message: "OpenAPI components.schemas must define ErrorEnvelope",
		});
	}

	if (
		!hasResponseMediaType(
			openapi,
			capabilities.contract.errorMediaType,
			(status) => status === "default" || /^[45](?:\d{2}|XX)$/i.test(status),
		)
	) {
		issues.push({
			code: "error_media_type_missing",
			message: `OpenAPI does not declare ${capabilities.contract.errorMediaType}`,
		});
	}

	if (
		capabilities.transports.sse.enabled &&
		!hasResponseMediaType(openapi, "text/event-stream", (status) => /^2(?:\d{2}|XX)$/i.test(status))
	) {
		issues.push({
			code: "sse_contract_missing",
			message: "SSE is enabled but OpenAPI declares no text/event-stream response",
		});
	}

	const requiredPaths: Array<[string, string | null]> = [
		["websocket_connection_path_missing", capabilities.transports.websocket.connectionPath],
		["upload_descriptor_path_missing", capabilities.uploads.descriptorPath],
	];
	for (const [code, endpoint] of requiredPaths) {
		if (endpoint && !(contractPath(capabilities.http.basePath, endpoint) in openapi.paths)) {
			issues.push({
				code,
				message: `OpenAPI does not declare ${contractPath(capabilities.http.basePath, endpoint)}`,
			});
		}
	}

	const availableOperations = operationIds(openapi);
	for (const required of capabilities.contract.requiredOperationIds) {
		if (!availableOperations.has(required)) {
			issues.push({
				code: "required_operation_missing",
				message: `OpenAPI does not declare required operationId ${required}`,
			});
		}
	}

	if (expectedAudience && capabilities.auth.audience !== expectedAudience) {
		issues.push({
			code: "audience_mismatch",
			message: `capability audience ${capabilities.auth.audience} does not match BACKEND_JWT_AUDIENCE`,
		});
	}

	return issues;
}

function readJSON(filePath: string): unknown {
	return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

export function checkConfiguredBackendCompatibility(root = process.cwd()): CompatibilityIssue[] {
	const capabilitiesPath = path.resolve(
		root,
		process.env.BACKEND_CAPABILITIES_PATH ?? "config/contracts/backend.capabilities.json",
	);
	const capabilities = CapabilitiesSchema.parse(readJSON(capabilitiesPath));
	const openapiPath = path.resolve(root, capabilities.http.openapiPath);
	return checkBackendCompatibility(
		capabilities,
		readJSON(openapiPath),
		process.env.BACKEND_JWT_AUDIENCE,
	);
}

function main(): void {
	const issues = checkConfiguredBackendCompatibility();
	if (issues.length === 0) {
		process.stdout.write("Backend capability manifest is compatible with the OpenAPI contract.\n");
		return;
	}
	for (const issue of issues) process.stderr.write(`[${issue.code}] ${issue.message}\n`);
	process.exitCode = 1;
}

const entrypoint = process.argv[1];
if (entrypoint && import.meta.url === pathToFileURL(entrypoint).href) main();
