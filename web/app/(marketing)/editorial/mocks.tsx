/**
 * Product illustrations for the public site. They use the real result
 * vocabulary (categories, scores, reason codes, fix actions) with sample data
 * on fictional domains. They are drawings of the product, not screenshots.
 */
import type { ReactNode } from "react";

import {
	Check,
	ClockCounterClockwise,
	EnvelopeSimple,
	SquaresFour,
	ListChecks,
	MagnifyingGlass,
	PlugsConnected,
	Prohibit,
	UserCircle,
} from "@/lib/icons";
import { cn } from "@/lib/utils";

export type Category = "valid" | "risky" | "invalid" | "unknown";

const CATEGORY_STYLE: Record<Category, string> = {
	valid: "bg-[#4ade80]/10 text-[#4ade80]",
	risky: "bg-[#fbbf24]/10 text-[#fbbf24]",
	invalid: "bg-[#f87171]/10 text-[#f87171]",
	unknown: "bg-[#1d1d1d] text-[#adadad]",
};

const CATEGORY_DOT: Record<Category, string> = {
	valid: "bg-[#22c55e]",
	risky: "bg-[#f59e0b]",
	invalid: "bg-[#ef4444]",
	unknown: "bg-[#757575]",
};

export function CategoryBadge({ category }: { category: Category }) {
	return (
		<span
			className={cn(
				"inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-[10.5px] font-medium capitalize",
				CATEGORY_STYLE[category],
			)}
		>
			<span className={cn("size-1.5 rounded-full", CATEGORY_DOT[category])} />
			{category}
		</span>
	);
}

function MockShell({ className, children }: { className?: string; children: ReactNode }) {
	return (
		<div
			className={cn(
				"overflow-hidden rounded-xl bg-[#111111] text-white shadow-[0_16px_40px_-12px_rgba(0,0,0,0.75)]",
				className,
			)}
		>
			{children}
		</div>
	);
}

/* ---------- List board (home: sorted list) ---------- */

const BOARD: {
	category: Category;
	count: string;
	rows: [string, number, string][];
}[] = [
	{
		category: "valid",
		count: "3,812",
		rows: [
			["maya@brightlane.co", 100, "deliverable"],
			["theo@northwind.io", 100, "deliverable"],
			["ines@harborops.com", 100, "deliverable"],
			["sam@fieldnote.app", 100, "deliverable"],
		],
	},
	{
		category: "risky",
		count: "412",
		rows: [
			["hello@quarrylabs.com", 65, "role account"],
			["dev@acmecloud.io", 75, "catch-all · high"],
			["jo@inboxfull.net", 70, "full inbox"],
		],
	},
	{
		category: "invalid",
		count: "236",
		rows: [
			["r.lee@closedco.com", 0, "disabled mailbox"],
			["anna@gmial.com", 0, "possible typo"],
			["ops@nomx.dev", 0, "no mail server"],
		],
	},
	{
		category: "unknown",
		count: "51",
		rows: [
			["cfo@slowmail.org", 25, "no answer"],
			["kai@greylist.co", 25, "re-check queued"],
		],
	},
];

