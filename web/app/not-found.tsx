import { marketingFontVariables } from "@/lib/fonts";

import { EditorialNotFound } from "./(marketing)/editorial/pages";
import { MarketingLayoutClient } from "./(marketing)/marketing-layout-client";

import "./(marketing)/marketing-sim-theme.css";
import "./(marketing)/marketing-subpages.css";
import "./(marketing)/editorial/editorial.css";

/** Unmatched URLs get the public site shell, so visitors keep the header and footer. */
export default function NotFound() {
	return (
		<div className={marketingFontVariables}>
			<MarketingLayoutClient>
				<EditorialNotFound />
			</MarketingLayoutClient>
		</div>
	);
}
