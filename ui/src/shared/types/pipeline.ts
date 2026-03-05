import type { Specification } from "./specification";
import type { TestSuite } from "./test-suite";
import type { TraceabilityMatrix } from "./traceability";

export interface PipelineResultResponse {
  specification: Specification;
  test_suite: TestSuite;
  traceability: TraceabilityMatrix;
  spec_path: string;
  feature_paths: string[];
  traceability_path: string | null;
}

export interface PipelineProgressPayload {
  stage: string;
  message: string;
  progress_pct: number | null;
}

export interface LlmStatusResponse {
  ready: boolean;
  model_name: string;
  provider: string;
  url: string;
}

export interface OllamaSystemStatus {
  ollama_running: boolean;
  model_name: string;
  model_installed: boolean;
  url: string;
}

export interface ModelPullProgressPayload {
  status: string;
  completed: number | null;
  total: number | null;
  digest: string | null;
}
