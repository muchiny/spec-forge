import { useEffect } from "react";
import { create } from "zustand";
import { persist } from "zustand/middleware";
import i18n from "@/shared/config/i18n";
import type { Locale } from "@/shared/config/i18n";

interface LocaleStore {
  locale: Locale;
  setLocale: (locale: Locale) => void;
}

function applyLocale(locale: Locale) {
  i18n.changeLanguage(locale);
  document.documentElement.setAttribute("lang", locale);
}

export const useLocaleStore = create<LocaleStore>()(
  persist(
    (set) => ({
      locale: "fr",
      setLocale: (locale) => {
        applyLocale(locale);
        set({ locale });
      },
    }),
    {
      name: "spec-forge-locale",
      onRehydrateStorage: () => (state) => {
        if (state) applyLocale(state.locale);
      },
    },
  ),
);

export function useLocaleSync() {
  const locale = useLocaleStore((s) => s.locale);
  useEffect(() => {
    applyLocale(locale);
  }, [locale]);
}
