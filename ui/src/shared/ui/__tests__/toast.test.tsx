import { describe, it, expect, beforeEach, vi } from "vitest";
import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { renderWithProviders } from "@/test/test-utils";
import { useToastStore } from "../toast-store";
import { ToastContainer } from "../toast";

describe("ToastContainer", () => {
  beforeEach(() => {
    useToastStore.setState({ toasts: [] });
  });

  it("renders nothing when no toasts", () => {
    const { container } = renderWithProviders(<ToastContainer />);
    expect(container.innerHTML).toBe("");
  });

  it("renders toast items", () => {
    useToastStore.setState({
      toasts: [
        { id: "t1", message: "Success!", variant: "success", duration: 4000 },
        { id: "t2", message: "Error!", variant: "error", duration: 6000 },
      ],
    });
    renderWithProviders(<ToastContainer />);
    expect(screen.getByText("Success!")).toBeInTheDocument();
    expect(screen.getByText("Error!")).toBeInTheDocument();
  });

  it("has role=alert on toast items", () => {
    useToastStore.setState({
      toasts: [
        { id: "t1", message: "Alert!", variant: "info", duration: 4000 },
      ],
    });
    renderWithProviders(<ToastContainer />);
    expect(screen.getByRole("alert")).toBeInTheDocument();
  });

  it("has aria-live=polite on container", () => {
    useToastStore.setState({
      toasts: [{ id: "t1", message: "Msg", variant: "info", duration: 4000 }],
    });
    renderWithProviders(<ToastContainer />);
    expect(screen.getByRole("alert").parentElement).toHaveAttribute(
      "aria-live",
      "polite",
    );
  });

  it("auto-dismisses after duration", () => {
    vi.useFakeTimers();
    useToastStore.setState({
      toasts: [{ id: "t1", message: "Auto", variant: "info", duration: 3000 }],
    });
    renderWithProviders(<ToastContainer />);
    expect(screen.getByText("Auto")).toBeInTheDocument();

    vi.advanceTimersByTime(3000);
    expect(useToastStore.getState().toasts).toHaveLength(0);
    vi.useRealTimers();
  });

  it("dismiss button removes toast", async () => {
    vi.useRealTimers();
    const user = userEvent.setup();
    useToastStore.setState({
      toasts: [{ id: "t1", message: "Dismiss me", variant: "warning" }],
    });
    renderWithProviders(<ToastContainer />);

    const dismissButton = screen.getByRole("button");
    await user.click(dismissButton);
    expect(useToastStore.getState().toasts).toHaveLength(0);
  });
});
