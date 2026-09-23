import type { Meta, StoryObj } from "@storybook/react";

import { WorkspaceUrlScope } from "./workspace-url-scope";
import { WorkspaceUrlScopePropsSchema } from "./workspace-url-scope.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for WorkspaceUrlScope. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/workspaces/WorkspaceUrlScope",
	component: WorkspaceUrlScope,
	tags: ["autodocs"],
} satisfies Meta<typeof WorkspaceUrlScope>;

export default meta;

type Story = StoryObj<typeof meta>;

const workspaces = [
	{ id: "org_1", name: "Acme", slug: "acme", role: "owner", logoUrl: null },
	{ id: "org_2", name: "Northwind", slug: "northwind", role: "member", logoUrl: null },
];

export const Default: Story = {
	args: {
		...WorkspaceUrlScopePropsSchema.parse({ workspaces, activeWorkspaceId: "org_1" }),
	},
};

export const Empty: Story = {
	args: {
		...WorkspaceUrlScopePropsSchema.parse({ workspaces: [], activeWorkspaceId: "org_1" }),
	},
};
