import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { OutcomesPanel } from "./outcomes-panel";
import { OutcomesPanelPropsSchema } from "./outcomes-panel.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for OutcomesPanel. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/outcomes/OutcomesPanel",
	component: OutcomesPanel,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof OutcomesPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example outcomes-panel",
		children: "Content",
		...OutcomesPanelPropsSchema.parse({}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty outcomes-panel",
	},
};
