import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Job rows — Shows the result of each row in one bulk job and opens one row in a sheet.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const JobRowsLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("job-rows"),
	domain: z.literal("lists"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const JobRowsLit = JobRowsLitSchema.parse({
	kind: "component",
	name: "job-rows",
	domain: "lists",
	owner: "feature",
	client: true,
	summary: "Shows the result of each row in one bulk job and opens one row in a sheet.",
	schemas: ["components/features/lists/job-rows/job-rows.schema.ts"],
	files: [
		"components/features/lists/job-rows/job-rows.tsx",
		"components/features/lists/job-rows/job-rows.test.tsx",
		"components/features/lists/job-rows/job-rows.schema.ts",
		"components/features/lists/job-rows/job-rows.schema.test.ts",
		"components/features/lists/job-rows/job-rows.lit.ts",
		"components/features/lists/job-rows/job-rows.stories.tsx",
	],
});
