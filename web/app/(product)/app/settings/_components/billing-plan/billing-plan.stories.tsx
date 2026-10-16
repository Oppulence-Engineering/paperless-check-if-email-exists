import type { Meta, StoryObj } from "@storybook/react";

import { BillingPlan } from "./billing-plan";
import { BillingPlanPropsSchema } from "./billing-plan.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for BillingPlan. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/settings/BillingPlan",
	component: BillingPlan,
	tags: ["autodocs"],
} satisfies Meta<typeof BillingPlan>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Free: Story = {
	args: BillingPlanPropsSchema.parse({ plan: "free", billingEnabled: true, canManage: true }),
};

export const Starter: Story = {
	args: BillingPlanPropsSchema.parse({ plan: "starter", billingEnabled: true, canManage: true }),
};

export const Member: Story = {
	args: BillingPlanPropsSchema.parse({ plan: "free", billingEnabled: true, canManage: false }),
};

export const BillingOff: Story = {
	args: BillingPlanPropsSchema.parse({ plan: "free", billingEnabled: false, canManage: true }),
};
