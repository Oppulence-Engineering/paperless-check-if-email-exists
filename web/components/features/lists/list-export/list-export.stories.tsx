import type { Meta, StoryObj } from "@storybook/react";

import { ListExport } from "./list-export";
import { ListExportPropsSchema } from "./list-export.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ListExport. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/lists/ListExport",
	component: ListExport,
	tags: ["autodocs"],
} satisfies Meta<typeof ListExport>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example list-export",
		...ListExportPropsSchema.parse({ listId: 3 }),
	},
};
