import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { SuppressionAudit } from "./suppression-audit";
import { SuppressionAuditPropsSchema } from "./suppression-audit.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for SuppressionAudit. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/deliverability/SuppressionAudit",
	component: SuppressionAudit,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<QueryClientProvider client={new QueryClient()}>
				<Story />
			</QueryClientProvider>
		),
	],
} satisfies Meta<typeof SuppressionAudit>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Audit timeline",
		...SuppressionAuditPropsSchema.parse({ entryId: 3 }),
	},
};
