import { describe, it, expect, beforeEach } from "vitest";
import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { renderWithProviders } from "@/test/test-utils";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import { usePreferencesStore } from "@/shared/stores/use-preferences-store";
import { createMockSpecification } from "@/test/fixtures";
import { SpecificationPage } from "../specification-page";

describe("SpecificationPage", () => {
  beforeEach(() => {
    usePipelineStore.setState({ specification: null });
    usePreferencesStore.setState({ specViewTab: 0 });
  });

  it("shows empty state when no spec", () => {
    renderWithProviders(<SpecificationPage />);
    expect(screen.getByTestId("spec-no-data")).toBeInTheDocument();
  });

  it("renders spec page when data exists", () => {
    usePipelineStore.setState({ specification: createMockSpecification() });
    renderWithProviders(<SpecificationPage />);
    expect(screen.getByTestId("spec-page")).toBeInTheDocument();
    expect(screen.queryByTestId("spec-no-data")).not.toBeInTheDocument();
  });

  it("renders tabs", () => {
    usePipelineStore.setState({ specification: createMockSpecification() });
    renderWithProviders(<SpecificationPage />);
    expect(screen.getByTestId("spec-tabs")).toBeInTheDocument();
    expect(screen.getByTestId("spec-tab-scenarios")).toBeInTheDocument();
    expect(screen.getByTestId("spec-tab-requirements")).toBeInTheDocument();
    expect(screen.getByTestId("spec-tab-entities")).toBeInTheDocument();
    expect(screen.getByTestId("spec-tab-clarifications")).toBeInTheDocument();
  });

  it("renders tab content", () => {
    usePipelineStore.setState({ specification: createMockSpecification() });
    renderWithProviders(<SpecificationPage />);
    expect(screen.getByTestId("spec-tab-content")).toBeInTheDocument();
  });

  it("switches tabs on click", async () => {
    const user = userEvent.setup();
    usePipelineStore.setState({ specification: createMockSpecification() });
    renderWithProviders(<SpecificationPage />);

    await user.click(screen.getByTestId("spec-tab-requirements"));
    expect(usePreferencesStore.getState().specViewTab).toBe(1);
  });

  it("displays spec title and version", () => {
    usePipelineStore.setState({ specification: createMockSpecification() });
    renderWithProviders(<SpecificationPage />);
    expect(screen.getByText("Test Specification")).toBeInTheDocument();
    expect(screen.getByText(/v1\.0\.0/)).toBeInTheDocument();
  });

  it("displays scenario data on scenarios tab", () => {
    usePipelineStore.setState({ specification: createMockSpecification() });
    renderWithProviders(<SpecificationPage />);
    expect(screen.getByText("US-001")).toBeInTheDocument();
    expect(screen.getByText("User login")).toBeInTheDocument();
  });
});