export function ListBoardMock() {
	return (
		<MockShell className="flex min-h-[380px] text-[12px]">
			<aside className="hidden w-[180px] shrink-0 border-r border-white/[0.06] bg-[#181818] p-3 sm:block">
				<div className="mb-3 flex items-center gap-2 border border-white/[0.06] bg-[#111111] px-2 py-1.5 text-[11px] text-[#858585]">
					<SquaresFour aria-hidden="true" className="size-3" /> Home
				</div>
				<p className="mb-1 text-[10px] text-[#858585]">Workspace</p>
				{[
					[ListChecks, "Lists"],
					[Prohibit, "Suppressions"],
					[PlugsConnected, "Delivery feedback"],
					[ClockCounterClockwise, "Re-checks"],
				].map(([Icon, label]) => {
					const I = Icon as typeof SquaresFour;
					return (
						<div
							className="flex items-center gap-2 py-1 text-[11px] text-[#d9d9d9]"
							key={label as string}
						>
							<I aria-hidden="true" className="size-3 text-[#858585]" /> {label as string}
						</div>
					);
				})}
				<p className="mt-3 mb-1 text-[10px] text-[#858585]">Recent lists</p>
				{["October newsletter", "Webinar signups", "CRM export"].map((name, index) => (
					<div className="flex items-center gap-2 py-1 text-[11px] text-[#d9d9d9]" key={name}>
						<span
							className={cn("size-2 rounded-[2px]", index === 0 ? "bg-[#155dfc]" : "bg-[#3a3a3a]")}
						/>
						{name}
					</div>
				))}
			</aside>
			<div className="min-w-0 flex-1 p-4">
				<div className="mb-3 flex items-center justify-between">
					<div className="flex items-center gap-2">
						<span className="inline-flex size-6 items-center justify-center bg-white text-[#090909]">
							<EnvelopeSimple aria-hidden="true" className="size-3.5" />
						</span>
						<span className="text-[15px] font-semibold">October newsletter</span>
					</div>
					<span className="bg-[#155dfc] px-2 py-1 text-[11px] text-white">Export</span>
				</div>
				<div className="mb-3 flex gap-4 border-b border-white/[0.06] text-[11px] text-[#858585]">
					<span className="border-b-2 border-white pb-1.5 text-white">Results</span>
					<span className="pb-1.5">Fix plan</span>
					<span className="pb-1.5">History</span>
					<span className="pb-1.5">Quality 88</span>
				</div>
				<div className="grid grid-cols-2 gap-2.5 lg:grid-cols-4">
					{BOARD.map((column) => (
						<div key={column.category}>
							<div className="mb-1.5 flex items-center gap-1.5 text-[11px]">
								<span className={cn("size-1.5 rounded-full", CATEGORY_DOT[column.category])} />
								<span className="capitalize">{column.category}</span>
								<span className="text-[#858585]">{column.count}</span>
							</div>
							<div className="flex flex-col gap-1.5">
								{column.rows.map(([email, score, reason]) => (
									<div
										className="rounded-lg border border-white/[0.08] bg-[#111111] p-2"
										key={email}
									>
										<div className="truncate text-[11px] font-medium">{email}</div>
										<div className="mt-1 flex items-center justify-between text-[10px] text-[#858585]">
											<span className="truncate">{reason}</span>
											<span className="tabular-nums">{score}</span>
										</div>
									</div>
								))}
							</div>
						</div>
					))}
				</div>
			</div>
		</MockShell>
	);
}

/* ---------- Fix plan table + result detail ---------- */

const FIX_ROWS: [string, Category, string][] = [
	["maya@brightlane.co", "valid", "Keep"],
	["anna@gmial.com", "invalid", "Fix → anna@gmail.com"],
	["dev@acmecloud.io", "risky", "Review"],
	["r.lee@closedco.com", "invalid", "Suppress"],
	["kai@greylist.co", "unknown", "Re-check"],
];

export function FixPlanTableMock() {
	return (
		<MockShell className="border border-white/[0.08] text-[11.5px] shadow-[0_1px_2px_rgb(0_0_0/0.04)]">
			<div className="grid grid-cols-[1.4fr_0.8fr_1fr] border-b border-white/[0.06] bg-[#181818] px-3 py-2 text-[10.5px] text-[#858585]">
				<span>Address</span>
				<span>Result</span>
				<span>Action</span>
			</div>
			{FIX_ROWS.map(([email, category, action]) => (
				<div
					className="grid grid-cols-[1.4fr_0.8fr_1fr] items-center border-b border-white/[0.04] px-3 py-2 last:border-0"
					key={email}
				>
					<span className="truncate">{email}</span>
					<span>
						<CategoryBadge category={category} />
					</span>
					<span className="truncate text-[#d9d9d9]">{action}</span>
				</div>
			))}
		</MockShell>
	);
}

/* ---------- Bento minis ---------- */

export function SingleCheckMini() {
	return (
		<MockShell className="border border-white/[0.08] p-3 text-[11.5px]">
			<div className="mb-3 flex items-center gap-2 border border-white/[0.08] px-2 py-1.5 text-[#858585]">
				<MagnifyingGlass aria-hidden="true" className="size-3" />
				<span className="text-white">theo@northwind.io</span>
				<span className="ml-auto bg-white rounded-full px-2 py-0.5 text-[10px] text-[#090909]">
					Check
				</span>
			</div>
			<div className="flex items-center justify-between">
				<CategoryBadge category="valid" />
				<span className="text-[10.5px] text-[#858585]">Score 94 · safe to send</span>
			</div>
			<div className="mt-2 grid grid-cols-3 gap-1.5 text-[10px]">
				{["Format", "MX", "Mailbox"].map((label) => (
					<span
						className="flex items-center gap-1 bg-[#181818] px-1.5 py-1 text-[#d9d9d9]"
						key={label}
					>
						<Check aria-hidden="true" className="size-3 text-[#4ade80]" /> {label}
					</span>
				))}
			</div>
		</MockShell>
	);
}

