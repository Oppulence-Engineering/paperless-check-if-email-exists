import { EditorialNotFound } from "./(marketing)/editorial/pages";
import { MarketingLayoutClient } from "./(marketing)/marketing-layout-client";

import "./(marketing)/editorial/editorial.css";

/** Unmatched URLs get the public site shell, so visitors keep the header and footer. */
export default function NotFound() {
	return (
		<MarketingLayoutClient>
			<EditorialNotFound />
		</MarketingLayoutClient>
	);
}
