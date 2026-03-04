export interface Config {
  pipeline: PipelineConfig;
  llm: LlmConfig;
  templates: TemplatesConfig;
  output: OutputConfig;
  validation: ValidationConfig;
  compliance: ComplianceConfig;
  logging: LoggingConfig;
}

export interface PipelineConfig {
  max_retries: number;
  default_language: string;
  token_budget: number;
}

export interface LlmConfig {
  enabled: boolean;
  provider: string;
  model_name: string;
  api_base_url: string;
  api_key: string | null;
  max_tokens: number;
  temperature: number;
  timeout_secs: number;
  context_size: number;
}

export interface TemplatesConfig {
  directory: string;
}

export interface OutputConfig {
  spec_format: string;
  traceability: boolean;
  gherkin_language: string;
}

export interface ValidationConfig {
  min_coverage_percent: number;
  validate_gherkin_syntax: boolean;
  max_clarifications: number;
}

export interface ComplianceConfig {
  profile: string;
  safety_level: string | null;
  include_metadata: boolean;
  strict_validation: boolean;
  require_rationale: boolean;
  require_risk_level: boolean;
}

export interface LoggingConfig {
  level: string;
  format: string;
  colors: boolean;
}
