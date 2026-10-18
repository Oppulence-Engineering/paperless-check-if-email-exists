import { fireEvent, screen, within } from "@testing-library/react";
import { format, parse } from "date-fns";

/**
 * Opens a DatePicker or DateTimePicker by its label, moves the calendar to the
 * month of `value` (yyyy-MM-dd), and clicks that day.
 */
export async function chooseDate(label: string, value: string) {
	const target = parse(value, "yyyy-MM-dd", new Date());
	fireEvent.click(await screen.findByLabelText(label));
	await screen.findByRole("grid");
	// The popover opens last, so it is the last dialog in the document.
	const calendar = within(screen.getAllByRole("dialog").at(-1) ?? document.body);
	const caption = format(target, "MMMM yyyy");
	for (let step = 0; step < 240; step++) {
		const shown = calendar.getByRole("status").textContent;
		if (shown === caption) break;
		const forward = parse(shown, "MMMM yyyy", new Date()) < target;
		fireEvent.click(
			calendar.getByRole("button", { name: forward ? /next month/i : /previous month/i }),
		);
	}
	fireEvent.click(calendar.getByRole("button", { name: new RegExp(format(target, "PPPP")) }));
}
