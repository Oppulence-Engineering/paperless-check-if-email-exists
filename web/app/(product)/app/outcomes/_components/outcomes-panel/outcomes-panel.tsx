"use client";

import "client-only";

import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type ReactNode } from "react";

import { Button } from "@oppulence/ui/components/button";
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
	SheetTrigger,
} from "@oppulence/ui/components/sheet";
import { cn } from "@oppulence/ui/lib/utils";
import { Page, Section } from "@/components/platform/page-kit";
import { CampaignOutcomeList } from "@/components/features/deliverability/campaign-outcome-list/campaign-outcome-list";
import { CampaignUpload } from "@/components/features/deliverability/campaign-upload/campaign-upload";
import { OutcomeEntry } from "@/components/features/deliverability/outcome-entry/outcome-entry";
import { OutcomeEvents } from "@/components/features/deliverability/outcome-events/outcome-events";
import { OutcomePolicyForm } from "@/components/features/deliverability/outcome-policy-form/outcome-policy-form";
import { OutcomePolicyList } from "@/components/features/deliverability/outcome-policy-list/outcome-policy-list";
import { ProviderEndpointForm } from "@/components/features/deliverability/provider-endpoint-form/provider-endpoint-form";
import { ProviderEndpointList } from "@/components/features/deliverability/provider-endpoint-list/provider-endpoint-list";

import { type OutcomesPanelPropsFields } from "./outcomes-panel.schema";

/** @oppulence-gen kind=component Owned by `outcomes-panel.lit.ts`. */
export type OutcomesPanelProps = OutcomesPanelPropsFields & ComponentPropsWithoutRef<"section">;

/** A button that opens a form in a sheet. `children` gets a function that closes the sheet. */
function SheetAction({
	children,
	description,
	label,
	title,
	variant = "outline",
}: {
	children: (close: () => void) => ReactNode;
	description: string;
	label: string;
	title: string;
	variant?: "default" | "outline";
}) {
	const [open, setOpen] = useState(false);
	return (
		<Sheet onOpenChange={setOpen} open={open}>
			<SheetTrigger asChild>
				<Button className="rounded-lg" size="sm" type="button" variant={variant}>
					{label}
				</Button>
			</SheetTrigger>
			<SheetContent className="flex w-full flex-col gap-6 overflow-y-auto pb-8 sm:max-w-xl">
				<SheetHeader>
					<SheetTitle>{title}</SheetTitle>
					<SheetDescription>{description}</SheetDescription>
				</SheetHeader>
				<div className="px-4">
					{children(() => {
						setOpen(false);
					})}
				</div>
			</SheetContent>
		</Sheet>
	);
}

export function OutcomesPanel({ className, ...props }: OutcomesPanelProps) {
	return (
		<Page
			actions={
				<SheetAction
					description="Bounce, complaint, and unsubscribe events add the address to the suppression list."
					label="Record an outcome"
					title="Record a manual outcome"
					variant="default"
				>
					{() => <OutcomeEntry />}
				</SheetAction>
			}
			description="Review feedback and configure trusted provider deliveries."
			title="Delivery feedback"
		>
			<section
				data-slot="outcomes-panel"
				className={cn("flex flex-col gap-12", className)}
				{...props}
			>
				<Section
					description="Events from your email platforms, from the API, and from manual entry. A bounce, a complaint, or an unsubscribe suppresses the address."
					title="Recent outcomes"
				>
					<OutcomeEvents />
				</Section>
				<Section
					actions={
						<SheetAction
							description="Upload a CSV file of campaign results. The default outcome policy applies to each row."
							label="Upload campaign results"
							title="Upload campaign results"
						>
							{() => <CampaignUpload />}
						</SheetAction>
					}
					description="Results from your campaign tools. The default outcome policy decides what each result does."
					title="Campaign results"
				>
					<CampaignOutcomeList />
				</Section>
				<Section
					actions={
						<SheetAction
							description="New endpoints start paused. Copy the one-time callback URL into your provider dashboard, send a provider test event, then activate."
							label="Register a provider"
							title="Register a provider"
						>
							{(close) => <ProviderEndpointForm onSaved={close} />}
						</SheetAction>
					}
					description={
						<>
							Endpoints receive events from Postmark, Mailgun, SendGrid, and Amazon SES.{" "}
							<Link
								className="text-neutral-300 underline underline-offset-4 hover:text-white"
								href="/app/integrations"
							>
								Provider setup and callback guide
							</Link>
						</>
					}
					title="Provider endpoints"
				>
					<ProviderEndpointList />
				</Section>
				<Section
					actions={
						<SheetAction
							description="Set what each campaign event does to the address."
							label="New policy"
							title="New outcome policy"
						>
							{(close) => <OutcomePolicyForm onSaved={close} />}
						</SheetAction>
					}
					description="The default policy decides which campaign results suppress an address. Provider events do not use these policies."
					title="Outcome policies"
				>
					<OutcomePolicyList />
				</Section>
			</section>
		</Page>
	);
}
