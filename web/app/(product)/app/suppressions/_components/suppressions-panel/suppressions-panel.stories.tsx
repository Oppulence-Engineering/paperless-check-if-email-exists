import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { SuppressionsPanel } from "./suppressions-panel";
import { SuppressionsPanelPropsSchema } from "./suppressions-panel.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for SuppressionsPanel. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/suppressions/SuppressionsPanel",
	component: SuppressionsPanel,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof SuppressionsPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example suppressions-panel",
		children: "Content",
		...SuppressionsPanelPropsSchema.parse({}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty suppressions-panel",
	},
};
