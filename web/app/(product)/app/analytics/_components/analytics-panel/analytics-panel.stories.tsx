import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { AnalyticsPanel } from "./analytics-panel";
import { AnalyticsPanelPropsSchema } from "./analytics-panel.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for AnalyticsPanel. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/analytics/AnalyticsPanel",
	component: AnalyticsPanel,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof AnalyticsPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example analytics-panel",
		children: "Content",
		...AnalyticsPanelPropsSchema.parse({}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty analytics-panel",
	},
};
