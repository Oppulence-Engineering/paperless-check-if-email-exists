"use client";

import "client-only";

import * as React from "react";
import { create } from "zustand";

import { SettingsUnsavedStateSchema, type SettingsUnsavedState } from "./settings-unsaved.schema";

/**
 * @oppulence-gen kind=store
 * Ephemeral Settings unsaved client state. Zustand is not for navigation, remote data,
 * or shareable filters. Persist, if ever required, must go through
 * `lib/storage/scoped-storage.ts`. Owned by `settings-unsaved.lit.ts`.
 *
 * The settings rail lives in a different tree from the cards that hold the
 * edits, so the unsaved flag cannot travel through React context. Each card
 * registers its own key; navigation reads the set before it discards anything.
 */
const initialState = SettingsUnsavedStateSchema.parse({
	dirtyKeys: [],
});

export const useSettingsUnsavedStore = create<
	SettingsUnsavedState & {
		setDirty: (key: string, dirty: boolean) => void;
		reset: () => void;
	}
>()((set) => ({
	...initialState,
	setDirty: (key, dirty) =>
		set((state) => {
			const held = state.dirtyKeys.includes(key);
			if (dirty === held) return state;
			return {
				dirtyKeys: dirty
					? [...state.dirtyKeys, key]
					: state.dirtyKeys.filter((entry) => entry !== key),
			};
		}),
	reset: () => set(initialState),
}));

/** Readable outside React, where navigation handlers run. */
export function hasUnsavedSettings(): boolean {
	return useSettingsUnsavedStore.getState().dirtyKeys.length > 0;
}

/** Registers one card's dirty flag and clears it when the card unmounts. */
export function useUnsavedFlag(key: string, dirty: boolean): void {
	const setDirty = useSettingsUnsavedStore((state) => state.setDirty);
	React.useEffect(() => {
		setDirty(key, dirty);
		return () => setDirty(key, false);
	}, [dirty, key, setDirty]);
}
