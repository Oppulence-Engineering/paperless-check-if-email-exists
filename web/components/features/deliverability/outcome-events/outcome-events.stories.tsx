import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { OutcomeEvents } from "./outcome-events";
import { OutcomeEventsPropsSchema } from "./outcome-events.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for OutcomeEvents. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/deliverability/OutcomeEvents",
	component: OutcomeEvents,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof OutcomeEvents>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Outcome events",
		...OutcomeEventsPropsSchema.parse({}),
	},
};
