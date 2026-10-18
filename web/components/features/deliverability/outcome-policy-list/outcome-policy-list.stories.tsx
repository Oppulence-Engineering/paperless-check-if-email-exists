import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { OutcomePolicyList } from "./outcome-policy-list";
import { OutcomePolicyListPropsSchema } from "./outcome-policy-list.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for OutcomePolicyList. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/deliverability/OutcomePolicyList",
	component: OutcomePolicyList,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof OutcomePolicyList>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Outcome policy list",
		...OutcomePolicyListPropsSchema.parse({}),
	},
};
