import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Integration contract: Canonical backend request, response, pagination, retry, concurrency, and upload contracts.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const IntegrationContractLitSchema = z.object({
	kind: z.literal("lib"),
	name: z.literal("integration-contract"),
	domain: z.literal("backend"),
	owner: z.literal("lib"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const IntegrationContractLit = IntegrationContractLitSchema.parse({
	kind: "lib",
	name: "integration-contract",
	domain: "backend",
	owner: "lib",
	client: false,
	summary:
		"Canonical backend request, response, pagination, retry, concurrency, and upload contracts.",
	schemas: ["lib/backend/integration-contract.schema.ts"],
	files: [
		"lib/backend/integration-contract.ts",
		"lib/backend/integration-contract.schema.ts",
		"lib/backend/integration-contract.schema.test.ts",
		"lib/backend/integration-contract.test.ts",
		"lib/backend/integration-contract.lit.ts",
	],
});
