"use client";

import Link from "next/link";
import { useState } from "react";

import type { Family, PortalOperation } from "@/lib/developer-portal/operations";

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
			<label htmlFor="api-operation-search">Find an operation</label>
			<input
				id="api-operation-search"
				type="search"
				value={query}
				onChange={(event) => setQuery(event.target.value)}
				placeholder="Method, path, or operation ID"
				className="mb-6 block w-full rounded-md border border-border bg-background px-3 py-2"
			/>
			{matching.length === 0 ? <p>No matching operations.</p> : null}
			{families.map((family) => {
				const entries = matching.filter((operation) => operation.family === family);
				return entries.length === 0 ? null : (
					<section key={family}>
						<h2>{family}</h2>
						<ul>
							{entries.map((operation) => (
								<li key={operation.id}>
									<Link href={`/developers/reference/${operation.id}`}>
										<code>{operation.method}</code> <code>{operation.path}</code>
									</Link>
									{operation.description ? ` — ${operation.description}` : ""}
								</li>
							))}
						</ul>
					</section>
				);
			})}
		</>
	);
}
