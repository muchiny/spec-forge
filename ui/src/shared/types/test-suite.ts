// Types mirroring src/domain/test_case.rs — must stay in sync

export type TestLevel = "Unit" | "Integration" | "System" | "Acceptance";

export type CoverageTechnique =
  | "EquivalencePartitioning"
  | "BoundaryValueAnalysis"
  | "DecisionTable"
  | "StateTransition"
  | "ErrorGuessing";

export type ScenarioType =
  | "HappyPath"
  | "EdgeCase"
  | "ErrorScenario"
  | "BoundaryCondition";

export type StepKeyword = "Given" | "When" | "Then" | "And" | "But";

export interface TestSuite {
  features: Feature[];
  source_spec_id: string;
  total_scenarios: number;
  coverage: TestCoverage;
}

export interface Feature {
  id: string;
  name: string;
  description: string;
  tags: string[];
  background: Background | null;
  scenarios: Scenario[];
  source_scenario_ids: string[];
  covered_requirements: string[];
  test_level: TestLevel;
}

export interface Background {
  steps: Step[];
}

export interface Scenario {
  name: string;
  tags: string[];
  scenario_type: ScenarioType;
  steps: Step[];
  examples: Examples | null;
  test_data_suggestions: string[];
  verification_of: string[];
  coverage_technique: CoverageTechnique | null;
}

export interface Step {
  keyword: StepKeyword;
  text: string;
  doc_string: string | null;
  data_table: string[][] | null;
}

export interface Examples {
  headers: string[];
  rows: string[][];
}

export interface TestCoverage {
  requirements_covered: string[];
  requirements_total: number;
  coverage_percentage: number;
  scenarios_by_type: ScenarioCounts;
}

export interface ScenarioCounts {
  happy_path: number;
  edge_case: number;
  error_scenario: number;
  boundary: number;
}
