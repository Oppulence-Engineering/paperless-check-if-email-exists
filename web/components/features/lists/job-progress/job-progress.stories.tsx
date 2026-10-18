import type { Meta, StoryObj } from "@storybook/react";

import { JobProgress } from "./job-progress";
import { JobProgressPropsSchema } from "./job-progress.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for JobProgress. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/lists/JobProgress",
	component: JobProgress,
	tags: ["autodocs"],
} satisfies Meta<typeof JobProgress>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example job-progress",
		...JobProgressPropsSchema.parse({ jobId: 9 }),
	},
};
