import { create } from "zustand";
import type { Specification } from "@/shared/types/specification";
import type { TestSuite } from "@/shared/types/test-suite";
import type { TraceabilityMatrix } from "@/shared/types/traceability";

export type PipelineStage =
  | "ReadingInput"
  | "RefiningSpec"
  | "GeneratingTests"
  | "WritingOutput"
  | "Completed";
export type PipelineStatus = "idle" | "running" | "completed" | "error";

interface ProgressMessage {
  stage: string;
  message: string;
  timestamp: Date;
  progressPct?: number;
}

interface PipelineStore {
  status: PipelineStatus;
  currentStage: PipelineStage | null;
  progressMessages: ProgressMessage[];
  error: string | null;

  specification: Specification | null;
  testSuite: TestSuite | null;
  traceability: TraceabilityMatrix | null;

  selectedFiles: string[];
  outputDir: string;
  constitution: string | null;

  addFile: (path: string) => void;
  removeFile: (path: string) => void;
  setFiles: (paths: string[]) => void;
  setOutputDir: (dir: string) => void;
  setConstitution: (text: string | null) => void;
  setPipelineRunning: (stage: PipelineStage) => void;
  addProgress: (msg: ProgressMessage) => void;
  setPipelineCompleted: (
    spec: Specification,
    testSuite: TestSuite,
    traceability: TraceabilityMatrix,
  ) => void;
  setPipelineError: (error: string) => void;
  reset: () => void;
}

export const usePipelineStore = create<PipelineStore>((set) => ({
  status: "idle",
  currentStage: null,
  progressMessages: [],
  error: null,
  specification: null,
  testSuite: null,
  traceability: null,
  selectedFiles: [],
  outputDir: "output",
  constitution: null,

  addFile: (path) =>
    set((s) => ({ selectedFiles: [...s.selectedFiles, path] })),
  removeFile: (path) =>
    set((s) => ({ selectedFiles: s.selectedFiles.filter((f) => f !== path) })),
  setFiles: (paths) => set({ selectedFiles: paths }),
  setOutputDir: (dir) => set({ outputDir: dir }),
  setConstitution: (text) => set({ constitution: text }),
  setPipelineRunning: (stage) =>
    set({ status: "running", currentStage: stage, error: null }),
  addProgress: (msg) =>
    set((s) => ({
      progressMessages: [...s.progressMessages, msg],
      currentStage: msg.stage as PipelineStage,
    })),
  setPipelineCompleted: (spec, testSuite, traceability) =>
    set({
      status: "completed",
      currentStage: "Completed",
      specification: spec,
      testSuite,
      traceability,
    }),
  setPipelineError: (error) => set({ status: "error", error }),
  reset: () =>
    set({
      status: "idle",
      currentStage: null,
      progressMessages: [],
      error: null,
      specification: null,
      testSuite: null,
      traceability: null,
    }),
}));
