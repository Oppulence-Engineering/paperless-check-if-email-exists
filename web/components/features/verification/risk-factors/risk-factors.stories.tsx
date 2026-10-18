import type { Meta, StoryObj } from "@storybook/react";

import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";

import { RiskFactors } from "./risk-factors";
import { RiskFactorsPropsSchema } from "./risk-factors.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for RiskFactors. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/verification/RiskFactors",
	component: RiskFactors,
	tags: ["autodocs"],
} satisfies Meta<typeof RiskFactors>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Risk factors",
		...RiskFactorsPropsSchema.parse({ result: SAMPLE_CHECK_RESULT }),
	},
};
