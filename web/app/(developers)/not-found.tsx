import { EditorialNotFound } from "@/app/(marketing)/editorial/pages";

/**
 * Segment-level 404. The developer layout already supplies header and
 * footer, so this file must not wrap the public shell again.
 */
export default function DeveloperNotFound() {
	return <EditorialNotFound />;
}
