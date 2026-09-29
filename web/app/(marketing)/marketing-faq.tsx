"use client";

import "client-only";

import {
	Accordion,
	AccordionContent,
	AccordionItem,
	AccordionTrigger,
} from "@oppulence/ui/components/accordion";
import { MarketingSpan } from "./marketing-primitives";
import { cn } from "@/lib/utils";

export type MarketingFaqItem = {
	question: string;
	answer: string;
};

/** Public FAQ using the shared shadcn disclosure component. */
export function MarketingFaq({
	items,
	className,
	heading = "Questions people actually ask",
	lede,
}: {
	items: MarketingFaqItem[];
	className?: string;
	heading?: string;
	lede?: string;
}) {
	return (
		<section className={cn("linear-faq mk-faq", className)}>
			<header>
				<h2>{heading}</h2>
				{lede ? <p>{lede}</p> : null}
			</header>
			<Accordion type="multiple">
				{items.map((item, index) => (
					<AccordionItem className="border-b-0" key={item.question} value={item.question}>
						<AccordionTrigger className="py-0 hover:no-underline [&>svg]:hidden">
							{String(index + 1).padStart(2, "0")} {item.question}
							<MarketingSpan aria-hidden="true">+</MarketingSpan>
						</AccordionTrigger>
						<AccordionContent className="pb-0" forceMount>
							<p>{item.answer}</p>
						</AccordionContent>
					</AccordionItem>
				))}
			</Accordion>
		</section>
	);
}
