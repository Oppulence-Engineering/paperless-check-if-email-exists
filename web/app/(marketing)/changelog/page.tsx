import { SimChangelogPage } from "../sim-landing/subpages/sim-changelog-page";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Changelog",
	description: "Published changes to Check If Email Exists.",
	path: "/changelog",
});

export default function ChangelogPage() {
	return <SimChangelogPage entries={[]} repoLabel="Check If Email Exists" />;
}
