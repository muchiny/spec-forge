export interface TraceabilityMatrix {
  entries: TraceabilityEntry[];
  summary: TraceabilitySummary;
  compliance_notes: ComplianceNote[];
}

export interface TraceabilityEntry {
  requirement_id: string;
  statement: string;
  priority: string;
  risk_level: string | null;
  source_stories: string[];
  verification_method: string;
  covering_features: string[];
  covering_scenarios: string[];
  coverage_techniques: string[];
  status: string;
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
  status: string;
  details: string;
}