export function ScoreMini() {
	const rows: [string, Category, string][] = [
		["dev@acmecloud.io", "risky", "catch-all · high"],
		["hello@quarrylabs.com", "risky", "role account"],
		["jo@inboxfull.net", "risky", "full inbox"],
	];
	return (
		<MockShell className="border border-white/[0.08] text-[11.5px]">
			<div className="flex items-center justify-between border-b border-white/[0.06] px-3 py-2 text-[10.5px] text-[#858585]">
				<span>Reasons</span>
				<span>Score</span>
			</div>
			{rows.map(([email, category, reason], index) => (
				<div
					className="flex items-center justify-between border-b border-white/[0.04] px-3 py-2 last:border-0"
					key={email}
				>
					<div className="min-w-0">
						<div className="truncate">{email}</div>
						<div className="mt-0.5 flex items-center gap-1.5">
							<CategoryBadge category={category} />
							<span className="text-[10px] text-[#858585]">{reason}</span>
						</div>
					</div>
					<span className="tabular-nums text-[#d9d9d9]">{[75, 65, 70][index]}</span>
				</div>
			))}
		</MockShell>
	);
}

export function FeedbackStreamMini() {
	const events: [string, string, string, boolean][] = [
		["SendGrid", "bounce", "r.lee@closedco.com", true],
		["Mailgun", "complaint", "kim@quietmail.co", true],
		["Amazon SES", "delivered", "maya@brightlane.co", false],
		["Postmark", "unsubscribe", "lou@fieldnote.app", true],
	];
	return (
		<MockShell className="border border-white/[0.08] text-[11.5px]">
			<div className="flex items-center justify-between border-b border-white/[0.06] px-3 py-2">
				<span className="flex items-center gap-1.5 font-medium">
					<PlugsConnected aria-hidden="true" className="size-3.5" /> Delivery events
				</span>
				<span className="text-[10.5px] text-[#4ade80]">● live</span>
			</div>
			{events.map(([provider, event, email, suppressed]) => (
				<div
					className="grid grid-cols-[82px_70px_1fr_auto] items-center gap-2 border-b border-white/[0.04] px-3 py-2 last:border-0"
					key={email}
				>
					<span className="text-[#d9d9d9]">{provider}</span>
					<span className="text-[#858585]">{event}</span>
					<span className="truncate">{email}</span>
					{suppressed ? (
						<span className="bg-[#f87171]/10 rounded-full px-2 py-0.5 text-[10px] text-[#f87171]">
							suppressed
						</span>
					) : (
						<span className="bg-[#4ade80]/10 rounded-full px-2 py-0.5 text-[10px] text-[#4ade80]">
							recorded
						</span>
					)}
				</div>
			))}
		</MockShell>
	);
}

export function SuppressionMini() {
	const rows: [string, string][] = [
		["r.lee@closedco.com", "bounce"],
		["kim@quietmail.co", "complaint"],
		["lou@fieldnote.app", "unsubscribe"],
		["ops@nomx.dev", "invalid"],
	];
	return (
		<MockShell className="border border-white/[0.08] text-[11.5px]">
			<div className="flex items-center justify-between border-b border-white/[0.06] px-3 py-2">
				<span className="flex items-center gap-1.5 font-medium">
					<Prohibit aria-hidden="true" className="size-3.5" /> Suppressions
				</span>
				<span className="text-[10.5px] text-[#858585]">1,284</span>
			</div>
			{rows.map(([email, reason]) => (
				<div
					className="flex items-center justify-between border-b border-white/[0.04] px-3 py-2 last:border-0"
					key={email}
				>
					<span className="truncate">{email}</span>
					<span className="border border-white/[0.08] rounded-full px-2 py-0.5 text-[10px] text-[#adadad]">
						{reason}
					</span>
				</div>
			))}
		</MockShell>
	);
}

