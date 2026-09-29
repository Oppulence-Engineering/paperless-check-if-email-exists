"use client";

import "client-only";

import { useMutation } from "@tanstack/react-query";

import {
	fetchV1CheckEmail,
	type V1CheckEmailInput,
} from "@/hooks/queries/utils/mutate-v1-check-email";
/**
 * @oppulence-gen kind=mutation
 * Client mutation hook for V1 check email. The transport stays in a non-client
 * module. Owned by `use-v1-check-email.lit.ts`.
 */
export function useV1CheckEmail() {
	return useMutation({
		mutationFn: (body: V1CheckEmailInput) => fetchV1CheckEmail(body),
	});
}
