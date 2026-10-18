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
