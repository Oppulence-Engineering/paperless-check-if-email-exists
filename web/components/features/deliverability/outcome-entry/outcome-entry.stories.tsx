import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { OutcomeEntry } from "./outcome-entry";
import { OutcomeEntryPropsSchema } from "./outcome-entry.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for OutcomeEntry. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/deliverability/OutcomeEntry",
	component: OutcomeEntry,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof OutcomeEntry>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Outcome entry",
		...OutcomeEntryPropsSchema.parse({}),
	},
};
