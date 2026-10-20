import type { Meta, StoryObj } from "@storybook/react";

import { Group } from "@/components/platform/page-kit";

import { ReputationReport } from "./reputation-report";
import { ReputationReportPropsSchema, SAMPLE_REPUTATION_REPORT } from "./reputation-report.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ReputationReport. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/deliverability/ReputationReport",
	component: ReputationReport,
	tags: ["autodocs"],
	decorators: [
		(Story) => (
			<Group>
				<Story />
			</Group>
		),
	],
} satisfies Meta<typeof ReputationReport>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Reputation report",
		...ReputationReportPropsSchema.parse({ report: SAMPLE_REPUTATION_REPORT }),
	},
};

export const NoMailServer: Story = {
	args: {
		"aria-label": "Reputation report without MX records",
		...ReputationReportPropsSchema.parse({
			report: {
				...SAMPLE_REPUTATION_REPORT,
				score: 52,
				risk_level: "medium",
				dns_records: {
					has_spf: false,
					spf_policy_explicit: false,
					has_dkim: null,
					dkim_selectors: [],
					has_dmarc: false,
					dmarc_policy: null,
					has_mx: false,
					mx_records: [],
				},
				domain_info: { domain_age_days: null, registrar: null, created_at: null },
			},
		}),
	},
};
