import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Accept invitation — Authenticated confirmation before accepting an organization invitation.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const AcceptLitSchema = z.object({
	kind: z.literal("page"),
	name: z.literal("accept"),
	domain: z.literal("accept"),
	owner: z.literal("page"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const AcceptLit = AcceptLitSchema.parse({
	kind: "page",
	name: "accept",
	domain: "accept",
	owner: "page",
	client: false,
	summary: "Authenticated confirmation before accepting an organization invitation.",
	schemas: ["app/(product)/app/invitations/accept/search-params.ts"],
	files: [
		"app/(product)/app/invitations/accept/accept.lit.ts",
		"app/(product)/app/invitations/accept/page.tsx",
		"app/(product)/app/invitations/accept/loading.tsx",
		"app/(product)/app/invitations/accept/error.tsx",
		"app/(product)/app/invitations/accept/search-params.ts",
	],
});
