"use client";

/**
 * Animated figures for the home page, drawn like the plates on
 * playbookmedia.io: black ground, hairlines, mono labels, dimension lines, and
 * plain markers instead of colored badges. Each figure moves only while it is
 * on screen and rests in its finished state on the server render and when the
 * visitor prefers reduced motion. Sample data uses fictional domains.
 */
import { AnimatePresence, animate, motion, useInView, useReducedMotion } from "framer-motion";
import { type ReactNode, type RefObject, useEffect, useRef, useState } from "react";

import { cn } from "@/lib/utils";

import { Sheet } from "./ui";

const EASE = [0.16, 1, 0.3, 1] as const;
const FIG = "#9db8ff";
const LINE = "rgb(255 255 255 / 0.22)";
const LABEL = "ed-fig-label text-[#9e9e9e]";

/**
 * Steps through `steps` states every `ms` while `ref` is in view. It starts
 * at `settled`, so the server render and reduced motion show the finished state.
 */
function useTicker(ref: RefObject<Element | null>, steps: number, ms: number, settled = steps - 1) {
	const inView = useInView(ref, { amount: 0.3 });
	const reduce = useReducedMotion();
	const [tick, setTick] = useState(settled);
	useEffect(() => {
		if (reduce || !inView) return;
		const id = window.setInterval(() => setTick((value) => (value + 1) % steps), ms);
		return () => window.clearInterval(id);
	}, [inView, reduce, steps, ms]);
	return tick;
}

function format(value: number, decimals = 0) {
	return value.toLocaleString("en-US", {
		minimumFractionDigits: decimals,
		maximumFractionDigits: decimals,
	});
}

/** Counts up once when it scrolls into view. */
export function CountUp({ to, decimals = 0 }: { to: number; decimals?: number }) {
	const ref = useRef<HTMLSpanElement>(null);
	const inView = useInView(ref, { once: true });
	const reduce = useReducedMotion();
	const [value, setValue] = useState(to);
	useEffect(() => {
		if (reduce || !inView) return;
		const controls = animate(0, to, { duration: 1.4, ease: EASE, onUpdate: setValue });
		return () => controls.stop();
	}, [inView, reduce, to]);
	return (
		<span className="tabular-nums" ref={ref}>
			{format(value, decimals)}
		</span>
	);
}

/** Fades content up the first time it scrolls into view. */
export function Reveal({
	className,
	delay = 0,
	children,
}: {
	className?: string;
	delay?: number;
	children: ReactNode;
}) {
	return (
		<motion.div
			className={className}
			initial={{ opacity: 0, y: 16 }}
			transition={{ duration: 0.7, delay, ease: EASE }}
			viewport={{ once: true, amount: 0.2 }}
			whileInView={{ opacity: 1, y: 0 }}
		>
			{children}
		</motion.div>
	);
}

/* ------------------------------------------------------------------ */
/* Drawing parts                                                       */
/* ------------------------------------------------------------------ */

type Kind = "valid" | "risky" | "invalid" | "unknown";

const KIND_COLOR: Record<Kind, string> = {
	valid: "text-[var(--ed-fig)]",
	risky: "text-[#d9d9d9]",
	invalid: "text-[#fafafa]",
	unknown: "text-[#9e9e9e]",
};

/** Result marker: filled for valid, half for risky, a cross for invalid, dashed for unknown. */
export function Marker({ kind, className }: { kind: Kind; className?: string }) {
	return (
		<svg
			aria-hidden="true"
			className={cn("inline-block size-2.5 shrink-0", KIND_COLOR[kind], className)}
			viewBox="0 0 10 10"
		>
			<MarkerShape kind={kind} />
		</svg>
	);
}

/** The marker's shape in a 10 by 10 box, for use inside a larger drawing. */
function MarkerShape({ kind }: { kind: Kind }) {
	if (kind === "valid") return <circle cx="5" cy="5" fill="currentColor" r="3.5" />;
	if (kind === "risky") {
		return (
			<>
				<circle cx="5" cy="5" fill="#090909" r="3.5" stroke="currentColor" />
				<path d="M5 1.5a3.5 3.5 0 0 1 0 7z" fill="currentColor" />
			</>
		);
	}
	if (kind === "invalid") {
		return <path d="M1.5 1.5l7 7M8.5 1.5l-7 7" stroke="currentColor" strokeWidth="1.2" />;
	}
	return (
		<circle cx="5" cy="5" fill="none" r="3.5" stroke="currentColor" strokeDasharray="1.5 1.5" />
	);
}

const KIND_HEX: Record<Kind, string> = {
	valid: FIG,
	risky: "#d9d9d9",
	invalid: "#fafafa",
	unknown: "#9e9e9e",
};

/** A marker placed at a point inside an SVG drawing. */
function PlacedMarker({ kind, x, y }: { kind: Kind; x: number; y: number }) {
	return (
		<g color={KIND_HEX[kind]} transform={`translate(${x - 5} ${y - 5})`}>
			<MarkerShape kind={kind} />
		</g>
	);
}

/** Dimension line with end ticks and a label above it. */
function Dimension({
	label,
	align = "center",
	className,
}: {
	label: string;
	align?: "start" | "center" | "end";
	className?: string;
}) {
	return (
		<div className={cn("relative pt-5", className)}>
			<span
				className={cn(
					"ed-fig-label absolute top-0 whitespace-nowrap text-[var(--ed-fig)]",
					align === "start" && "left-0",
					align === "center" && "left-1/2 -translate-x-1/2",
					align === "end" && "right-0",
				)}
			>
				{label}
			</span>
			<div className="relative h-2.5">
				<span className="absolute inset-x-0 top-1/2 h-px bg-[var(--ed-fig)]/60" />
				<span className="absolute inset-y-0 left-0 w-px bg-[var(--ed-fig)]" />
				<span className="absolute inset-y-0 right-0 w-px bg-[var(--ed-fig)]" />
			</div>
		</div>
	);
}

/** Axis ruler with minor ticks, major ticks, and labels under the majors. */
function Ruler({ steps, major, labels }: { steps: number; major: number; labels: string[] }) {
	return (
		<div className="relative h-7">
			<span className="absolute inset-x-0 top-0 h-px bg-white/25" />
			{Array.from({ length: steps + 1 }, (_, index) => (
				<span
					className={cn("absolute top-0 w-px bg-white/25", index % major === 0 ? "h-2" : "h-1")}
					key={index}
					style={{ left: `${(index / steps) * 100}%` }}
				/>
			))}
			{labels.map((label, index) => (
				<span
					className="ed-fig-label absolute top-3 -translate-x-1/2 text-[9.5px] text-[#7a7a7a]"
					key={label}
					style={{ left: `${(index / (labels.length - 1)) * 100}%` }}
				>
					{label}
				</span>
			))}
		</div>
	);
}

/**
 * Reveals its content left to right the first time it scrolls into view. The
 * wrapper is observed, not the clipped child, because a fully clipped element
 * never counts as visible.
 */
function Wipe({
	className,
	duration = 1.2,
	children,
}: {
	className?: string;
	duration?: number;
	children: ReactNode;
}) {
	const ref = useRef<HTMLDivElement>(null);
	const inView = useInView(ref, { once: true, amount: 0.5 });
	return (
		<div ref={ref}>
			<motion.div
				animate={inView ? { clipPath: "inset(0 0% 0 0)" } : undefined}
				className={className}
				initial={{ clipPath: "inset(0 100% 0 0)" }}
				transition={{ duration, ease: EASE }}
			>
				{children}
			</motion.div>
		</div>
	);
}

const SEGMENT_FILL: Record<Kind, string> = {
	valid: "border-[var(--ed-fig)] bg-[#9db8ff]/15",
	risky: "ed-hatch border-white/50",
	invalid: "ed-hatch-fine border-white/50",
	unknown: "border-dashed border-white/40",
};

/** One horizontal bar split into result segments, drawn left to right on view. */
function CompositionBar({ parts, total }: { parts: [Kind, number][]; total: number }) {
	return (
		<Wipe className="flex h-7 gap-px">
			{parts.map(([kind, count]) => (
				<span
					className={cn("h-full border", SEGMENT_FILL[kind])}
					key={kind}
					style={{ width: `${(count / total) * 100}%` }}
				/>
			))}
		</Wipe>
	);
}

function Legend({ parts, counts = true }: { parts: [Kind, number][]; counts?: boolean }) {
	return (
		<ul className="flex flex-wrap gap-x-5 gap-y-1.5">
			{parts.map(([kind, count]) => (
				<li className="flex items-center gap-1.5" key={kind}>
					<Marker kind={kind} />
					<span className={LABEL}>
						{kind} {counts ? <span className="text-[#d9d9d9]">{format(count)}</span> : null}
					</span>
				</li>
			))}
		</ul>
	);
}

/* ------------------------------------------------------------------ */
/* Fig. 1 — hero                                                       */
/* ------------------------------------------------------------------ */

const NEWSLETTER: [Kind, number][] = [
	["valid", 3812],
	["risky", 412],
	["invalid", 236],
	["unknown", 51],
];

