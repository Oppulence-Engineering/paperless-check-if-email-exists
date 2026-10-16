import { EditorialChangelog, type ChangelogEntry } from "../editorial/pages";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Changelog",
	description: "What changed in Verifier.",
	path: "/changelog",
});

/** User-facing changes, newest first. Each entry maps to a merged pull request. */
const entries: ChangelogEntry[] = [
	{
		version: "Scoring",
		date: "2026-09-24",
		title: "Catch-all grading",
		body: "Catch-all addresses now get a severity tier. A clean address on a free-provider domain is low risk, and a company domain is high risk. The score, the safe-to-send flag, and policy decisions use the same tier.",
	},
	{
		version: "Results",
		date: "2026-09-23",
		title: "Every check field in one view",
		body: "Open any result to see every field of the check: format, domain, mail server, mailbox, risk, and the reasons behind the score.",
	},
	{
		version: "Checks",
		date: "2026-09-23",
		title: "Clearer feedback for malformed addresses",
		body: "When an address is malformed, the check form tells you why before you run it.",
	},
	{
		version: "Integrations",
		date: "2026-07-17",
		title: "Delivery feedback from your email platform",
		body: "Connect SendGrid, Mailgun, Postmark, or Amazon SES. Bounces, complaints, and unsubscribes update your suppression list.",
	},
	{
		version: "Re-checks",
		date: "2026-05-10",
		title: "Automatic re-check of unknown results",
		body: "Addresses with an unknown result are checked again later, when the mail server is more likely to answer.",
	},
	{
		version: "Lists",
		date: "2026-05-08",
		title: "Fix plans for lists",
		body: "Each list gets a fix plan with the rows to keep, fix, review, or suppress, and files you can export.",
	},
];

export default function ChangelogPage() {
	return <EditorialChangelog entries={entries} />;
}
