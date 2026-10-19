import { z } from "zod";

const RelativeBackendPathSchema = z
	.string()
	.startsWith("/")
	.refine((value) => !value.startsWith("//") && !/[?#]/.test(value), {
		message: "must be an absolute backend path without query or fragment",
	});

const OptionalBackendPathSchema = RelativeBackendPathSchema.nullable();

const RepositoryPathSchema = z
	.string()
	.min(1)
	.refine(
		(value) =>
			!value.startsWith("/") &&
			!value.includes("\\") &&
			value.split("/").every((segment) => segment !== ".." && segment !== "." && segment !== ""),
		{ message: "must be a safe repository-relative path" },
	);

/**
 * @oppulence-gen kind=lib
 * Runtime contract for Backend Capabilities.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `capabilities.lit.ts`.
 */
export const CapabilitiesSchema = z
	.strictObject({
		schemaVersion: z.literal(1),
		service: z.strictObject({
			name: z.string().min(1).max(100),
			apiVersion: z.string().min(1).max(100),
		}),
		auth: z.strictObject({
			audience: z.string().min(1).max(200),
		}),
		http: z.strictObject({
			basePath: RelativeBackendPathSchema,
			openapiPath: RepositoryPathSchema,
			requestTimeoutMs: z.number().int().min(1_000).max(120_000),
			maxRequestBodyBytes: z
				.number()
				.int()
				.positive()
				.max(100 * 1024 * 1024),
		}),
		transports: z.strictObject({
			rest: z.strictObject({ enabled: z.literal(true) }),
			sse: z.strictObject({ enabled: z.boolean() }),
			websocket: z.strictObject({
				enabled: z.boolean(),
				connectionPath: OptionalBackendPathSchema,
			}),
		}),
		uploads: z.strictObject({
			enabled: z.boolean(),
			descriptorPath: OptionalBackendPathSchema,
			maxBytes: z
				.number()
				.int()
				.positive()
				.max(5 * 1024 * 1024 * 1024),
			allowedContentTypes: z.array(z.string().min(1).max(200)).max(100),
		}),
		health: z.strictObject({
			livenessPath: RelativeBackendPathSchema,
			readinessPath: RelativeBackendPathSchema,
			timeoutMs: z.number().int().min(250).max(30_000),
		}),
		pagination: z.strictObject({
			strategy: z.enum(["cursor", "offset", "none"]),
			defaultPageSize: z.number().int().positive().max(10_000),
			maxPageSize: z.number().int().positive().max(100_000),
		}),
		contract: z.strictObject({
			errorMediaType: z.enum(["application/json", "application/problem+json"]),
			requiredOperationIds: z.array(z.string().min(1).max(200)).max(500).default([]),
		}),
		features: z.record(z.string().min(1).max(100), z.boolean()).default({}),
	})
	.superRefine((manifest, context) => {
		if (manifest.transports.websocket.enabled && !manifest.transports.websocket.connectionPath) {
			context.addIssue({
				code: "custom",
				path: ["transports", "websocket", "connectionPath"],
				message: "is required when WebSocket transport is enabled",
			});
		}
		if (!manifest.transports.websocket.enabled && manifest.transports.websocket.connectionPath) {
			context.addIssue({
				code: "custom",
				path: ["transports", "websocket", "connectionPath"],
				message: "must be null when WebSocket transport is disabled",
			});
		}
		if (manifest.uploads.enabled && !manifest.uploads.descriptorPath) {
			context.addIssue({
				code: "custom",
				path: ["uploads", "descriptorPath"],
				message: "is required when uploads are enabled",
			});
		}
		if (!manifest.uploads.enabled && manifest.uploads.descriptorPath) {
			context.addIssue({
				code: "custom",
				path: ["uploads", "descriptorPath"],
				message: "must be null when uploads are disabled",
			});
		}
		if (manifest.pagination.defaultPageSize > manifest.pagination.maxPageSize) {
			context.addIssue({
				code: "custom",
				path: ["pagination", "defaultPageSize"],
				message: "must not exceed maxPageSize",
			});
		}
	});

/** Browser-safe projection. Internal origins, auth audience, and health paths are omitted. */
export const PublicCapabilitiesSchema = z.strictObject({
	schemaVersion: z.literal(1),
	service: CapabilitiesSchema.shape.service,
	transports: z.strictObject({
		rest: z.boolean(),
		sse: z.boolean(),
		websocket: z.boolean(),
	}),
	uploads: z.strictObject({
		enabled: z.boolean(),
		maxBytes: z.number().int().positive(),
		allowedContentTypes: z.array(z.string()),
	}),
	pagination: CapabilitiesSchema.shape.pagination,
	features: z.record(z.string(), z.boolean()),
});

export type Capabilities = z.infer<typeof CapabilitiesSchema>;
export type PublicCapabilities = z.infer<typeof PublicCapabilitiesSchema>;
