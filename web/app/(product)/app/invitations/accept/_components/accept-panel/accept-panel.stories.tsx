import type { Meta, StoryObj } from "@storybook/react";

import { AcceptPanel } from "./accept-panel";
import { AcceptPanelPropsSchema } from "./accept-panel.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for AcceptPanel. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/invitations/accept/AcceptPanel",
	component: AcceptPanel,
	tags: ["autodocs"],
} satisfies Meta<typeof AcceptPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example accept-panel",
		...AcceptPanelPropsSchema.parse({
			invitationId: "invite-1",
			returnTo: "/app/settings?settings=identity",
		}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty accept-panel",
		invitationId: "invite-2",
		returnTo: "/app",
	},
};
