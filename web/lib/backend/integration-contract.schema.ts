import { z } from "zod";

export const RequestIdSchema = z
	.string()
	.min(1)
	.max(128)
	.regex(/^[A-Za-z0-9][A-Za-z0-9._:-]*$/, "must be a safe request identifier");

export const IdempotencyKeySchema = z
	.string()
	.min(8)
	.max(255)
	.regex(/^[\x21-\x7E]+$/, "must contain visible ASCII characters only");

export const EntityTagSchema = z
	.string()
	.min(1)
	.max(512)
	.refine((value) => value === "*" || /^(W\/)?"[^"\r\n]*"$/.test(value), {
		message: "must be a valid HTTP entity tag",
	});

/**
 * @oppulence-gen kind=lib
 * Runtime contract for Backend Integration Contract.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `integration-contract.lit.ts`.
 */
export const BackendErrorEnvelopeSchema = z.strictObject({
	error: z.string().min(1).max(500),
	code: z
		.string()
		.min(1)
		.max(100)
		.regex(/^[a-z][a-z0-9_]*$/),
	requestId: RequestIdSchema.optional(),
	retryable: z.boolean().optional(),
	details: z.record(z.string(), z.unknown()).optional(),
});

export const BackendRequestContextSchema = z.strictObject({
	requestId: RequestIdSchema,
	userId: z.string().min(1).max(255),
	organizationId: z.string().min(1).max(255),
	organizationRole: z.string().min(1).max(100),
	sessionId: z.string().min(1).max(255),
});

export const CursorPaginationQuerySchema = z.strictObject({
	cursor: z.string().min(1).max(2_048).optional(),
	limit: z.number().int().positive().max(10_000).optional(),
});

export const CursorPageMetadataSchema = z.strictObject({
	nextCursor: z.string().min(1).max(2_048).nullable(),
	hasMore: z.boolean(),
});

export const RetryPolicySchema = z.strictObject({
	maxAttempts: z.number().int().min(1).max(5).default(2),
	baseDelayMs: z.number().int().min(0).max(30_000).default(250),
	maxDelayMs: z.number().int().min(0).max(120_000).default(5_000),
	retryStatuses: z
		.array(z.number().int().min(400).max(599))
		.default([408, 425, 429, 502, 503, 504]),
});

export const UploadRequestSchema = z.strictObject({
	fileName: z.string().min(1).max(255),
	contentType: z.string().min(1).max(200),
	sizeBytes: z.number().int().nonnegative(),
	checksumSha256: z
		.string()
		.regex(/^[A-Fa-f0-9]{64}$/)
		.optional(),
});

const UploadURLSchema = z.url().superRefine((value, context) => {
	const url = new URL(value);
	if (!["http:", "https:"].includes(url.protocol)) {
		context.addIssue({ code: "custom", message: "must use HTTP or HTTPS" });
	}
	const localDevelopmentHost = ["localhost", "127.0.0.1", "[::1]"].includes(url.hostname);
	if (url.protocol === "http:" && !localDevelopmentHost) {
		context.addIssue({ code: "custom", message: "must use HTTPS outside local development" });
	}
	if (url.username || url.password || url.hash) {
		context.addIssue({ code: "custom", message: "must not contain credentials or a fragment" });
	}
});

const UploadHeadersSchema = z.record(z.string(), z.string()).superRefine((headers, context) => {
	const forbidden = new Set([
		"authorization",
		"cookie",
		"host",
		"origin",
		"proxy-authorization",
		"referer",
	]);
	for (const name of Object.keys(headers)) {
		const normalized = name.toLowerCase();
		if (
			forbidden.has(normalized) ||
			normalized.startsWith("sec-") ||
			normalized === "content-length"
		) {
			context.addIssue({
				code: "custom",
				path: [name],
				message: "is not allowed in an upload descriptor",
			});
		}
	}
});

export const UploadDescriptorSchema = z.strictObject({
	uploadId: z.string().min(1).max(255),
	url: UploadURLSchema,
	method: z.enum(["PUT", "POST"]),
	headers: UploadHeadersSchema.default({}),
	expiresAt: z.iso.datetime({ offset: true }),
	asset: z.record(z.string(), z.unknown()).optional(),
});

export type BackendErrorEnvelope = z.infer<typeof BackendErrorEnvelopeSchema>;
export type BackendRequestContext = z.infer<typeof BackendRequestContextSchema>;
export type CursorPaginationQuery = z.infer<typeof CursorPaginationQuerySchema>;
export type CursorPageMetadata = z.infer<typeof CursorPageMetadataSchema>;
export type RetryPolicy = z.output<typeof RetryPolicySchema>;
export type RetryPolicyInput = z.input<typeof RetryPolicySchema>;
export type UploadRequest = z.infer<typeof UploadRequestSchema>;
export type UploadDescriptor = z.infer<typeof UploadDescriptorSchema>;
