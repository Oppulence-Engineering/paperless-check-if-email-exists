#!/usr/bin/env node
/**
 * Generates lib/icons.tsx: Hugeicons icons with stable app-facing names.
 * Each icon is imported from its own module, so tests and dev builds skip the
 * 4,600-icon barrel.
 * Run after you add an icon, so call sites stay on `@/lib/icons`.
 */
import { writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const outPath = path.resolve(__dirname, "../lib/icons.tsx");

/** App export name -> @hugeicons/core-free-icons name, without the `Icon` suffix. */
const MAP = {
	AddressBook: "ContactBook",
	Alarm: "AlarmClock",
	Analytics: "Analytics01",
	Api: "Api",
	ArrowClockwise: "RotateRight01",
	ArrowCounterClockwise: "RotateLeft01",
	ArrowDown: "ArrowDown02",
	ArrowElbowDownLeft: "ArrowTurnBackward",
	ArrowLeft: "ArrowLeft01",
	ArrowRight: "ArrowRight01",
	ArrowRightIcon: "ArrowRight01",
	ArrowRightLine: "ArrowRight02",
	ArrowSquareOut: "LinkSquare02",
	ArrowUUpLeft: "Undo",
	ArrowUUpRight: "Redo",
	ArrowUp: "ArrowUp02",
	ArrowUpRight: "ArrowUpRight01",
	ArrowsOut: "Maximize01",
	Bell: "Notification01",
	Book: "Book02",
	BookOpen: "BookOpen01",
	BookmarkSimple: "Bookmark01",
	Brain: "Brain",
	BrainIcon: "Brain",
	Briefcase: "Briefcase01",
	BriefcaseIcon: "Briefcase01",
	Buildings: "Building03",
	CalendarBlank: "Calendar01",
	CalendarDots: "Calendar03",
	CalendarDotsIcon: "Calendar03",
	CaretDown: "ArrowDown01",
	CaretLeft: "ArrowLeft01",
	CaretRight: "ArrowRight01",
	CaretUpDown: "UnfoldMore",
	ChartLineIcon: "ChartLineData01",
	ChartLineUp: "ChartIncrease",
	ChatCircle: "Comment01",
	ChatsCircle: "BubbleChat",
	Check: "Tick02",
	CheckCircle: "CheckmarkCircle02",
	CheckCircleIcon: "CheckmarkCircle02",
	CheckSquare: "CheckmarkSquare01",
	Circle: "Circle",
	CircleIcon: "Circle",
	CircleNotch: "Loading03",
	CircleNotchIcon: "Loading03",
	Clipboard: "Clipboard",
	Clock: "Clock01",
	ClockCounterClockwise: "Clock04",
	Cloud: "Cloud",
	Code: "SourceCode",
	CodeIcon: "SourceCode",
	Copy: "Copy01",
	Cpu: "Cpu",
	CurrencyDollar: "Dollar01",
	DotsThree: "MoreHorizontal",
	DownloadSimple: "Download04",
	EnvelopeIcon: "Mail01",
	EnvelopeSimple: "Mail01",
	Export: "Upload04",
	ExportIcon: "Upload04",
	FilePlus: "FileAdd",
	FileText: "File02",
	FileTextIcon: "File02",
	FlagBanner: "Flag01",
	FloppyDisk: "FloppyDisk",
	FlowArrow: "WorkflowSquare01",
	FlowArrowIcon: "WorkflowSquare01",
	Folder: "Folder01",
	Funnel: "Filter",
	Gear: "Settings01",
	GearSix: "Settings02",
	Globe: "Globe",
	GlobeIcon: "Globe",
	Graph: "ChartRelationship",
	GridFour: "GridView",
	Handshake: "Agreement01",
	HardDrives: "ServerStack01",
	HardDrivesIcon: "ServerStack01",
	Headset: "Headset",
	HeadsetIcon: "Headset",
	Image: "Image01",
	Internet: "Internet",
	Key: "Key01",
	Lightning: "Flash",
	Link: "Link01",
	LinkSimple: "Link04",
	List: "Menu01",
	ListBullets: "LeftToRightListBullet",
	ListChecks: "CheckList",
	ListNumbers: "LeftToRightListNumber",
	LockSimple: "Lock",
	MagnifyingGlass: "Search01",
	MagnifyingGlassIcon: "Search01",
	MailBlock: "MailBlock01",
	MailSearch: "MailSearch01",
	MailValidation: "MailValidation01",
	Microphone: "Mic01",
	Minus: "MinusSign",
	Monitor: "Computer",
	MonitorIcon: "Computer",
	Moon: "Moon02",
	NetworkIcon: "NeuralNetwork",
	Note: "Note01",
	NotePencil: "NoteEdit",
	Palette: "PaintBoard",
	PaperPlaneTilt: "Sent",
	Paperclip: "Attachment01",
	Path: "Route01",
	PathIcon: "Route01",
	Pause: "Pause",
	PencilSimple: "PencilEdit01",
	Play: "Play",
	Plugs: "Plug01",
	PlugsConnected: "Plug02",
	PlugsConnectedIcon: "Plug02",
	PlugsIcon: "Plug01",
	Plus: "PlusSign",
	Prohibit: "Unavailable",
	Question: "HelpCircle",
	Quotes: "QuoteDown",
	Receipt: "Invoice01",
	Robot: "Robot01",
	Rocket: "Rocket01",
	SealCheck: "CheckmarkBadge01",
	SealCheckIcon: "CheckmarkBadge01",
	ShareNetwork: "Share08",
	ShieldCheck: "SecurityCheck",
	SidebarSimple: "SidebarLeft",
	SignOut: "Logout01",
	SlidersHorizontal: "SlidersHorizontal",
	Sparkle: "Sparkles",
	SparkleIcon: "Sparkles",
	SquaresFour: "DashboardSquare01",
	Stack: "Layers01",
	StackIcon: "Layers01",
	Stop: "Stop",
	Sun: "Sun03",
	Target: "Target02",
	Task: "Task01",
	TextB: "TextBold",
	TextHOne: "Heading01",
	TextHThree: "Heading03",
	TextHTwo: "Heading02",
	TextItalic: "TextItalic",
	TextStrikethrough: "TextStrikethrough",
	TextUnderline: "TextUnderline",
	Trash: "Delete02",
	Tray: "Inbox",
	TrayIcon: "Inbox",
	User: "User",
	UserCircle: "UserCircle",
	UserFocus: "UserSearch01",
	Wallet: "Wallet01",
	Warning: "Alert02",
	WarningCircle: "AlertCircle",
	WarningCircleIcon: "AlertCircle",
	WarningDiamond: "AlertDiamond",
	WarningIcon: "Alert02",
	Wrench: "Wrench01",
	X: "Cancel01",
	XCircle: "CancelCircle",
};

const imports = [...new Set(Object.values(MAP))].sort();

const header = `"use client";

import * as React from "react";
import { HugeiconsIcon, type IconSvgElement } from "@hugeicons/react";
${imports.map((name) => `import Hi${name} from "@hugeicons/core-free-icons/${name}Icon";`).join("\n")}

import { cn } from "@/lib/utils";

export type IconWeight = "thin" | "light" | "regular" | "bold" | "fill" | "duotone";

export type IconProps = React.SVGProps<SVGSVGElement> & {
	size?: number | string;
	weight?: IconWeight;
	title?: string;
};

export type IconComponent = React.ForwardRefExoticComponent<
	IconProps & React.RefAttributes<SVGSVGElement>
>;

/** Icon component type used in nav/config props. */
export type Icon = IconComponent;

/** Back-compat aliases for code that still references Lucide types. */
export type LucideIcon = IconComponent;
export type LucideProps = IconProps;
/** @deprecated Use Icon */
export type PhosphorIcon = IconComponent;

/** Phosphor weights map to Hugeicons stroke widths; \`regular\` keeps the icon default. */
const STROKE: Record<IconWeight, number | undefined> = {
	thin: 1,
	light: 1.25,
	regular: undefined,
	bold: 2,
	fill: 2,
	duotone: undefined,
};

function styled(icon: IconSvgElement, displayName: string): IconComponent {
	const Wrapped = React.forwardRef<SVGSVGElement, IconProps>(function AppIcon(
		{ className, size, strokeWidth, style, title, weight = "regular", ...props },
		ref,
	) {
		return (
			<HugeiconsIcon
				ref={ref}
				aria-hidden={title ? undefined : "true"}
				aria-label={title}
				className={cn("app-icon shrink-0", className)}
				icon={icon}
				size="1em"
				style={size != null ? { width: size, height: size, ...style } : style}
				strokeWidth={strokeWidth != null ? Number(strokeWidth) : STROKE[weight]}
				{...props}
			/>
		);
	});
	Wrapped.displayName = displayName;
	return Wrapped as unknown as IconComponent;
}

`;

const exports = Object.entries(MAP)
	.map(([alias, name]) => `export const ${alias} = styled(Hi${name}, "${alias}");`)
	.join("\n");

writeFileSync(outPath, `${header}${exports}\n`, "utf8");
console.log(`Wrote ${outPath} (${Object.keys(MAP).length} icons)`);