export function RecheckMini() {
	return (
		<MockShell className="border border-white/[0.08] text-[11.5px]">
			<div className="flex items-center justify-between border-b border-white/[0.06] px-3 py-2">
				<span className="flex items-center gap-1.5 font-medium">
					<ClockCounterClockwise aria-hidden="true" className="size-3.5" /> Re-checks
				</span>
				<span className="bg-[#4ade80]/10 rounded-full px-2 py-0.5 text-[10px] text-[#4ade80]">
					on
				</span>
			</div>
			{[
				["Results stay fresh for", "30 days"],
				["Next run", "Tonight, 2:00"],
				["Stale addresses", "1,906"],
				["Changed last run", "38"],
			].map(([label, value]) => (
				<div
					className="flex items-center justify-between border-b border-white/[0.04] px-3 py-2 last:border-0"
					key={label}
				>
					<span className="text-[#858585]">{label}</span>
					<span className="tabular-nums">{value}</span>
				</div>
			))}
		</MockShell>
	);
}

export function TeamMini() {
	return (
		<MockShell className="border border-white/[0.08] text-[11.5px]">
			<div className="flex items-center justify-between border-b border-white/[0.06] px-3 py-2">
				<span className="flex items-center gap-1.5 font-medium">
					<UserCircle aria-hidden="true" className="size-3.5" /> Members
				</span>
				<span className="text-[10.5px] text-[#858585]">4 of 10 seats</span>
			</div>
			{[
				["Maya Okafor", "Owner"],
				["Theo Brandt", "Admin"],
				["Ines Duarte", "Member"],
			].map(([name, role]) => (
				<div
					className="flex items-center justify-between border-b border-white/[0.04] px-3 py-2 last:border-0"
					key={name}
				>
					<span className="flex items-center gap-2">
						<span className="inline-flex size-5 items-center justify-center rounded-full bg-[#2e2e2e] text-[9px] text-[#adadad]">
							{name
								.split(" ")
								.map((part) => part[0])
								.join("")}
						</span>
						{name}
					</span>
					<span className="text-[#858585]">{role}</span>
				</div>
			))}
		</MockShell>
	);
}

const INTEGRATION_NAMES = [
	"SendGrid",
	"Mailgun",
	"Postmark",
	"Amazon SES",
	"CSV files",
	"REST API",
	"Go SDK",
	"TypeScript SDK",
	"Webhooks",
] as const;

export function IntegrationChipsMini() {
	return (
		<MockShell className="border border-white/[0.08] p-3 text-[11.5px]">
			<div className="mb-3 flex items-center justify-between">
				<span className="flex items-center gap-1.5 font-medium">
					<PlugsConnected aria-hidden="true" className="size-3.5" /> Connections
				</span>
				<span className="text-[10.5px] text-[#4ade80]">✓ 4 connected</span>
			</div>
			<div className="flex flex-wrap gap-1.5">
				{INTEGRATION_NAMES.map((name, index) => (
					<span
						className={cn(
							"border px-2 py-1",
							index < 4
								? "border-[#4ade80]/30 bg-[#4ade80]/10 text-[#86efac]"
								: "border-white/[0.08] bg-[#111111] text-[#d9d9d9]",
						)}
						key={name}
					>
						{name}
					</span>
				))}
			</div>
		</MockShell>
	);
}

/* ---------- Report, comparison, API, gate (use-case tabs) ---------- */

export function QualityReportMock() {
	const bars: [Category, number, string][] = [
		["valid", 84, "3,812"],
		["risky", 9, "412"],
		["invalid", 5, "236"],
		["unknown", 2, "51"],
	];
	return (
		<MockShell className="border border-white/[0.08] p-4 text-[12px]">
			<div className="mb-1 flex items-center justify-between">
				<span className="font-semibold">List quality · October newsletter</span>
				<span className="text-[11px] text-[#858585]">4,511 addresses</span>
			</div>
			<p className="mb-4 text-[11px] text-[#858585]">
				Score 88. Safe to send: 3,812. Remove or review: 699.
			</p>
			<div className="flex flex-col gap-2.5">
				{bars.map(([category, percent, count]) => (
					<div className="grid grid-cols-[70px_1fr_48px] items-center gap-3" key={category}>
						<span className="capitalize text-[#d9d9d9]">{category}</span>
						<span className="h-5 bg-[#1d1d1d]">
							<span
								className={cn("block h-full", CATEGORY_DOT[category])}
								style={{ width: `${Math.max(percent, 3)}%` }}
							/>
						</span>
						<span className="text-right tabular-nums text-[#d9d9d9]">{count}</span>
					</div>
				))}
			</div>
		</MockShell>
	);
}

