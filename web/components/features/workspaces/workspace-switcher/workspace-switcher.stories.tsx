import type { Meta, StoryObj } from "@storybook/react";

import { WorkspaceSwitcher } from "./workspace-switcher";
import { WorkspaceSwitcherPropsSchema } from "./workspace-switcher.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for WorkspaceSwitcher. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/workspaces/WorkspaceSwitcher",
	component: WorkspaceSwitcher,
	tags: ["autodocs"],
} satisfies Meta<typeof WorkspaceSwitcher>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Workspace switcher",
		...WorkspaceSwitcherPropsSchema.parse({
			activeWorkspaceId: "workspace-1",
			planLabel: "Pro",
			workspaces: [
				{
					id: "workspace-1",
					name: "Northstar Freight",
					slug: "northstar-freight",
					role: "owner",
					logoUrl: null,
				},
				{
					id: "workspace-2",
					name: "Juniper Health",
					slug: "juniper-health",
					role: "member",
					logoUrl: null,
				},
			],
		}),
	},
};

export const SingleWorkspace: Story = {
	args: {
		"aria-label": "Single workspace",
		...WorkspaceSwitcherPropsSchema.parse({
			activeWorkspaceId: "workspace-1",
			workspaces: [
				{
					id: "workspace-1",
					name: "Northstar Freight",
					slug: "northstar-freight",
					role: "owner",
					logoUrl: null,
				},
			],
		}),
	},
};
