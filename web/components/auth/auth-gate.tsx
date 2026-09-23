"use client";

import { createContext, useContext, useEffect, type ReactNode } from "react";

import type { BrowserSessionResponse } from "@/lib/auth/schemas";
import { entitlementsFor } from "@/lib/entitlements/entitlements";
import type { Entitlements } from "@/lib/entitlements/entitlements.schema";
import { removeBrowserStorageKeys } from "@/lib/storage/scoped-storage";

type AuthenticatedSession = Extract<BrowserSessionResponse, { authenticated: true }>;

const AuthSessionContext = createContext<AuthenticatedSession | null>(null);
const LEGACY_UNSCOPED_STORAGE_KEYS = [
	"oppulence.relationship-graph.saved-views.v1",
	"oppulence.relationship-graph.saved-views.v2-imported",
];

export function useAuthSession() {
	const value = useContext(AuthSessionContext);
	if (!value) {
		throw new Error("useAuthSession must be used inside AuthGate");
	}
	return value;
}

/**
 * What this workspace is allowed to do, derived from the session it already
 * has. The Go API enforces the same policy; this exists so the interface can
 * explain a limit instead of hiding a control and confusing a person.
 */
export function useEntitlements(): Entitlements {
	const session = useAuthSession();
	return entitlementsFor({
		plan: session.billing?.plan,
		status: session.billing?.status,
	});
}

export function AuthGate({
	children,
	initialSession,
}: {
	children: ReactNode;
	initialSession: AuthenticatedSession;
}) {
	useEffect(() => {
		removeBrowserStorageKeys(LEGACY_UNSCOPED_STORAGE_KEYS);
	}, [initialSession.user.id, initialSession.user.organizationId]);

	return (
		<AuthSessionContext.Provider value={initialSession}>{children}</AuthSessionContext.Provider>
	);
}
