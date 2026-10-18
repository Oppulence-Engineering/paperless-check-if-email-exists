import type { Meta, StoryObj } from "@storybook/react";

import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";

import { ProviderDetails } from "./provider-details";
import { ProviderDetailsPropsSchema } from "./provider-details.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ProviderDetails. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/verification/ProviderDetails",
	component: ProviderDetails,
	tags: ["autodocs"],
} satisfies Meta<typeof ProviderDetails>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Provider details",
		...ProviderDetailsPropsSchema.parse({ result: SAMPLE_CHECK_RESULT }),
	},
};
