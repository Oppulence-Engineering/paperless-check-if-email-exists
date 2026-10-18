// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { V1Api } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { SuppressionForm } from "./suppression-form";

function renderForm(mode: "add" | "import") {
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	return render(
		<QueryClientProvider client={client}>
			<SuppressionForm aria-label="Suppression form" mode={mode} />
		</QueryClientProvider>,
	);
}

describe("SuppressionForm", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("adds typed addresses with a reason, detail, notes, and expiry", async () => {
		// gitleaks reads the method name as a key.
		const add = vi.spyOn(V1Api.prototype, "v1AddSuppressions"); // gitleaks:allow
		add.mockResolvedValue({
			data: { added: 1, updated: 1, duplicates: 1, entry_ids: [4, 5] },
		} as never);
		renderForm("add");
		expect(screen.getByRole("form", { name: "Suppression form" })).toHaveAttribute(
			"data-slot",
			"suppression-form",
		);
		fireEvent.change(screen.getByLabelText("Addresses"), {
			target: { value: "a@example.com\nb@example.com, a@example.com" },
		});
		fireEvent.change(screen.getByLabelText("Reason"), { target: { value: "bounce" } });
		fireEvent.change(screen.getByLabelText("Reason detail"), { target: { value: "550 unknown" } });
		fireEvent.change(screen.getByLabelText("Notes"), { target: { value: "Spring send" } });
		fireEvent.change(screen.getByLabelText("Expires on"), { target: { value: "2027-01-01" } });
		fireEvent.click(screen.getByRole("button", { name: "Add" }));

		await waitFor(() =>
			expect(add).toHaveBeenCalledWith({
				addSuppressionsRequest: {
					emails: ["a@example.com", "b@example.com", "a@example.com"],
					reason: "bounce",
					reason_detail: "550 unknown",
					notes: "Spring send",
					expires_at: new Date("2027-01-01T00:00").toISOString(),
				},
			}),
		);
		const result = within(await screen.findByRole("status"));
		expect(await result.findByText("Duplicates")).toBeInTheDocument();
		expect(result.getByText("Addresses sent").nextSibling).toHaveTextContent("3");
		expect(screen.getByLabelText("Addresses")).toHaveValue("");
	});

	it("imports the email column of a CSV file and shows the counts", async () => {
		const add = vi.spyOn(V1Api.prototype, "v1ImportSuppressions"); // gitleaks:allow
		add.mockResolvedValue({ data: { added: 2, updated: 0, duplicates: 0 } } as never);
		renderForm("import");
		const csv =
			'name,Email\n"Lee, Maya",maya@example.com\nNo address,\n"Bo ""B""",bo@example.com\n';
		fireEvent.change(screen.getByLabelText("CSV file"), {
			target: { files: [new File([csv], "bounces.csv", { type: "text/csv" })] },
		});
		const form = screen.getByRole("form", { name: "Suppression form" });
		fireEvent.submit(form);

		await waitFor(() =>
			expect(add).toHaveBeenCalledWith({
				addSuppressionsRequest: {
					emails: ["maya@example.com", "bo@example.com"],
					reason: "manual",
					source_type: "import",
					source_ref: "bounces.csv",
				},
			}),
		);
		const result = within(await screen.findByRole("status"));
		expect(await result.findByText("Rows skipped")).toBeInTheDocument();
		expect(result.getByText("Rows skipped").nextSibling).toHaveTextContent("1");
		expect(result.getByText("Added").nextSibling).toHaveTextContent("2");
	});

	it("uses the first column of a file with no email header", async () => {
		const add = vi.spyOn(V1Api.prototype, "v1ImportSuppressions"); // gitleaks:allow
		add.mockResolvedValue({ data: { added: 2, duplicates: 0 } } as never);
		renderForm("import");
		fireEvent.change(screen.getByLabelText("CSV file"), {
			target: { files: [new File(["x@example.com\r\ny@example.com\r\n"], "plain.csv")] },
		});
		fireEvent.submit(screen.getByRole("form", { name: "Suppression form" }));
		await waitFor(() =>
			expect(add).toHaveBeenCalledWith({
				addSuppressionsRequest: expect.objectContaining({
					emails: ["x@example.com", "y@example.com"],
				}),
			}),
		);
	});

	it("shows an error when the file has no addresses", async () => {
		const add = vi.spyOn(V1Api.prototype, "v1ImportSuppressions"); // gitleaks:allow
		renderForm("import");
		fireEvent.change(screen.getByLabelText("CSV file"), {
			target: { files: [new File(["email\nnone\n"], "empty.csv")] },
		});
		fireEvent.submit(screen.getByRole("form", { name: "Suppression form" }));
		expect(await screen.findByRole("alert")).toHaveTextContent("The file has no email addresses.");
		expect(add).not.toHaveBeenCalled();
	});
});
