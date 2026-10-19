import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { SuppressionForm } from "./suppression-form";
import { SuppressionFormPropsSchema } from "./suppression-form.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for SuppressionForm. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/deliverability/SuppressionForm",
	component: SuppressionForm,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof SuppressionForm>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Add: Story = {
	args: {
		"aria-label": "Add addresses",
		...SuppressionFormPropsSchema.parse({ mode: "add" }),
	},
};

export const Import: Story = {
	args: {
		"aria-label": "Import a CSV file",
		...SuppressionFormPropsSchema.parse({ mode: "import" }),
	},
};
