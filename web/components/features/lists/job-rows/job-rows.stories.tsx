import type { Meta, StoryObj } from "@storybook/react";

import { JobRows } from "./job-rows";
import { JobRowsPropsSchema } from "./job-rows.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for JobRows. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/lists/JobRows",
	component: JobRows,
	tags: ["autodocs"],
} satisfies Meta<typeof JobRows>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example job-rows",
		...JobRowsPropsSchema.parse({ jobId: 9 }),
	},
};