/* Checks per day, October 1 to 14. Day 11 is a list upload. */
const DAILY = [1820, 2410, 1950, 3120, 2880, 860, 640, 3340, 3610, 3190, 12400, 3460, 1100, 3980];
const DAILY_MAX = 14000;

const LOG: [string, Kind, number][] = [
	["maya@brightlane.co", "valid", 100],
	["dev@acmecloud.io", "risky", 75],
	["anna@gmial.com", "invalid", 0],
	["theo@northwind.io", "valid", 100],
	["kai@greylist.co", "unknown", 25],
	["ines@harborops.com", "valid", 100],
	["hello@quarrylabs.com", "risky", 65],
	["ops@nomx.dev", "invalid", 0],
	["sam@fieldnote.app", "valid", 90],
];

function clock(sequence: number) {
	const seconds = 10 * 3600 + 42 * 60 + sequence * 3;
	const pad = (value: number) => String(value).padStart(2, "0");
	return `${pad(Math.floor(seconds / 3600))}:${pad(Math.floor(seconds / 60) % 60)}:${pad(seconds % 60)}`;
}

function DailyChart() {
	const points = DAILY.map((value, index) => ({
		x: (index / (DAILY.length - 1)) * 100,
		y: 100 - (value / DAILY_MAX) * 100,
	}));
	const peak = points[10];
	return (
		<div className="grid grid-cols-[34px_1fr] gap-2">
			<div className="relative h-[150px]">
				{["14K", "7K", "0"].map((label, index) => (
					<span
						className="ed-fig-label absolute right-0 -translate-y-1/2 text-[9.5px] text-[#7a7a7a]"
						key={label}
						style={{ top: `${index * 50}%` }}
					>
						{label}
					</span>
				))}
			</div>
			<div>
				<Wipe className="relative h-[150px] border-b border-l border-white/25" duration={1.6}>
					<svg
						aria-hidden="true"
						className="absolute inset-0 size-full overflow-visible"
						preserveAspectRatio="none"
						viewBox="0 0 100 100"
					>
						<line
							stroke={LINE}
							strokeDasharray="2 3"
							vectorEffect="non-scaling-stroke"
							x1="0"
							x2="100"
							y1="50"
							y2="50"
						/>
						<polyline
							fill="none"
							points={points.map((point) => `${point.x},${point.y}`).join(" ")}
							stroke="#fafafa"
							strokeWidth="1.4"
							vectorEffect="non-scaling-stroke"
						/>
					</svg>
					{points.map((point, index) => (
						<span
							className={cn(
								"absolute size-[7px] -translate-x-1/2 -translate-y-1/2 rounded-full border bg-[#090909]",
								index === 10 ? "border-[var(--ed-fig)]" : "border-white/70",
							)}
							key={index}
							style={{ left: `${point.x}%`, top: `${point.y}%` }}
						/>
					))}
					<span
						className="absolute w-px -translate-x-1/2 bg-[var(--ed-fig)]/60"
						style={{ left: `${peak.x}%`, top: 0, height: `${peak.y}%` }}
					/>
					<span
						className="ed-fig-label absolute top-0 -translate-x-full pr-2 whitespace-nowrap text-[var(--ed-fig)]"
						style={{ left: `${peak.x}%` }}
					>
						Oct 11 · list upload · 12,400
					</span>
				</Wipe>
				<div className="mt-2 flex justify-between">
					{["Oct 1", "Oct 7", "Oct 14"].map((label) => (
						<span className="ed-fig-label text-[9.5px] text-[#7a7a7a]" key={label}>
							{label}
						</span>
					))}
				</div>
			</div>
		</div>
	);
}

function CheckLog() {
	const ref = useRef<HTMLDivElement>(null);
	const tick = useTicker(ref, 60, 1600, 0);
	const rows = Array.from({ length: 9 }, (_, index) => tick + index);
	return (
		<div ref={ref}>
			<p className={LABEL}>1.3 Check log</p>
			<ul className="mt-4 overflow-hidden">
				<AnimatePresence initial={false}>
					{rows.map((sequence) => {
						const [email, kind, score] = LOG[sequence % LOG.length];
						return (
							<motion.li
								animate={{ opacity: 1, y: 0 }}
								className="ed-mono grid grid-cols-[58px_1fr_auto] items-center gap-3 border-b border-dashed border-white/10 py-[7px] text-[11px]"
								exit={{ opacity: 0 }}
								initial={{ opacity: 0, y: 8 }}
								key={sequence}
								layout
								transition={{ duration: 0.4, ease: EASE }}
							>
								<span className="text-[#6b6b6b]">{clock(sequence)}</span>
								<span className="truncate text-[#d9d9d9]">{email}</span>
								<span className="flex items-center gap-2">
									<Marker kind={kind} />
									<span className="w-6 text-right text-[#9e9e9e] tabular-nums">{score}</span>
								</span>
							</motion.li>
						);
					})}
				</AnimatePresence>
			</ul>
		</div>
	);
}

/** Hero figure: a list's composition, daily check volume, and a running check log. */
export function HeroConsole() {
	const total = NEWSLETTER.reduce((sum, [, count]) => sum + count, 0);
	const keep = NEWSLETTER[0][1];
	return (
		<Sheet
			bodyClassName="grid lg:grid-cols-[1.6fr_1fr]"
			caption="Sample workspace · fictional domains · no email is sent during a check"
			fig="1"
			title="Workspace report · Northwind"
		>
			<div className="flex flex-col gap-10 p-4 sm:p-6 lg:border-r lg:border-white/10">
				<div>
					<p className={LABEL}>
						1.1 October newsletter · <CountUp to={total} /> addresses
					</p>
					<div className="mt-6 flex">
						<Dimension align="start" className="w-[84.5%]" label={`Keep · ${format(keep)}`} />
						<Dimension
							align="end"
							className="w-[15.5%]"
							label={`Review or remove · ${format(total - keep)}`}
						/>
					</div>
					<div className="mt-2">
						<CompositionBar parts={NEWSLETTER} total={total} />
					</div>
					<div className="mt-2">
						<Ruler labels={["0", "1.5K", "3K", "4.5K"]} major={5} steps={15} />
					</div>
					<div className="mt-3">
						<Legend parts={NEWSLETTER} />
					</div>
				</div>
				<div>
					<p className={LABEL}>1.2 Checks per day · workspace</p>
					<div className="mt-5">
						<DailyChart />
					</div>
				</div>
			</div>
			<div className="border-t border-white/10 p-4 sm:p-6 lg:border-t-0">
				<CheckLog />
			</div>
		</Sheet>
	);
}

/* ------------------------------------------------------------------ */
/* Fig. 3 — building blocks                                            */
/* ------------------------------------------------------------------ */

const svgLabel = {
	className: "ed-mono",
	fill: "#9e9e9e",
	fontSize: 8.5,
	letterSpacing: "0.08em",
} as const;

const STATIONS = [
	{ x: 84, label: "FORMAT" },
	{ x: 138, label: "DOMAIN" },
	{ x: 192, label: "MX" },
	{ x: 246, label: "MAILBOX" },
];

/** Single check: the address passes four readings and gets a result. */
export function SingleCheckDemo() {
	const ref = useRef<SVGSVGElement>(null);
	const tick = useTicker(ref, 9, 700, 5);
	const passed = Math.min(tick, STATIONS.length);
	const done = tick > STATIONS.length;
	const markerX = done ? 292 : passed === 0 ? 40 : STATIONS[passed - 1].x;
	return (
		<svg aria-hidden="true" className="h-auto w-full" ref={ref} viewBox="0 0 320 190">
			<text {...svgLabel} x="16" y="20">
				INPUT
			</text>
			<rect fill="#090909" height="26" stroke={LINE} width="168" x="16" y="28" />
			<text className="ed-mono" fill="#fafafa" fontSize="10.5" x="26" y="45">
				theo@northwind.io
			</text>
			<path d="M40 54 V112 H292" fill="none" stroke={LINE} />
			{STATIONS.map((station, index) => (
				<g key={station.label}>
					<circle
						cx={station.x}
						cy="112"
						fill={index < passed ? FIG : "#090909"}
						r="5"
						stroke={index < passed ? FIG : "rgb(255 255 255 / 0.4)"}
						style={{ transition: "fill 300ms, stroke 300ms" }}
					/>
					<text {...svgLabel} textAnchor="middle" x={station.x} y="134">
						{station.label}
					</text>
				</g>
			))}
			<circle cx="292" cy="112" fill="#090909" r="8" stroke={done ? FIG : LINE} />
			<circle cx="292" cy="112" fill={done ? FIG : "transparent"} r="3" />
			<motion.circle animate={{ cx: markerX }} cy="112" fill="#fafafa" initial={false} r="2.5" />
			<text
				{...svgLabel}
				fill={FIG}
				opacity={done ? 1 : 0}
				style={{ transition: "opacity 400ms" }}
				textAnchor="end"
				x="300"
				y="92"
			>
				VALID · SCORE 94
			</text>
			<path
				d="M84 154 V164 M84 159 H246 M246 154 V164"
				fill="none"
				stroke={FIG}
				strokeOpacity="0.7"
			/>
			<text {...svgLabel} fill={FIG} textAnchor="middle" x="165" y="180">
				NO EMAIL SENT
			</text>
		</svg>
	);
}

