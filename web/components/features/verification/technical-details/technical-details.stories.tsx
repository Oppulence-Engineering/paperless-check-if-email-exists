import type { Meta, StoryObj } from "@storybook/react";

import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";

import { TechnicalDetails } from "./technical-details";
import { TechnicalDetailsPropsSchema } from "./technical-details.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for TechnicalDetails. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/verification/TechnicalDetails",
	component: TechnicalDetails,
	tags: ["autodocs"],
} satisfies Meta<typeof TechnicalDetails>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Technical details",
		...TechnicalDetailsPropsSchema.parse({ result: SAMPLE_CHECK_RESULT }),
	},
};
