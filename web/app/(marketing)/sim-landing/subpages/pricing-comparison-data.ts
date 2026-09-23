/** Cell values for the three access paths — mirrors sim.ai comparison transpose. */
export type PricingCellValue = boolean | string;

export type PricingComparisonRow = {
	label: string;
	values: [PricingCellValue, PricingCellValue, PricingCellValue];
};

export type PricingComparisonSection = {
	key: string;
	title?: string;
	rows: PricingComparisonRow[];
};

/** Web · Bulk · Self-hosted API — one shared row order, transposed per card. */
export const PRICING_COMPARISON_SECTIONS: PricingComparisonSection[] = [
	{
		key: "checks",
		title: "Verification",
		rows: [
			{ label: "Syntax, DNS, and MX signals", values: [true, true, true] },
			{ label: "Mailbox result details", values: [true, true, true] },
		],
	},
	{
		key: "workflow",
		title: "Workflow",
		rows: [
			{ label: "Single checks", values: [true, true, true] },
			{ label: "CSV list jobs", values: [false, true, true] },
			{ label: "History", values: [true, true, true] },
		],
	},
	{
		key: "deployment",
		title: "Deployment",
		rows: [
			{ label: "Rust API", values: [false, false, true] },
			{ label: "Own PostgreSQL and RabbitMQ", values: [false, false, true] },
		],
	},
];

/** Transpose shared comparison sections into one plan column for {@link SimPricingCard}. */
export function pricingSectionsForColumn(columnIndex: 0 | 1 | 2) {
	return PRICING_COMPARISON_SECTIONS.map((section) => ({
		key: section.key,
		title: section.title,
		rows: section.rows.map((row) => ({
			label: row.label,
			value: row.values[columnIndex],
		})),
	}));
}
