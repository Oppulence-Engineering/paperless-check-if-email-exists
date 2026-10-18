import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { IntegrationStack } from "./integration-stack";
import { IntegrationStackPropsSchema } from "./integration-stack.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for IntegrationStack. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/automation/IntegrationStack",
	component: IntegrationStack,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof IntegrationStack>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Integration stack",
		...IntegrationStackPropsSchema.parse({}),
	},
};
