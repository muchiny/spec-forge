import { describe, it, expect, beforeEach } from "vitest";
import { screen, within } from "@testing-library/react";
import { renderWithProviders } from "@/test/test-utils";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import { ProgressPanel } from "../progress-panel";

describe("ProgressPanel", () => {
  beforeEach(() => {
    usePipelineStore.setState({
      status: "idle",
      currentStage: null,
      progressMessages: [],
      error: null,
    });
  });

  it("returns null when idle", () => {
    const { container } = renderWithProviders(<ProgressPanel />);
    expect(container.innerHTML).toBe("");
  });

  it("renders panel when running", () => {
    usePipelineStore.setState({
      status: "running",
      currentStage: "ReadingInput",
    });
    renderWithProviders(<ProgressPanel />);
    expect(screen.getByTestId("progress-panel")).toBeInTheDocument();
  });

  it("renders all pipeline stages", () => {
    usePipelineStore.setState({
      status: "running",
      currentStage: "RefiningSpec",
    });
    renderWithProviders(<ProgressPanel />);
    expect(
      screen.getByTestId("progress-stage-ReadingInput"),
    ).toBeInTheDocument();
    expect(
      screen.getByTestId("progress-stage-RefiningSpec"),
    ).toBeInTheDocument();
    expect(
      screen.getByTestId("progress-stage-GeneratingTests"),
    ).toBeInTheDocument();
    expect(
      screen.getByTestId("progress-stage-WritingOutput"),
    ).toBeInTheDocument();
  });

  it("shows error display when error exists", () => {
    usePipelineStore.setState({
      status: "error",
      error: "LLM timeout",
      currentStage: "RefiningSpec",
    });
    renderWithProviders(<ProgressPanel />);
    expect(screen.getByTestId("progress-error")).toBeInTheDocument();
    expect(screen.getByText("LLM timeout")).toBeInTheDocument();
  });

  it("shows progress log when messages exist", () => {
    usePipelineStore.setState({
      status: "running",
      currentStage: "ReadingInput",
      progressMessages: [
        {
          stage: "ReadingInput",
          message: "Reading 2 files...",
          timestamp: new Date(),
        },
      ],
    });
    renderWithProviders(<ProgressPanel />);
    const log = screen.getByTestId("progress-log");
    expect(log).toBeInTheDocument();
    expect(within(log).getByText("Reading 2 files...")).toBeInTheDocument();
  });
});
