import { useTauriEvent } from "@/shared/hooks/use-tauri-event";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import type { PipelineProgressPayload } from "@/shared/types/pipeline";
import type { PipelineStage } from "@/shared/stores/use-pipeline-store";

export function usePipelineProgress() {
  const addProgress = usePipelineStore((s) => s.addProgress);
  const setPipelineRunning = usePipelineStore((s) => s.setPipelineRunning);

  useTauriEvent<PipelineProgressPayload>("pipeline-progress", (payload) => {
    addProgress({
      stage: payload.stage,
      message: payload.message,
      timestamp: new Date(),
      progressPct: payload.progress_pct ?? undefined,
    });
    if (payload.stage !== "Completed") {
      setPipelineRunning(payload.stage as PipelineStage);
    }
  });
}
