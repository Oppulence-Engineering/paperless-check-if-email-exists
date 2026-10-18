import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { PipelineList } from "./pipeline-list";
import { PipelineListPropsSchema } from "./pipeline-list.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for PipelineList. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/automation/PipelineList",
	component: PipelineList,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof PipelineList>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Pipeline list",
		...PipelineListPropsSchema.parse({}),
	},
};
