"use client";

import * as React from "react";
import { HugeiconsIcon, type IconSvgElement } from "@hugeicons/react";
import HiAgreement01 from "@hugeicons/core-free-icons/Agreement01Icon";
import HiAlarmClock from "@hugeicons/core-free-icons/AlarmClockIcon";
import HiAlert02 from "@hugeicons/core-free-icons/Alert02Icon";
import HiAlertCircle from "@hugeicons/core-free-icons/AlertCircleIcon";
import HiAlertDiamond from "@hugeicons/core-free-icons/AlertDiamondIcon";
import HiAnalytics01 from "@hugeicons/core-free-icons/Analytics01Icon";
import HiApi from "@hugeicons/core-free-icons/ApiIcon";
import HiArrowDown01 from "@hugeicons/core-free-icons/ArrowDown01Icon";
import HiArrowDown02 from "@hugeicons/core-free-icons/ArrowDown02Icon";
import HiArrowLeft01 from "@hugeicons/core-free-icons/ArrowLeft01Icon";
import HiArrowRight01 from "@hugeicons/core-free-icons/ArrowRight01Icon";
import HiArrowRight02 from "@hugeicons/core-free-icons/ArrowRight02Icon";
import HiArrowTurnBackward from "@hugeicons/core-free-icons/ArrowTurnBackwardIcon";
import HiArrowUp02 from "@hugeicons/core-free-icons/ArrowUp02Icon";
import HiArrowUpRight01 from "@hugeicons/core-free-icons/ArrowUpRight01Icon";
import HiAttachment01 from "@hugeicons/core-free-icons/Attachment01Icon";
import HiBook02 from "@hugeicons/core-free-icons/Book02Icon";
import HiBookOpen01 from "@hugeicons/core-free-icons/BookOpen01Icon";
import HiBookmark01 from "@hugeicons/core-free-icons/Bookmark01Icon";
import HiBrain from "@hugeicons/core-free-icons/BrainIcon";
import HiBriefcase01 from "@hugeicons/core-free-icons/Briefcase01Icon";
import HiBubbleChat from "@hugeicons/core-free-icons/BubbleChatIcon";
import HiBuilding03 from "@hugeicons/core-free-icons/Building03Icon";
import HiCalendar01 from "@hugeicons/core-free-icons/Calendar01Icon";
import HiCalendar03 from "@hugeicons/core-free-icons/Calendar03Icon";
import HiCancel01 from "@hugeicons/core-free-icons/Cancel01Icon";
import HiCancelCircle from "@hugeicons/core-free-icons/CancelCircleIcon";
import HiChartIncrease from "@hugeicons/core-free-icons/ChartIncreaseIcon";
import HiChartLineData01 from "@hugeicons/core-free-icons/ChartLineData01Icon";
import HiChartRelationship from "@hugeicons/core-free-icons/ChartRelationshipIcon";
import HiCheckList from "@hugeicons/core-free-icons/CheckListIcon";
import HiCheckmarkBadge01 from "@hugeicons/core-free-icons/CheckmarkBadge01Icon";
import HiCheckmarkCircle02 from "@hugeicons/core-free-icons/CheckmarkCircle02Icon";
import HiCheckmarkSquare01 from "@hugeicons/core-free-icons/CheckmarkSquare01Icon";
import HiCircle from "@hugeicons/core-free-icons/CircleIcon";
import HiClipboard from "@hugeicons/core-free-icons/ClipboardIcon";
import HiClock01 from "@hugeicons/core-free-icons/Clock01Icon";
import HiClock04 from "@hugeicons/core-free-icons/Clock04Icon";
import HiCloud from "@hugeicons/core-free-icons/CloudIcon";
import HiComment01 from "@hugeicons/core-free-icons/Comment01Icon";
import HiComputer from "@hugeicons/core-free-icons/ComputerIcon";
import HiContactBook from "@hugeicons/core-free-icons/ContactBookIcon";
import HiCopy01 from "@hugeicons/core-free-icons/Copy01Icon";
import HiCpu from "@hugeicons/core-free-icons/CpuIcon";
import HiDashboardSquare01 from "@hugeicons/core-free-icons/DashboardSquare01Icon";
import HiDelete02 from "@hugeicons/core-free-icons/Delete02Icon";
import HiDollar01 from "@hugeicons/core-free-icons/Dollar01Icon";
import HiDownload04 from "@hugeicons/core-free-icons/Download04Icon";
import HiFile02 from "@hugeicons/core-free-icons/File02Icon";
import HiFileAdd from "@hugeicons/core-free-icons/FileAddIcon";
import HiFilter from "@hugeicons/core-free-icons/FilterIcon";
import HiFlag01 from "@hugeicons/core-free-icons/Flag01Icon";
import HiFlash from "@hugeicons/core-free-icons/FlashIcon";
import HiFloppyDisk from "@hugeicons/core-free-icons/FloppyDiskIcon";
import HiFolder01 from "@hugeicons/core-free-icons/Folder01Icon";
import HiGlobe from "@hugeicons/core-free-icons/GlobeIcon";
import HiGridView from "@hugeicons/core-free-icons/GridViewIcon";
import HiHeading01 from "@hugeicons/core-free-icons/Heading01Icon";
import HiHeading02 from "@hugeicons/core-free-icons/Heading02Icon";
import HiHeading03 from "@hugeicons/core-free-icons/Heading03Icon";
import HiHeadset from "@hugeicons/core-free-icons/HeadsetIcon";
import HiHelpCircle from "@hugeicons/core-free-icons/HelpCircleIcon";
import HiImage01 from "@hugeicons/core-free-icons/Image01Icon";
import HiInbox from "@hugeicons/core-free-icons/InboxIcon";
import HiInternet from "@hugeicons/core-free-icons/InternetIcon";
import HiInvoice01 from "@hugeicons/core-free-icons/Invoice01Icon";
import HiKey01 from "@hugeicons/core-free-icons/Key01Icon";
import HiLayers01 from "@hugeicons/core-free-icons/Layers01Icon";
import HiLeftToRightListBullet from "@hugeicons/core-free-icons/LeftToRightListBulletIcon";
import HiLeftToRightListNumber from "@hugeicons/core-free-icons/LeftToRightListNumberIcon";
import HiLink01 from "@hugeicons/core-free-icons/Link01Icon";
import HiLink04 from "@hugeicons/core-free-icons/Link04Icon";
import HiLinkSquare02 from "@hugeicons/core-free-icons/LinkSquare02Icon";
import HiLoading03 from "@hugeicons/core-free-icons/Loading03Icon";
import HiLock from "@hugeicons/core-free-icons/LockIcon";
import HiLogout01 from "@hugeicons/core-free-icons/Logout01Icon";
import HiMail01 from "@hugeicons/core-free-icons/Mail01Icon";
import HiMailBlock01 from "@hugeicons/core-free-icons/MailBlock01Icon";
import HiMailSearch01 from "@hugeicons/core-free-icons/MailSearch01Icon";
import HiMailValidation01 from "@hugeicons/core-free-icons/MailValidation01Icon";
import HiMaximize01 from "@hugeicons/core-free-icons/Maximize01Icon";
import HiMenu01 from "@hugeicons/core-free-icons/Menu01Icon";
import HiMic01 from "@hugeicons/core-free-icons/Mic01Icon";
import HiMinusSign from "@hugeicons/core-free-icons/MinusSignIcon";
import HiMoon02 from "@hugeicons/core-free-icons/Moon02Icon";
import HiMoreHorizontal from "@hugeicons/core-free-icons/MoreHorizontalIcon";
import HiNeuralNetwork from "@hugeicons/core-free-icons/NeuralNetworkIcon";
import HiNote01 from "@hugeicons/core-free-icons/Note01Icon";
import HiNoteEdit from "@hugeicons/core-free-icons/NoteEditIcon";
import HiNotification01 from "@hugeicons/core-free-icons/Notification01Icon";
import HiPaintBoard from "@hugeicons/core-free-icons/PaintBoardIcon";
import HiPause from "@hugeicons/core-free-icons/PauseIcon";
import HiPencilEdit01 from "@hugeicons/core-free-icons/PencilEdit01Icon";
import HiPlay from "@hugeicons/core-free-icons/PlayIcon";
import HiPlug01 from "@hugeicons/core-free-icons/Plug01Icon";
import HiPlug02 from "@hugeicons/core-free-icons/Plug02Icon";
import HiPlusSign from "@hugeicons/core-free-icons/PlusSignIcon";
import HiQuoteDown from "@hugeicons/core-free-icons/QuoteDownIcon";
import HiRedo from "@hugeicons/core-free-icons/RedoIcon";
import HiRobot01 from "@hugeicons/core-free-icons/Robot01Icon";
import HiRocket01 from "@hugeicons/core-free-icons/Rocket01Icon";
import HiRotateLeft01 from "@hugeicons/core-free-icons/RotateLeft01Icon";
import HiRotateRight01 from "@hugeicons/core-free-icons/RotateRight01Icon";
import HiRoute01 from "@hugeicons/core-free-icons/Route01Icon";
import HiSearch01 from "@hugeicons/core-free-icons/Search01Icon";
import HiSecurityCheck from "@hugeicons/core-free-icons/SecurityCheckIcon";
import HiSent from "@hugeicons/core-free-icons/SentIcon";
import HiServerStack01 from "@hugeicons/core-free-icons/ServerStack01Icon";
import HiSettings01 from "@hugeicons/core-free-icons/Settings01Icon";
import HiSettings02 from "@hugeicons/core-free-icons/Settings02Icon";
import HiShare08 from "@hugeicons/core-free-icons/Share08Icon";
import HiSidebarLeft from "@hugeicons/core-free-icons/SidebarLeftIcon";
import HiSlidersHorizontal from "@hugeicons/core-free-icons/SlidersHorizontalIcon";
import HiSourceCode from "@hugeicons/core-free-icons/SourceCodeIcon";
import HiSparkles from "@hugeicons/core-free-icons/SparklesIcon";
import HiStop from "@hugeicons/core-free-icons/StopIcon";
import HiSun03 from "@hugeicons/core-free-icons/Sun03Icon";
import HiTarget02 from "@hugeicons/core-free-icons/Target02Icon";
import HiTask01 from "@hugeicons/core-free-icons/Task01Icon";
import HiTextBold from "@hugeicons/core-free-icons/TextBoldIcon";
import HiTextItalic from "@hugeicons/core-free-icons/TextItalicIcon";
import HiTextStrikethrough from "@hugeicons/core-free-icons/TextStrikethroughIcon";
import HiTextUnderline from "@hugeicons/core-free-icons/TextUnderlineIcon";
import HiTick02 from "@hugeicons/core-free-icons/Tick02Icon";
import HiUnavailable from "@hugeicons/core-free-icons/UnavailableIcon";
import HiUndo from "@hugeicons/core-free-icons/UndoIcon";
import HiUnfoldMore from "@hugeicons/core-free-icons/UnfoldMoreIcon";
import HiUpload04 from "@hugeicons/core-free-icons/Upload04Icon";
import HiUser from "@hugeicons/core-free-icons/UserIcon";
import HiUserCircle from "@hugeicons/core-free-icons/UserCircleIcon";
import HiUserSearch01 from "@hugeicons/core-free-icons/UserSearch01Icon";
import HiWallet01 from "@hugeicons/core-free-icons/Wallet01Icon";
import HiWorkflowSquare01 from "@hugeicons/core-free-icons/WorkflowSquare01Icon";
import HiWrench01 from "@hugeicons/core-free-icons/Wrench01Icon";

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

