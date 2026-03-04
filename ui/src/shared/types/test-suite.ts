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
  test_level: string;
}

export interface Background {
  steps: Step[];
}

export interface Scenario {
  name: string;
  tags: string[];
  scenario_type: "HappyPath" | "EdgeCase" | "ErrorScenario";
  steps: Step[];
  examples: Examples | null;
  test_data_suggestions: string[];
  verification_of: string[];
  coverage_technique: string | null;
}

export interface Step {
  keyword: string;
  text: string;
  doc_string: string | null;
  data_table: string[][] | null;
}

export interface Examples {
  header: string[];
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
}
