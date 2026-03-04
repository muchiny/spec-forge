import { create } from "zustand";
import { persist } from "zustand/middleware";

interface PreferencesStore {
  specViewTab: number;
  setSpecViewTab: (tab: number) => void;
}

export const usePreferencesStore = create<PreferencesStore>()(
  persist(
    (set) => ({
      specViewTab: 0,
      setSpecViewTab: (tab) => set({ specViewTab: tab }),
    }),
    { name: "spec-forge-preferences" },
  ),
);
