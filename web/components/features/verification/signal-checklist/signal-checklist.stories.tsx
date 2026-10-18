import type { Meta, StoryObj } from "@storybook/react";

import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";

import { SignalChecklist } from "./signal-checklist";
import { SignalChecklistPropsSchema } from "./signal-checklist.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for SignalChecklist. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/verification/SignalChecklist",
	component: SignalChecklist,
	tags: ["autodocs"],
} satisfies Meta<typeof SignalChecklist>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Signal checklist",
		...SignalChecklistPropsSchema.parse({ result: SAMPLE_CHECK_RESULT }),
	},
};
