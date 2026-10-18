import type { Meta, StoryObj } from "@storybook/react";

import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";

import { VerdictSummary } from "./verdict-summary";
import { VerdictSummaryPropsSchema } from "./verdict-summary.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for VerdictSummary. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/verification/VerdictSummary",
	component: VerdictSummary,
	tags: ["autodocs"],
} satisfies Meta<typeof VerdictSummary>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Verdict",
		...VerdictSummaryPropsSchema.parse({ result: SAMPLE_CHECK_RESULT }),
	},
};

export const DoNotSend: Story = {
	args: {
		"aria-label": "Verdict",
		...VerdictSummaryPropsSchema.parse({
			result: {
				...SAMPLE_CHECK_RESULT,
				recommendation: undefined,
				bounce_risk: null,
				is_reachable: "invalid",
				score: {
					...SAMPLE_CHECK_RESULT.score,
					score: 0,
					category: "invalid",
					safe_to_send: false,
					reason_codes: ["invalid_recipient"],
				},
			},
		}),
	},
};
