import type { Meta, StoryObj } from "@storybook/react";

import { HistoryPanel } from "./history-panel";
import { HistoryPanelPropsSchema } from "./history-panel.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for HistoryPanel. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/app/history/HistoryPanel",
	component: HistoryPanel,
	tags: ["autodocs"],
} satisfies Meta<typeof HistoryPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example history-panel",
		children: "Content",
		...HistoryPanelPropsSchema.parse({}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty history-panel",
	},
};
