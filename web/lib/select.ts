/** Radix Select items cannot use "", so "All" and "Any" items use this value. */
export const ALL = "__all";

/** Maps the "All" item back to the empty filter value. */
export function fromSelect(value: string): string {
	return value === ALL ? "" : value;
}
