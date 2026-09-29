import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Operation runner — Run a workspace API operation through the session BFF.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const OperationRunnerLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("operation-runner"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const OperationRunnerLit = OperationRunnerLitSchema.parse({
	kind: "component",
	name: "operation-runner",
	domain: "",
	owner: "route",
	client: true,
	summary: "Run a workspace API operation through the session BFF.",
	schemas: ["app/(product)/app/api/_components/operation-runner/operation-runner.schema.ts"],
	files: [
		"app/(product)/app/api/_components/operation-runner/operation-runner.tsx",
		"app/(product)/app/api/_components/operation-runner/operation-runner.test.tsx",
		"app/(product)/app/api/_components/operation-runner/operation-runner.schema.ts",
		"app/(product)/app/api/_components/operation-runner/operation-runner.schema.test.ts",
		"app/(product)/app/api/_components/operation-runner/operation-runner.lit.ts",
		"app/(product)/app/api/_components/operation-runner/operation-runner.stories.tsx",
	],
});
