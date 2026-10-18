import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { PipelineRuns } from "./pipeline-runs";
import { PipelineRunsPropsSchema } from "./pipeline-runs.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for PipelineRuns. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/automation/PipelineRuns",
	component: PipelineRuns,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof PipelineRuns>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Pipeline runs",
		...PipelineRunsPropsSchema.parse({ pipelineId: 1 }),
	},
};
