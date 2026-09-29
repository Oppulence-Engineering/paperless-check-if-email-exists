// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, expect, it } from "vitest";

import { MarketingFaq } from "./marketing-faq";

afterEach(cleanup);

it("opens a public FAQ answer with the shared accordion", () => {
	render(
		<MarketingFaq
			items={[
				{
					question: "How does it work?",
					answer: "Check a single address.",
				},
			]}
		/>,
	);

	const question = screen.getByRole("button", { name: /How does it work\?/ });
	expect(question).toHaveAttribute("aria-expanded", "false");
	fireEvent.click(question);
	expect(question).toHaveAttribute("aria-expanded", "true");
	expect(screen.getByText("Check a single address.")).toBeVisible();
});
