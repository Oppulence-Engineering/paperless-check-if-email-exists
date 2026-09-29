"use client";

import "client-only";

import { createTRPCClient, httpBatchLink } from "@trpc/client";
import { QueryClientProvider, type QueryClient } from "@tanstack/react-query";
import { createTRPCContext } from "@trpc/tanstack-react-query";
import { useEffect, useMemo, useRef, type ReactNode } from "react";

import { QueryDevtoolsPanel } from "@/components/dev/query-devtools";
import { WORKSPACE_CHANGE_CHANNEL_NAME, WORKSPACE_CHANGED_EVENT } from "@/lib/auth/client";
import { WorkspaceSwitchResponseSchema } from "@/lib/auth/schemas";
import type { BackendTrpcRouter } from "@/lib/backend/trpc-bridge";
import { createAppQueryClient } from "@/lib/query/get-query-client";

export const { TRPCProvider, useTRPC } = createTRPCContext<BackendTrpcRouter>();

export async function clearWorkspaceQueryCache(client: QueryClient): Promise<void> {
	await client.cancelQueries();
	client.clear();
}

function createOrganizationQueryClient(organizationId: string) {
	WorkspaceSwitchResponseSchema.shape.activeOrganizationId.parse(organizationId);
	return createAppQueryClient();
}

function createOrganizationTrpcClient(organizationId: string) {
	WorkspaceSwitchResponseSchema.shape.activeOrganizationId.parse(organizationId);
	return createTRPCClient<BackendTrpcRouter>({
		links: [httpBatchLink({ url: "/api/trpc" })],
	});
}

export function QueryProvider({
	children,
	organizationId,
}: {
	children: ReactNode;
	organizationId: string;
}) {
	const client = useMemo(() => createOrganizationQueryClient(organizationId), [organizationId]);
	const previousClient = useRef(client);
	const handlingWorkspaceChange = useRef(false);
	const trpcClient = useMemo(() => createOrganizationTrpcClient(organizationId), [organizationId]);

	useEffect(() => {
		const staleClient = previousClient.current;
		previousClient.current = client;
		if (staleClient === client) return;
		void clearWorkspaceQueryCache(staleClient);
	}, [client]);

	useEffect(() => {
		const handleWorkspaceChange = (data: unknown) => {
			const message = WorkspaceSwitchResponseSchema.safeParse(data);
			if (
				!message.success ||
				message.data.activeOrganizationId === organizationId ||
				handlingWorkspaceChange.current
			) {
				return;
			}
			handlingWorkspaceChange.current = true;
			void clearWorkspaceQueryCache(client).finally(() => window.location.reload());
		};

		const onLocalWorkspaceChange = (event: Event) => {
			handleWorkspaceChange((event as CustomEvent<unknown>).detail);
		};
		const onRemoteWorkspaceChange = (event: MessageEvent<unknown>) => {
			handleWorkspaceChange(event.data);
		};

		window.addEventListener(WORKSPACE_CHANGED_EVENT, onLocalWorkspaceChange);
		const channel =
			typeof BroadcastChannel === "undefined"
				? null
				: new BroadcastChannel(WORKSPACE_CHANGE_CHANNEL_NAME);
		channel?.addEventListener("message", onRemoteWorkspaceChange);
		return () => {
			window.removeEventListener(WORKSPACE_CHANGED_EVENT, onLocalWorkspaceChange);
			channel?.removeEventListener("message", onRemoteWorkspaceChange);
			channel?.close();
		};
	}, [client, organizationId]);

	return (
		<QueryClientProvider key={organizationId} client={client}>
			<TRPCProvider queryClient={client} trpcClient={trpcClient}>
				{children}
				<QueryDevtoolsPanel />
			</TRPCProvider>
		</QueryClientProvider>
	);
}
