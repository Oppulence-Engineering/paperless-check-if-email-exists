/** Cell values for the four plans — mirrors sim.ai comparison transpose. */
export type PricingCellValue = boolean | string;

export type PricingColumn = 0 | 1 | 2 | 3;

export type PricingComparisonRow = {
	label: string;
	values: [PricingCellValue, PricingCellValue, PricingCellValue, PricingCellValue];
};

export type PricingComparisonSection = {
	key: string;
	title?: string;
	rows: PricingComparisonRow[];
};

/** Free · Starter · Professional · Enterprise — one shared row order, transposed per card. */
export const PRICING_COMPARISON_SECTIONS: PricingComparisonSection[] = [
	{
		key: "usage",
		title: "Usage",
		rows: [
			{ label: "Checks each month", values: ["100", "4,000", "12,000", "Custom"] },
			{ label: "Team seats", values: ["3", "10", "50", "Unlimited"] },
		],
	},
	{
		key: "checks",
		title: "Checking",
		rows: [
			{ label: "Single checks and CSV lists", values: [true, true, true, true] },
			{ label: "Find email", values: [true, true, true, true] },
			{ label: "Scores and reason codes", values: [true, true, true, true] },
			{ label: "Catch-all grading", values: [true, true, true, true] },
		],
	},
	{
		key: "hygiene",
		title: "Keeping lists clean",
		rows: [
			{ label: "Suppression list", values: [true, true, true, true] },
			{ label: "Delivery feedback", values: [true, true, true, true] },
			{ label: "Scheduled re-checks", values: [true, true, true, true] },
		],
	},
	{
		key: "team",
		title: "API and team",
		rows: [
			{ label: "REST API and SDKs", values: [false, false, true, true] },
			{ label: "SCIM provisioning", values: [false, false, false, true] },
		],
	},
];

/** Transpose shared comparison sections into one plan column for {@link SimPricingCard}. */
export function pricingSectionsForColumn(columnIndex: PricingColumn) {
	return PRICING_COMPARISON_SECTIONS.map((section) => ({
		key: section.key,
		title: section.title,
		rows: section.rows.map((row) => ({
			label: row.label,
			value: row.values[columnIndex],
		})),
	}));
}
