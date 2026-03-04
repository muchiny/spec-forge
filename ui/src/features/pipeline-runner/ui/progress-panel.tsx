import { useTranslation } from "react-i18next";
import { CheckCircle2, Loader2, Circle, AlertCircle } from "lucide-react";
import { cn } from "@/shared/lib/utils";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import type { PipelineStage } from "@/shared/stores/use-pipeline-store";

const stages: { key: PipelineStage; labelKey: string }[] = [
  { key: "ReadingInput", labelKey: "pipeline.stages.reading" },
  { key: "RefiningSpec", labelKey: "pipeline.stages.refining" },
  { key: "GeneratingTests", labelKey: "pipeline.stages.generating" },
  { key: "WritingOutput", labelKey: "pipeline.stages.writing" },
];

const stageOrder: PipelineStage[] = ["ReadingInput", "RefiningSpec", "GeneratingTests", "WritingOutput", "Completed"];

function getStageStatus(stage: PipelineStage, current: PipelineStage | null, pipelineStatus: string) {
  if (pipelineStatus === "error") return "error";
  if (!current) return "pending";
  const currentIdx = stageOrder.indexOf(current);
  const stageIdx = stageOrder.indexOf(stage);
  if (stageIdx < currentIdx) return "done";
  if (stageIdx === currentIdx) return "active";
  return "pending";
}

export function ProgressPanel() {
  const { t } = useTranslation();
  const status = usePipelineStore((s) => s.status);
  const currentStage = usePipelineStore((s) => s.currentStage);
  const progressMessages = usePipelineStore((s) => s.progressMessages);
  const error = usePipelineStore((s) => s.error);

  if (status === "idle") return null;

  return (
    <div className="bg-mantle rounded-xl border p-4">
      <h3 className="text-text mb-4 text-sm font-semibold">{t("pipeline.progress")}</h3>

      <div className="mb-4 space-y-3">
        {stages.map(({ key, labelKey }) => {
          const stageStatus = getStageStatus(key, currentStage, status);
          return (
            <div key={key} className="flex items-center gap-3">
              {stageStatus === "done" && <CheckCircle2 className="text-green h-4 w-4" />}
              {stageStatus === "active" && <Loader2 className="text-blue h-4 w-4 animate-spin" />}
              {stageStatus === "pending" && <Circle className="text-surface-2 h-4 w-4" />}
              {stageStatus === "error" && <AlertCircle className="text-red h-4 w-4" />}
              <span
                className={cn(
                  "text-sm",
                  stageStatus === "active" ? "text-text font-medium" : "text-subtext-0",
                  stageStatus === "done" && "text-green",
                )}
              >
                {t(labelKey)}
              </span>
            </div>
          );
        })}
      </div>

      {error && (
        <div className="bg-red/10 border-red/30 rounded-lg border p-3">
          <p className="text-red text-xs">{error}</p>
        </div>
      )}

      {progressMessages.length > 0 && (
        <div className="bg-crust max-h-40 overflow-y-auto rounded-lg p-3">
          {progressMessages.map((msg, i) => (
            <p key={i} className="text-subtext-0 text-xs">
              <span className="text-overlay-1">[{msg.stage}]</span> {msg.message}
            </p>
          ))}
        </div>
      )}
    </div>
  );
}
