import type { Meta, StoryObj } from "@storybook/react";

import { SettingsDashboardRoute } from "./settings-dashboard-route";
import { SettingsDashboardRoutePropsSchema } from "./settings-dashboard-route.schema";

/** @oppulence-gen kind=component */
const meta = {
	title: "routes/settings/SettingsDashboardRoute",
	component: SettingsDashboardRoute,
	tags: ["autodocs"],
} satisfies Meta<typeof SettingsDashboardRoute>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: SettingsDashboardRoutePropsSchema.parse({
		section: "overview",
		organizationId: "org-1",
		organizationRole: "owner",
		userId: "user-1",
		userName: "Owner",
		userEmail: "owner@example.com",
	}),
};
