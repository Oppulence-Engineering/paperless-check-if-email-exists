import Link from "next/link";
import type { ComponentPropsWithoutRef } from "react";

import { Button } from "@oppulence/ui/components/button";
import { cn } from "@oppulence/ui/lib/utils";
import { Group, Page, Row } from "@/components/platform/page-kit";

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
		<Page
			description="Review this action before joining the organization and switching your active workspace."
			title="Accept organization invitation?"
		>
			<section
				data-slot="accept-panel"
				className={cn("flex flex-col gap-10", className)}
				{...props}
			>
				<Group>
					<Row>
						<div className="flex flex-wrap justify-end gap-2">
							<Button asChild className="rounded-lg" variant="outline">
								<Link href="/app/settings?settings=identity">Cancel</Link>
							</Button>
							<form action="/api/auth/complete-invitation" method="post">
								<input name="invitation" type="hidden" value={invitationId} />
								<input name="return_to" type="hidden" value={returnTo} />
								<Button className="rounded-lg" type="submit">
									Accept invitation
								</Button>
							</form>
						</div>
					</Row>
				</Group>
			</section>
		</Page>
	);
}
