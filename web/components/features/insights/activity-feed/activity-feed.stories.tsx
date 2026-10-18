import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { ActivityFeed } from "./activity-feed";
import { ActivityFeedPropsSchema } from "./activity-feed.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ActivityFeed. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/insights/ActivityFeed",
	component: ActivityFeed,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof ActivityFeed>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example activity-feed",
		...ActivityFeedPropsSchema.parse({}),
	},
};
