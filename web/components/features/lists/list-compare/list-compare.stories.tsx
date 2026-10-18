import type { Meta, StoryObj } from "@storybook/react";

import { ListCompare } from "./list-compare";
import { ListComparePropsSchema } from "./list-compare.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ListCompare. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/lists/ListCompare",
	component: ListCompare,
	tags: ["autodocs"],
} satisfies Meta<typeof ListCompare>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example list-compare",
		...ListComparePropsSchema.parse({ listId: 3 }),
	},
};
