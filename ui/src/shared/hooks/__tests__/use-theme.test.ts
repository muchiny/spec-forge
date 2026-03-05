import { describe, it, expect, beforeEach } from "vitest";
import { useThemeStore } from "../use-theme";

describe("useThemeStore", () => {
  beforeEach(() => {
    useThemeStore.setState({ theme: "dark" });
    document.documentElement.removeAttribute("data-theme");
  });

  it("has default theme dark", () => {
    expect(useThemeStore.getState().theme).toBe("dark");
  });

  it("toggleTheme switches dark to light", () => {
    useThemeStore.getState().toggleTheme();
    expect(useThemeStore.getState().theme).toBe("light");
  });

  it("toggleTheme switches light back to dark", () => {
    useThemeStore.setState({ theme: "light" });
    useThemeStore.getState().toggleTheme();
    expect(useThemeStore.getState().theme).toBe("dark");
  });

  it("toggleTheme sets data-theme attribute on document", () => {
    useThemeStore.getState().toggleTheme();
    expect(document.documentElement.getAttribute("data-theme")).toBe("light");
  });
});
