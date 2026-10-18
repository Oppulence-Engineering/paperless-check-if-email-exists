import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { OutcomePolicyForm } from "./outcome-policy-form";
import { OutcomePolicyFormPropsSchema } from "./outcome-policy-form.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for OutcomePolicyForm. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/deliverability/OutcomePolicyForm",
	component: OutcomePolicyForm,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof OutcomePolicyForm>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Outcome policy form",
		...OutcomePolicyFormPropsSchema.parse({}),
	},
};
