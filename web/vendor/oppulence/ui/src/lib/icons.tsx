"use client";

import * as React from "react";
import { HugeiconsIcon, type IconSvgElement } from "@hugeicons/react";
import HiAlert02 from "@hugeicons/core-free-icons/Alert02Icon";
import HiAlertCircle from "@hugeicons/core-free-icons/AlertCircleIcon";
import HiArrowDown01 from "@hugeicons/core-free-icons/ArrowDown01Icon";
import HiArrowDown02 from "@hugeicons/core-free-icons/ArrowDown02Icon";
import HiArrowLeft01 from "@hugeicons/core-free-icons/ArrowLeft01Icon";
import HiArrowLeft02 from "@hugeicons/core-free-icons/ArrowLeft02Icon";
import HiArrowRight01 from "@hugeicons/core-free-icons/ArrowRight01Icon";
import HiArrowRight02 from "@hugeicons/core-free-icons/ArrowRight02Icon";
import HiArrowUp01 from "@hugeicons/core-free-icons/ArrowUp01Icon";
import HiCalendar03 from "@hugeicons/core-free-icons/Calendar03Icon";
import HiCancel01 from "@hugeicons/core-free-icons/Cancel01Icon";
import HiCheckmarkCircle02 from "@hugeicons/core-free-icons/CheckmarkCircle02Icon";
import HiCircle from "@hugeicons/core-free-icons/CircleIcon";
import HiDragDropVertical from "@hugeicons/core-free-icons/DragDropVerticalIcon";
import HiInformationCircle from "@hugeicons/core-free-icons/InformationCircleIcon";
import HiLoading03 from "@hugeicons/core-free-icons/Loading03Icon";
import HiMinusSign from "@hugeicons/core-free-icons/MinusSignIcon";
import HiMoreHorizontal from "@hugeicons/core-free-icons/MoreHorizontalIcon";
import HiSearch01 from "@hugeicons/core-free-icons/Search01Icon";
import HiSidebarLeft from "@hugeicons/core-free-icons/SidebarLeftIcon";
import HiTick02 from "@hugeicons/core-free-icons/Tick02Icon";

import { cn } from "#lib/utils";

export type IconWeight = "thin" | "light" | "regular" | "bold" | "fill" | "duotone";

export type AppIconProps = Omit<React.SVGProps<SVGSVGElement>, "ref"> & {
  title?: string;
  size?: number | string;
  weight?: IconWeight;
};

export type IconComponent = React.ForwardRefExoticComponent<
  AppIconProps & React.RefAttributes<SVGSVGElement>
>;

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
  const Wrapped = React.forwardRef<SVGSVGElement, AppIconProps>(function AppIcon(
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

export const AlertCircleIcon = styled(HiAlertCircle, "AlertCircleIcon");
export const ArrowDownIcon = styled(HiArrowDown02, "ArrowDownIcon");
export const ArrowLeft = styled(HiArrowLeft02, "ArrowLeft");
export const ArrowRight = styled(HiArrowRight02, "ArrowRight");
export const CalendarClockIcon = styled(HiCalendar03, "CalendarClockIcon");
export const CheckIcon = styled(HiTick02, "CheckIcon");
export const ChevronDownIcon = styled(HiArrowDown01, "ChevronDownIcon");
export const ChevronLeftIcon = styled(HiArrowLeft01, "ChevronLeftIcon");
export const ChevronRight = styled(HiArrowRight01, "ChevronRight");
export const ChevronRightIcon = styled(HiArrowRight01, "ChevronRightIcon");
export const ChevronUpIcon = styled(HiArrowUp01, "ChevronUpIcon");
export const CircleCheckIcon = styled(HiCheckmarkCircle02, "CircleCheckIcon");
export const CircleIcon = styled(HiCircle, "CircleIcon");
export const GripVerticalIcon = styled(HiDragDropVertical, "GripVerticalIcon");
export const InfoIcon = styled(HiInformationCircle, "InfoIcon");
export const Loader2Icon = styled(HiLoading03, "Loader2Icon");
export const MinusIcon = styled(HiMinusSign, "MinusIcon");
export const MoreHorizontal = styled(HiMoreHorizontal, "MoreHorizontal");
export const MoreHorizontalIcon = styled(HiMoreHorizontal, "MoreHorizontalIcon");
export const PanelLeftIcon = styled(HiSidebarLeft, "PanelLeftIcon");
export const SearchIcon = styled(HiSearch01, "SearchIcon");
export const TriangleAlertIcon = styled(HiAlert02, "TriangleAlertIcon");
export const XIcon = styled(HiCancel01, "XIcon");
