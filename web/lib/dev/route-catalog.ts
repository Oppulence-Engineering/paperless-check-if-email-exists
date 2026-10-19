export type RouteCatalogEntry = {
	path: string;
	label: string;
	segment: "product" | "marketing" | "auth" | "dev";
	auth: "public" | "session";
	clientBoundary: string;
	bffEndpoints: string[];
	e2e?: string;
	readme?: string;
};

export function productRouteCatalog(): RouteCatalogEntry[] {
	return [
		{
			path: "/app",
			label: "Overview",
			segment: "product",
			auth: "session",
			clientBoundary: "app/(product)/app/_components/overview-panel",
			bffEndpoints: ["/api/backend/v1/me/usage", "/api/backend/v1/lists", "/api/backend/v1/alerts"],
		},
		{
			path: "/app/check",
			label: "Check an address",
			segment: "product",
			auth: "session",
			clientBoundary: "app/(product)/app/check/_components/check-panel",
			bffEndpoints: ["/api/backend/v1/check_email"],
		},
		{
			path: "/app/lists",
			label: "Lists",
			segment: "product",
			auth: "session",
			clientBoundary: "app/(product)/app/lists",
			bffEndpoints: ["/api/backend/v1/lists"],
		},
		{
			path: "/app/history",
			label: "Address history",
			segment: "product",
			auth: "session",
			clientBoundary: "app/(product)/app/history",
			bffEndpoints: ["/api/backend/v1/email-history"],
		},
		{
			path: "/app/settings",
			label: "Settings",
			segment: "product",
			auth: "session",
			clientBoundary: "app/(product)/app/settings",
			bffEndpoints: ["/api/auth/workspace"],
		},
	];
}

export const marketingRouteSamples: RouteCatalogEntry[] = [
	{
		path: "/",
		label: "Home",
		segment: "marketing",
		auth: "public",
		clientBoundary: "app/(marketing)/page.tsx",
		bffEndpoints: [],
	},
	{
		path: "/blog",
		label: "Blog",
		segment: "marketing",
		auth: "public",
		clientBoundary: "app/(marketing)/blog/page.tsx",
		bffEndpoints: [],
	},
];

export function fullRouteCatalog(): RouteCatalogEntry[] {
	return [...productRouteCatalog(), ...marketingRouteSamples];
}