export function SourceCompareMock() {
	const rows: [string, number, string][] = [
		["Webinar signups", 92, "1.1% invalid"],
		["CRM export", 81, "6.4% invalid"],
		["Trade show scans", 63, "14.9% invalid"],
		["Purchased list", 38, "31.2% invalid"],
	];
	return (
		<MockShell className="border border-white/[0.08] text-[12px]">
			<div className="grid grid-cols-[1.4fr_0.6fr_1fr] border-b border-white/[0.06] bg-[#181818] px-4 py-2 text-[10.5px] text-[#858585]">
				<span>Source</span>
				<span>Quality</span>
				<span>Invalid share</span>
			</div>
			{rows.map(([source, quality, invalid]) => (
				<div
					className="grid grid-cols-[1.4fr_0.6fr_1fr] items-center border-b border-white/[0.04] px-4 py-2.5 last:border-0"
					key={source}
				>
					<span>{source}</span>
					<span className="tabular-nums">{quality}</span>
					<span className="flex items-center gap-2">
						<span className="h-1.5 flex-1 bg-[#1d1d1d]">
							<span
								className={cn(
									"block h-full",
									quality > 80 ? "bg-[#22c55e]" : quality > 60 ? "bg-[#f59e0b]" : "bg-[#ef4444]",
								)}
								style={{ width: `${100 - quality}%` }}
							/>
						</span>
						<span className="w-20 text-right text-[11px] text-[#858585]">{invalid}</span>
					</span>
				</div>
			))}
		</MockShell>
	);
}

export function ApiResponseMock() {
	return (
		<div className="ed-mono overflow-hidden rounded-xl border border-white/[0.08] bg-[#090909] text-[11.5px] leading-[1.65] text-[#e7e5e4]">
			<div className="flex items-center gap-1.5 border-b border-white/10 px-3 py-2">
				<span className="size-2 rounded-full bg-[#ef4444]" />
				<span className="size-2 rounded-full bg-[#f59e0b]" />
				<span className="size-2 rounded-full bg-[#22c55e]" />
				<span className="ml-2 text-[10.5px] text-[#858585]">signup form · real-time check</span>
			</div>
			<pre className="overflow-x-auto p-4">
				<code>
					<span className="text-[#858585]">{"// POST /v1/check_email"}</span>
					{"\n"}
					{"{ "}
					<span className="text-[#93c5fd]">{'"to_email"'}</span>
					{": "}
					<span className="text-[#86efac]">{'"anna@gmial.com"'}</span>
					{" }\n\n"}
					<span className="text-[#858585]">{"// response (shortened)"}</span>
					{"\n{\n  "}
					<span className="text-[#93c5fd]">{'"is_reachable"'}</span>
					{": "}
					<span className="text-[#86efac]">{'"invalid"'}</span>
					{",\n  "}
					<span className="text-[#93c5fd]">{'"score"'}</span>
					{": {\n    "}
					<span className="text-[#93c5fd]">{'"category"'}</span>
					{": "}
					<span className="text-[#86efac]">{'"invalid"'}</span>
					{",\n    "}
					<span className="text-[#93c5fd]">{'"safe_to_send"'}</span>
					{": "}
					<span className="text-[#fca5a5]">false</span>
					{",\n    "}
					<span className="text-[#93c5fd]">{'"domain_suggestion"'}</span>
					{": "}
					<span className="text-[#86efac]">{'"gmail.com"'}</span>
					{"\n  }\n}"}
				</code>
			</pre>
		</div>
	);
}