const LANES: { y: number; label: string; count: string; kind: Kind }[] = [
	{ y: 34, label: "KEEP", count: "3,812", kind: "valid" },
	{ y: 76, label: "FIX TYPO", count: "64", kind: "risky" },
	{ y: 118, label: "REVIEW", count: "463", kind: "unknown" },
	{ y: 160, label: "SUPPRESS", count: "172", kind: "invalid" },
];

function lanePath(y: number) {
	return `M100 97 C 160 97, 170 ${y}, 232 ${y}`;
}

/** List cleaning: rows from one file fan out into four actions. */
export function ListSortDemo() {
	return (
		<svg aria-hidden="true" className="h-auto w-full" viewBox="0 0 320 190">
			<text {...svgLabel} x="16" y="20">
				NEWSLETTER.CSV
			</text>
			{Array.from({ length: 10 }, (_, index) => (
				<line
					key={index}
					stroke="rgb(255 255 255 / 0.3)"
					x1="16"
					x2={index % 3 === 0 ? 62 : 70}
					y1={36 + index * 13}
					y2={36 + index * 13}
				/>
			))}
			<text {...svgLabel} x="16" y="180">
				4,511 ROWS
			</text>
			<path d="M76 34 H84 V160 H76 M84 97 H100" fill="none" stroke={LINE} />
			{LANES.map((lane, index) => (
				<g key={lane.label}>
					<path d={lanePath(lane.y)} fill="none" stroke={LINE} />
					{[0, 1].map((offset) => (
						<circle className="motion-reduce:hidden" fill={FIG} key={offset} r="1.8">
							<animateMotion
								begin={`-${index * 0.35 + offset * 1.3}s`}
								dur="2.6s"
								path={lanePath(lane.y)}
								repeatCount="indefinite"
							/>
						</circle>
					))}
					<PlacedMarker kind={lane.kind} x={241} y={lane.y} />
					<text {...svgLabel} fill="#d9d9d9" x="252" y={lane.y - 1}>
						{lane.label}
					</text>
					<text {...svgLabel} x="252" y={lane.y + 10}>
						{lane.count}
					</text>
				</g>
			))}
		</svg>
	);
}

const SOURCES = [
	{ y: 44, label: "BOUNCE" },
	{ y: 96, label: "COMPLAINT" },
	{ y: 148, label: "UNSUBSCRIBE" },
];

function sourcePath(y: number) {
	return `M16 ${y + 8} H70 C 110 ${y + 8}, 110 96, 150 96`;
}

/** Suppression list: three kinds of events land on one list, and the next send skips them. */
export function SuppressionDemo() {
	const ref = useRef<SVGSVGElement>(null);
	const tick = useTicker(ref, 40, 1500, 0);
	return (
		<svg aria-hidden="true" className="h-auto w-full" ref={ref} viewBox="0 0 320 190">
			{SOURCES.map((source, index) => (
				<g key={source.label}>
					<text {...svgLabel} x="16" y={source.y}>
						{source.label}
					</text>
					<path d={sourcePath(source.y)} fill="none" stroke={LINE} />
					<circle className="motion-reduce:hidden" fill={FIG} r="1.8">
						<animateMotion
							begin={`-${index * 0.7}s`}
							dur="2.2s"
							path={sourcePath(source.y)}
							repeatCount="indefinite"
						/>
					</circle>
				</g>
			))}
			<rect fill="#090909" height="52" stroke={FIG} strokeOpacity="0.8" width="96" x="150" y="70" />
			<text {...svgLabel} fill={FIG} textAnchor="middle" x="198" y="88">
				SUPPRESSION
			</text>
			<text className="ed-mono" fill="#fafafa" fontSize="12" textAnchor="middle" x="198" y="108">
				{format(1284 + tick)}
			</text>
			<path d="M246 96 H296 M290 92 L296 96 L290 100" fill="none" stroke={LINE} />
			<text {...svgLabel} textAnchor="end" x="304" y="84">
				NEXT SEND
			</text>
			<PlacedMarker kind="invalid" x={271} y={114} />
			<text {...svgLabel} textAnchor="end" x="304" y="134">
				SKIPPED
			</text>
		</svg>
	);
}

/* ------------------------------------------------------------------ */
/* Fig. 4 — platform                                                   */
/* ------------------------------------------------------------------ */

const SCORED: { score: number; label: string; kind: Kind; row: number }[] = [
	{ score: 0, label: "ANNA@GMIAL.COM · 0", kind: "invalid", row: 0 },
	{ score: 75, label: "DEV@ACMECLOUD.IO · 75", kind: "risky", row: 1 },
	{ score: 100, label: "MAYA@BRIGHTLANE.CO · 100", kind: "valid", row: 2 },
];

/** Scores: three sample addresses placed on a 0 to 100 scale. */
export function ScoreScaleFigure() {
	const x = (score: number) => 20 + score * 2.8;
	return (
		<svg aria-hidden="true" className="h-auto w-full" viewBox="0 0 320 150">
			<line stroke="rgb(255 255 255 / 0.3)" x1="20" x2="300" y1="112" y2="112" />
			{Array.from({ length: 21 }, (_, index) => (
				<line
					key={index}
					stroke="rgb(255 255 255 / 0.3)"
					x1={20 + index * 14}
					x2={20 + index * 14}
					y1="112"
					y2={index % 10 === 0 ? 120 : 116}
				/>
			))}
			{[0, 50, 100].map((value) => (
				<text {...svgLabel} key={value} textAnchor="middle" x={x(value)} y="134">
					{value}
				</text>
			))}
			{SCORED.map((item, index) => {
				const top = 30 + item.row * 26;
				const anchor = item.score < 30 ? "start" : item.score > 80 ? "end" : "middle";
				return (
					<motion.g
						initial={{ opacity: 0 }}
						key={item.label}
						transition={{ duration: 0.5, delay: 0.2 + index * 0.25 }}
						viewport={{ once: true, amount: 0.6 }}
						whileInView={{ opacity: 1 }}
					>
						<line
							stroke={FIG}
							strokeOpacity="0.5"
							x1={x(item.score)}
							x2={x(item.score)}
							y1={top + 4}
							y2="104"
						/>
						<PlacedMarker kind={item.kind} x={x(item.score)} y={104} />
						<text {...svgLabel} fill="#d9d9d9" textAnchor={anchor} x={x(item.score)} y={top}>
							{item.label}
						</text>
					</motion.g>
				);
			})}
		</svg>
	);
}

type EventKind = "bounce" | "complaint" | "unsubscribe" | "delivered";

const EVENTS: { hour: number; kind: EventKind; label?: string; stem: number }[] = [
	{ hour: 2.5, kind: "bounce", label: "BOUNCE · SENDGRID", stem: 58 },
	{ hour: 5, kind: "delivered", stem: 18 },
	{ hour: 7.5, kind: "delivered", stem: 18 },
	{ hour: 9, kind: "complaint", label: "COMPLAINT · MAILGUN", stem: 34 },
	{ hour: 12, kind: "delivered", stem: 18 },
	{ hour: 14.5, kind: "unsubscribe", label: "UNSUBSCRIBE · POSTMARK", stem: 58 },
	{ hour: 17, kind: "delivered", stem: 18 },
	{ hour: 19.5, kind: "bounce", stem: 30 },
	{ hour: 22, kind: "delivered", stem: 18 },
];

/** Delivery feedback: a day of platform events on a time axis, read left to right. */
export function EventTimelineFigure() {
	const ref = useRef<SVGSVGElement>(null);
	const tick = useTicker(ref, 14, 450, 12);
	const x = (hour: number) => 20 + hour * (280 / 24);
	const scanHour = Math.min(tick * 2, 24);
	return (
		<svg aria-hidden="true" className="h-auto w-full" ref={ref} viewBox="0 0 320 150">
			<line stroke="rgb(255 255 255 / 0.3)" x1="20" x2="300" y1="112" y2="112" />
			{Array.from({ length: 25 }, (_, hour) => (
				<line
					key={hour}
					stroke="rgb(255 255 255 / 0.3)"
					x1={x(hour)}
					x2={x(hour)}
					y1="112"
					y2={hour % 6 === 0 ? 120 : 115}
				/>
			))}
			{["00", "06", "12", "18", "24"].map((label, index) => (
				<text {...svgLabel} key={label} textAnchor="middle" x={x(index * 6)} y="134">
					{label}
				</text>
			))}
			{EVENTS.map((event) => {
				const shown = event.hour <= scanHour;
				const ex = x(event.hour);
				const top = 112 - event.stem;
				const delivered = event.kind === "delivered";
				return (
					<g key={event.hour} opacity={shown ? 1 : 0.12} style={{ transition: "opacity 300ms" }}>
						<line
							stroke={delivered ? LINE : FIG}
							strokeOpacity="0.7"
							x1={ex}
							x2={ex}
							y1={top}
							y2="112"
						/>
						{delivered ? (
							<circle cx={ex} cy={top} fill="#090909" r="2.5" stroke="rgb(255 255 255 / 0.6)" />
						) : (
							<PlacedMarker kind="invalid" x={ex} y={top} />
						)}
						{event.label ? (
							<text
								{...svgLabel}
								fill="#d9d9d9"
								textAnchor={ex > 200 ? "end" : "start"}
								x={ex > 200 ? ex + 4 : ex - 4}
								y={top - 9}
							>
								{event.label}
							</text>
						) : null}
					</g>
				);
			})}
			<line stroke={FIG} x1={x(scanHour)} x2={x(scanHour)} y1="40" y2="122" />
			<text {...svgLabel} fill={FIG} textAnchor="end" x="300" y="20">
				LAST 24 H
			</text>
		</svg>
	);
}

