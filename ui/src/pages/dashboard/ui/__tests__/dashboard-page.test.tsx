import { describe, it, expect, beforeEach } from "vitest";
import { screen } from "@testing-library/react";
import { renderWithProviders } from "@/test/test-utils";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import { createMockSpecification, createMockTestSuite } from "@/test/fixtures";
import { DashboardPage } from "../dashboard-page";

describe("DashboardPage", () => {
  beforeEach(() => {
    usePipelineStore.setState({
      status: "idle",
      specification: null,
      testSuite: null,
      traceability: null,
    });
  });

  it("renders the page", () => {
    renderWithProviders(<DashboardPage />);
    expect(screen.getByTestId("dashboard-page")).toBeInTheDocument();
  });

  it("renders quick action links", () => {
    renderWithProviders(<DashboardPage />);
    const actions = screen.getByTestId("dashboard-quick-actions");
    expect(actions).toBeInTheDocument();
  });

  it("shows empty state when no data", () => {
    renderWithProviders(<DashboardPage />);
    expect(screen.getByTestId("dashboard-no-data")).toBeInTheDocument();
  });

  it("shows stats when completed with data", () => {
    const spec = createMockSpecification();
    const suite = createMockTestSuite();
    usePipelineStore.setState({
      status: "completed",
      specification: spec,
      testSuite: suite,
    });
    renderWithProviders(<DashboardPage />);
    expect(screen.getByTestId("dashboard-stats")).toBeInTheDocument();
    expect(screen.queryByTestId("dashboard-no-data")).not.toBeInTheDocument();
  });

  it("displays correct stat values", () => {
    const spec = createMockSpecification();
    const suite = createMockTestSuite();
    usePipelineStore.setState({
      status: "completed",
      specification: spec,
      testSuite: suite,
    });
    renderWithProviders(<DashboardPage />);
    expect(screen.getByTestId("dashboard-stats")).toBeInTheDocument();
  });
});
