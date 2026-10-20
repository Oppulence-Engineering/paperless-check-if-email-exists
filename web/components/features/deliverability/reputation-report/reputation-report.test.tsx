// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { ReputationReport } from "./reputation-report";
import { SAMPLE_REPUTATION_REPORT } from "./reputation-report.schema";

describe("ReputationReport", () => {
	afterEach(cleanup);

	it("shows every field of the reputation check", () => {
		render(
			<ReputationReport
				action={<button type="button">Check again</button>}
				aria-label="Reputation"
				report={SAMPLE_REPUTATION_REPORT}
			/>,
		);

		const report = screen.getByRole("region", { name: "Reputation" });
		expect(report).toHaveAttribute("data-slot", "reputation-report");
		expect(screen.getByText("82/100")).toBeInTheDocument();
		expect(screen.getByText("Low risk")).toBeInTheDocument();
		expect(screen.getByText(/Saved result/)).toBeInTheDocument();
		expect(screen.getByRole("button", { name: "Check again" })).toBeInTheDocument();
		expect(screen.getByText("Monitor only")).toBeInTheDocument();
		expect(screen.getByText("2 found")).toBeInTheDocument();
		expect(screen.getByText("Listed on 1 · 0 unknown")).toBeInTheDocument();
		expect(screen.getByText("mx1.example.com")).toBeInTheDocument();
		expect(screen.getByText("Example Registrar, Inc.")).toBeInTheDocument();
		expect(screen.getByText("9,000 days")).toBeInTheDocument();
		expect(screen.getByText(/only common selectors were checked/)).toBeInTheDocument();
		const spamCop = screen.getByText("SpamCop").closest("li");
		expect(spamCop && within(spamCop).getByText("Listed")).toBeInTheDocument();
	});

	it("shows failed blacklist queries and undiscovered DKIM as unknown", () => {
		render(
			<ReputationReport
				report={{
					...SAMPLE_REPUTATION_REPORT,
					risk_level: "unknown",
					blacklist_results: [
						{
							provider: "Spamhaus ZEN",
							listed: null,
							status: "error",
							answers: ["127.255.255.254"],
							error: "Query refused",
							lookup_time_ms: 10,
						},
					],
				}}
			/>,
		);
		expect(screen.getByText("Query failed")).toBeInTheDocument();
		expect(screen.getByText("Incomplete · 1 unknown")).toBeInTheDocument();
		expect(screen.getByText("Unknown")).toBeInTheDocument();
		expect(screen.queryByText("Not listed")).not.toBeInTheDocument();
		expect(screen.queryByText("Not valid")).not.toBeInTheDocument();
		expect(screen.getByText("Record found · syntax not checked")).toBeInTheDocument();
	});

	it("does not report blocklists as clear when the domain has no MX record", () => {
		render(
			<ReputationReport
				report={{
					...SAMPLE_REPUTATION_REPORT,
					cached: false,
					dns_records: { ...SAMPLE_REPUTATION_REPORT.dns_records, has_mx: false, mx_records: [] },
					domain_info: { domain_age_days: null, registrar: null, created_at: null },
				}}
			/>,
		);

		expect(screen.getByText("Not checked")).toBeInTheDocument();
		expect(screen.queryByText("Not listed")).not.toBeInTheDocument();
		expect(screen.getByText(/needs an MX record/)).toBeInTheDocument();
		expect(screen.getAllByText("Not available")).toHaveLength(3);
		expect(screen.getByText(/New result/)).toBeInTheDocument();
	});

	it("explains a null MX record", () => {
		render(
			<ReputationReport
				report={{
					...SAMPLE_REPUTATION_REPORT,
					dns_records: { ...SAMPLE_REPUTATION_REPORT.dns_records, mx_records: ["."] },
				}}
			/>,
		);

		expect(screen.getByText(". (null MX: the domain does not accept mail)")).toBeInTheDocument();
	});
});
