import { describe, it, expect, beforeEach, vi } from "vitest";
import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { renderWithProviders } from "@/test/test-utils";
import { useLocaleStore } from "@/shared/stores/use-locale-store";
import { LanguageSwitcher } from "../language-switcher";

vi.mock("@/shared/assets/flags/gb.svg", () => ({ default: "gb.svg" }));
vi.mock("@/shared/assets/flags/fr.svg", () => ({ default: "fr.svg" }));

describe("LanguageSwitcher", () => {
  beforeEach(() => {
    useLocaleStore.setState({ locale: "fr" });
  });

  it("renders the switcher button", () => {
    renderWithProviders(<LanguageSwitcher />);
    expect(screen.getByTestId("lang-switcher-button")).toBeInTheDocument();
  });

  it("opens dropdown on click", async () => {
    const user = userEvent.setup();
    renderWithProviders(<LanguageSwitcher />);

    expect(
      screen.queryByTestId("lang-switcher-dropdown"),
    ).not.toBeInTheDocument();
    await user.click(screen.getByTestId("lang-switcher-button"));
    expect(screen.getByTestId("lang-switcher-dropdown")).toBeInTheDocument();
  });

  it("shows all locale options", async () => {
    const user = userEvent.setup();
    renderWithProviders(<LanguageSwitcher />);
    await user.click(screen.getByTestId("lang-switcher-button"));

    expect(screen.getByTestId("lang-switcher-option-en")).toBeInTheDocument();
    expect(screen.getByTestId("lang-switcher-option-fr")).toBeInTheDocument();
    expect(screen.getByText("English")).toBeInTheDocument();
    expect(screen.getByText("Français")).toBeInTheDocument();
  });

  it("selects a locale and closes dropdown", async () => {
    const user = userEvent.setup();
    renderWithProviders(<LanguageSwitcher />);
    await user.click(screen.getByTestId("lang-switcher-button"));
    await user.click(screen.getByTestId("lang-switcher-option-en"));

    expect(useLocaleStore.getState().locale).toBe("en");
    expect(
      screen.queryByTestId("lang-switcher-dropdown"),
    ).not.toBeInTheDocument();
  });

  it("has correct aria attributes", async () => {
    const user = userEvent.setup();
    renderWithProviders(<LanguageSwitcher />);

    const button = screen.getByTestId("lang-switcher-button");
    expect(button).toHaveAttribute("aria-haspopup", "listbox");
    expect(button).toHaveAttribute("aria-expanded", "false");

    await user.click(button);
    expect(button).toHaveAttribute("aria-expanded", "true");
    expect(screen.getByRole("listbox")).toBeInTheDocument();
  });

  it("closes on Escape key", async () => {
    const user = userEvent.setup();
    renderWithProviders(<LanguageSwitcher />);

    await user.click(screen.getByTestId("lang-switcher-button"));
    expect(screen.getByTestId("lang-switcher-dropdown")).toBeInTheDocument();

    await user.keyboard("{Escape}");
    expect(
      screen.queryByTestId("lang-switcher-dropdown"),
    ).not.toBeInTheDocument();
  });
});
