import type { Meta, StoryObj } from "@storybook/react";

import { ListsPanel } from "./lists-panel";
import { ListsPanelPropsSchema } from "./lists-panel.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ListsPanel. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/app/lists/ListsPanel",
	component: ListsPanel,
	tags: ["autodocs"],
} satisfies Meta<typeof ListsPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example lists-panel",
		children: "Content",
		...ListsPanelPropsSchema.parse({}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty lists-panel",
	},
};
