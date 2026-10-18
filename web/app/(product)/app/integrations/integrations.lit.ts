import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Connect your stack — See which parts of your stack are connected and open the page that sets up each one
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const IntegrationsLitSchema = z.object({
	kind: z.literal("page"),
	name: z.literal("integrations"),
	domain: z.literal("integrations"),
	owner: z.literal("page"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const IntegrationsLit = IntegrationsLitSchema.parse({
	kind: "page",
	name: "integrations",
	domain: "integrations",
	owner: "page",
	client: false,
	summary: "See which parts of your stack are connected and open the page that sets up each one",
	schemas: ["app/(product)/app/integrations/search-params.ts"],
	files: [
		"app/(product)/app/integrations/integrations.lit.ts",
		"app/(product)/app/integrations/page.tsx",
		"app/(product)/app/integrations/loading.tsx",
		"app/(product)/app/integrations/error.tsx",
		"app/(product)/app/integrations/search-params.ts",
	],
});
