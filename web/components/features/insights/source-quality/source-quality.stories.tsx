import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { SourceQuality } from "./source-quality";
import { SourceQualityPropsSchema } from "./source-quality.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for SourceQuality. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/insights/SourceQuality",
	component: SourceQuality,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof SourceQuality>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example source-quality",
		...SourceQualityPropsSchema.parse({}),
	},
};
