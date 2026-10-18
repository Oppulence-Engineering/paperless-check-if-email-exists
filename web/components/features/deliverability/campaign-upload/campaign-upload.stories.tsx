import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { CampaignUpload } from "./campaign-upload";
import { CampaignUploadPropsSchema } from "./campaign-upload.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for CampaignUpload. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/deliverability/CampaignUpload",
	component: CampaignUpload,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof CampaignUpload>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Campaign upload",
		...CampaignUploadPropsSchema.parse({}),
	},
};
