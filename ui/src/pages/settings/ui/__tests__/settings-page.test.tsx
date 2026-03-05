import { describe, it, expect, beforeEach } from "vitest";
import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { renderWithProviders } from "@/test/test-utils";
import { useThemeStore } from "@/shared/hooks/use-theme";
import { SettingsPage } from "../settings-page";

describe("SettingsPage", () => {
  beforeEach(() => {
    useThemeStore.setState({ theme: "dark" });
  });

  it("renders the page", () => {
    renderWithProviders(<SettingsPage />);
    expect(screen.getByTestId("settings-page")).toBeInTheDocument();
  });

  it("renders theme toggle button", () => {
    renderWithProviders(<SettingsPage />);
    expect(screen.getByTestId("settings-theme-toggle")).toBeInTheDocument();
  });

  it("toggles theme on click", async () => {
    const user = userEvent.setup();
    renderWithProviders(<SettingsPage />);

    await user.click(screen.getByTestId("settings-theme-toggle"));
    expect(useThemeStore.getState().theme).toBe("light");
  });

  it("displays language options", () => {
    renderWithProviders(<SettingsPage />);
    expect(screen.getByText("English")).toBeInTheDocument();
    expect(screen.getByText("Français")).toBeInTheDocument();
  });
});
