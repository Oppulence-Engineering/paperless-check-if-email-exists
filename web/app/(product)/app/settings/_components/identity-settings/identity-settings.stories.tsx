import type { Meta, StoryObj } from "@storybook/react";

import { IdentitySettings } from "./identity-settings";
import { IdentitySettingsPropsSchema } from "./identity-settings.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for IdentitySettings. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/settings/IdentitySettings",
	component: IdentitySettings,
	tags: ["autodocs"],
} satisfies Meta<typeof IdentitySettings>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example identity-settings",
		...IdentitySettingsPropsSchema.parse({
			organizationId: "org_1",
			organizationRole: "owner",
			userId: "user_1",
		}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty identity-settings",
		organizationId: "org_1",
		organizationRole: "member",
		userId: "user_1",
	},
};
