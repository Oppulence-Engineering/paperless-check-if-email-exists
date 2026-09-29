import type { Meta, StoryObj } from "@storybook/react";

import { ApiExplorerPanel } from "./api-explorer-panel";
import { ApiExplorerPanelPropsSchema } from "./api-explorer-panel.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ApiExplorerPanel. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/api/ApiExplorerPanel",
	component: ApiExplorerPanel,
	tags: ["autodocs"],
} satisfies Meta<typeof ApiExplorerPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example api-explorer-panel",
		children: "Content",
		...ApiExplorerPanelPropsSchema.parse({}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty api-explorer-panel",
	},
};
