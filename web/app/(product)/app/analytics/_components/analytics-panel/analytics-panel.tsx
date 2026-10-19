"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef } from "react";

import { Button } from "@oppulence/ui/components/button";
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from "@oppulence/ui/components/sheet";
import { cn } from "@oppulence/ui/lib/utils";
import { ActivityFeed } from "@/components/features/insights/activity-feed/activity-feed";
import { ResultExplorer } from "@/components/features/insights/result-explorer/result-explorer";
import { type ExplorerFilters } from "@/components/features/insights/result-explorer/result-explorer.schema";
import { SavedSegments } from "@/components/features/insights/saved-segments/saved-segments";
import { SourceQuality } from "@/components/features/insights/source-quality/source-quality";
import { Page, Section } from "@/components/platform/page-kit";

import { type AnalyticsPanelPropsFields } from "./analytics-panel.schema";

/**
 * @oppulence-gen kind=component Owned by `analytics-panel.lit.ts`.
 * The panel keeps the explorer filters, so a saved segment can set them.
 */
export type AnalyticsPanelProps = AnalyticsPanelPropsFields & ComponentPropsWithoutRef<"section">;

export function AnalyticsPanel({ className, ...props }: AnalyticsPanelProps) {
	const [filters, setFilters] = useState<ExplorerFilters>({});
	const [segmentsOpen, setSegmentsOpen] = useState(false);
	return (
		<Page
			description="Explore results, job activity, and source quality."
			title="Verification analytics"
		>
			<section
				data-slot="analytics-panel"
				className={cn("flex flex-col gap-10", className)}
				{...props}
			>
				<Section
					actions={
						<Button
							className="rounded-lg"
							onClick={() => {
								setSegmentsOpen(true);
							}}
							size="sm"
							type="button"
							variant="outline"
						>
							Saved segments
						</Button>
					}
					description="Results from all jobs and single checks. Select an address to see the full verdict."
					title="Result explorer"
				>
					<ResultExplorer filters={filters} onFiltersChange={setFilters} />
				</Section>
				<Section description="Events from your bulk jobs, newest first." title="Job activity">
					<ActivityFeed />
				</Section>
				<Section
					description="Counters for each source key that you set when you upload a list."
					title="Source quality"
				>
					<SourceQuality />
				</Section>
			</section>
			<Sheet onOpenChange={setSegmentsOpen} open={segmentsOpen}>
				<SheetContent className="flex w-full flex-col gap-6 overflow-y-auto pb-8 sm:max-w-xl">
					<SheetHeader>
						<SheetTitle>Saved segments</SheetTitle>
						<SheetDescription>
							Save the explorer filters with a name. Open a segment to use its filters again.
						</SheetDescription>
					</SheetHeader>
					<SavedSegments
						className="px-4"
						filters={filters}
						onOpen={(next) => {
							setFilters(next);
							setSegmentsOpen(false);
						}}
					/>
				</SheetContent>
			</Sheet>
		</Page>
	);
}
