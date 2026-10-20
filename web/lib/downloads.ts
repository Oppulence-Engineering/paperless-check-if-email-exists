/**
 * Saves an API response body as a file in the browser. Panels pass the data
 * of a download operation, for example a CSV export.
 */
export function saveFile(data: unknown, filename: string): void {
	const blob = data instanceof Blob ? data : new Blob([data as BlobPart]);
	const url = URL.createObjectURL(blob);
	const link = document.createElement("a");
	link.href = url;
	link.download = filename;
	link.click();
	// The browser reads the URL after the click event, so release it later.
	setTimeout(() => {
		URL.revokeObjectURL(url);
	}, 1_000);
}

/** Quote CSV fields and neutralize spreadsheet formulas in customer-supplied labels. */
export function reportCsv(rows: (string | number)[][]): string {
	return rows
		.map((row) =>
			row
				.map((value) => {
					const text = String(value);
					const safe =
						/^[\s\u0000-\u001f]*[=+@-]/u.test(text) || /^[\t\r\n]/u.test(text) ? `'${text}` : text;
					return `"${safe.replaceAll('"', '""')}"`;
				})
				.join(","),
		)
		.join("\r\n");
}
