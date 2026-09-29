import type { Meta, StoryObj } from "@storybook/react";

import { CheckPanel } from "./check-panel";
import { CheckPanelPropsSchema } from "./check-panel.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for CheckPanel. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/check/CheckPanel",
	component: CheckPanel,
	tags: ["autodocs"],
} satisfies Meta<typeof CheckPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example check-panel",
		children: "Content",
		...CheckPanelPropsSchema.parse({}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty check-panel",
	},
};