/** Re-checks: a 30-day dial with the time since the last run and the next run. */
export function RecheckDialFigure() {
	const cx = 76;
	const cy = 75;
	const r = 50;
	const elapsed = 22;
	const angle = (day: number) => (day / 30) * Math.PI * 2 - Math.PI / 2;
	const point = (radius: number, a: number) => [
		Math.round((cx + radius * Math.cos(a)) * 100) / 100,
		Math.round((cy + radius * Math.sin(a)) * 100) / 100,
	];
	const [endX, endY] = point(r, angle(elapsed));
	const arc = `M${cx} ${cy - r} A${r} ${r} 0 1 1 ${endX} ${endY}`;
	return (
		<svg aria-hidden="true" className="h-auto w-full" viewBox="0 0 320 150">
			<circle cx={cx} cy={cy} fill="none" r={r} stroke={LINE} />
			{Array.from({ length: 30 }, (_, day) => {
				const [x1, y1] = point(day % 5 === 0 ? r - 8 : r - 4, angle(day));
				const [x2, y2] = point(r, angle(day));
				return <line key={day} stroke="rgb(255 255 255 / 0.35)" x1={x1} x2={x2} y1={y1} y2={y2} />;
			})}
			<motion.path
				d={arc}
				fill="none"
				initial={{ pathLength: 0 }}
				stroke={FIG}
				strokeWidth="1.5"
				transition={{ duration: 1.4, ease: EASE }}
				viewport={{ once: true }}
				whileInView={{ pathLength: 1 }}
			/>
			<g className="ed-spin" style={{ transformOrigin: `${cx}px ${cy}px` }}>
				<line stroke="#fafafa" strokeWidth="1.1" x1={cx} x2={cx} y1={cy} y2={cy - r + 14} />
			</g>
			<circle cx={cx} cy={cy} fill="#090909" r="3" stroke="#fafafa" />
			<text className="ed-mono" fill="#fafafa" fontSize="11" textAnchor="middle" x={cx} y={cy + 24}>
				30 D
			</text>
			{[
				["INTERVAL", "30 DAYS"],
				["NEXT RUN", "02:00"],
				["STALE", "1,906"],
				["CHANGED", "38"],
			].map(([label, value], index) => (
				<g key={label}>
					<text {...svgLabel} x="160" y={44 + index * 24}>
						{label}
					</text>
					<text {...svgLabel} fill="#fafafa" textAnchor="end" x="304" y={44 + index * 24}>
						{value}
					</text>
					<line
						stroke="rgb(255 255 255 / 0.1)"
						strokeDasharray="2 3"
						x1="160"
						x2="304"
						y1={50 + index * 24}
						y2={50 + index * 24}
					/>
				</g>
			))}
		</svg>
	);
}

/* ------------------------------------------------------------------ */
/* Fig. 5 — how a check works                                          */
/* ------------------------------------------------------------------ */

type Reading = "pass" | "warn" | "fail" | "skip";

const PIPELINE_STAGES = ["Format", "Domain", "Mail server", "Mailbox"];

const PIPELINE_SAMPLES: {
	email: string;
	readings: [Reading, string][];
	kind: Kind;
	score: number;
	action: string;
}[] = [
	{
		email: "maya@brightlane.co",
		readings: [
			["pass", "Syntax ok"],
			["pass", "brightlane.co"],
			["pass", "mx1.brightlane.co"],
			["pass", "Accepts mail"],
		],
		kind: "valid",
		score: 100,
		action: "Keep",
	},
	{
		email: "dev@acmecloud.io",
		readings: [
			["pass", "Syntax ok"],
			["pass", "acmecloud.io"],
			["pass", "mx.acmecloud.io"],
			["warn", "Accepts all (catch-all)"],
		],
		kind: "risky",
		score: 75,
		action: "Review",
	},
	{
		email: "ops@nomx.dev",
		readings: [
			["pass", "Syntax ok"],
			["pass", "nomx.dev"],
			["fail", "No MX record"],
			["skip", "Skipped"],
		],
		kind: "invalid",
		score: 0,
		action: "Suppress",
	},
];

const PER_SAMPLE = 9;

function StationMark({ reading }: { reading?: Reading }) {
	return (
		<svg aria-hidden="true" className="relative size-[15px]" viewBox="0 0 15 15">
			<circle cx="7.5" cy="7.5" fill="#090909" r="7" />
			{!reading ? (
				<circle cx="7.5" cy="7.5" fill="none" r="6.5" stroke="rgb(255 255 255 / 0.35)" />
			) : null}
			{reading === "pass" ? <circle cx="7.5" cy="7.5" fill={FIG} r="6.5" /> : null}
			{reading === "warn" ? (
				<>
					<circle cx="7.5" cy="7.5" fill="none" r="6.5" stroke="#d9d9d9" />
					<path d="M7.5 1a6.5 6.5 0 0 1 0 13z" fill="#d9d9d9" />
				</>
			) : null}
			{reading === "fail" ? (
				<>
					<circle cx="7.5" cy="7.5" fill="none" r="6.5" stroke="#fafafa" />
					<path d="M5 5l5 5M10 5l-5 5" stroke="#fafafa" strokeWidth="1.2" />
				</>
			) : null}
			{reading === "skip" ? (
				<circle
					cx="7.5"
					cy="7.5"
					fill="none"
					r="6.5"
					stroke="rgb(255 255 255 / 0.35)"
					strokeDasharray="2 2"
				/>
			) : null}
		</svg>
	);
}

/** How a check works: one address, four readings in order, then the result. */
export function CheckPipeline({ fig = "5" }: { fig?: string }) {
	const ref = useRef<HTMLDivElement>(null);
	const tick = useTicker(ref, PIPELINE_SAMPLES.length * PER_SAMPLE, 650, 5);
	const sample = PIPELINE_SAMPLES[Math.floor(tick / PER_SAMPLE)];
	const step = tick % PER_SAMPLE;
	const reached = Math.min(step, PIPELINE_STAGES.length);
	const done = step > PIPELINE_STAGES.length;
	return (
		<div ref={ref}>
			<Sheet
				caption="Readings run in this order · a check stops before delivery"
				fig={fig}
				title="One check, four readings"
			>
				<div className="p-4 sm:p-6">
					<div className="flex items-center justify-between gap-4 border-b border-dashed border-white/10 pb-4">
						<span className={LABEL}>Input</span>
						<span className="ed-mono truncate text-[13px] text-[#fafafa]">{sample.email}</span>
					</div>
					<div className="relative mt-8 grid grid-cols-4">
						<span className="absolute top-[7px] right-[12.5%] left-[12.5%] h-px bg-white/20" />
						<motion.span
							animate={{ width: `${(Math.max(reached - 1, 0) / 3) * 75}%` }}
							className="absolute top-[7px] left-[12.5%] h-px bg-[var(--ed-fig)]"
							initial={false}
							transition={{ duration: 0.4, ease: EASE }}
						/>
						{PIPELINE_STAGES.map((stage, index) => {
							const [reading, text] = sample.readings[index];
							const shown = index < reached;
							return (
								<div className="flex flex-col items-center gap-3 px-1 text-center" key={stage}>
									<StationMark reading={shown ? reading : undefined} />
									<span className={LABEL}>{stage}</span>
									<span
										className={cn(
											"ed-mono min-h-[2.6em] text-[11px] leading-[1.3] break-all transition-opacity duration-300 sm:break-normal",
											reading === "skip" ? "text-[#6b6b6b]" : "text-[#d9d9d9]",
											shown ? "opacity-100" : "opacity-0",
										)}
									>
										{text}
									</span>
								</div>
							);
						})}
					</div>
					<div className="mx-[12.5%] mt-2">
						<Dimension label="No email is sent" />
					</div>
					<div
						className={cn(
							"mt-6 grid grid-cols-3 border-t border-white/10 pt-4 transition-opacity duration-500",
							done ? "opacity-100" : "opacity-25",
						)}
					>
						<div>
							<p className={LABEL}>Result</p>
							<p className="ed-mono mt-1.5 flex items-center gap-2 text-[13px] text-[#fafafa] capitalize">
								<Marker kind={sample.kind} />
								{sample.kind}
							</p>
						</div>
						<div>
							<p className={LABEL}>Score</p>
							<p className="ed-mono mt-1.5 text-[13px] text-[#fafafa] tabular-nums">
								{sample.score}
							</p>
						</div>
						<div>
							<p className={LABEL}>Action</p>
							<p className="ed-mono mt-1.5 text-[13px] text-[#fafafa]">{sample.action}</p>
						</div>
					</div>
				</div>
			</Sheet>
		</div>
	);
}

