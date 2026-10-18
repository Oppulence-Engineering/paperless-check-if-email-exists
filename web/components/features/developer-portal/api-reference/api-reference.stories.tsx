import type { Meta, StoryObj } from "@storybook/react";

import { ApiReference } from "./api-reference";
import { ApiReferencePropsSchema } from "./api-reference.schema";

/**
 * @oppulence-gen kind=component
 * Visual contract for ApiReference. Domain args are built from the Zod
 * schema so Storybook fixtures stay honest.
 */
const meta = {
	title: "features/developer-portal/ApiReference",
	component: ApiReference,
	tags: ["autodocs"],
} satisfies Meta<typeof ApiReference>;

export default meta;

type Story = StoryObj<typeof meta>;

const document = {
	openapi: "3.0.3",
	info: { title: "Verifier API", version: "1.0.0" },
	paths: {
		"/v1/check_email": {
			post: {
				operationId: "v1_check_email",
				summary: "Check one email address",
				tags: ["Verification and finder"],
				responses: { "200": { description: "The result of the check" } },
			},
		},
	},
};

export const Public: Story = {
	args: {
		"aria-label": "API reference",
		...ApiReferencePropsSchema.parse({ document, surface: "public" }),
	},
};

export const Workspace: Story = {
	args: {
		"aria-label": "Workspace API",
		...ApiReferencePropsSchema.parse({ document, surface: "workspace" }),
	},
};
