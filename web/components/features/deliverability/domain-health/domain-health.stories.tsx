import type { Meta, StoryObj } from "@storybook/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { SAMPLE_REPUTATION_REPORT } from "@/components/features/deliverability/reputation-report/reputation-report.schema";
import { Group } from "@/components/platform/page-kit";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { DomainHealth } from "./domain-health";
import { DomainHealthPropsSchema } from "./domain-health.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for DomainHealth. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest. The query cache holds a sample
 * result, so the story does not call the backend.
 */
const meta = {
	title: "features/deliverability/DomainHealth",
	component: DomainHealth,
	tags: ["autodocs"],
	decorators: [
		(Story) => {
			const client = new QueryClient({ defaultOptions: { queries: { staleTime: Infinity } } });
			client.setQueryData(
				workflowQueryKey("domain-reputation", "example.com"),
				SAMPLE_REPUTATION_REPORT,
			);
			return (
				<QueryClientProvider client={client}>
					<Group>
						<Story />
					</Group>
				</QueryClientProvider>
			);
		},
	],
} satisfies Meta<typeof DomainHealth>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Health of example.com",
		...DomainHealthPropsSchema.parse({ domain: "example.com" }),
	},
};
