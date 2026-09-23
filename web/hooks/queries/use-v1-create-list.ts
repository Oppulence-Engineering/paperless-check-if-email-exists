"use client";

import "client-only";

import { useMutation, useQueryClient } from "@tanstack/react-query";

import {
	fetchV1CreateList,
	type V1CreateListInput,
} from "@/hooks/queries/utils/mutate-v1-create-list";
import { v1ListListsKeys } from "@/hooks/queries/utils/v1-list-lists-keys";
/**
 * @oppulence-gen kind=mutation
 * Client mutation hook for V1 create list. The transport stays in a non-client
 * module. Owned by `use-v1-create-list.lit.ts`.
 */
export function useV1CreateList() {
	const queryClient = useQueryClient();
	return useMutation({
		mutationFn: (body: V1CreateListInput) => fetchV1CreateList(body),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: v1ListListsKeys.all });
		},
	});
}
