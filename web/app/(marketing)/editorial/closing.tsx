import { ButtonLink, Container, Display, Muted } from "./ui";

/** Closing call to action above the footer on every marketing page, centered as on polar.sh. */
export function EditorialClosing() {
	return (
		<section
			aria-labelledby="closing-heading"
			className="border-t border-[var(--ed-line)] bg-[var(--ed-bg)] py-24 sm:py-32"
		>
			<Container className="flex flex-col items-center text-center">
				<Display className="max-w-[20ch]" id="closing-heading" size="lg">
					<span className="block">From a raw list</span>
					<Muted>to a safe send.</Muted>
				</Display>
				<div className="mt-10 flex flex-wrap justify-center gap-2.5">
					<ButtonLink arrow href="/sign-up" size="md">
						Get started free
					</ButtonLink>
				</div>
				<p className="ed-sans mt-6 text-[13px] text-[var(--ed-muted)]">
					No email is sent during a check. The free plan includes 100 checks a month.
				</p>
			</Container>
		</section>
	);
}
