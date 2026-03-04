export interface Specification {
  id: string;
  title: string;
  created_at: string;
  status: "Draft" | "NeedsClarification" | "Validated";
  version: string;
  baseline: string | null;
  author: string | null;
  tool_version: string;
  compliance_profile: string | null;
  user_scenarios: UserScenario[];
  functional_requirements: FunctionalRequirement[];
  key_entities: KeyEntity[];
  edge_cases: EdgeCase[];
  success_criteria: SuccessCriterion[];
  clarifications_needed: Clarification[];
  validation: SpecValidation | null;
  source_stories: string[];
}

export interface UserScenario {
  id: string;
  title: string;
  priority: Priority;
  description: string;
  why_priority: string;
  independent_test: string;
  acceptance_scenarios: AcceptanceScenario[];
  source_story_id: string;
}

export interface AcceptanceScenario {
  given: string;
  when: string;
  then: string;
}

export interface FunctionalRequirement {
  id: string;
  statement: string;
  priority: Priority;
  category: string;
  testable: boolean;
  rationale: string | null;
  source: string | null;
  verification_method: string;
  risk_level: string | null;
  parent_requirement: string | null;
  allocated_to: string[];
  quality_characteristic: string | null;
}

export interface KeyEntity {
  name: string;
  description: string;
  attributes: string[];
  relationships: string[];
}

export interface EdgeCase {
  description: string;
  related_scenario: string | null;
  mitigation: string | null;
}

export interface SuccessCriterion {
  criterion: string;
  measurable: boolean;
  metric: string | null;
}

export interface Clarification {
  question: string;
  context: string;
  options: string[];
  resolved: boolean;
  resolution: string | null;
}

export interface SpecValidation {
  completeness_score: number;
  clarity_score: number;
  testability_score: number;
  iso_compliance: boolean;
  issues: ValidationIssue[];
}

export interface ValidationIssue {
  criterion: string;
  severity: string;
  message: string;
  requirement_id: string | null;
}

export type Priority = "P1" | "P2" | "P3";
