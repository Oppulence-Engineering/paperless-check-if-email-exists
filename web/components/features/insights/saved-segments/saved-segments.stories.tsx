import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { SavedSegments } from "./saved-segments";
import { SavedSegmentsPropsSchema } from "./saved-segments.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for SavedSegments. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/insights/SavedSegments",
	component: SavedSegments,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof SavedSegments>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example saved-segments",
		onOpen: () => undefined,
		...SavedSegmentsPropsSchema.parse({ filters: { category: "valid", jobId: 18 } }),
	},
};