/* ------------------------------------------------------------------ */
/* Fig. 6 — delivery feedback                                          */
/* ------------------------------------------------------------------ */

const HANDLED = [
	"Bounce events",
	"Complaints",
	"Unsubscribes",
	"Suppression list",
	"Scheduled re-checks",
	"Clean export files",
];

function Connector({ vertical = false }: { vertical?: boolean }) {
	if (vertical) {
		return (
			<div aria-hidden="true" className="flex justify-center py-1">
				<div className="relative h-8 w-px bg-white/25">
					<span className="ed-travel-y absolute left-1/2 size-[5px] -translate-x-1/2 -translate-y-1/2 rounded-full bg-[var(--ed-fig)]" />
					<svg
						className="absolute bottom-0 left-1/2 size-2 -translate-x-1/2 translate-y-px text-white/50"
						viewBox="0 0 8 8"
					>
						<path d="M0.5 1 L4 7 L7.5 1" fill="none" stroke="currentColor" />
					</svg>
				</div>
			</div>
		);
	}
	return (
		<div
			aria-hidden="true"
			className="flex items-center justify-center py-2 lg:min-w-[64px] lg:flex-1 lg:py-0"
		>
			<div className="relative h-10 w-px bg-white/25 lg:h-px lg:w-full">
				<span className="ed-travel absolute top-1/2 hidden size-[5px] -translate-x-1/2 -translate-y-1/2 rounded-full bg-[var(--ed-fig)] lg:block" />
				<svg
					className="absolute right-0 bottom-0 hidden size-2 translate-x-px translate-y-[3.5px] text-white/50 lg:block"
					viewBox="0 0 8 8"
				>
					<path d="M1 0.5 L7 4 L1 7.5" fill="none" stroke="currentColor" />
				</svg>
				<svg
					className="absolute bottom-0 left-1/2 size-2 -translate-x-1/2 translate-y-px text-white/50 lg:hidden"
					viewBox="0 0 8 8"
				>
					<path d="M0.5 1 L4 7 L7.5 1" fill="none" stroke="currentColor" />
				</svg>
			</div>
		</div>
	);
}

function Block({
	label,
	className,
	children,
}: {
	label: string;
	className?: string;
	children: ReactNode;
}) {
	return (
		<div className={cn("border border-white/15 bg-[#090909] p-4", className)}>
			<p className="ed-fig-label text-[var(--ed-fig)]">{label}</p>
			<div className="mt-3">{children}</div>
		</div>
	);
}

/** Delivery feedback: events come in from the platform, get handled, and the next send skips them. */
export function FeedbackFlow({ fig = "6" }: { fig?: string }) {
	const ref = useRef<HTMLDivElement>(null);
	const tick = useTicker(ref, HANDLED.length + 4, 700, HANDLED.length);
	const skipped = 1447;
	const sent = 46766;
	const skippedShare = (skipped / (skipped + sent)) * 100;
	return (
		<div ref={ref}>
			<Sheet
				caption="Your platform reports the bounce · the address goes on your suppression list · your next send skips it"
				fig={fig}
				title="Delivery feedback"
			>
				<div className="flex flex-col p-4 sm:p-6 lg:flex-row lg:items-stretch">
					<Block className="lg:w-[220px]" label="6.1 Email platform">
						<ul className="flex flex-col">
							{["SendGrid", "Mailgun", "Postmark", "Amazon SES"].map((name) => (
								<li
									className="ed-mono flex items-center justify-between border-b border-dashed border-white/10 py-2 text-[12px] text-[#d9d9d9] last:border-0"
									key={name}
								>
									{name}
									<span className="size-1.5 rounded-full border border-white/50" />
								</li>
							))}
						</ul>
					</Block>
					<Connector />
					<Block className="lg:w-[280px]" label="6.2 Verifier">
						<ol className="flex flex-col">
							{HANDLED.map((item, index) => (
								<li
									className={cn(
										"ed-mono grid grid-cols-[24px_1fr_auto] items-center gap-2 border-b border-dashed border-white/10 py-[7px] text-[12px] transition-colors duration-300 last:border-0",
										index < tick ? "text-[#fafafa]" : "text-[#5a5a5a]",
									)}
									key={item}
								>
									<span className="text-[#6b6b6b]">{String(index + 1).padStart(2, "0")}</span>
									{item}
									<Marker className={index < tick ? "" : "opacity-20"} kind="valid" />
								</li>
							))}
						</ol>
					</Block>
					<Connector />
					<Block className="lg:w-[250px]" label="6.3 Next send">
						<p className="ed-mono text-[26px] leading-none text-[#fafafa]">
							<CountUp to={skipped} />
						</p>
						<p className={cn(LABEL, "mt-2")}>Addresses skipped</p>
						<div className="mt-6 flex gap-px">
							<Dimension className="flex-1" label={`Sent · ${format(sent)}`} />
							<span style={{ width: `${skippedShare}%`, minWidth: 6 }} />
						</div>
						<div className="mt-2 flex h-5 gap-px">
							<span className="h-full flex-1 border border-[var(--ed-fig)] bg-[#9db8ff]/15" />
							<span
								className="ed-hatch h-full border border-white/50"
								style={{ width: `${skippedShare}%`, minWidth: 6 }}
							/>
						</div>
						<p className={cn(LABEL, "mt-3 text-right")}>Skipped · {format(skipped)}</p>
					</Block>
				</div>
			</Sheet>
		</div>
	);
}

/* ------------------------------------------------------------------ */
/* Fig. 7 — list quality                                               */
/* ------------------------------------------------------------------ */

const ACME: [Kind, number][] = [
	["valid", 1240],
	["risky", 489],
	["invalid", 397],
	["unknown", 54],
];

const QUALITY_LISTS: {
	name: string;
	score: number;
	parts: [Kind, number][];
	kind: Kind;
	note: string;
}[] = [
	{
		name: "October newsletter",
		score: 88,
		parts: NEWSLETTER,
		kind: "valid",
		note: "Ready to send",
	},
	{ name: "acme-launch.csv", score: 41, parts: ACME, kind: "invalid", note: "Held for review" },
];

/** List quality: two lists on a score scale against a threshold, and what each one holds. */
export function QualityFigure({ fig = "7" }: { fig?: string }) {
	return (
		<Sheet caption="Sample lists · fictional domains" fig={fig} title="List quality">
			<div className="flex flex-col gap-10 p-4 sm:p-6">
				<div>
					<p className={LABEL}>7.1 Quality score</p>
					<div className="relative mt-24">
						<span
							className="absolute -top-[84px] bottom-7 w-px border-l border-dashed border-[var(--ed-fig)]/70"
							style={{ left: "70%" }}
						/>
						<span
							className="ed-fig-label absolute -top-[84px] pl-2 text-[var(--ed-fig)]"
							style={{ left: "70%" }}
						>
							Threshold 70
						</span>
						{QUALITY_LISTS.map((list, index) => (
							<motion.div
								className="absolute bottom-[30px] flex -translate-x-1/2 flex-col items-center"
								initial={{ opacity: 0, y: 6 }}
								key={list.name}
								style={{ left: `${list.score}%` }}
								transition={{ duration: 0.5, delay: 0.3 + index * 0.25 }}
								viewport={{ once: true }}
								whileInView={{ opacity: 1, y: 0 }}
							>
								<span className="ed-fig-label mb-1 whitespace-nowrap text-[#d9d9d9]">
									{list.score} · {list.note}
								</span>
								<span className="h-4 w-px bg-white/40" />
								<Marker className="size-3" kind={list.kind} />
							</motion.div>
						))}
						<Ruler labels={["0", "25", "50", "75", "100"]} major={5} steps={20} />
					</div>
				</div>
				<div>
					<p className={LABEL}>7.2 What each list holds</p>
					<div className="mt-4 flex flex-col gap-5">
						{QUALITY_LISTS.map((list) => {
							const total = list.parts.reduce((sum, [, count]) => sum + count, 0);
							return (
								<div key={list.name}>
									<div className="mb-2 flex items-center justify-between">
										<span className="ed-mono text-[12px] text-[#d9d9d9]">{list.name}</span>
										<span className={LABEL}>{format(total)} rows</span>
									</div>
									<CompositionBar parts={list.parts} total={total} />
								</div>
							);
						})}
						<Legend counts={false} parts={NEWSLETTER} />
					</div>
				</div>
			</div>
		</Sheet>
	);
}

/* ------------------------------------------------------------------ */
/* Fig. 8 — API                                                        */
/* ------------------------------------------------------------------ */

type Segment = [string, string?];

const KEY = "text-[#fafafa]";
const STRING = "text-[var(--ed-fig)]";
const VALUE = "text-[#d9d9d9]";
const DIM = "text-[#6b6b6b]";

