import type { Meta, StoryObj } from "@storybook/react";

import { JobReview } from "./job-review";
import { JobReviewPropsSchema } from "./job-review.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for JobReview. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/lists/JobReview",
	component: JobReview,
	tags: ["autodocs"],
} satisfies Meta<typeof JobReview>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example job-review",
		...JobReviewPropsSchema.parse({ jobId: 9 }),
	},
};
