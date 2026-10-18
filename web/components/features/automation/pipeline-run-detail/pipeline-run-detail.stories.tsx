import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { PipelineRunDetail } from "./pipeline-run-detail";
import { PipelineRunDetailPropsSchema } from "./pipeline-run-detail.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for PipelineRunDetail. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/automation/PipelineRunDetail",
	component: PipelineRunDetail,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof PipelineRunDetail>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Pipeline run detail",
		...PipelineRunDetailPropsSchema.parse({ pipelineId: 1, runId: 1 }),
	},
};
