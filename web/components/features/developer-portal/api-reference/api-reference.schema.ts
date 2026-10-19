import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Api reference.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `api-reference.lit.ts`.
 *
 * `document` is an OpenAPI document that the server builds
 * (`referenceDocument` in lib/developer-portal/operations.ts). `surface`
 * picks the signed-in workspace explorer or the public reference.
 */
export const ApiReferencePropsSchema = z.object({
	document: z.record(z.string(), z.unknown()),
	surface: z.enum(["workspace", "public"]),
});

export type ApiReferencePropsFields = z.infer<typeof ApiReferencePropsSchema>;
