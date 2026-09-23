import type { Meta, StoryObj } from "@storybook/react";

import { PlatformOperationRunner } from "./platform-operation-runner";
import { PlatformOperationRunnerPropsSchema } from "./platform-operation-runner.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for PlatformOperationRunner. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/admin/api/PlatformOperationRunner",
	component: PlatformOperationRunner,
	tags: ["autodocs"],
} satisfies Meta<typeof PlatformOperationRunner>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example platform-operation-runner",
		children: "Content",
		...PlatformOperationRunnerPropsSchema.parse({
			operations: [
				{ id: "list_tenants", method: "GET", path: "/v1/admin/tenants", requestBody: false },
			],
		}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty platform-operation-runner",
		operations: [],
	},
};
