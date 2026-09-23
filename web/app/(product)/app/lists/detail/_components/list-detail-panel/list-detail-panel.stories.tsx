import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { ListDetailPanel } from "./list-detail-panel";
import { ListDetailPanelPropsSchema } from "./list-detail-panel.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ListDetailPanel. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/lists/detail/ListDetailPanel",
	component: ListDetailPanel,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof ListDetailPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example list-detail-panel",
		children: "Content",
		...ListDetailPanelPropsSchema.parse({}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty list-detail-panel",
	},
};
