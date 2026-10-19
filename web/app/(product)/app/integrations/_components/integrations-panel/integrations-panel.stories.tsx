import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { IntegrationsPanel } from "./integrations-panel";
import { IntegrationsPanelPropsSchema } from "./integrations-panel.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for IntegrationsPanel. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/integrations/IntegrationsPanel",
	component: IntegrationsPanel,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof IntegrationsPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example integrations-panel",
		children: "Content",
		...IntegrationsPanelPropsSchema.parse({}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty integrations-panel",
	},
};
