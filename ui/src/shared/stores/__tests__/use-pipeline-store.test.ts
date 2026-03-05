import { describe, it, expect, beforeEach } from "vitest";
import { usePipelineStore } from "../use-pipeline-store";
import {
  createMockSpecification,
  createMockTestSuite,
  createMockTraceabilityMatrix,
} from "@/test/fixtures";

describe("usePipelineStore", () => {
  beforeEach(() => {
    usePipelineStore.setState({
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
    });
  });

  it("has correct initial state", () => {
    const state = usePipelineStore.getState();
    expect(state.status).toBe("idle");
    expect(state.currentStage).toBeNull();
    expect(state.progressMessages).toEqual([]);
    expect(state.error).toBeNull();
    expect(state.selectedFiles).toEqual([]);
    expect(state.outputDir).toBe("output");
    expect(state.constitution).toBeNull();
  });

  it("addFile appends a file to selectedFiles", () => {
    usePipelineStore.getState().addFile("/path/a.md");
    usePipelineStore.getState().addFile("/path/b.md");
    expect(usePipelineStore.getState().selectedFiles).toEqual([
      "/path/a.md",
      "/path/b.md",
    ]);
  });

  it("removeFile filters out the given path", () => {
    usePipelineStore.setState({ selectedFiles: ["/a.md", "/b.md", "/c.md"] });
    usePipelineStore.getState().removeFile("/b.md");
    expect(usePipelineStore.getState().selectedFiles).toEqual([
      "/a.md",
      "/c.md",
    ]);
  });

  it("setFiles replaces the entire array", () => {
    usePipelineStore.setState({ selectedFiles: ["/old.md"] });
    usePipelineStore.getState().setFiles(["/new1.md", "/new2.md"]);
    expect(usePipelineStore.getState().selectedFiles).toEqual([
      "/new1.md",
      "/new2.md",
    ]);
  });

  it("setOutputDir updates outputDir", () => {
    usePipelineStore.getState().setOutputDir("/custom/output");
    expect(usePipelineStore.getState().outputDir).toBe("/custom/output");
  });

  it("setConstitution sets and clears constitution", () => {
    usePipelineStore.getState().setConstitution("Some rules");
    expect(usePipelineStore.getState().constitution).toBe("Some rules");

    usePipelineStore.getState().setConstitution(null);
    expect(usePipelineStore.getState().constitution).toBeNull();
  });

  it("setPipelineRunning sets status to running and clears error", () => {
    usePipelineStore.setState({ error: "previous error" });
    usePipelineStore.getState().setPipelineRunning("ReadingInput");

    const state = usePipelineStore.getState();
    expect(state.status).toBe("running");
    expect(state.currentStage).toBe("ReadingInput");
    expect(state.error).toBeNull();
  });

  it("addProgress appends a message and updates currentStage", () => {
    const msg = {
      stage: "RefiningSpec",
      message: "Refining...",
      timestamp: new Date(),
    };
    usePipelineStore.getState().addProgress(msg);

    const state = usePipelineStore.getState();
    expect(state.progressMessages).toHaveLength(1);
    expect(state.progressMessages[0].stage).toBe("RefiningSpec");
    expect(state.currentStage).toBe("RefiningSpec");
  });

  it("setPipelineCompleted stores results and sets status", () => {
    const spec = createMockSpecification();
    const suite = createMockTestSuite();
    const trace = createMockTraceabilityMatrix();

    usePipelineStore.getState().setPipelineCompleted(spec, suite, trace);

    const state = usePipelineStore.getState();
    expect(state.status).toBe("completed");
    expect(state.currentStage).toBe("Completed");
    expect(state.specification).toBe(spec);
    expect(state.testSuite).toBe(suite);
    expect(state.traceability).toBe(trace);
  });

  it("setPipelineError sets error and status", () => {
    usePipelineStore.getState().setPipelineError("LLM timeout");

    const state = usePipelineStore.getState();
    expect(state.status).toBe("error");
    expect(state.error).toBe("LLM timeout");
  });

  it("reset clears status but preserves selectedFiles and outputDir", () => {
    usePipelineStore.setState({
      status: "completed",
      currentStage: "Completed",
      progressMessages: [
        { stage: "Done", message: "ok", timestamp: new Date() },
      ],
      specification: createMockSpecification(),
      selectedFiles: ["/keep.md"],
      outputDir: "/custom",
    });

    usePipelineStore.getState().reset();

    const state = usePipelineStore.getState();
    expect(state.status).toBe("idle");
    expect(state.currentStage).toBeNull();
    expect(state.progressMessages).toEqual([]);
    expect(state.specification).toBeNull();
    // selectedFiles and outputDir are preserved by reset
    expect(state.selectedFiles).toEqual(["/keep.md"]);
    expect(state.outputDir).toBe("/custom");
  });
});
