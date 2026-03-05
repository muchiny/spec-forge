import { describe, it, expect, vi } from "vitest";
import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { renderWithProviders } from "@/test/test-utils";
import { ExportButton } from "../export-button";

describe("ExportButton", () => {
  it("renders the export button", () => {
    renderWithProviders(<ExportButton onExport={vi.fn()} />);
    expect(screen.getByTestId("export-button")).toBeInTheDocument();
  });

  it("opens dropdown on click", async () => {
    const user = userEvent.setup();
    renderWithProviders(<ExportButton onExport={vi.fn()} />);

    expect(screen.queryByTestId("export-dropdown")).not.toBeInTheDocument();
    await user.click(screen.getByTestId("export-button"));
    expect(screen.getByTestId("export-dropdown")).toBeInTheDocument();
  });

  it("calls onExport with csv and closes dropdown", async () => {
    const onExport = vi.fn();
    const user = userEvent.setup();
    renderWithProviders(<ExportButton onExport={onExport} />);

    await user.click(screen.getByTestId("export-button"));
    await user.click(screen.getByTestId("export-csv"));

    expect(onExport).toHaveBeenCalledWith("csv");
    expect(screen.queryByTestId("export-dropdown")).not.toBeInTheDocument();
  });

  it("calls onExport with xlsx and closes dropdown", async () => {
    const onExport = vi.fn();
    const user = userEvent.setup();
    renderWithProviders(<ExportButton onExport={onExport} />);

    await user.click(screen.getByTestId("export-button"));
    await user.click(screen.getByTestId("export-xlsx"));

    expect(onExport).toHaveBeenCalledWith("xlsx");
    expect(screen.queryByTestId("export-dropdown")).not.toBeInTheDocument();
  });
});
