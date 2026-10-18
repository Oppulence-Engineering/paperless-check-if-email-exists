import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { ScorePolicyList } from "./score-policy-list";
import { ScorePolicyListPropsSchema } from "./score-policy-list.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ScorePolicyList. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/automation/ScorePolicyList",
	component: ScorePolicyList,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof ScorePolicyList>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Score policy list",
		...ScorePolicyListPropsSchema.parse({}),
	},
};
