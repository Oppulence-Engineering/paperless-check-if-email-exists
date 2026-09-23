import Link from "next/link";
import type { ComponentPropsWithoutRef } from "react";

import { Button } from "@oppulence/ui/components/button";
import {
	Card,
	CardContent,
	CardDescription,
	CardHeader,
	CardTitle,
} from "@oppulence/ui/components/card";
import { cn } from "@oppulence/ui/lib/utils";

import { type AcceptPanelPropsFields } from "./accept-panel.schema";

/**
 * @oppulence-gen kind=component
 * AcceptPanel is a server presentation component.
 * Accept invitation route-private panel. Presentation only.
 *
 * Data and callbacks arrive through validated props. This module does not fetch,
 * persist, or read cookies. Owned by `accept-panel.lit.ts`.
 */
export type AcceptPanelProps = AcceptPanelPropsFields & ComponentPropsWithoutRef<"section">;

export function AcceptPanel({ className, invitationId, returnTo, ...props }: AcceptPanelProps) {
	return (
		<section
			data-slot="accept-panel"
			className={cn("grid min-h-full place-items-center p-6", className)}
			{...props}
		>
			<Card className="w-full max-w-lg">
				<CardHeader>
					<CardTitle>Accept organization invitation?</CardTitle>
					<CardDescription>
						Review this action before joining the organization and switching your active workspace.
					</CardDescription>
				</CardHeader>
				<CardContent className="flex flex-wrap gap-3">
					<form action="/api/auth/complete-invitation" method="post">
						<input name="invitation" type="hidden" value={invitationId} />
						<input name="return_to" type="hidden" value={returnTo} />
						<Button type="submit">Accept invitation</Button>
					</form>
					<Button asChild variant="outline">
						<Link href="/app/settings?settings=identity">Cancel</Link>
					</Button>
				</CardContent>
			</Card>
		</section>
	);
}
