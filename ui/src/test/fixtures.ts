import type { Specification } from "@/shared/types/specification";
import type { TestSuite } from "@/shared/types/test-suite";
import type { TraceabilityMatrix } from "@/shared/types/traceability";
import type { Config } from "@/shared/types/config";

export function createMockSpecification(
  overrides?: Partial<Specification>,
): Specification {
  return {
    id: "spec-001",
    title: "Test Specification",
    created_at: "2025-01-01T00:00:00Z",
    status: "Draft",
    version: "1.0.0",
    baseline: null,
    author: null,
    tool_version: "0.1.0",
    compliance_profile: "General",
    user_scenarios: [
      {
        id: "US-001",
        title: "User login",
        priority: "P1",
        description: "User logs in",
        why_priority: "Critical",
        independent_test: "Can be tested independently",
        acceptance_scenarios: [
          {
            given: "a user",
            when: "they log in",
            then: "they see the dashboard",
          },
        ],
        source_story_id: "story-001",
      },
    ],
    functional_requirements: [
      {
        id: "REQ-001",
        statement: "The system MUST allow user login",
        priority: "P1",
        category: "Functional",
        testable: true,
        rationale: "Security",
        source: "US-001",
        verification_method: "Test",
        risk_level: "High",
        parent_requirement: null,
        allocated_to: [],
        quality_characteristic: "Security",
      },
    ],
    key_entities: [
      {
        name: "User",
        description: "System user",
        attributes: ["email", "password"],
        relationships: ["has sessions"],
      },
    ],
    edge_cases: [
      {
        description: "Invalid credentials",
        related_scenario: "US-001",
        severity: "P1",
      },
    ],
    success_criteria: [
      {
        id: "SC-001",
        description: "Login works",
        measurable_metric: "100% success rate",
      },
    ],
    clarifications_needed: [],
    validation: null,
    source_stories: ["story-001"],
    ...overrides,
  };
}

export function createMockTestSuite(overrides?: Partial<TestSuite>): TestSuite {
  return {
    features: [
      {
        id: "feature-001",
        name: "User Authentication",
        description: "Authentication feature",
        tags: ["@auth"],
        background: null,
        scenarios: [
          {
            name: "Successful login",
            tags: ["@happy"],
            scenario_type: "HappyPath",
            steps: [
              {
                keyword: "Given",
                text: "a registered user",
                doc_string: null,
                data_table: null,
              },
              {
                keyword: "When",
                text: "they enter valid credentials",
                doc_string: null,
                data_table: null,
              },
              {
                keyword: "Then",
                text: "they are logged in",
                doc_string: null,
                data_table: null,
              },
            ],
            examples: null,
            test_data_suggestions: ["valid email", "valid password"],
            verification_of: ["REQ-001"],
            coverage_technique: "EquivalencePartitioning",
          },
        ],
        source_scenario_ids: ["US-001"],
        covered_requirements: ["REQ-001"],
        test_level: "System",
      },
    ],
    source_spec_id: "spec-001",
    total_scenarios: 1,
    coverage: {
      requirements_covered: ["REQ-001"],
      requirements_total: 1,
      coverage_percentage: 100,
      scenarios_by_type: {
        happy_path: 1,
        edge_case: 0,
        error_scenario: 0,
        boundary: 0,
      },
    },
    ...overrides,
  };
}

export function createMockTraceabilityMatrix(
  overrides?: Partial<TraceabilityMatrix>,
): TraceabilityMatrix {
  return {
    entries: [
      {
        requirement_id: "REQ-001",
        statement: "The system MUST allow user login",
        priority: "P1",
        risk_level: "High",
        source_stories: ["US-001"],
        verification_method: "Test",
        covering_features: ["feature-001"],
        covering_scenarios: ["Successful login"],
        coverage_techniques: ["EquivalencePartitioning"],
        status: "FullyCovered",
      },
    ],
    summary: {
      total_requirements: 1,
      covered: 1,
      partially_covered: 0,
      not_covered: 0,
      verified_other: 0,
      forward_coverage_pct: 100,
      orphan_tests: [],
    },
    compliance_notes: [
      {
        standard: "ISO 29148",
        section: "5.2.1",
        status: "Compliant",
        details: "All requirements are well-formed",
      },
    ],
    ...overrides,
  };
}

export function createMockConfig(overrides?: Partial<Config>): Config {
  return {
    pipeline: { max_retries: 3, default_language: "fr", token_budget: 4096 },
    llm: {
      enabled: true,
      provider: "ollama",
      model_name: "qwen3:8b",
      api_base_url: "http://localhost:11434",
      api_key: null,
      max_tokens: 4096,
      temperature: 0.1,
      timeout_secs: 120,
      context_size: 8192,
    },
    templates: { directory: "templates" },
    output: {
      spec_format: "markdown",
      traceability: true,
      gherkin_language: "fr",
    },
    validation: {
      min_coverage_percent: 80,
      validate_gherkin_syntax: true,
      max_clarifications: 5,
    },
    compliance: {
      profile: "General",
      safety_level: null,
      include_metadata: true,
      strict_validation: false,
      require_rationale: false,
      require_risk_level: false,
      normative_keywords: true,
      min_p1_coverage: 100,
      min_p2_coverage: 80,
      min_p3_coverage: 60,
    },
    logging: { level: "info", format: "text", colors: true },
    paths: {
      input_dir: "input",
      output_dir: "output",
      specs_dir: "specs",
      features_dir: "features",
    },
    ...overrides,
  };
}

export function createMockLlmStatus(
  overrides?: Partial<{
    ready: boolean;
    model_name: string;
    provider: string;
    url: string;
  }>,
) {
  return {
    ready: true,
    model_name: "qwen3:8b",
    provider: "ollama",
    url: "http://localhost:11434",
    ...overrides,
  };
}
