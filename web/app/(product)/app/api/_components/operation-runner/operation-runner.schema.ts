import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Operation runner.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `operation-runner.lit.ts`.
 */
export const OperationRunnerPropsSchema = z.object({
	operations: z.array(
		z.object({
			id: z.string(),
			method: z.enum(["GET", "POST", "PUT", "PATCH", "DELETE"]),
			path: z.string(),
			family: z.string(),
			description: z.string().optional(),
			scope: z.string().optional(),
			requestMedia: z.array(z.string()),
		}),
	),
});

export type OperationRunnerPropsFields = z.infer<typeof OperationRunnerPropsSchema>;