const TERMINAL_LINES: Segment[][] = [
	[
		["$ ", DIM],
		["curl -X POST ", VALUE],
		['"$API_URL/v1/check_email"', STRING],
		[" \\", DIM],
	],
	[
		["    -H ", VALUE],
		['"Authorization: Bearer $API_KEY"', STRING],
		[" \\", DIM],
	],
	[
		["    -d ", VALUE],
		['\'{ "to_email": "maya@brightlane.co" }\'', STRING],
	],
	[[""]],
	[["{", DIM]],
	[["  "], ['"is_reachable"', KEY], [": ", DIM], ['"safe"', STRING], [",", DIM]],
	[["  "], ['"score"', KEY], [": {", DIM]],
	[["    "], ['"score"', KEY], [": ", DIM], ["100", VALUE], [",", DIM]],
	[["    "], ['"category"', KEY], [": ", DIM], ['"valid"', STRING], [",", DIM]],
	[["    "], ['"safe_to_send"', KEY], [": ", DIM], ["true", VALUE], [",", DIM]],
	[["    "], ['"reason_codes"', KEY], [": [", DIM], ['"deliverable"', STRING], ["]", DIM]],
	[["  }", DIM]],
	[["}", DIM]],
];

/** API: the request, then the response arriving line by line. */
export function ApiTerminal({ fig = "8" }: { fig?: string }) {
	const ref = useRef<HTMLDivElement>(null);
	const total = TERMINAL_LINES.length;
	const tick = useTicker(ref, total + 8, 260, total);
	const shown = Math.min(tick + 1, total);
	return (
		<div ref={ref}>
			<Sheet
				caption="Request and response · values from a sample check"
				fig={fig}
				title="POST /v1/check_email"
			>
				<pre className="ed-mono min-h-[330px] overflow-x-auto p-4 text-[12px] leading-[1.75] sm:p-6">
					<code>
						{TERMINAL_LINES.slice(0, shown).map((line, index) => (
							<span className="block min-h-[1.75em]" key={index}>
								{line.map(([text, color], part) => (
									<span className={color} key={part}>
										{text}
									</span>
								))}
								{index === shown - 1 ? (
									<span className="ml-0.5 inline-block h-3.5 w-1.5 translate-y-0.5 animate-pulse bg-[#d9d9d9] motion-reduce:hidden" />
								) : null}
							</span>
						))}
					</code>
				</pre>
			</Sheet>
		</div>
	);
}

/* ------------------------------------------------------------------ */
/* Platform marquee                                                    */
/* ------------------------------------------------------------------ */

const MARQUEE = [
	"SendGrid",
	"Mailgun",
	"Postmark",
	"Amazon SES",
	"CSV files",
	"REST API",
	"Go SDK",
	"TypeScript SDK",
	"Webhooks",
];

/** Endless row of the platforms and formats checks work with. */
export function PlatformMarquee() {
	return (
		<div
			aria-hidden="true"
			className="relative overflow-hidden [mask-image:linear-gradient(to_right,transparent,black_12%,black_88%,transparent)]"
		>
			<div className="ed-marquee flex w-max">
				{[...MARQUEE, ...MARQUEE].map((name, index) => (
					<span
						className="ed-sans flex items-center gap-14 pr-14 text-[22px] font-medium tracking-[-0.01em] whitespace-nowrap text-[#757575]"
						key={index}
					>
						{name}
						<span className="ed-mono text-[14px] text-[#3a3a3a]">+</span>
					</span>
				))}
			</div>
		</div>
	);
}

/* ------------------------------------------------------------------ */
/* Product page figures                                                */
/* ------------------------------------------------------------------ */

/** Numbered steps on one line; a marker moves from step to step. */
export function StepFlowFigure({
	fig,
	title,
	caption,
	steps,
}: {
	fig: string;
	title: string;
	caption?: string;
	steps: string[];
}) {
	const ref = useRef<HTMLDivElement>(null);
	const tick = useTicker(ref, steps.length + 3, 900, steps.length);
	const reached = Math.min(tick, steps.length);
	const edge = 50 / steps.length;
	const span = 100 - 2 * edge;
	return (
		<div ref={ref}>
			<Sheet caption={caption} fig={fig} title={title}>
				<div className="p-4 sm:p-6">
					<ol
						className={cn(
							"relative grid gap-5 md:gap-0",
							steps.length > 3 ? "md:grid-cols-4" : "md:grid-cols-3",
						)}
					>
						<span
							className="absolute top-[7px] hidden h-px bg-white/20 md:block"
							style={{ left: `${edge}%`, right: `${edge}%` }}
						/>
						<motion.span
							animate={{
								width: `${(Math.max(reached - 1, 0) / Math.max(steps.length - 1, 1)) * span}%`,
							}}
							className="absolute top-[7px] hidden h-px bg-[var(--ed-fig)] md:block"
							initial={false}
							style={{ left: `${edge}%` }}
							transition={{ duration: 0.5, ease: EASE }}
						/>
						{steps.map((step, index) => {
							const shown = index < reached;
							return (
								<li
									className="relative flex gap-3 md:flex-col md:items-center md:px-3 md:text-center"
									key={step}
								>
									<StationMark reading={shown ? "pass" : undefined} />
									<span>
										<span className="ed-mono block text-[11px] text-[var(--ed-fig)]">
											{fig}.{index + 1}
										</span>
										<span
											className={cn(
												"ed-sans mt-1 block text-[14px] leading-[1.45] transition-colors duration-300",
												shown ? "text-[#fafafa]" : "text-[#757575]",
											)}
										>
											{step.replace(/\.$/, "")}
										</span>
									</span>
								</li>
							);
						})}
					</ol>
				</div>
			</Sheet>
		</div>
	);
}

type ProviderKey = "sendgrid" | "mailgun" | "postmark" | "amazon-ses";

const PROVIDERS: Record<
	ProviderKey,
	{ name: string; path: string; source: string; checks: string[]; events: [string, string][] }
> = {
	sendgrid: {
		name: "SendGrid",
		path: "sendgrid",
		source: "Event Webhook",
		checks: ["Token in the URL", "Signed webhook, public key", "Timestamp window"],
		events: [
			["delivered", "delivered"],
			["bounce", "bounced"],
			["spamreport", "complained"],
			["unsubscribe", "unsubscribed"],
			["open", "opened"],
			["click", "clicked"],
		],
	},
	mailgun: {
		name: "Mailgun",
		path: "mailgun",
		source: "Webhooks",
		checks: ["Token in the URL", "Signature with your signing key", "Timestamp window"],
		events: [
			["delivered", "delivered"],
			["failed (permanent)", "bounced"],
			["complained", "complained"],
			["unsubscribed", "unsubscribed"],
			["opened", "opened"],
			["clicked", "clicked"],
		],
	},
	postmark: {
		name: "Postmark",
		path: "postmark",
		source: "Webhooks",
		checks: ["Token in the URL", "IP allow list, optional", "Postmark does not sign events"],
		events: [
			["Delivery", "delivered"],
			["Bounce", "bounced"],
			["SpamComplaint", "complained"],
			["SubscriptionChange", "unsubscribed"],
			["Open", "opened"],
			["Click", "clicked"],
		],
	},
	"amazon-ses": {
		name: "Amazon SES",
		path: "ses",
		source: "SNS notifications",
		checks: ["Token in the URL", "SNS message signature", "Allowed topic ARNs"],
		events: [
			["Delivery", "delivered"],
			["Bounce", "bounced"],
			["Complaint", "complained"],
		],
	},
};

const SUPPRESSING = new Set(["bounced", "complained", "unsubscribed"]);

/** Integration hero: the platform's events reach a private endpoint and land on the list. */
export function ProviderFigure({ provider, logo }: { provider: ProviderKey; logo: ReactNode }) {
	const ref = useRef<HTMLDivElement>(null);
	const data = PROVIDERS[provider];
	const tick = useTicker(ref, data.events.length + 4, 800, data.events.length);
	return (
		<div ref={ref}>
			<Sheet
				caption={`POST /v1/inbound/providers/${data.path}/{endpoint}/{token}`}
				fig="1"
				title={`${data.name} → Verifier`}
			>
				<div className="flex flex-col p-4 sm:p-5">
					<Block label={`1.1 ${data.name}`}>
						<div className="flex items-center gap-3">
							<span className="inline-flex size-10 items-center justify-center border border-white/15 bg-[#111111] p-2 [&_svg]:size-full">
								{logo}
							</span>
							<span>
								<span className="ed-sans block text-[14px] text-[#fafafa]">{data.name}</span>
								<span className={LABEL}>{data.source}</span>
							</span>
						</div>
					</Block>
					<Connector vertical />
					<Block label="1.2 Delivery endpoint">
						<ul className="flex flex-col">
							{data.checks.map((check) => (
								<li
									className="ed-mono flex items-center gap-2 border-b border-dashed border-white/10 py-[7px] text-[12px] text-[#d9d9d9] last:border-0"
									key={check}
								>
									<Marker kind="valid" />
									{check}
								</li>
							))}
						</ul>
					</Block>
					<Connector vertical />
					<Block label="1.3 Recorded as">
						<ul className="flex flex-col">
							{data.events.map(([source, recorded], index) => (
								<li
									className={cn(
										"ed-mono grid grid-cols-[1fr_auto] items-center gap-3 border-b border-dashed border-white/10 py-[7px] text-[12px] transition-colors duration-300 last:border-0",
										index < tick ? "text-[#fafafa]" : "text-[#4a4a4a]",
									)}
									key={source}
								>
									<span>
										<span className="text-[#757575]">{source}</span> → {recorded}
									</span>
									{SUPPRESSING.has(recorded) ? (
										<span className="ed-fig-label text-[var(--ed-fig)]">Suppressed</span>
									) : (
										<span className="ed-fig-label text-[#5a5a5a]">History</span>
									)}
								</li>
							))}
						</ul>
					</Block>
				</div>
			</Sheet>
		</div>
	);
}

