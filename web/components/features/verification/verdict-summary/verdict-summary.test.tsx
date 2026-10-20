// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { SAMPLE_CHECK_RESULT, type CheckResult } from "@/lib/verification/check-result";

import { VerdictSummary } from "./verdict-summary";

describe("VerdictSummary", () => {
	afterEach(cleanup);

	it("shows the action, the summary, and the key numbers", () => {
		render(<VerdictSummary aria-label="Verdict" result={SAMPLE_CHECK_RESULT} />);

		const region = screen.getByRole("region", { name: "Verdict" });
		expect(region).toHaveAttribute("data-slot", "verdict-summary");
		expect(screen.getAllByText("Send").length).toBeGreaterThan(0);
		expect(screen.getByText("The mailbox exists and accepts mail.")).toBeInTheDocument();
		expect(screen.getByText("94/100")).toBeInTheDocument();
		expect(screen.getByText("High confidence")).toBeInTheDocument();
		expect(screen.getByText("The mailbox accepts mail.")).toBeInTheDocument();
	});

	it("offers the corrected address as a new check", () => {
		const typo: CheckResult = {
			...SAMPLE_CHECK_RESULT,
			recommendation: undefined,
			score: {
				...SAMPLE_CHECK_RESULT.score,
				domain_suggestion: "maya@gmail.com",
			},
		};
		render(<VerdictSummary result={typo} />);

		expect(screen.getByRole("link", { name: "maya@gmail.com" })).toHaveAttribute(
			"href",
			"/app/check?email=maya%40gmail.com",
		);
	});

	it("does not call a suppressed result send-ready despite a positive quality score", () => {
		render(
			<VerdictSummary
				result={{
					...SAMPLE_CHECK_RESULT,
					recommendation: {
						...SAMPLE_CHECK_RESULT.recommendation,
						action: "suppress",
					},
				}}
			/>,
		);
		expect(screen.getByText("Send-ready").nextElementSibling).toHaveTextContent("No");
		expect(screen.getByText(/Exclude this address from future sends/)).toBeInTheDocument();
	});

	it("falls back to reason codes without a decision", () => {
		render(<VerdictSummary result={{ ...SAMPLE_CHECK_RESULT, recommendation: undefined }} />);

		expect(screen.getByText("The mailbox accepts mail")).toBeInTheDocument();
	});
});
