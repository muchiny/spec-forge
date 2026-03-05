import { describe, it, expect, beforeEach, vi } from "vitest";
import { useLocaleStore } from "../use-locale-store";

vi.mock("@/shared/config/i18n", () => ({
  default: { changeLanguage: vi.fn() },
  supportedLocales: ["en", "fr"] as const,
  localeLabels: { en: "English", fr: "Français" },
}));

describe("useLocaleStore", () => {
  beforeEach(() => {
    useLocaleStore.setState({ locale: "fr" });
    document.documentElement.removeAttribute("lang");
  });

  it("has default locale fr", () => {
    expect(useLocaleStore.getState().locale).toBe("fr");
  });

  it("setLocale updates locale to en", () => {
    useLocaleStore.getState().setLocale("en");
    expect(useLocaleStore.getState().locale).toBe("en");
  });

  it("setLocale sets document lang attribute", () => {
    useLocaleStore.getState().setLocale("en");
    expect(document.documentElement.getAttribute("lang")).toBe("en");
  });

  it("setLocale calls i18n.changeLanguage", async () => {
    const i18n = await import("@/shared/config/i18n");
    useLocaleStore.getState().setLocale("en");
    expect(i18n.default.changeLanguage).toHaveBeenCalledWith("en");
  });
});
