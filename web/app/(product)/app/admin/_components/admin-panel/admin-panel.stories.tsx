import type { Meta, StoryObj } from "@storybook/react";

import { AdminPanel } from "./admin-panel";
import { AdminPanelPropsSchema } from "./admin-panel.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for AdminPanel. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/admin/AdminPanel",
	component: AdminPanel,
	tags: ["autodocs"],
} satisfies Meta<typeof AdminPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

const tenants = [
	{
		id: "org_1",
		name: "Acme Operations",
		slug: "acme-operations",
		createdAt: new Date("2026-02-01T00:00:00.000Z"),
		archivedAt: null,
		memberCount: 4,
	},
	{
		id: "org_2",
		name: "Northwind",
		slug: "northwind",
		createdAt: new Date("2026-03-11T00:00:00.000Z"),
		archivedAt: null,
		memberCount: 1,
	},
];

export const Default: Story = {
	args: { "aria-label": "Tenants", ...AdminPanelPropsSchema.parse({ tenants }) },
};

export const Empty: Story = {
	args: { "aria-label": "Tenants", ...AdminPanelPropsSchema.parse({ tenants: [] }) },
};
