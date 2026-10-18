import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { PipelineDetail } from "./pipeline-detail";
import { PipelineDetailPropsSchema } from "./pipeline-detail.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for PipelineDetail. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/automation/PipelineDetail",
	component: PipelineDetail,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof PipelineDetail>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Pipeline detail",
		...PipelineDetailPropsSchema.parse({ pipelineId: 1 }),
	},
};
