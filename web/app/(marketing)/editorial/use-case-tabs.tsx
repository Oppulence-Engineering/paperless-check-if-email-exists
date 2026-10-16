"use client";

import Link from "next/link";
import { useState } from "react";

import { AddressBook, ArrowRight, Code, EnvelopeSimple, Stack } from "@/lib/icons";
import { cn } from "@/lib/utils";

import { ApiResponseMock, QualityGateMock, QualityReportMock, SourceCompareMock } from "./mocks";
import { Stage } from "./ui";

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
		<div className="grid overflow-hidden rounded-3xl border border-[var(--ed-line)] lg:grid-cols-[300px_1fr]">
			<div
				aria-label="Teams"
				className="flex flex-col border-b border-[var(--ed-line)] lg:border-r lg:border-b-0"
				role="tablist"
			>
				{TABS.map((tab) => {
					const Icon = tab.icon;
					const selected = tab.id === active;
					return (
						<button
							aria-controls="use-case-panel"
							aria-selected={selected}
							className={cn(
								"ed-sans flex gap-3 border-b border-[var(--ed-line)] p-4 text-left transition-colors last:border-b-0",
								selected
									? "bg-[var(--ed-subtle)]"
									: "bg-[var(--ed-bg)] hover:bg-[var(--ed-subtle)]",
							)}
							key={tab.id}
							onClick={() => setActive(tab.id)}
							role="tab"
							type="button"
						>
							<Icon aria-hidden="true" className="mt-0.5 size-4 shrink-0 text-[var(--ed-ink)]" />
							<span>
								<span className="block text-[14px] font-medium text-[var(--ed-ink)]">
									{tab.title}
								</span>
								<span className="mt-1 block text-[12.5px] leading-[1.45] text-[var(--ed-muted)]">
									{tab.body}
								</span>
							</span>
						</button>
					);
				})}
			</div>
			<div id="use-case-panel" role="tabpanel">
				<Stage className="flex h-full min-h-[420px] flex-col justify-center border-0">
					<div className="mx-auto w-full max-w-[620px]">{current.mock}</div>
					<Link
						className="ed-sans mx-auto mt-6 inline-flex items-center gap-1.5 text-[13px] font-medium text-[var(--ed-ink)] hover:underline"
						href={current.href}
					>
						More on {current.title.toLowerCase()}{" "}
						<ArrowRight aria-hidden="true" className="size-3.5" />
					</Link>
				</Stage>
			</div>
		</div>
	);
}
