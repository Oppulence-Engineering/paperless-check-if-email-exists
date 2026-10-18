import type { Meta, StoryObj } from "@storybook/react";

import { AddressTimeline } from "./address-timeline";
import { AddressTimelinePropsSchema } from "./address-timeline.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for AddressTimeline. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/verification/AddressTimeline",
	component: AddressTimeline,
	tags: ["autodocs"],
} satisfies Meta<typeof AddressTimeline>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Address timeline",
		...AddressTimelinePropsSchema.parse({ email: "maya@brightlane.co" }),
	},
};
