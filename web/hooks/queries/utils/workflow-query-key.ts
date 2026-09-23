/** Keep product workflow query keys consistent with their invalidation prefixes. */
export function workflowQueryKey(...parts: (string | number | boolean | null | undefined)[]) {
	return parts;
}
