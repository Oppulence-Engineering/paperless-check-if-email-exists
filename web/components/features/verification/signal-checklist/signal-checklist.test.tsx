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
		expect(screen.getByText("Mailbox accepts mail")).toBeInTheDocument();
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
});
