import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { ScorePolicyForm } from "./score-policy-form";
import { ScorePolicyFormPropsSchema } from "./score-policy-form.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ScorePolicyForm. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/automation/ScorePolicyForm",
	component: ScorePolicyForm,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof ScorePolicyForm>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Create: Story = {
	args: {
		"aria-label": "Score policy form",
		...ScorePolicyFormPropsSchema.parse({}),
	},
};

export const Edit: Story = {
	args: {
		"aria-label": "Score policy form",
		...ScorePolicyFormPropsSchema.parse({
			policy: {
				id: 1,
				name: "strict",
				is_default: true,
				rules: { suppress: { score_max: 30 }, send: { score_min: 80, safe_to_send: true } },
				created_at: "2026-10-18T05:28:23.249217Z",
				updated_at: "2026-10-18T05:28:23.249217Z",
			},
		}),
	},
};
