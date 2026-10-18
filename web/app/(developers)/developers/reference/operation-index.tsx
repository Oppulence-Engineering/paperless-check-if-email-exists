"use client";

import Link from "next/link";
import { useState } from "react";

import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import type { Family, PortalOperation } from "@/lib/developer-portal/operations";

import { DocsSection, bodyClassName, cardSurface } from "../docs-kit";

type Entry = Pick<PortalOperation, "id" | "method" | "path" | "family" | "description">;

export function OperationIndex({
	families,
	operations,
}: {
	families: readonly Family[];
	operations: Entry[];
}) {
	const [query, setQuery] = useState("");
	const matching = operations.filter((operation) =>
		`${operation.method} ${operation.path} ${operation.description ?? ""} ${operation.id}`
			.toLowerCase()
			.includes(query.toLowerCase().trim()),
	);

	return (
		<>
			<div className="space-y-2">
				<Label className="text-sm font-medium text-neutral-300" htmlFor="api-operation-search">
					Find an operation
				</Label>
				<Input
					id="api-operation-search"
					type="search"
					value={query}
					onChange={(event) => setQuery(event.target.value)}
					placeholder="Method, path, or operation ID"
					className="h-10 rounded-lg"
				/>
			</div>
			{matching.length === 0 ? <p className={bodyClassName}>No matching operations.</p> : null}
			{families.map((family) => {
				const entries = matching.filter((operation) => operation.family === family);
				return entries.length === 0 ? null : (
					<DocsSection key={family} title={family}>
						<ul className="grid grid-cols-1 gap-3">
							{entries.map((operation) => (
								<li key={operation.id}>
									<Link
										className={`block ${cardSurface} transition-colors hover:bg-white/[0.04]`}
										href={`/developers/reference/${operation.id}`}
									>
										<p className="font-mono text-[13px] text-foreground">
											{operation.method} {operation.path}
										</p>
										{operation.description ? (
											<p className="mt-1 text-sm text-muted-foreground">{operation.description}</p>
										) : null}
									</Link>
								</li>
							))}
						</ul>
					</DocsSection>
				);
			})}
		</>
	);
}
