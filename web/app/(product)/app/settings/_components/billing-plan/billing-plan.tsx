"use client";

import "client-only";

import type { ComponentPropsWithoutRef } from "react";

import { Button } from "@oppulence/ui/components/button";
import { cn } from "@oppulence/ui/lib/utils";

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
			className={cn("space-y-4 text-sm", className)}
			data-slot="billing-plan"
			{...props}
		>
			<div>
				<p className="text-xs text-muted-foreground">Current plan</p>
				<p className="mt-1 text-base font-medium">{PLAN_NAME[plan]}</p>
				<p className="text-xs text-muted-foreground">{allowance(plan)}</p>
			</div>

			{error ? (
				<p className="text-xs text-destructive" role="alert">
					{error}
				</p>
			) : null}

			{!billingEnabled ? (
				<p className="text-xs text-muted-foreground">
					Online billing is not set up here. Contact us to change your plan.
				</p>
			) : !canManage ? (
				<p className="text-xs text-muted-foreground">Only owners and admins can change the plan.</p>
			) : (
				<>
					{upgrades.length > 0 ? (
						<ul className="divide-y divide-border border-y border-border">
							{upgrades.map((option) => (
								<li
									className={cn(
										"flex items-center justify-between gap-4 py-3",
										option === chosen && "-mx-3 bg-primary/5 px-3",
									)}
									key={option}
								>
									<span>
										{option === chosen ? (
											<span className="block text-xs text-primary">The plan you picked</span>
										) : null}
										<span className="block font-medium">
											{PLAN_NAME[option]} · ${MONTHLY_PRICE_USD[option]} a month
										</span>
										<span className="block text-xs text-muted-foreground">{allowance(option)}</span>
									</span>
									<Button
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
						<Button
							disabled={pending}
							onClick={() => onManage?.()}
							size="sm"
							type="button"
							variant="outline"
						>
							Manage billing
						</Button>
					) : null}
				</>
			)}
			{plan !== "enterprise" ? (
				<p className="text-xs text-muted-foreground">
					Need more than {MONTHLY_CHECKS.professional?.toLocaleString("en-US")} checks a month?{" "}
					<a className="underline underline-offset-2" href="/contact">
						Ask about Enterprise
					</a>
					.
				</p>
			) : null}
		</section>
	);
}
