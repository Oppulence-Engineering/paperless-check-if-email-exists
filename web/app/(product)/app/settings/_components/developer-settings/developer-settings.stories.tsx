import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { DeveloperSettings } from "./developer-settings";
import { DeveloperSettingsPropsSchema } from "./developer-settings.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for DeveloperSettings. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/settings/DeveloperSettings",
	component: DeveloperSettings,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof DeveloperSettings>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: DeveloperSettingsPropsSchema.parse({ organizationId: "org-1", organizationRole: "member" }),
};
