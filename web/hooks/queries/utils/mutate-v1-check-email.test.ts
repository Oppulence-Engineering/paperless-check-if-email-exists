import { V1Api } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";
import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";
import { fetchV1CheckEmail } from "./mutate-v1-check-email";

afterEach(() => {
	vi.useRealTimers();
	vi.restoreAllMocks();
});

it("polls one pending operation without submitting another check", async () => {
	vi.useFakeTimers();
	const id = "123e4567-e89b-42d3-a456-426614174000";
	const pending = { operation_id: id, status: "pending", status_url: `/v1/operations/${id}` };
	const submit = vi
		.spyOn(V1Api.prototype, "v1CheckEmail")
		.mockResolvedValue({ status: 202, data: pending } as never);
	const poll = vi
		.spyOn(V1Api.prototype, "v1GetVerificationOperation") // gitleaks:allow -- SDK method name
		.mockResolvedValueOnce({ status: 202, data: pending } as never)
		.mockResolvedValueOnce({ status: 200, data: SAMPLE_CHECK_RESULT } as never);
	const result = fetchV1CheckEmail({ to_email: SAMPLE_CHECK_RESULT.input });
	await vi.advanceTimersByTimeAsync(2000);
	expect(await result).toEqual(SAMPLE_CHECK_RESULT);
	expect(submit).toHaveBeenCalledTimes(1);
	expect(poll).toHaveBeenCalledTimes(2);
	expect(poll).toHaveBeenCalledWith({ operationId: id }, expect.any(Object));
});

it("cancels polling without submitting another check", async () => {
	vi.useFakeTimers();
	const id = "123e4567-e89b-42d3-a456-426614174000";
	const submit = vi.spyOn(V1Api.prototype, "v1CheckEmail").mockResolvedValue({
		status: 202,
		data: { operation_id: id, status: "pending", status_url: `/v1/operations/${id}` },
	} as never);
	const poll = vi.spyOn(V1Api.prototype, "v1GetVerificationOperation"); // gitleaks:allow -- SDK method name
	const controller = new AbortController();
	const result = fetchV1CheckEmail({ to_email: SAMPLE_CHECK_RESULT.input }, controller.signal);
	const assertion = expect(result).rejects.toThrow("Cancelled");
	controller.abort(new Error("Cancelled"));
	await assertion;
	await vi.advanceTimersByTimeAsync(2000);
	expect(submit).toHaveBeenCalledTimes(1);
	expect(poll).not.toHaveBeenCalled();
});
