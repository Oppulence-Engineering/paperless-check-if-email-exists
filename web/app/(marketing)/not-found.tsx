import { MarketingButtonLink } from "./marketing-primitives";

/**
 * Segment-level 404. The marketing group layout already supplies header and
 * footer, so this file must not wrap the public shell again.
 */
export default function MarketingNotFound() {
	return (
		<div className="linear-inset px-6 pt-32 pb-20">
			<p className="linear-eyebrow">[404]</p>
			<h1 className="linear-subpage-title mt-4">We could not find that page.</h1>
			<p className="linear-body mt-5 max-w-xl">
				It may have moved. Start from the home page, or browse the guides and features.
			</p>
			<div className="mt-8 flex flex-col gap-3 sm:flex-row">
				<MarketingButtonLink href="/">Return home</MarketingButtonLink>
				<MarketingButtonLink href="/resources" variant="outline">
					Browse resources
				</MarketingButtonLink>
			</div>
		</div>
	);
}
