import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { ProviderEndpointList } from "./provider-endpoint-list";
import { ProviderEndpointListPropsSchema } from "./provider-endpoint-list.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ProviderEndpointList. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/deliverability/ProviderEndpointList",
	component: ProviderEndpointList,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof ProviderEndpointList>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Provider endpoint list",
		...ProviderEndpointListPropsSchema.parse({}),
	},
};
