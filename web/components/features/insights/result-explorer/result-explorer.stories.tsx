import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { ResultExplorer } from "./result-explorer";
import { ResultExplorerPropsSchema } from "./result-explorer.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ResultExplorer. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/insights/ResultExplorer",
	component: ResultExplorer,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof ResultExplorer>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example result-explorer",
		onFiltersChange: () => undefined,
		...ResultExplorerPropsSchema.parse({ filters: {} }),
	},
};
