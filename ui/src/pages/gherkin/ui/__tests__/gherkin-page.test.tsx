import { describe, it, expect, beforeEach } from "vitest";
import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { renderWithProviders } from "@/test/test-utils";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import { createMockTestSuite } from "@/test/fixtures";
import { GherkinPage } from "../gherkin-page";

describe("GherkinPage", () => {
  beforeEach(() => {
    usePipelineStore.setState({ testSuite: null });
  });

  it("shows empty state when no suite", () => {
    renderWithProviders(<GherkinPage />);
    expect(screen.getByTestId("gherkin-no-data")).toBeInTheDocument();
  });

  it("renders page when data exists", () => {
    usePipelineStore.setState({ testSuite: createMockTestSuite() });
    renderWithProviders(<GherkinPage />);
    expect(screen.getByTestId("gherkin-page")).toBeInTheDocument();
  });

  it("renders feature list sidebar", () => {
    usePipelineStore.setState({ testSuite: createMockTestSuite() });
    renderWithProviders(<GherkinPage />);
    expect(screen.getByTestId("gherkin-feature-list")).toBeInTheDocument();
    expect(screen.getByTestId("gherkin-feature-0")).toBeInTheDocument();
  });

  it("displays feature name in sidebar and detail", () => {
    usePipelineStore.setState({ testSuite: createMockTestSuite() });
    renderWithProviders(<GherkinPage />);
    const elements = screen.getAllByText("User Authentication");
    expect(elements.length).toBeGreaterThanOrEqual(1);
  });

  it("displays scenario steps", () => {
    usePipelineStore.setState({ testSuite: createMockTestSuite() });
    renderWithProviders(<GherkinPage />);
    expect(screen.getByText("a registered user")).toBeInTheDocument();
    expect(
      screen.getByText("they enter valid credentials"),
    ).toBeInTheDocument();
    expect(screen.getByText("they are logged in")).toBeInTheDocument();
  });

  it("displays feature count in header", () => {
    usePipelineStore.setState({ testSuite: createMockTestSuite() });
    renderWithProviders(<GherkinPage />);
    expect(screen.getByText(/1 features/)).toBeInTheDocument();
    expect(screen.getByText(/1 scenarios/)).toBeInTheDocument();
  });

  it("can select a different feature", async () => {
    const suite = createMockTestSuite({
      features: [
        ...createMockTestSuite().features,
        {
          id: "feature-002",
          name: "User Registration",
          description: "Registration feature",
          tags: [],
          background: null,
          scenarios: [
            {
              name: "Successful registration",
              tags: [],
              scenario_type: "HappyPath",
              steps: [
                {
                  keyword: "Given",
                  text: "a new user",
                  doc_string: null,
                  data_table: null,
                },
                {
                  keyword: "When",
                  text: "they register",
                  doc_string: null,
                  data_table: null,
                },
                {
                  keyword: "Then",
                  text: "account is created",
                  doc_string: null,
                  data_table: null,
                },
              ],
              examples: null,
              test_data_suggestions: [],
              verification_of: [],
              coverage_technique: null,
            },
          ],
          source_scenario_ids: [],
          covered_requirements: [],
          test_level: "System",
        },
      ],
      total_scenarios: 2,
    });
    usePipelineStore.setState({ testSuite: suite });
    const user = userEvent.setup();
    renderWithProviders(<GherkinPage />);

    await user.click(screen.getByTestId("gherkin-feature-1"));
    // After clicking feature 1, the detail panel should show "a new user" step
    expect(screen.getByText("a new user")).toBeInTheDocument();
  });
});
