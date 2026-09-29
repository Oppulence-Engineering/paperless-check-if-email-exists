import type { ComponentType } from "react";

import { cn } from "@/lib/sim/cn";

import {
	ApprovalMark,
	KeysMark,
	LocalMark,
	SelfHostMark,
	SourceMark,
	SyncMark,
	type GovernanceMarkProps,
} from "./governance-marks";
import { HOME_INSET, HOME_TYPE, LANDING_CONTENT_WIDTH, LANDING_GUTTER } from "./tokens";

/** Everything under the title — mark and description stay on the quiet tier. */
const CONTROL_QUIET = "text-[var(--text-secondary)]";

/** Same caption measure as Sim's feature rail and lifecycle grid (15px/1.45). */
const CONTROL_COPY = "text-[15px] leading-[1.45]";

interface Control {
	title: string;
	description: string;
	Mark: ComponentType<GovernanceMarkProps>;
}

const controls: Control[] = [
	{
		title: "No message sent",
		description: "Verification checks signals without sending an email to the recipient.",
		Mark: ApprovalMark,
	},
	{
		title: "Signals with each result",
		description: "Review syntax, DNS, MX, and mailbox details with the outcome.",
		Mark: SourceMark,
	},
	{
		title: "Self-hosted option",
		description: "Run the open-source web app and Rust API on your own infrastructure.",
		Mark: LocalMark,
	},
	{
		title: "Organization access",
		description: "Keep checks, lists, and history in your team workspace.",
		Mark: SyncMark,
	},
	{
		title: "API and SDKs",
		description: "Use the documented backend API and generated clients in your workflow.",
		Mark: KeysMark,
	},
	{
		title: "Bulk jobs",
		description: "Upload CSV lists and monitor verification jobs.",
		Mark: SelfHostMark,
	},
];

/** Sim `WorkspaceControls` — six-cell governance grid with quiet outline marks. */
export function SimWorkspaceControls() {
	return (
		<section
			aria-label="Email verification capabilities"
			className={cn("flex w-full flex-col", LANDING_CONTENT_WIDTH, LANDING_GUTTER)}
			id="controls"
		>
			<div className={cn(HOME_INSET, "pt-12 max-sm:pt-8")}>
				<ul className="grid grid-cols-3 gap-px overflow-hidden rounded-none border border-[var(--border)] bg-[var(--border)] max-sm:grid-cols-1 max-lg:grid-cols-2">
					{controls.map(({ title, description, Mark }) => (
						<li
							className="flex flex-col items-start gap-3 bg-[var(--surface-2)] p-7 max-sm:p-6"
							key={title}
						>
							<Mark className={cn("size-[56px]", CONTROL_QUIET)} />
							<div>
								<h3 className={cn("text-[var(--text-primary)]", HOME_TYPE.body)}>{title}</h3>
								<p className={cn("mt-1.5 max-w-[30ch] text-pretty", CONTROL_QUIET, CONTROL_COPY)}>
									{description}
								</p>
							</div>
						</li>
					))}
				</ul>
			</div>
		</section>
	);
}
