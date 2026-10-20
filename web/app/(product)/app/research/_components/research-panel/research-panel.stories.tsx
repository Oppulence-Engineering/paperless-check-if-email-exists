import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
const queryClient = new QueryClient({
	defaultOptions: { queries: { enabled: false } },
});

import { ResearchPanel } from "./research-panel";
import { ResearchPanelPropsSchema } from "./research-panel.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ResearchPanel. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/research/ResearchPanel",
	component: ResearchPanel,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={queryClient}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof ResearchPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example research-panel",
		...ResearchPanelPropsSchema.parse({}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty research-panel",
	},
};
