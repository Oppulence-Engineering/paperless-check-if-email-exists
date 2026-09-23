import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Platform operation runner — Run audited platform API operations.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const PlatformOperationRunnerLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("platform-operation-runner"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const PlatformOperationRunnerLit = PlatformOperationRunnerLitSchema.parse({
	kind: "component",
	name: "platform-operation-runner",
	domain: "",
	owner: "route",
	client: true,
	summary: "Run audited platform API operations.",
	schemas: [
		"app/(product)/app/admin/api/_components/platform-operation-runner/platform-operation-runner.schema.ts",
	],
	files: [
		"app/(product)/app/admin/api/_components/platform-operation-runner/platform-operation-runner.tsx",
		"app/(product)/app/admin/api/_components/platform-operation-runner/platform-operation-runner.test.tsx",
		"app/(product)/app/admin/api/_components/platform-operation-runner/platform-operation-runner.schema.ts",
		"app/(product)/app/admin/api/_components/platform-operation-runner/platform-operation-runner.schema.test.ts",
		"app/(product)/app/admin/api/_components/platform-operation-runner/platform-operation-runner.lit.ts",
		"app/(product)/app/admin/api/_components/platform-operation-runner/platform-operation-runner.stories.tsx",
	],
});
