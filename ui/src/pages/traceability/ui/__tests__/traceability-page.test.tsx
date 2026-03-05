import { describe, it, expect, beforeEach } from "vitest";
import { screen } from "@testing-library/react";
import { renderWithProviders } from "@/test/test-utils";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import { createMockTraceabilityMatrix } from "@/test/fixtures";
import { TraceabilityPage } from "../traceability-page";

describe("TraceabilityPage", () => {
  beforeEach(() => {
    usePipelineStore.setState({ traceability: null });
  });

  it("shows empty state when no matrix", () => {
    renderWithProviders(<TraceabilityPage />);
    expect(screen.getByTestId("trace-no-data")).toBeInTheDocument();
  });

  it("renders page when data exists", () => {
    usePipelineStore.setState({ traceability: createMockTraceabilityMatrix() });
    renderWithProviders(<TraceabilityPage />);
    expect(screen.getByTestId("trace-page")).toBeInTheDocument();
  });

  it("renders summary cards", () => {
    usePipelineStore.setState({ traceability: createMockTraceabilityMatrix() });
    renderWithProviders(<TraceabilityPage />);
    expect(screen.getByTestId("trace-summary")).toBeInTheDocument();
  });

  it("renders matrix table", () => {
    usePipelineStore.setState({ traceability: createMockTraceabilityMatrix() });
    renderWithProviders(<TraceabilityPage />);
    expect(screen.getByTestId("trace-matrix")).toBeInTheDocument();
  });

  it("displays requirement data in table", () => {
    usePipelineStore.setState({ traceability: createMockTraceabilityMatrix() });
    renderWithProviders(<TraceabilityPage />);
    expect(screen.getByText("REQ-001")).toBeInTheDocument();
    expect(
      screen.getByText("The system MUST allow user login"),
    ).toBeInTheDocument();
  });

  it("renders compliance notes", () => {
    usePipelineStore.setState({ traceability: createMockTraceabilityMatrix() });
    renderWithProviders(<TraceabilityPage />);
    expect(screen.getByTestId("trace-compliance")).toBeInTheDocument();
    expect(screen.getByText("ISO 29148")).toBeInTheDocument();
  });

  it("displays summary values", () => {
    usePipelineStore.setState({ traceability: createMockTraceabilityMatrix() });
    renderWithProviders(<TraceabilityPage />);
    expect(screen.getByText("100%")).toBeInTheDocument();
  });

  it("hides compliance when no notes", () => {
    usePipelineStore.setState({
      traceability: createMockTraceabilityMatrix({ compliance_notes: [] }),
    });
    renderWithProviders(<TraceabilityPage />);
    expect(screen.queryByTestId("trace-compliance")).not.toBeInTheDocument();
  });
});
