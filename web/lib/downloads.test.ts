// @vitest-environment jsdom

import { afterEach, describe, expect, it, vi } from "vitest";

import { reportCsv, saveFile } from "./downloads";

describe("saveFile", () => {
	afterEach(() => {
		vi.restoreAllMocks();
		vi.useRealTimers();
	});

	it("clicks a download link and releases the URL afterwards", () => {
		vi.useFakeTimers();
		URL.createObjectURL = vi.fn(() => "blob:file");
		URL.revokeObjectURL = vi.fn();
		const click = vi.spyOn(HTMLAnchorElement.prototype, "click").mockImplementation(() => {});

		saveFile("email\na@b.co\n", "list.csv");

		expect(click).toHaveBeenCalledTimes(1);
		expect(URL.revokeObjectURL).not.toHaveBeenCalled();
		vi.runAllTimers();
		expect(URL.revokeObjectURL).toHaveBeenCalledWith("blob:file");
	});
});

it("quotes report labels and prevents spreadsheet formulas", () => {
	expect(
		reportCsv([
			["source", "count"],
			['a,"b"\nc', 2],
			["  =SUM(1,2)", 3],
			["\tformula", 1],
		]),
	).toBe('"source","count"\r\n"a,""b""\nc","2"\r\n"\'  =SUM(1,2)","3"\r\n"\'\tformula","1"');
});
