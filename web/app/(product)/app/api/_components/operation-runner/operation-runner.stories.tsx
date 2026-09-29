import type { Meta, StoryObj } from "@storybook/react";

import { OperationRunner } from "./operation-runner";
import { OperationRunnerPropsSchema } from "./operation-runner.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for OperationRunner. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "routes/api/OperationRunner",
	component: OperationRunner,
	tags: ["autodocs"],
} satisfies Meta<typeof OperationRunner>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
	args: {
		"aria-label": "Example operation-runner",
		children: "Content",
		...OperationRunnerPropsSchema.parse({
			operations: [
				{
					id: "v1_check_email",
					method: "POST",
					path: "/v1/check_email",
					family: "Verification and finder",
					scope: "verify",
					requestMedia: ["application/json"],
				},
			],
		}),
	},
};

export const Empty: Story = {
	args: {
		"aria-label": "Empty operation-runner",
		operations: [],
	},
};
