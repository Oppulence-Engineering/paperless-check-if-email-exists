import Image from "next/image";

/** Keep the Sim preview frame while showing this application's workflow. */
export function HeroPlatformLoopMount() {
	return (
		<Image
			alt="Sample email check result"
			fill
			priority
			sizes="(min-width: 1280px) 1100px, 100vw"
			src="/marketing/email-check-preview.svg"
		/>
	);
}
