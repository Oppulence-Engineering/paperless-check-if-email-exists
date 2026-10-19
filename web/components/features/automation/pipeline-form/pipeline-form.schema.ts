import { z } from "zod";

import { V1GetPipeline200Response } from "@/lib/api/generated/zod/pipelines/pipelines";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Pipeline form.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `pipeline-form.lit.ts`.
 */
export type Pipeline = z.infer<typeof V1GetPipeline200Response>;

/** Webhook headers typed as `Name: value`, one header on each line. */
export const WebhookHeadersSchema = z.string().transform((text, context) => {
	const headers: Record<string, string> = {};
	for (const line of text.split("\n")) {
		if (!line.trim()) continue;
		const colon = line.indexOf(":");
		const name = line.slice(0, Math.max(colon, 0)).trim();
		if (!name) {
			context.addIssue({
				code: "custom",
				message: `Write each webhook header as "Name: value". Check "${line.trim()}".`,
			});
			return z.NEVER;
		}
		headers[name] = line.slice(colon + 1).trim();
	}
	return headers;
});

/** Without `pipeline` the form creates a paused pipeline. With it, the form edits that pipeline. */
export const PipelineFormPropsSchema = z.object({
	pipeline: V1GetPipeline200Response.optional(),
});

export type PipelineFormPropsFields = z.infer<typeof PipelineFormPropsSchema>;
