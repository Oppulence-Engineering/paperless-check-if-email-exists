"use client";

import { useState } from "react";

import { ArrowLink } from "@/components/platform/landing/blocks";
import { MUTED, Stage } from "@/components/platform/landing/primitives";
import { AddressBook, Code, EnvelopeSimple, Stack } from "@/lib/icons";
import { cn } from "@/lib/utils";

import { ApiResponseMock, QualityGateMock, QualityReportMock, SourceCompareMock } from "./mocks";

const TABS = [
	{
		id: "email-marketing",
		icon: EnvelopeSimple,
		title: "Email marketing",
		body: "Check the list before each campaign and keep bounce rates low, so mail reaches the inbox.",
		href: "/use-cases/email-marketing",
		mock: <QualityReportMock />,
	},
	{
		id: "marketing-operations",
		icon: AddressBook,
		title: "Marketing operations",
		body: "Clean CRM and event exports, and see which sources bring in bad addresses.",
		href: "/use-cases/marketing-operations",
		mock: <SourceCompareMock />,
	},
	{
		id: "signup-forms",
		icon: Code,
		title: "Signup forms",
		body: "Check each address in real time, and ask the user to fix a likely typo.",
		href: "/use-cases/signup-forms",
		mock: <ApiResponseMock />,
	},
	{
		id: "sending-platforms",
		icon: Stack,
		title: "Platforms and agencies",
		body: "Hold a customer's list when its quality score is too low to send safely.",
		href: "/use-cases/sending-platforms",
		mock: <QualityGateMock />,
	},
] as const;

/** Team list on the left; the selected team's product view on the right. */
export function UseCaseTabs() {
	const [active, setActive] = useState<(typeof TABS)[number]["id"]>(TABS[0].id);
	const current = TABS.find((tab) => tab.id === active) ?? TABS[0];

	return (
		<div className="grid gap-4 lg:grid-cols-[300px_1fr]">
			<div aria-label="Teams" className="flex flex-col gap-1" role="tablist">
				{TABS.map((tab) => {
					const Icon = tab.icon;
					const selected = tab.id === active;
					return (
						<button
							aria-controls="use-case-panel"
							aria-selected={selected}
							className={cn(
								"flex gap-3 p-5 text-left transition-colors duration-150",
								selected ? "bg-[#181818] ring-1 ring-white/15" : "bg-[#111] hover:bg-[#161616]",
							)}
							key={tab.id}
							onClick={() => setActive(tab.id)}
							role="tab"
							type="button"
						>
							<Icon
								aria-hidden="true"
								className="mt-0.5 size-5 shrink-0 text-white/80"
								strokeWidth={1.5}
							/>
							<span>
								<span className="block text-base text-white">{tab.title}</span>
								<span className={cn("mt-1.5 block text-[15px] leading-[1.55]", MUTED)}>
									{tab.body}
								</span>
							</span>
						</button>
					);
				})}
			</div>
			<div className="flex flex-col" id="use-case-panel" role="tabpanel">
				<Stage className="min-h-[420px] flex-1 items-center px-5 py-10 sm:p-12" variant={2}>
					<div className="w-full max-w-[620px]">{current.mock}</div>
				</Stage>
				<ArrowLink className="mt-6" href={current.href}>
					More on {current.title.toLowerCase()}
				</ArrowLink>
			</div>
		</div>
	);
}
