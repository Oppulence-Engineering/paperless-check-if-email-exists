import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { VerificationSettings } from "./verification-settings";
import { VerificationSettingsPropsSchema } from "./verification-settings.schema";

const queryClient = new QueryClient();

/**
 * @oppulence-gen kind=component
 * Visual contract for VerificationSettings. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/settings/VerificationSettings",
	component: VerificationSettings,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={queryClient}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof VerificationSettings>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example verification-settings",
		...VerificationSettingsPropsSchema.parse({
			organizationId: "example-org",
			organizationRole: "owner",
		}),
	},
};

export const Member: Story = {
	args: {
		"aria-label": "Member verification-settings",
		organizationId: "example-org",
		organizationRole: "member",
	},
};