export function QualityGateMock() {
	return (
		<MockShell className="border border-white/[0.08] p-4 text-[12px]">
			<div className="mb-3 flex items-center justify-between">
				<span className="font-semibold">Customer upload · acme-launch.csv</span>
				<span className="bg-[#fbbf24]/10 rounded-full px-2 py-0.5 text-[10.5px] text-[#fbbf24]">
					held for review
				</span>
			</div>
			{[
				["List quality score", "41 (threshold 70)"],
				["Invalid addresses", "18.2%"],
				["Spam-trap domains", "3"],
				["Job webhook", "job.completed sent"],
			].map(([label, value]) => (
				<div
					className="flex items-center justify-between border-t border-white/[0.06] py-2"
					key={label}
				>
					<span className="text-[#858585]">{label}</span>
					<span className="tabular-nums">{value}</span>
				</div>
			))}
			<div className="mt-3 flex gap-2">
				<span className="bg-white px-2 py-1 text-[11px] text-[#090909]">Send safe rows only</span>
				<span className="border border-white/[0.1] px-2 py-1 text-[11px]">
					Ask customer to clean
				</span>
			</div>
		</MockShell>
	);
}

/* ---------- Suppression table (peach section) ---------- */

/* ---------- Phone signup ---------- */

/* ---------- Terminal (API section) ---------- */

/* ---------- Security ---------- */

export function RetentionMock() {
	return (
		<MockShell className="border border-white/[0.08] p-4 text-[12px] shadow-[0_1px_2px_rgb(0_0_0/0.04),0_16px_40px_-16px_rgb(0_0_0/0.25)]">
			<div className="mb-3 flex items-center justify-between">
				<span className="flex items-center gap-1.5 font-medium">
					<ClockCounterClockwise aria-hidden="true" className="size-3.5" /> Result retention
				</span>
				<span className="bg-[#4ade80]/10 rounded-full px-2 py-0.5 text-[10.5px] text-[#4ade80]">
					+ organization only
				</span>
			</div>
			<p className="mb-1.5 text-[10px] tracking-wide text-[#858585] uppercase">Keep results for</p>
			{[
				["30 days", "Default", true],
				["90 days", "For quarterly reviews", false],
				["365 days", "For audits", false],
			].map(([label, hint, selected]) => (
				<div
					className={cn(
						"mb-1.5 flex items-center justify-between border px-3 py-2",
						selected ? "border-[#155dfc] bg-[#155dfc]/15" : "border-white/[0.08]",
					)}
					key={label as string}
				>
					<span>
						<span className="block font-medium">{label as string}</span>
						<span className="block text-[10.5px] text-[#858585]">{hint as string}</span>
					</span>
					<span
						className={cn(
							"inline-flex size-4 items-center justify-center rounded-full border",
							selected ? "border-[#155dfc] bg-[#155dfc] text-white" : "border-white/20",
						)}
					>
						{selected ? <Check aria-hidden="true" className="size-2.5" /> : null}
					</span>
				</div>
			))}
			<p className="mt-3 text-[10.5px] text-[#858585]">
				Scoped API keys · Signed webhooks · No email sent
			</p>
		</MockShell>
	);
}

export function ApiKeysMock() {
	const keys: [string, string, string, boolean][] = [
		["Signup form", "verify", "Oct 02", true],
		["Nightly CRM sync", "bulk, lists", "Sep 18", true],
		["Old import script", "bulk", "Jun 11", false],
	];
	return (
		<MockShell className="border border-white/[0.08] text-[11.5px] shadow-[0_1px_2px_rgb(0_0_0/0.04),0_16px_40px_-16px_rgb(0_0_0/0.25)]">
			<div className="border-b border-white/[0.06] px-3 py-2 font-medium">API keys</div>
			<div className="grid grid-cols-[1.3fr_1fr_0.7fr_0.7fr] border-b border-white/[0.06] bg-[#181818] px-3 py-1.5 text-[10px] text-[#858585]">
				<span>Name</span>
				<span>Scopes</span>
				<span>Created</span>
				<span>Status</span>
			</div>
			{keys.map(([name, scopes, created, active]) => (
				<div
					className="grid grid-cols-[1.3fr_1fr_0.7fr_0.7fr] items-center border-b border-white/[0.04] px-3 py-2 last:border-0"
					key={name}
				>
					<span>{name}</span>
					<span className="text-[#adadad]">{scopes}</span>
					<span className="text-[#858585]">{created}</span>
					<span className={active ? "text-[#4ade80]" : "text-[#858585]"}>
						{active ? "● Active" : "Revoked"}
					</span>
				</div>
			))}
		</MockShell>
	);
}
