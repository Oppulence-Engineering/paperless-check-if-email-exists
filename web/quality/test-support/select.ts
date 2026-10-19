import { fireEvent, screen } from "@testing-library/react";

/** Opens a Radix Select by its label and clicks the option with the given name. */
export async function chooseOption(label: string, option: string) {
	fireEvent.click(await screen.findByLabelText(label));
	fireEvent.click(await screen.findByRole("option", { name: option }));
}
