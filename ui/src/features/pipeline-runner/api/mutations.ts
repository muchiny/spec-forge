import { tauriInvoke } from "@/shared/api/tauri-client";
import { useTauriMutation } from "@/shared/api/use-tauri-mutation";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import type { PipelineResultResponse } from "@/shared/types/pipeline";
import i18next from "i18next";

interface RunPipelineArgs {
  paths: string[];
  outputDir: string;
  constitution: string | null;
}

export function useRunPipeline() {
  const setPipelineRunning = usePipelineStore.getState().setPipelineRunning;
  const setPipelineCompleted = usePipelineStore.getState().setPipelineCompleted;
  return useTauriMutation<PipelineResultResponse, RunPipelineArgs>({
    mutationFn: (args) => {
      setPipelineRunning("ReadingInput");
      return tauriInvoke<PipelineResultResponse>("run_full_pipeline", {
        paths: args.paths,
        outputDir: args.outputDir,
        constitution: args.constitution,
      });
    },
    successMessage: () => i18next.t("pipeline.completed"),
    errorMessage: (err) => i18next.t("pipeline.failed", { error: err.message }),
    onSuccess: (data) => {
      setPipelineCompleted(data.specification, data.test_suite, data.traceability);
    },
  });
}
