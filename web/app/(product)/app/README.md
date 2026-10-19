# Product shell (`/app`)

**Goal:** Authenticated Oppulence dashboard — chat home, agents, workflows, revenue, report, settings.

**Auth:** Server layout calls `requireSession`; `proxy.ts` redirects anonymous users before stream.

**Client boundary:** `components/features/dashboard/dashboard-shell/` (sidebar, header, command palette, chat provider). Leaf routes own their panels under `_components/`.

**Assistant surface (the loom):** `/app` does not render a transcript. A run
is drawn as a numbered spine of moves — ask, think, read, write, approve,
answer — beside the yield those moves produced, so findings, drafts and
approvals stay put instead of scrolling away. One density dial (`1` `2` `3`)
redraws every move at outline, working or full depth. `j`/`k` walk the spine,
`Enter` opens a move, `/` returns to the composer. The derivation is pure and
lives in `lib/assistant/conversation-moves.ts`; the surface is
`components/features/assistant/loom-surface/`.

**BFF:** Browser data goes through `/api/backend/...`. Server prefetch calls the configured backend with a server-minted JWT (`prefetch.ts` on revenue and report).

**State:** TanStack Query for remote data, nuqs (`search-params.ts`) for shareable view-state, `useState` for local UI.

**New routes:** `pnpm gen -- page --route <name> --title "<Title>"`. Add `--operation <operationId>` when the page should prefetch a GET. Do not add surfaces to `components/features/dashboard/product-dashboard-client/product-dashboard-client.tsx`. See `docs/growth-standard.md`.

**Perf:** Leaf pages return a Suspense boundary immediately (`PrefetchHydration` seeds TanStack Query behind it). Do not export `instant = false` on a leaf — auth already blocks in the product layout. Prefer TanStack Query over manual polling; use React Scan + Dev toolkit API tab when tuning.
