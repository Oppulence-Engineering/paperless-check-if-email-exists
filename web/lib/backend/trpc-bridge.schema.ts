import { z } from "zod";

import { BackendProxyPathSchema } from "@/lib/api/routes/schemas/proxy";
import {
	EntityTagSchema,
	IdempotencyKeySchema,
	RequestIdSchema,
} from "@/lib/backend/integration-contract.schema";

/**
 * @oppulence-gen kind=lib
 * Runtime contract for authenticated JSON requests sent through the tRPC BFF.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `trpc-bridge.lit.ts`.
 */
const QueryValueSchema = z.union([z.string().max(4_096), z.array(z.string().max(4_096)).max(100)]);

const RequestBaseSchema = z.strictObject({
	path: BackendProxyPathSchema.min(1).max(100),
	query: z.record(z.string().min(1).max(100), QueryValueSchema).optional(),
	requestId: RequestIdSchema.optional(),
});

export const TrpcBackendReadInputSchema = RequestBaseSchema;

export const TrpcBackendWriteInputSchema = RequestBaseSchema.extend({
	method: z.enum(["POST", "PUT", "PATCH", "DELETE"]),
	body: z.unknown().optional(),
	idempotencyKey: IdempotencyKeySchema.optional(),
	ifMatch: EntityTagSchema.optional(),
});

export const TrpcBackendResponseSchema = z.strictObject({
	status: z.number().int().min(200).max(299),
	data: z.unknown(),
	requestId: z.string().nullable(),
	etag: z.string().nullable(),
});

export type TrpcBackendReadInput = z.infer<typeof TrpcBackendReadInputSchema>;
export type TrpcBackendWriteInput = z.infer<typeof TrpcBackendWriteInputSchema>;