/** Phosphor weights map to Hugeicons stroke widths; `regular` keeps the icon default. */
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

export const AddressBook = styled(HiContactBook, "AddressBook");
export const Alarm = styled(HiAlarmClock, "Alarm");
export const Analytics = styled(HiAnalytics01, "Analytics");
export const Api = styled(HiApi, "Api");
export const ArrowClockwise = styled(HiRotateRight01, "ArrowClockwise");
export const ArrowCounterClockwise = styled(HiRotateLeft01, "ArrowCounterClockwise");
export const ArrowDown = styled(HiArrowDown02, "ArrowDown");
export const ArrowElbowDownLeft = styled(HiArrowTurnBackward, "ArrowElbowDownLeft");
export const ArrowLeft = styled(HiArrowLeft01, "ArrowLeft");
export const ArrowRight = styled(HiArrowRight01, "ArrowRight");
export const ArrowRightIcon = styled(HiArrowRight01, "ArrowRightIcon");
export const ArrowRightLine = styled(HiArrowRight02, "ArrowRightLine");
export const ArrowSquareOut = styled(HiLinkSquare02, "ArrowSquareOut");
export const ArrowUUpLeft = styled(HiUndo, "ArrowUUpLeft");
export const ArrowUUpRight = styled(HiRedo, "ArrowUUpRight");
export const ArrowUp = styled(HiArrowUp02, "ArrowUp");
export const ArrowUpRight = styled(HiArrowUpRight01, "ArrowUpRight");
export const ArrowsOut = styled(HiMaximize01, "ArrowsOut");
export const Bell = styled(HiNotification01, "Bell");
export const Book = styled(HiBook02, "Book");
export const BookOpen = styled(HiBookOpen01, "BookOpen");
export const BookmarkSimple = styled(HiBookmark01, "BookmarkSimple");
export const Brain = styled(HiBrain, "Brain");
export const BrainIcon = styled(HiBrain, "BrainIcon");
export const Briefcase = styled(HiBriefcase01, "Briefcase");
export const BriefcaseIcon = styled(HiBriefcase01, "BriefcaseIcon");
export const Buildings = styled(HiBuilding03, "Buildings");
export const CalendarBlank = styled(HiCalendar01, "CalendarBlank");
export const CalendarDots = styled(HiCalendar03, "CalendarDots");
export const CalendarDotsIcon = styled(HiCalendar03, "CalendarDotsIcon");
export const CaretDown = styled(HiArrowDown01, "CaretDown");
export const CaretLeft = styled(HiArrowLeft01, "CaretLeft");
export const CaretRight = styled(HiArrowRight01, "CaretRight");
export const CaretUpDown = styled(HiUnfoldMore, "CaretUpDown");
export const ChartLineIcon = styled(HiChartLineData01, "ChartLineIcon");
export const ChartLineUp = styled(HiChartIncrease, "ChartLineUp");
export const ChatCircle = styled(HiComment01, "ChatCircle");
export const ChatsCircle = styled(HiBubbleChat, "ChatsCircle");
export const Check = styled(HiTick02, "Check");
export const CheckCircle = styled(HiCheckmarkCircle02, "CheckCircle");
export const CheckCircleIcon = styled(HiCheckmarkCircle02, "CheckCircleIcon");
export const CheckSquare = styled(HiCheckmarkSquare01, "CheckSquare");
export const Circle = styled(HiCircle, "Circle");
export const CircleIcon = styled(HiCircle, "CircleIcon");
export const CircleNotch = styled(HiLoading03, "CircleNotch");
export const CircleNotchIcon = styled(HiLoading03, "CircleNotchIcon");
export const Clipboard = styled(HiClipboard, "Clipboard");
export const Clock = styled(HiClock01, "Clock");
export const ClockCounterClockwise = styled(HiClock04, "ClockCounterClockwise");
export const Cloud = styled(HiCloud, "Cloud");
export const Code = styled(HiSourceCode, "Code");
export const CodeIcon = styled(HiSourceCode, "CodeIcon");
export const Copy = styled(HiCopy01, "Copy");
export const Cpu = styled(HiCpu, "Cpu");
export const CurrencyDollar = styled(HiDollar01, "CurrencyDollar");
export const DotsThree = styled(HiMoreHorizontal, "DotsThree");
export const DownloadSimple = styled(HiDownload04, "DownloadSimple");
export const EnvelopeIcon = styled(HiMail01, "EnvelopeIcon");
export const EnvelopeSimple = styled(HiMail01, "EnvelopeSimple");
export const Export = styled(HiUpload04, "Export");
export const ExportIcon = styled(HiUpload04, "ExportIcon");
export const FilePlus = styled(HiFileAdd, "FilePlus");
export const FileText = styled(HiFile02, "FileText");
export const FileTextIcon = styled(HiFile02, "FileTextIcon");
export const FlagBanner = styled(HiFlag01, "FlagBanner");
export const FloppyDisk = styled(HiFloppyDisk, "FloppyDisk");
export const FlowArrow = styled(HiWorkflowSquare01, "FlowArrow");
export const FlowArrowIcon = styled(HiWorkflowSquare01, "FlowArrowIcon");
export const Folder = styled(HiFolder01, "Folder");
export const Funnel = styled(HiFilter, "Funnel");
export const Gear = styled(HiSettings01, "Gear");
export const GearSix = styled(HiSettings02, "GearSix");
export const Globe = styled(HiGlobe, "Globe");
export const GlobeIcon = styled(HiGlobe, "GlobeIcon");
export const Graph = styled(HiChartRelationship, "Graph");
export const GridFour = styled(HiGridView, "GridFour");
export const Handshake = styled(HiAgreement01, "Handshake");
export const HardDrives = styled(HiServerStack01, "HardDrives");
export const HardDrivesIcon = styled(HiServerStack01, "HardDrivesIcon");
export const Headset = styled(HiHeadset, "Headset");
export const HeadsetIcon = styled(HiHeadset, "HeadsetIcon");
export const Image = styled(HiImage01, "Image");
export const Internet = styled(HiInternet, "Internet");
export const Key = styled(HiKey01, "Key");
export const Lightning = styled(HiFlash, "Lightning");
export const Link = styled(HiLink01, "Link");
export const LinkSimple = styled(HiLink04, "LinkSimple");
export const List = styled(HiMenu01, "List");
export const ListBullets = styled(HiLeftToRightListBullet, "ListBullets");
export const ListChecks = styled(HiCheckList, "ListChecks");
export const ListNumbers = styled(HiLeftToRightListNumber, "ListNumbers");
export const LockSimple = styled(HiLock, "LockSimple");
export const MagnifyingGlass = styled(HiSearch01, "MagnifyingGlass");
export const MagnifyingGlassIcon = styled(HiSearch01, "MagnifyingGlassIcon");
export const MailBlock = styled(HiMailBlock01, "MailBlock");
export const MailSearch = styled(HiMailSearch01, "MailSearch");
export const MailValidation = styled(HiMailValidation01, "MailValidation");
export const Microphone = styled(HiMic01, "Microphone");
export const Minus = styled(HiMinusSign, "Minus");
export const Monitor = styled(HiComputer, "Monitor");
export const MonitorIcon = styled(HiComputer, "MonitorIcon");
export const Moon = styled(HiMoon02, "Moon");
export const NetworkIcon = styled(HiNeuralNetwork, "NetworkIcon");
export const Note = styled(HiNote01, "Note");
export const NotePencil = styled(HiNoteEdit, "NotePencil");
export const Palette = styled(HiPaintBoard, "Palette");
export const PaperPlaneTilt = styled(HiSent, "PaperPlaneTilt");
export const Paperclip = styled(HiAttachment01, "Paperclip");
export const Path = styled(HiRoute01, "Path");
export const PathIcon = styled(HiRoute01, "PathIcon");
export const Pause = styled(HiPause, "Pause");
export const PencilSimple = styled(HiPencilEdit01, "PencilSimple");
export const Play = styled(HiPlay, "Play");
export const Plugs = styled(HiPlug01, "Plugs");
export const PlugsConnected = styled(HiPlug02, "PlugsConnected");
export const PlugsConnectedIcon = styled(HiPlug02, "PlugsConnectedIcon");
export const PlugsIcon = styled(HiPlug01, "PlugsIcon");
export const Plus = styled(HiPlusSign, "Plus");
export const Prohibit = styled(HiUnavailable, "Prohibit");
export const Question = styled(HiHelpCircle, "Question");
export const Quotes = styled(HiQuoteDown, "Quotes");
export const Receipt = styled(HiInvoice01, "Receipt");
export const Robot = styled(HiRobot01, "Robot");
export const Rocket = styled(HiRocket01, "Rocket");
export const SealCheck = styled(HiCheckmarkBadge01, "SealCheck");
export const SealCheckIcon = styled(HiCheckmarkBadge01, "SealCheckIcon");
export const ShareNetwork = styled(HiShare08, "ShareNetwork");
export const ShieldCheck = styled(HiSecurityCheck, "ShieldCheck");
export const SidebarSimple = styled(HiSidebarLeft, "SidebarSimple");
export const SignOut = styled(HiLogout01, "SignOut");
export const SlidersHorizontal = styled(HiSlidersHorizontal, "SlidersHorizontal");
export const Sparkle = styled(HiSparkles, "Sparkle");
export const SparkleIcon = styled(HiSparkles, "SparkleIcon");
export const SquaresFour = styled(HiDashboardSquare01, "SquaresFour");
export const Stack = styled(HiLayers01, "Stack");
export const StackIcon = styled(HiLayers01, "StackIcon");
export const Stop = styled(HiStop, "Stop");
export const Sun = styled(HiSun03, "Sun");
export const Target = styled(HiTarget02, "Target");
export const Task = styled(HiTask01, "Task");
export const TextB = styled(HiTextBold, "TextB");
export const TextHOne = styled(HiHeading01, "TextHOne");
export const TextHThree = styled(HiHeading03, "TextHThree");
export const TextHTwo = styled(HiHeading02, "TextHTwo");
export const TextItalic = styled(HiTextItalic, "TextItalic");
export const TextStrikethrough = styled(HiTextStrikethrough, "TextStrikethrough");
export const TextUnderline = styled(HiTextUnderline, "TextUnderline");
export const Trash = styled(HiDelete02, "Trash");
export const Tray = styled(HiInbox, "Tray");
export const TrayIcon = styled(HiInbox, "TrayIcon");
export const User = styled(HiUser, "User");
export const UserCircle = styled(HiUserCircle, "UserCircle");
export const UserFocus = styled(HiUserSearch01, "UserFocus");
export const Wallet = styled(HiWallet01, "Wallet");
export const Warning = styled(HiAlert02, "Warning");
export const WarningCircle = styled(HiAlertCircle, "WarningCircle");
export const WarningCircleIcon = styled(HiAlertCircle, "WarningCircleIcon");
export const WarningDiamond = styled(HiAlertDiamond, "WarningDiamond");
export const WarningIcon = styled(HiAlert02, "WarningIcon");
export const Wrench = styled(HiWrench01, "Wrench");
export const X = styled(HiCancel01, "X");
export const XCircle = styled(HiCancelCircle, "XCircle");
