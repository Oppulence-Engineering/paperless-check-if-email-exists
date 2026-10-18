// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";

import { RiskFactors } from "./risk-factors";

describe("RiskFactors", () => {
	afterEach(cleanup);

	it("lists each factor with its direction and weight", () => {
		render(<RiskFactors aria-label="Risk" result={SAMPLE_CHECK_RESULT} />);

		expect(screen.getByRole("region", { name: "Risk" })).toHaveAttribute(
			"data-slot",
			"risk-factors",
		);
		expect(screen.getByText("The mail server accepts this mailbox.")).toBeInTheDocument();
		expect(screen.getByText("Lowers risk · 40")).toBeInTheDocument();
		expect(screen.getByText("SMTP confirmed the mailbox")).toBeInTheDocument();
	});

	it("says when the model gave no score", () => {
		render(<RiskFactors result={{ ...SAMPLE_CHECK_RESULT, bounce_risk: null }} />);

		expect(
			screen.getByText("The bounce risk model gave no score for this check."),
		).toBeInTheDocument();
	});
});