const CSV_ROWS: [string, string, Kind, string][] = [
	["maya@brightlane.co", "Maya", "valid", "Keep"],
	["anna@gmial.com", "Anna", "risky", "Fix → gmail.com"],
	["kai@greylist.co", "Kai", "unknown", "Review"],
	["r.lee@closedco.com", "Rae", "invalid", "Drop"],
];

const CSV_FILES: [string, string][] = [
	["safe.csv", "3,812"],
	["fixed.csv", "64"],
	["review.csv", "463"],
	["drop.csv", "172"],
];

/** CSV: your columns stay, the result and the action are added, then the rows split into files. */
export function CsvFigure() {
	const ref = useRef<HTMLDivElement>(null);
	const tick = useTicker(ref, CSV_ROWS.length + 4, 700, CSV_ROWS.length);
	return (
		<div ref={ref}>
			<Sheet caption="Sample rows · fictional domains" fig="1" title="newsletter.csv → clean files">
				<div className="p-4 sm:p-5">
					<div className="overflow-x-auto">
						<table className="ed-mono w-full min-w-[420px] border-collapse text-left text-[11.5px]">
							<thead>
								<tr className="border-b border-white/15">
									<th className="py-2 pr-3 font-normal text-[#757575]">email</th>
									<th className="py-2 pr-3 font-normal text-[#757575]">first_name</th>
									<th className="py-2 pr-3 font-normal text-[var(--ed-fig)]">+ result</th>
									<th className="py-2 font-normal text-[var(--ed-fig)]">+ action</th>
								</tr>
							</thead>
							<tbody>
								{CSV_ROWS.map(([email, name, kind, action], index) => {
									const shown = index < tick;
									return (
										<tr className="border-b border-dashed border-white/10" key={email}>
											<td className="py-2 pr-3 text-[#d9d9d9]">{email}</td>
											<td className="py-2 pr-3 text-[#9e9e9e]">{name}</td>
											<td
												className={cn(
													"py-2 pr-3 transition-opacity duration-300",
													shown ? "opacity-100" : "opacity-0",
												)}
											>
												<span className="flex items-center gap-1.5 text-[#d9d9d9]">
													<Marker kind={kind} />
													{kind}
												</span>
											</td>
											<td
												className={cn(
													"py-2 text-[#fafafa] transition-opacity duration-300",
													shown ? "opacity-100" : "opacity-0",
												)}
											>
												{action}
											</td>
										</tr>
									);
								})}
							</tbody>
						</table>
					</div>
					<ul className="mt-5 grid grid-cols-2 gap-2 sm:grid-cols-4">
						{CSV_FILES.map(([file, rows]) => (
							<li
								className={cn(
									"border px-3 py-2.5 transition-colors duration-300",
									tick > CSV_ROWS.length ? "border-white/25" : "border-white/10",
								)}
								key={file}
							>
								<span className="ed-mono block text-[12px] text-[#fafafa]">{file}</span>
								<span className={LABEL}>{rows} rows</span>
							</li>
						))}
					</ul>
				</div>
			</Sheet>
		</div>
	);
}

const TYPED_SIGNUP = "anna@gmial.com";

/** Signup forms: the address is checked as it is entered, and a typo gets a hint. */
export function SignupFigure() {
	const ref = useRef<HTMLDivElement>(null);
	const steps = TYPED_SIGNUP.length + 10;
	const tick = useTicker(ref, steps, 140, steps - 3);
	const typed = TYPED_SIGNUP.slice(0, Math.min(tick, TYPED_SIGNUP.length));
	const checked = tick > TYPED_SIGNUP.length + 1;
	const hinted = tick > TYPED_SIGNUP.length + 4;
	return (
		<div ref={ref}>
			<Sheet
				caption="Sample request · fictional address"
				fig="1"
				title="Signup form · POST /v1/check_email"
			>
				<div className="flex flex-col p-4 sm:p-5">
					<Block label="1.1 Form">
						<p className={LABEL}>Work email</p>
						<div className="ed-mono mt-2 flex h-9 items-center border border-white/20 px-3 text-[13px] text-[#fafafa]">
							{typed}
							<span className="ml-px inline-block h-4 w-px animate-pulse bg-white motion-reduce:hidden" />
						</div>
						<p
							className={cn(
								"ed-sans mt-2 text-[12.5px] transition-opacity duration-300",
								hinted ? "text-[#fafafa] opacity-100" : "opacity-0",
							)}
						>
							Did you mean <span className="text-[var(--ed-fig)]">anna@gmail.com</span>?
						</p>
					</Block>
					<Connector vertical />
					<Block label="1.2 Result">
						<dl
							className={cn(
								"ed-mono grid grid-cols-[auto_1fr] gap-x-4 gap-y-1.5 text-[12px] transition-opacity duration-300",
								checked ? "opacity-100" : "opacity-20",
							)}
						>
							<dt className="text-[#757575]">category</dt>
							<dd className="flex items-center gap-1.5 text-[#fafafa]">
								<Marker kind="risky" /> risky
							</dd>
							<dt className="text-[#757575]">safe_to_send</dt>
							<dd className="text-[#fafafa]">false</dd>
							<dt className="text-[#757575]">reason_codes</dt>
							<dd className="text-[#fafafa]">possible_typo</dd>
							<dt className="text-[#757575]">suggestion</dt>
							<dd className="text-[var(--ed-fig)]">anna@gmail.com</dd>
						</dl>
					</Block>
				</div>
			</Sheet>
		</div>
	);
}

const SOURCES_QUALITY: { name: string; score: number; parts: [Kind, number][] }[] = [
	{
		name: "Website form",
		score: 91,
		parts: [
			["valid", 1968],
			["risky", 102],
			["invalid", 41],
			["unknown", 29],
		],
	},
	{
		name: "Webinar signups",
		score: 76,
		parts: [
			["valid", 1051],
			["risky", 168],
			["invalid", 121],
			["unknown", 40],
		],
	},
	{
		name: "Partner list",
		score: 58,
		parts: [
			["valid", 1830],
			["risky", 545],
			["invalid", 580],
			["unknown", 95],
		],
	},
];

/** Marketing operations: list quality by source, side by side. */
export function SourceCompareFigure() {
	return (
		<Sheet caption="Sample sources · fictional data" fig="1" title="Quality by source">
			<div className="flex flex-col gap-5 p-4 sm:p-6">
				{SOURCES_QUALITY.map((source) => {
					const total = source.parts.reduce((sum, [, count]) => sum + count, 0);
					return (
						<div key={source.name}>
							<div className="mb-2 flex items-baseline justify-between gap-3">
								<span className="ed-mono text-[12.5px] text-[#fafafa]">{source.name}</span>
								<span className={LABEL}>
									{format(total)} rows · score{" "}
									<span className="text-[var(--ed-fig)]">{source.score}</span>
								</span>
							</div>
							<CompositionBar parts={source.parts} total={total} />
						</div>
					);
				})}
				<Legend counts={false} parts={SOURCES_QUALITY[0].parts} />
			</div>
		</Sheet>
	);
}

const WORKSPACE_NODES: { x: number; label: string; value: string }[] = [
	{ x: 60, label: "MEMBERS", value: "4 of 10" },
	{ x: 160, label: "LISTS", value: "12" },
	{ x: 260, label: "SUPPRESSIONS", value: "1,284" },
	{ x: 360, label: "API KEYS", value: "2" },
	{ x: 460, label: "ENDPOINTS", value: "3" },
];

