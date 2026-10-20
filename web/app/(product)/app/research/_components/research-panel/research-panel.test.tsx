// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { V1Api } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";
import { ResearchPanel, researchRefetchInterval } from "./research-panel";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});
const candidate = {
	name: "Jane Doe",
	title: "CEO",
	email: "jane@acme.com",
	email_source: "published",
	person_status: "published",
	employment_status: "published_on_company_site",
	role_status: "title_matches",
	evidence_url: "https://acme.com/team",
	observed_at: "2026-10-20T12:00:00Z",
	mailbox: {
		provider: "millionverifier",
		result: "ok",
		action: "send",
		catch_all: false,
	},
};
const detail = {
	session: { id: "42", name: "Pilot", imported_rows: 2 },
	status: "completed",
	target_role: "CEO",
	max_candidates: 2,
	max_verifications: 3,
	verification_requests: 2,
	rows: [
		{
			row_key: "ready-1",
			company_name: "Acme",
			company_domain: "acme.com",
			status: "ready",
			reason: "published_person_and_matching_title_with_verified_mailbox",
			candidates: [candidate],
			checked_pages: ["https://acme.com/team"],
		},
		{
			row_key: "review-1",
			company_name: "Other",
			company_domain: "other.com",
			status: "needs_review",
			reason: "person_or_role_requires_review",
			candidates: [
				{
					...candidate,
					email: "jane@other.com",
					email_source: "pattern",
					person_status: "supplied",
					employment_status: "unconfirmed",
					role_status: "unconfirmed",
					evidence_url: null,
				},
			],
			checked_pages: [],
		},
	],
};
function setup(initialSessionId?: string) {
	vi.spyOn(V1Api.prototype, "v1ListResearch").mockResolvedValue({
		data: { sessions: [] },
	} as never);
	vi.spyOn(V1Api.prototype, "v1GetResearch").mockResolvedValue({
		data: detail,
	} as never);
	const client = new QueryClient({
		defaultOptions: {
			queries: { retry: false },
			mutations: { retry: false },
		},
	});
	render(
		<QueryClientProvider client={client}>
			<ResearchPanel initialSessionId={initialSessionId} aria-label="Research workspace" />
		</QueryClientProvider>,
	);
}
it("reuses the same paid request key on retry and shows separate evidence claims", async () => {
	const create = vi
		.spyOn(V1Api.prototype, "v1CreateResearch")
		.mockRejectedValueOnce(new Error("temporary"))
		.mockResolvedValue({ data: { id: "42" } } as never);
	setup();
	fireEvent.change(screen.getByLabelText("Run name"), {
		target: { value: "Pilot" },
	});
	fireEvent.change(screen.getByLabelText("Company data"), {
		target: { value: "company_name,company_domain\nAcme,acme.com" },
	});
	fireEvent.change(screen.getByLabelText("Maximum verifications for this run"), {
		target: { value: "3" },
	});
	fireEvent.click(
		screen.getByRole("button", {
			name: "Research with up to 3 verifications",
		}),
	);
	await screen.findByRole("alert");
	fireEvent.click(
		screen.getByRole("button", {
			name: "Research with up to 3 verifications",
		}),
	);
	await screen.findByText("jane@acme.com");
	expect(create).toHaveBeenCalledTimes(2);
	const request = create.mock.calls[0]?.[0];
	expect(request).toEqual(create.mock.calls[1]?.[0]);
	expect(request?.createResearchRequest).toMatchObject({
		max_verifications: 3,
		max_candidates: 2,
		target_role: "CEO",
	});
	expect(request?.createResearchRequest.idempotency_key).toMatch(/^[0-9a-f-]{36}$/);
	expect(screen.getByRole("region", { name: "Research workspace" })).toHaveAttribute(
		"data-slot",
		"research-panel",
	);
	expect(screen.getByText("Listed on company website; not independently confirmed")).toBeVisible();
	expect(screen.getByText(/inferred from a name pattern; ownership unconfirmed/)).toBeVisible();
});
it("only selects ready rows for approved export and reports fresh policy exclusions", async () => {
	const exportRows = vi.spyOn(V1Api.prototype, "v1ExportResearch").mockResolvedValue({
		data: {
			filename: "approved.csv",
			csv: "email\n",
			exported_rows: 0,
			excluded_row_keys: ["ready-1"],
		},
	} as never);
	setup("42");
	await screen.findByText("jane@acme.com");
	expect(
		screen.getByRole("button", {
			name: "Approve and export ready contacts",
		}),
	).toBeDisabled();
	fireEvent.click(screen.getByRole("button", { name: "Select ready contacts" }));
	fireEvent.click(
		screen.getByRole("button", {
			name: "Approve and export ready contacts",
		}),
	);
	await waitFor(() =>
		expect(exportRows).toHaveBeenCalledWith({
			sessionId: 42,
			researchExportRequest: { mode: "approved", row_keys: ["ready-1"] },
		}),
	);
	expect(
		await screen.findByText(
			"0 contact rows exported. 1 selected rows excluded by the current sending requirements.",
		),
	).toBeVisible();
});
it("stops polling completed runs", () => {
	expect(researchRefetchInterval("running")).toBe(5_000);
	expect(researchRefetchInterval("completed")).toBe(false);
	expect(researchRefetchInterval()).toBe(false);
});
