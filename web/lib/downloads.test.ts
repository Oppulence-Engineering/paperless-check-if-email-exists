// @vitest-environment jsdom

import { afterEach, describe, expect, it, vi } from "vitest";

import { saveFile } from "./downloads";

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
