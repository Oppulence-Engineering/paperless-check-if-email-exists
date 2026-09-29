import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Accept panel — Accept invitation route-private panel. Presentation only.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const AcceptPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("accept-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const AcceptPanelLit = AcceptPanelLitSchema.parse({
	kind: "component",
	name: "accept-panel",
	domain: "",
	owner: "route",
	client: false,
	summary: "Accept invitation route-private panel. Presentation only.",
	schemas: ["app/(product)/app/invitations/accept/_components/accept-panel/accept-panel.schema.ts"],
	files: [
		"app/(product)/app/invitations/accept/_components/accept-panel/accept-panel.tsx",
		"app/(product)/app/invitations/accept/_components/accept-panel/accept-panel.test.tsx",
		"app/(product)/app/invitations/accept/_components/accept-panel/accept-panel.schema.ts",
		"app/(product)/app/invitations/accept/_components/accept-panel/accept-panel.schema.test.ts",
		"app/(product)/app/invitations/accept/_components/accept-panel/accept-panel.lit.ts",
		"app/(product)/app/invitations/accept/_components/accept-panel/accept-panel.stories.tsx",
	],
});
