import { OverviewPanel } from "./_components/overview-panel/overview-panel";

export const metadata = {
	title: "Overview",
	description: "Usage, lists, deliverability, and automation for this workspace",
};

/** The signed-in home. The panel loads its data in the browser, section by section. */
export default function ProductPage() {
	return <OverviewPanel />;
}
