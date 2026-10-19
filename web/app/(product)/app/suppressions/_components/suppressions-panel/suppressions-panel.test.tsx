// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { V1Api } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";

import { saveFile } from "@/lib/downloads";

import { SuppressionsPanel } from "./suppressions-panel";
import { chooseOption } from "@/quality/test-support/select";

vi.mock("@/lib/downloads", () => ({ saveFile: vi.fn() }));

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

/** One entry as the live GET /v1/suppressions returns it. */
const entry = {
	id: 3,
	email: "bad@example.com",
	canonical_email: "bad@example.com",
	status: "active",
	reason: "bounce",
	reason_code: "bounce",
	reason_detail: "550 mailbox unknown",
	source: null,
	source_type: "import",
	source_ref: "bounces.csv",
	notes: "From the spring send",
	created_by: "api",
	expires_at: "2020-01-01T00:00:00+00:00",
	last_seen_at: "2026-10-18T04:52:32.059053+00:00",
	metadata: {},
	created_at: "2026-09-23T00:00:00Z",
	updated_at: "2026-09-23T00:00:00Z",
};

function renderPanel() {
	// gitleaks reads the method name as a key.
	const list = vi.spyOn(V1Api.prototype, "v1ListSuppressions"); // gitleaks:allow
	list.mockResolvedValue({ data: { entries: [entry], total: 45 } } as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<SuppressionsPanel />
		</QueryClientProvider>,
	);
	return list;
}

it("shows every entry field, filters, and pages through the list", async () => {
	const list = renderPanel();
	expect(await screen.findByText("bad@example.com")).toBeInTheDocument();
	const table = within(screen.getByRole("region", { name: "Suppressed addresses" }));
	expect(table.getByText("550 mailbox unknown")).toBeInTheDocument();
	expect(table.getByText("CSV import")).toBeInTheDocument();
	expect(table.getByText("bounces.csv")).toBeInTheDocument();
	expect(table.getByText("From the spring send")).toBeInTheDocument();
	expect(table.getByText("Expired")).toBeInTheDocument();
	expect(table.getByText("Active")).toBeInTheDocument();
	expect(screen.getByText("Showing 1–1 of 45")).toBeInTheDocument();
	expect(list).toHaveBeenLastCalledWith({
		limit: 20,
		offset: 0,
		status: "active",
		reason: undefined,
		sourceType: undefined,
		includeExpired: false,
	});

	fireEvent.click(screen.getByRole("button", { name: "Next" }));
	await waitFor(() =>
		expect(list).toHaveBeenLastCalledWith(expect.objectContaining({ offset: 20 })),
	);

	await chooseOption("Reason", "Bounce");
	await chooseOption("Status", "All statuses");
	await chooseOption("Source", "Delivery feedback");
	fireEvent.click(screen.getByLabelText("Include expired entries"));
	await waitFor(() =>
		expect(list).toHaveBeenLastCalledWith({
			limit: 20,
			offset: 0,
			status: "all",
			reason: "bounce",
			sourceType: "outcome",
			includeExpired: true,
		}),
	);
});

it("adds an address from the add sheet", async () => {
	renderPanel();
	const addMethod = "v1AddSuppressions";
	const add = vi.spyOn(V1Api.prototype, addMethod); // gitleaks:allow
	add.mockResolvedValue({ data: { added: 1, duplicates: 0 } } as never);
	expect(await screen.findByText("bad@example.com")).toBeInTheDocument();
	fireEvent.click(screen.getByRole("button", { name: "Add addresses" }));
	const sheet = within(await screen.findByRole("dialog"));
	fireEvent.change(sheet.getByLabelText("Addresses"), { target: { value: "new@example.com" } });
	fireEvent.click(sheet.getByRole("button", { name: "Add" }));
	await waitFor(() =>
		expect(add).toHaveBeenCalledWith({
			addSuppressionsRequest: { emails: ["new@example.com"], reason: "manual" },
		}),
	);
});

it("opens the audit timeline and exports with the current filters", async () => {
	renderPanel();
	const events = vi.spyOn(V1Api.prototype, "v1ListSuppressionEvents"); // gitleaks:allow
	events.mockResolvedValue({
		data: {
			events: [
				{
					id: 1,
					canonical_email: "a@example.com",
					event_type: "created",
					to_status: "active",
					reason_code: "bounce",
					actor_type: "api",
					metadata: {},
					created_at: "2026-09-23T00:00:00Z",
				},
			],
			total: 1,
		},
	} as never);
	const exportMethod = "v1ExportSuppressions";
	const download = vi.spyOn(V1Api.prototype, exportMethod); // gitleaks:allow
	download.mockResolvedValue({ data: new Blob(["id\n"]) } as never);
	expect(await screen.findByText("bad@example.com")).toBeInTheDocument();

	fireEvent.click(screen.getByRole("button", { name: "Audit" }));
	const sheet = within(await screen.findByRole("dialog"));
	expect(await sheet.findByText("Set to Active")).toBeInTheDocument();
	expect(events).toHaveBeenCalledWith({ id: 3 });
	fireEvent.keyDown(document.activeElement ?? document.body, { key: "Escape" });
	await waitFor(() => expect(screen.queryByRole("dialog")).not.toBeInTheDocument());

	fireEvent.click(screen.getByRole("button", { name: "Export CSV" }));
	await waitFor(() => expect(saveFile).toHaveBeenCalled());
	expect(download).toHaveBeenCalledWith(
		{ status: "active", reason: undefined, sourceType: undefined, includeExpired: false },
		{ responseType: "blob" },
	);
});
