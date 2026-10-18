import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Activity feed.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `activity-feed.lit.ts`.
 */
export const ActivityFeedPropsSchema = z.object({});

export type ActivityFeedPropsFields = z.infer<typeof ActivityFeedPropsSchema>;
