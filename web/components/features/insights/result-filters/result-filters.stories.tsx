import type { Meta, StoryObj } from "@storybook/react";

import { ResultFilters } from "./result-filters";
import { ResultFiltersPropsSchema } from "./result-filters.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ResultFilters. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/insights/ResultFilters",
	component: ResultFilters,
	tags: ["autodocs"],
} satisfies Meta<typeof ResultFilters>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example result-filters",
		onApply: () => undefined,
		...ResultFiltersPropsSchema.parse({ filters: { minScore: 10, policyMode: "growth" } }),
	},
};
