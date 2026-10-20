// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";

import { TechnicalDetails } from "./technical-details";

describe("TechnicalDetails", () => {
	afterEach(cleanup);

	it("lists the mail servers and timing inside a closed panel", () => {
		render(<TechnicalDetails aria-label="Details" result={SAMPLE_CHECK_RESULT} />);

		expect(screen.getByRole("region", { name: "Details" })).toHaveAttribute(
			"data-slot",
			"technical-details",
		);
		expect(screen.getByText("Technical details").closest("details")).not.toHaveAttribute("open");
		expect(screen.getByText("aspmx.l.google.com.")).toBeInTheDocument();
		expect(screen.getByText("1.20 s")).toBeInTheDocument();
	});

	it("shows the error when a check part failed", () => {
		render(
			<TechnicalDetails
				result={{
					...SAMPLE_CHECK_RESULT,
					smtp: { error: { type: "Timeout", message: "No answer" } },
				}}
			/>,
		);

		expect(screen.getByText("Timeout: No answer")).toBeInTheDocument();
	});
	it("explains external verification without presenting missing SMTP diagnostics as a failure", () => {
		render(
			<TechnicalDetails
				result={{
					...SAMPLE_CHECK_RESULT,
					external_verification: {
						service: "millionverifier",
						result: "ok",
						result_code: 1,
						subresult: "ok",
					},
					smtp: {
						error: {
							type: "AnyhowError",
							message: "Not performed",
						},
					},
				}}
			/>,
		);
		expect(
			screen.getByText(/Checked by MillionVerifier/),
		).toBeInTheDocument();
		expect(
			screen.queryByText("AnyhowError: Not performed"),
		).not.toBeInTheDocument();
	});
});