/** Workspace: one organization owns the lists, suppressions, keys, and endpoints. */
export function WorkspaceFigure() {
	const ref = useRef<SVGSVGElement>(null);
	const tick = useTicker(ref, WORKSPACE_NODES.length, 1200, 0);
	return (
		<Sheet caption="Sample workspace" fig="1" title="Organization · Northwind">
			<svg aria-hidden="true" className="h-auto w-full" ref={ref} viewBox="0 0 520 250">
				<rect fill="#090909" height="34" stroke={FIG} width="150" x="185" y="22" />
				<text {...svgLabel} fill={FIG} textAnchor="middle" x="260" y="43">
					NORTHWIND · ORG
				</text>
				<path d="M260 56 V92 M60 92 H460" fill="none" stroke={LINE} />
				{WORKSPACE_NODES.map((node, index) => {
					const active = index === tick;
					return (
						<g key={node.label}>
							<line stroke={active ? FIG : LINE} x1={node.x} x2={node.x} y1="92" y2="120" />
							<rect
								fill="#090909"
								height="58"
								stroke={active ? FIG : "rgb(255 255 255 / 0.25)"}
								style={{ transition: "stroke 300ms" }}
								width="88"
								x={node.x - 44}
								y="120"
							/>
							<text {...svgLabel} textAnchor="middle" x={node.x} y="140">
								{node.label}
							</text>
							<text
								className="ed-mono"
								fill="#fafafa"
								fontSize="13"
								textAnchor="middle"
								x={node.x}
								y="164"
							>
								{node.value}
							</text>
						</g>
					);
				})}
				{["OWNER", "ADMIN", "MEMBER"].map((role, index) => (
					<g key={role}>
						<line
							stroke={LINE}
							strokeDasharray="2 3"
							x1="60"
							x2="60"
							y1="178"
							y2={198 + index * 16}
						/>
						<line
							stroke={LINE}
							strokeDasharray="2 3"
							x1="60"
							x2="72"
							y1={198 + index * 16}
							y2={198 + index * 16}
						/>
						<text {...svgLabel} x="78" y={201 + index * 16}>
							{role}
						</text>
					</g>
				))}
			</svg>
		</Sheet>
	);
}

const PROBES: { to: string; reply: string }[] = [
	{ to: "RCPT TO <dev@acmecloud.io>", reply: "250 OK" },
	{ to: "RCPT TO <q7x2k9v1@acmecloud.io>", reply: "250 OK" },
];

/** Catch-all: the server accepts a real address and a made-up one, so neither is certain. */
export function CatchAllFigure() {
	const ref = useRef<SVGSVGElement>(null);
	const tick = useTicker(ref, 7, 900, 5);
	return (
		<Sheet
			caption="Sample exchange · fictional domain · no email is sent"
			fig="1"
			title="Catch-all domain"
		>
			<svg aria-hidden="true" className="h-auto w-full" ref={ref} viewBox="0 0 520 240">
				<rect fill="#090909" height="150" stroke={FIG} width="96" x="20" y="40" />
				<text {...svgLabel} fill={FIG} textAnchor="middle" x="68" y="62">
					VERIFIER
				</text>
				<rect
					fill="#090909"
					height="150"
					stroke="rgb(255 255 255 / 0.3)"
					width="120"
					x="380"
					y="40"
				/>
				<text {...svgLabel} textAnchor="middle" x="440" y="62">
					MX.ACMECLOUD.IO
				</text>
				{PROBES.map((probe, index) => {
					const y = 92 + index * 52;
					const asked = tick > index * 2;
					const answered = tick > index * 2 + 1;
					return (
						<g key={probe.to}>
							<g opacity={asked ? 1 : 0.15} style={{ transition: "opacity 300ms" }}>
								<path
									d={`M116 ${y} H380 M374 ${y - 4} L380 ${y} L374 ${y + 4}`}
									fill="none"
									stroke={LINE}
								/>
								<text className="ed-mono" fill="#d9d9d9" fontSize="9.5" x="128" y={y - 6}>
									{probe.to}
								</text>
							</g>
							<g opacity={answered ? 1 : 0.15} style={{ transition: "opacity 300ms" }}>
								<path
									d={`M380 ${y + 16} H116 M122 ${y + 12} L116 ${y + 16} L122 ${y + 20}`}
									fill="none"
									stroke={FIG}
									strokeOpacity="0.7"
								/>
								<text
									className="ed-mono"
									fill={FIG}
									fontSize="9.5"
									textAnchor="end"
									x="368"
									y={y + 12}
								>
									{probe.reply}
								</text>
							</g>
						</g>
					);
				})}
				<g opacity={tick > 4 ? 1 : 0.15} style={{ transition: "opacity 300ms" }}>
					<text {...svgLabel} fill="#fafafa" x="20" y="222">
						BOTH ACCEPTED → CATCH_ALL → RISKY
					</text>
				</g>
			</svg>
		</Sheet>
	);
}

const UPLOADS: { name: string; score: number; y: number }[] = [
	{ name: "CUSTOMER-A.CSV", score: 88, y: 60 },
	{ name: "CUSTOMER-B.CSV", score: 41, y: 120 },
	{ name: "CUSTOMER-C.CSV", score: 76, y: 180 },
];

/** Sending platforms: each customer upload meets a quality threshold before it is sent. */
export function GateFigure() {
	return (
		<Sheet caption="Sample uploads · threshold set by you" fig="1" title="Quality gate">
			<svg aria-hidden="true" className="h-auto w-full" viewBox="0 0 520 240">
				<line stroke={FIG} strokeDasharray="3 4" x1="300" x2="300" y1="30" y2="210" />
				<text {...svgLabel} fill={FIG} textAnchor="middle" x="300" y="22">
					THRESHOLD 70
				</text>
				<text {...svgLabel} x="470" y="22" textAnchor="end">
					SEND
				</text>
				{UPLOADS.map((upload, index) => {
					const pass = upload.score >= 70;
					const path = pass ? `M150 ${upload.y} H470` : `M150 ${upload.y} H296`;
					return (
						<g key={upload.name}>
							<text {...svgLabel} fill="#d9d9d9" x="20" y={upload.y - 6}>
								{upload.name}
							</text>
							<text className="ed-mono" fill="#9e9e9e" fontSize="9" x="20" y={upload.y + 10}>
								SCORE {upload.score}
							</text>
							<path d={path} fill="none" stroke={pass ? LINE : "rgb(255 255 255 / 0.4)"} />
							<circle className="motion-reduce:hidden" fill={pass ? FIG : "#fafafa"} r="2.2">
								<animateMotion
									begin={`-${index * 0.6}s`}
									dur="2.4s"
									path={path}
									repeatCount="indefinite"
								/>
							</circle>
							{pass ? (
								<PlacedMarker kind="valid" x={480} y={upload.y} />
							) : (
								<>
									<PlacedMarker kind="invalid" x={300} y={upload.y} />
									<text {...svgLabel} fill="#fafafa" x="314" y={upload.y + 4}>
										HELD FOR REVIEW
									</text>
								</>
							)}
						</g>
					);
				})}
			</svg>
		</Sheet>
	);
}

/* ------------------------------------------------------------------ */
/* Find email                                                          */
/* ------------------------------------------------------------------ */

/** Five of the 13 patterns for a fictional person; only first.last exists. */
const FINDER_ROWS: [string, Kind, number][] = [
	["ada.lovelace@acme.io", "valid", 100],
	["alovelace@acme.io", "invalid", 0],
	["a.lovelace@acme.io", "invalid", 0],
	["adalovelace@acme.io", "invalid", 0],
	["ada@acme.io", "invalid", 0],
];

/** Find email: a name and a domain in, the likely addresses checked, the best match marked. */
export function FinderFigure() {
	const ref = useRef<HTMLDivElement>(null);
	const tick = useTicker(ref, FINDER_ROWS.length + 4, 420, FINDER_ROWS.length + 2);
	const shown = Math.min(tick, FINDER_ROWS.length);
	const done = tick > FINDER_ROWS.length;
	return (
		<div ref={ref}>
			<Sheet
				caption="Sample search · fictional person · 5 of 13 patterns shown"
				fig="1"
				title="Find email · POST /v1/find_email"
			>
				<div className="flex flex-col p-4 sm:p-5">
					<Block label="1.1 Search">
						<dl className="ed-mono grid grid-cols-[auto_1fr] gap-x-4 gap-y-1.5 text-[12px]">
							<dt className="text-[#757575]">name</dt>
							<dd className="text-[#fafafa]">Ada Lovelace</dd>
							<dt className="text-[#757575]">domain</dt>
							<dd className="text-[#fafafa]">acme.io · mail server found</dd>
						</dl>
					</Block>
					<Connector vertical />
					<Block label="1.2 Addresses checked">
						<ul className="ed-mono space-y-1.5 text-[12px]">
							{FINDER_ROWS.map(([email, kind, score], index) => (
								<li
									className={cn(
										"flex items-center justify-between gap-3 transition-opacity duration-300",
										index < shown ? "opacity-100" : "opacity-20",
									)}
									key={email}
								>
									<span className="flex min-w-0 items-center gap-2">
										<Marker kind={kind} />
										<span className="truncate text-[#fafafa]">{email}</span>
									</span>
									<span className="text-[#757575] tabular-nums">{score}</span>
								</li>
							))}
						</ul>
					</Block>
					<Connector vertical />
					<Block label="1.3 Best match">
						<p
							className={cn(
								"ed-mono text-[12.5px] transition-opacity duration-300",
								done ? "opacity-100" : "opacity-20",
							)}
						>
							<span className="text-[var(--ed-fig)]">ada.lovelace@acme.io</span>
							<span className="text-[#757575]"> · first.last · confidence high</span>
						</p>
					</Block>
				</div>
			</Sheet>
		</div>
	);
}
