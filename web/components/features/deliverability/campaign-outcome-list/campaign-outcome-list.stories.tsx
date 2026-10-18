import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { CampaignOutcomeList } from "./campaign-outcome-list";
import { CampaignOutcomeListPropsSchema } from "./campaign-outcome-list.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for CampaignOutcomeList. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/deliverability/CampaignOutcomeList",
	component: CampaignOutcomeList,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof CampaignOutcomeList>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Campaign outcome list",
		...CampaignOutcomeListPropsSchema.parse({}),
	},
};
