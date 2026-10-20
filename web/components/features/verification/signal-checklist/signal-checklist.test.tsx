// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { SAMPLE_CHECK_RESULT, type CheckResult } from "@/lib/verification/check-result";

import { SignalChecklist, signalState } from "./signal-checklist";

describe("SignalChecklist", () => {
	afterEach(cleanup);

	it("marks good values as passed and bad values as failed", () => {
		expect(signalState(true, true)).toBe("pass");
		expect(signalState(false, true)).toBe("fail");
		expect(signalState(true, false)).toBe("fail");
		expect(signalState(false, false)).toBe("pass");
		expect(signalState(true, null)).toBe("neutral");
	});

	it("lists the signals of a result", () => {
		render(<SignalChecklist aria-label="Signals" result={SAMPLE_CHECK_RESULT} />);

		expect(screen.getByRole("region", { name: "Signals" })).toHaveAttribute(
			"data-slot",
			"signal-checklist",
		);
		expect(screen.getByText("Mailbox accepts mail: yes")).toBeInTheDocument();
		// A passed "bad" signal must still read as "no", not as a plain label.
		expect(screen.getByText("Mailbox is disabled: no")).toBeInTheDocument();
		expect(screen.getByText("Free email provider: no")).toBeInTheDocument();
		expect(screen.queryByText("Catch-all risk")).not.toBeInTheDocument();
	});

	it("shows the catch-all severity for catch-all domains", () => {
		const catchAll: CheckResult = {
			...SAMPLE_CHECK_RESULT,
			score: {
				...SAMPLE_CHECK_RESULT.score,
				catch_all_severity: "high",
				signals: { ...SAMPLE_CHECK_RESULT.score.signals, smtp_is_catch_all: true },
			},
		};
		render(<SignalChecklist result={catchAll} />);

		expect(screen.getByText("Catch-all risk")).toBeInTheDocument();
		expect(screen.getByText("High")).toBeInTheDocument();
	});

	it.each(["ok", "catch_all", "unknown"])(
		"does not present unavailable SMTP or MX checks as facts for external %s results",
		(result) => {
			render(
				<SignalChecklist
					result={{
						...SAMPLE_CHECK_RESULT,
						external_verification: {
							service: "millionverifier",
							result,
							result_code: 1,
							subresult: "ok",
						},
					}}
				/>,
			);
			expect(
				screen.getByText(/Individual SMTP and MX checks were not returned/),
			).toBeInTheDocument();
			expect(screen.queryByText(/Mailbox accepts mail:/)).not.toBeInTheDocument();
			expect(screen.queryByText(/Mail server answers:/)).not.toBeInTheDocument();
			expect(screen.queryByText(/Domain has a mail server:/)).not.toBeInTheDocument();
			expect(screen.queryByText(/Mailbox is disabled:/)).not.toBeInTheDocument();
			expect(screen.queryByText(/Domain accepts all addresses:/)).not.toBeInTheDocument();
			expect(screen.getByText("Valid address format: yes")).toBeInTheDocument();
		},
	);
});
