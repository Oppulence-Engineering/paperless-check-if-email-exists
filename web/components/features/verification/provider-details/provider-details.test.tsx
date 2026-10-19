// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { SAMPLE_CHECK_RESULT, type CheckResult } from "@/lib/verification/check-result";

import { ProviderDetails } from "./provider-details";

describe("ProviderDetails", () => {
	afterEach(cleanup);

	it("shows the provider and its confidence", () => {
		render(<ProviderDetails aria-label="Provider" result={SAMPLE_CHECK_RESULT} />);

		expect(screen.getByRole("region", { name: "Provider" })).toHaveAttribute(
			"data-slot",
			"provider-details",
		);
		expect(screen.getByText("Google Workspace")).toBeInTheDocument();
		expect(screen.getByText("High confidence")).toBeInTheDocument();
		expect(screen.queryByText("Workspace policy")).not.toBeInTheDocument();
	});

	it("shows the policy result, rejection reason, and partial check", () => {
		const result: CheckResult = {
			...SAMPLE_CHECK_RESULT,
			provider_rejection_reason: "provider_local_part_too_short",
			policy_evaluation: {
				mode: "signup_protection",
				decision: "review",
				reasons: [{ code: "catch_all", severity: "warning", message: "Catch-all domain." }],
				evaluated_at: "2026-10-17T12:00:00Z",
				result_age_days: 3,
				engine_version: "1",
			},
			score: {
				...SAMPLE_CHECK_RESULT.score,
				partial_confidence: { classification: "timeout", confidence: 40, factors: [] },
			},
		};
		render(<ProviderDetails result={result} />);

		expect(screen.getByText("Signup protection policy, result 3 days old")).toBeInTheDocument();
		expect(screen.getByText("Review")).toBeInTheDocument();
		expect(screen.getByText("Catch-all domain.")).toBeInTheDocument();
		expect(
			screen.getByText("The name before @ is too short for this provider"),
		).toBeInTheDocument();
		expect(screen.getByText("The mail server did not answer in time")).toBeInTheDocument();
	});
});
