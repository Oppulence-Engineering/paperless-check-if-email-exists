import type { Meta, StoryObj } from "@storybook/react";

import { JobProblems } from "./job-problems";
import { JobProblemsPropsSchema } from "./job-problems.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for JobProblems. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/lists/JobProblems",
	component: JobProblems,
	tags: ["autodocs"],
} satisfies Meta<typeof JobProblems>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example job-problems",
		...JobProblemsPropsSchema.parse({ jobId: 9 }),
	},
};
