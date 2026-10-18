// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { ResultFilters } from "./result-filters";

describe("ResultFilters", () => {
	afterEach(() => {
		cleanup();
	});

	it("keeps the quick filters and applies the other filters", () => {
		const onApply = vi.fn();
		render(<ResultFilters filters={{ category: "valid", minScore: 10 }} onApply={onApply} />);

		expect(screen.getByLabelText("Lowest score")).toHaveValue("10");
		fireEvent.change(screen.getByLabelText("Lowest score"), { target: { value: "" } });
		fireEvent.change(screen.getByLabelText("Highest score"), { target: { value: "80" } });
		fireEvent.change(screen.getByLabelText("Job ID"), { target: { value: "18" } });
		fireEvent.change(screen.getByLabelText("Checked from"), { target: { value: "2026-10-01" } });
		fireEvent.change(screen.getByLabelText("Recommendation"), {
			target: { value: "fix_then_send" },
		});
		fireEvent.change(screen.getByLabelText("Policy mode"), { target: { value: "growth" } });
		fireEvent.change(screen.getByLabelText("Policy decision"), { target: { value: "drop" } });
		fireEvent.click(screen.getByRole("button", { name: "Apply filters" }));

		expect(onApply).toHaveBeenCalledWith({
			category: "valid",
			maxScore: 80,
			jobId: 18,
			since: "2026-10-01",
			recommendationAction: "fix_then_send",
			policyMode: "growth",
			policyDecision: "drop",
		});
	});

	it("shows an error for a score out of range", () => {
		const onApply = vi.fn();
		render(<ResultFilters filters={{}} onApply={onApply} />);

		fireEvent.change(screen.getByLabelText("Highest score"), { target: { value: "120" } });
		fireEvent.click(screen.getByRole("button", { name: "Apply filters" }));

		expect(screen.getByRole("alert")).toHaveTextContent("Use a score from 0 to 100");
		expect(onApply).not.toHaveBeenCalled();
	});
});
