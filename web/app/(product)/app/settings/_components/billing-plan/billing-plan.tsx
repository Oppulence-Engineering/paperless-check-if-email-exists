"use client";

import "client-only";

import type { ComponentPropsWithoutRef } from "react";

import { Button } from "@oppulence/ui/components/button";
import { cn } from "@oppulence/ui/lib/utils";
import { Row } from "@/components/platform/page-kit";

import { MONTHLY_CHECKS, MONTHLY_PRICE_USD, PAID_PLANS, type PaidPlan } from "@/lib/billing/plans";
import { seatLimit } from "@/lib/entitlements/entitlements";
import type { Plan } from "@/lib/entitlements/entitlements.schema";

import { type BillingPlanPropsFields } from "./billing-plan.schema";

/**
 * @oppulence-gen kind=component
 * BillingPlan is a client presentation component.
 * Workspace plan with upgrade and billing portal actions.
 *
 * Data and callbacks arrive through validated props. This module does not fetch,
 * persist, or read cookies. Owned by `billing-plan.lit.ts`.
 */
export type BillingPlanProps = BillingPlanPropsFields &
	Omit<ComponentPropsWithoutRef<"section">, "onError"> & {
		onUpgrade?: (plan: PaidPlan) => void;
		onManage?: () => void;
	};

const PLAN_NAME: Record<Plan, string> = {
	free: "Free",
	starter: "Starter",
	professional: "Professional",
	enterprise: "Enterprise",
};

const ORDER: Plan[] = ["free", "starter", "professional", "enterprise"];

function allowance(plan: Plan): string {
	const checks = MONTHLY_CHECKS[plan];
	const seats = seatLimit(plan);
	return [
		checks === null ? "Checks by contract" : `${checks.toLocaleString("en-US")} checks a month`,
		seats === null ? "unlimited seats" : `${String(seats)} seats`,
	].join(" · ");
}

export function BillingPlan({
	plan,
	billingEnabled,
	canManage,
	pending = false,
	error,
	chosen,
	onUpgrade,
	onManage,
	className,
	...props
}: BillingPlanProps) {
	const upgrades = PAID_PLANS.filter((paid) => ORDER.indexOf(paid) > ORDER.indexOf(plan));
	const paid = plan === "starter" || plan === "professional";
	return (
		<section
			aria-label="Plan"
			className={cn("flex flex-col divide-y divide-white/[0.07]", className)}
			data-slot="billing-plan"
			{...props}
		>
			<Row description={allowance(plan)} label="Current plan" layout="inline">
				<span className="text-sm font-medium text-white">{PLAN_NAME[plan]}</span>
			</Row>

			{error ? (
				<Row>
					<p className="text-sm text-red-300" role="alert">
						{error}
					</p>
				</Row>
			) : null}

			{!billingEnabled ? (
				<Row>
					<p className="text-sm text-[#858585]">
						Online billing is not set up here. Contact us to change your plan.
					</p>
				</Row>
			) : !canManage ? (
				<Row>
					<p className="text-sm text-[#858585]">Only owners and admins can change the plan.</p>
				</Row>
			) : (
				<>
					{upgrades.length > 0 ? (
						<ul className="flex flex-col divide-y divide-white/[0.07]">
							{upgrades.map((option) => (
								<li
									className={cn(
										"flex items-center justify-between gap-4 px-5 py-4",
										option === chosen && "bg-white/[0.04]",
									)}
									key={option}
								>
									<span className="min-w-0">
										{option === chosen ? (
											<span className="block text-xs text-violet-300">The plan you picked</span>
										) : null}
										<span className="block text-sm font-medium text-white">
											{PLAN_NAME[option]} · ${MONTHLY_PRICE_USD[option]} a month
										</span>
										<span className="block text-xs text-[#858585]">{allowance(option)}</span>
									</span>
									<Button
										className="rounded-lg"
										disabled={pending}
										onClick={() => onUpgrade?.(option)}
										size="sm"
										type="button"
									>
										Upgrade
									</Button>
								</li>
							))}
						</ul>
					) : null}
					{paid ? (
						<Row>
							<div className="flex flex-wrap items-start gap-3">
								<Button
									className="rounded-lg"
									disabled={pending}
									onClick={() => onManage?.()}
									type="button"
									variant="outline"
								>
									Manage billing
								</Button>
							</div>
						</Row>
					) : null}
				</>
			)}
			{plan !== "enterprise" ? (
				<Row>
					<p className="text-sm text-[#858585]">
						Need more than {MONTHLY_CHECKS.professional?.toLocaleString("en-US")} checks a month?{" "}
						<a
							className="text-neutral-300 underline underline-offset-2 hover:text-white"
							href="/contact"
						>
							Ask about Enterprise
						</a>
						.
					</p>
				</Row>
			) : null}
		</section>
	);
}
