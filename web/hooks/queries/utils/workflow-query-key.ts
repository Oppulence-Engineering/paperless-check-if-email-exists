/** Keep product workflow query keys consistent with their invalidation prefixes. */
export function workflowQueryKey(
	...parts: (string | number | boolean | null | undefined | Readonly<Record<string, unknown>>)[]
) {
	return parts;
}
