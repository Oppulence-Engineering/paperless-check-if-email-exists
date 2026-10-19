import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Platform operation runner.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `platform-operation-runner.lit.ts`.
 */
export const PlatformOperationRunnerPropsSchema = z.object({
	operations: z.array(
		z.object({
			id: z.string(),
			method: z.enum(["GET", "POST", "PUT", "PATCH", "DELETE"]),
			path: z.string(),
			description: z.string().optional(),
			requestBody: z.boolean(),
		}),
	),
});

export type PlatformOperationRunnerPropsFields = z.infer<typeof PlatformOperationRunnerPropsSchema>;
