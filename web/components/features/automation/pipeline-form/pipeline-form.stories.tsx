import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { PipelineForm } from "./pipeline-form";
import { PipelineFormPropsSchema } from "./pipeline-form.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for PipelineForm. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/automation/PipelineForm",
	component: PipelineForm,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof PipelineForm>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Pipeline form",
		...PipelineFormPropsSchema.parse({}),
	},
};
