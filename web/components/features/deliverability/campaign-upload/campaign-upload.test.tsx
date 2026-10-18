// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { OutcomesApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { CampaignUpload } from "./campaign-upload";

describe("CampaignUpload", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("uploads the CSV file and shows the counts and the row errors", async () => {
		const upload = vi.spyOn(OutcomesApi.prototype, "v1UploadCampaignOutcomes"); // gitleaks:allow
		upload.mockResolvedValue({
			data: {
				accepted: 1,
				rejected: 2,
				suppressed: 1,
				policy_id: 7,
				errors: [
					{ index: 1, email: "e@example.com", message: "invalid outcome_type 'nope'" },
					{ index: 2, email: "", message: "missing email" },
				],
			},
		} as never);
		const client = new QueryClient();
		render(
			<QueryClientProvider client={client}>
				<CampaignUpload />
			</QueryClientProvider>,
		);
		const file = new File(["email,type,occurred_at\n"], "results.csv", { type: "text/csv" });

		fireEvent.change(screen.getByLabelText("CSV file"), { target: { files: [file] } });
		fireEvent.click(screen.getByRole("button", { name: "Upload results" }));

		await waitFor(() => expect(upload).toHaveBeenCalledWith({ file }));
		expect(await screen.findByText("Applied outcome policy 7.")).toBeInTheDocument();
		expect(screen.getByText("Accepted").nextSibling).toHaveTextContent("1");
		expect(screen.getByText("Rejected").nextSibling).toHaveTextContent("2");
		expect(screen.getByText("Suppressed").nextSibling).toHaveTextContent("1");
		expect(screen.getByText("Row 2 · e@example.com")).toBeInTheDocument();
		expect(screen.getByText("invalid outcome_type 'nope'")).toBeInTheDocument();
		expect(screen.getByText("missing email")).toBeInTheDocument();
	});
});
