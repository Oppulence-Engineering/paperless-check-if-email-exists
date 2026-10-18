import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Integration stack — Shows each part of the stack that a workspace can connect and whether it is set up.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const IntegrationStackLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("integration-stack"),
	domain: z.literal("automation"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const IntegrationStackLit = IntegrationStackLitSchema.parse({
	kind: "component",
	name: "integration-stack",
	domain: "automation",
	owner: "feature",
	client: true,
	summary: "Shows each part of the stack that a workspace can connect and whether it is set up.",
	schemas: ["components/features/automation/integration-stack/integration-stack.schema.ts"],
	files: [
		"components/features/automation/integration-stack/integration-stack.tsx",
		"components/features/automation/integration-stack/integration-stack.test.tsx",
		"components/features/automation/integration-stack/integration-stack.schema.ts",
		"components/features/automation/integration-stack/integration-stack.schema.test.ts",
		"components/features/automation/integration-stack/integration-stack.lit.ts",
		"components/features/automation/integration-stack/integration-stack.stories.tsx",
	],
});
