// Types mirroring src/domain/specification.rs — must stay in sync

export type Priority = "P1" | "P2" | "P3";

export type SpecStatus = "Draft" | "NeedsClarification" | "Validated";

export type VerificationMethod = "Inspection" | "Analysis" | "Demonstration" | "Test";

export type RiskLevel = "High" | "Medium" | "Low";

export type QualityCharacteristic =
  | "FunctionalSuitability"
  | "PerformanceEfficiency"
  | "Compatibility"
  | "InteractionCapability"
  | "Reliability"
  | "Security"
  | "Maintainability"
  | "Flexibility"
  | "Safety";

export type RequirementCategory = "Functional" | "NonFunctional" | "Constraint";

// ComplianceProfile is a tagged enum in Rust:
//   "General" | { "Aviation": "A"|"B"|... } | { "Medical": "A"|"B"|"C" } | ...
// We model it as a discriminated union-friendly type.
export type ComplianceProfile =
  | "General"
  | { Aviation: string }
  | { Medical: string }
  | { Automotive: string }
  | { Railway: string }
  | { Safety: string };

export interface Specification {
  id: string;
  title: string;
  created_at: string;
  status: SpecStatus;
  version: string;
  baseline: string | null;
  author: string | null;
  tool_version: string;
  compliance_profile: ComplianceProfile | null;
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
  category: RequirementCategory;
  testable: boolean;
  rationale: string | null;
  source: string | null;
  verification_method: VerificationMethod;
  risk_level: RiskLevel | null;
  parent_requirement: string | null;
  allocated_to: string[];
  quality_characteristic: QualityCharacteristic | null;
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
  severity: Priority;
}

export interface SuccessCriterion {
  id: string;
  description: string;
  measurable_metric: string;
}

export interface Clarification {
  question: string;
  context: string;
  suggested_options: string[];
  impact: string;
  resolved: boolean;
  answer: string | null;
}

export interface SpecValidation {
  completeness_score: number;
  clarity_score: number;
  testability_score: number;
  checklist_items: ChecklistItem[];
}

export interface ChecklistItem {
  description: string;
  passed: boolean;
  category: string;
}
