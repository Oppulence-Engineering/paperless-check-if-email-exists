import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { ResultDetail } from "./result-detail";
import { ResultDetailPropsSchema } from "./result-detail.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ResultDetail. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/insights/ResultDetail",
	component: ResultDetail,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof ResultDetail>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example result-detail",
		...ResultDetailPropsSchema.parse({
			row: {
				id: 7,
				job_id: null,
				email: "maya@brightlane.co",
				task_state: "completed",
				created_at: "2026-10-18T05:28:53Z",
				recommendation_action: "review",
				policy_mode: "deliverability",
				policy_decision: "review",
			},
		}),
	},
};
