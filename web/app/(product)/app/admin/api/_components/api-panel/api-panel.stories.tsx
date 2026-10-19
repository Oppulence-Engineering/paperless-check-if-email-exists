import type { Meta, StoryObj } from "@storybook/react";

import { ApiPanel } from "./api-panel";
import { ApiPanelPropsSchema } from "./api-panel.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ApiPanel. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/admin/api/ApiPanel",
	component: ApiPanel,
	tags: ["autodocs"],
} satisfies Meta<typeof ApiPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example api-panel",
		children: "Content",
		...ApiPanelPropsSchema.parse({}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty api-panel",
	},
};
