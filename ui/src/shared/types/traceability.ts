// Types mirroring src/domain/traceability.rs — must stay in sync

import type { Priority, RiskLevel, VerificationMethod } from "./specification";
import type { CoverageTechnique } from "./test-suite";

export type TraceabilityStatus =
  | "FullyCovered"
  | "PartiallyCovered"
  | "NotCovered"
  | "VerifiedByAnalysis"
  | "VerifiedByInspection"
  | "VerifiedByDemo";

export type ComplianceStatus =
  | "Compliant"
  | "PartiallyCompliant"
  | "NonCompliant";

export interface TraceabilityMatrix {
  entries: TraceabilityEntry[];
  summary: TraceabilitySummary;
  compliance_notes: ComplianceNote[];
}

export interface TraceabilityEntry {
  requirement_id: string;
  statement: string;
  priority: Priority;
  risk_level: RiskLevel | null;
  source_stories: string[];
  verification_method: VerificationMethod;
  covering_features: string[];
  covering_scenarios: string[];
  coverage_techniques: CoverageTechnique[];
  status: TraceabilityStatus;
}

export interface TraceabilitySummary {
  total_requirements: number;
  covered: number;
  partially_covered: number;
  not_covered: number;
  verified_other: number;
  forward_coverage_pct: number;
  orphan_tests: string[];
}

export interface ComplianceNote {
  standard: string;
  section: string;
  status: ComplianceStatus;
  details: string;
}
