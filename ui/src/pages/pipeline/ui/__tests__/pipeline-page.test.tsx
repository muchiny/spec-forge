import { describe, it, expect, beforeEach } from "vitest";
import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { renderWithProviders } from "@/test/test-utils";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import { PipelinePage } from "../pipeline-page";

describe("PipelinePage", () => {
  beforeEach(() => {
    usePipelineStore.setState({
      selectedFiles: [],
      outputDir: "./output",
      constitution: null,
      status: "idle",
      currentStage: null,
      progressMessages: [],
      error: null,
    });
  });

  it("renders the page", () => {
    renderWithProviders(<PipelinePage />);
    expect(screen.getByTestId("pipeline-page")).toBeInTheDocument();
  });

  it("renders output dir input with default value", () => {
    renderWithProviders(<PipelinePage />);
    const input = screen.getByTestId("pipeline-output-dir") as HTMLInputElement;
    expect(input.value).toBe("./output");
  });

  it("renders constitution textarea", () => {
    renderWithProviders(<PipelinePage />);
    expect(screen.getByTestId("pipeline-constitution")).toBeInTheDocument();
  });

  it("run button is disabled when no files selected", () => {
    renderWithProviders(<PipelinePage />);
    const button = screen.getByTestId("pipeline-run-button");
    expect(button).toBeDisabled();
  });

  it("run button is enabled when files are selected", () => {
    usePipelineStore.setState({ selectedFiles: ["/path/to/file.md"] });
    renderWithProviders(<PipelinePage />);
    const button = screen.getByTestId("pipeline-run-button");
    expect(button).not.toBeDisabled();
  });

  it("run button is disabled when running", () => {
    usePipelineStore.setState({
      selectedFiles: ["/path/to/file.md"],
      status: "running",
    });
    renderWithProviders(<PipelinePage />);
    const button = screen.getByTestId("pipeline-run-button");
    expect(button).toBeDisabled();
  });

  it("shows reset button when completed", () => {
    usePipelineStore.setState({ status: "completed" });
    renderWithProviders(<PipelinePage />);
    expect(screen.getByTestId("pipeline-reset-button")).toBeInTheDocument();
  });

  it("hides reset button when not completed", () => {
    renderWithProviders(<PipelinePage />);
    expect(
      screen.queryByTestId("pipeline-reset-button"),
    ).not.toBeInTheDocument();
  });

  it("updates output dir on input change", async () => {
    const user = userEvent.setup();
    renderWithProviders(<PipelinePage />);
    const input = screen.getByTestId("pipeline-output-dir");
    await user.clear(input);
    await user.type(input, "/new/path");
    expect(usePipelineStore.getState().outputDir).toBe("/new/path");
  });
});
