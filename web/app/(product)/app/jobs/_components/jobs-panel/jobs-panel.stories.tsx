import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { JobsPanel } from "./jobs-panel";
import { JobsPanelPropsSchema } from "./jobs-panel.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for JobsPanel. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/jobs/JobsPanel",
	component: JobsPanel,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof JobsPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example jobs-panel",
		children: "Content",
		...JobsPanelPropsSchema.parse({}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty jobs-panel",
	},
};
