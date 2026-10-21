import { create } from "zustand";
export const useAddressPrefill = create<{ email: string; setEmail: (email: string) => void }>(
	(set) => ({ email: "", setEmail: (email) => set({ email }) }),
);
