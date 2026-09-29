import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { PipelinesPanel } from "./pipelines-panel";
import { PipelinesPanelPropsSchema } from "./pipelines-panel.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for PipelinesPanel. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/pipelines/PipelinesPanel",
	component: PipelinesPanel,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof PipelinesPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example pipelines-panel",
		children: "Content",
		...PipelinesPanelPropsSchema.parse({}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty pipelines-panel",
	},
};
