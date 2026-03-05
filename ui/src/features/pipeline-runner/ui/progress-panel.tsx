import { useEffect, useRef } from "react";
import { useTranslation } from "react-i18next";
import {
  CheckCircle2,
  Loader2,
  Circle,
  AlertCircle,
  BookOpen,
  Sparkles,
  TestTube,
  FileOutput,
} from "lucide-react";
import { cn } from "@/shared/lib/utils";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import type { PipelineStage } from "@/shared/stores/use-pipeline-store";

interface StageConfig {
  key: PipelineStage;
  labelKey: string;
  icon: React.ComponentType<{ className?: string }>;
  color: string;
  bgColor: string;
  glowColor: string;
}

const stages: StageConfig[] = [
  {
    key: "ReadingInput",
    labelKey: "pipeline.stages.reading",
    icon: BookOpen,
    color: "text-blue",
    bgColor: "bg-blue",
    glowColor: "shadow-blue/40",
  },
  {
    key: "RefiningSpec",
    labelKey: "pipeline.stages.refining",
    icon: Sparkles,
    color: "text-mauve",
    bgColor: "bg-mauve",
    glowColor: "shadow-mauve/40",
  },
  {
    key: "GeneratingTests",
    labelKey: "pipeline.stages.generating",
    icon: TestTube,
    color: "text-teal",
    bgColor: "bg-teal",
    glowColor: "shadow-teal/40",
  },
  {
    key: "WritingOutput",
    labelKey: "pipeline.stages.writing",
    icon: FileOutput,
    color: "text-peach",
    bgColor: "bg-peach",
    glowColor: "shadow-peach/40",
  },
];

const stageOrder: PipelineStage[] = [
  "ReadingInput",
  "RefiningSpec",
  "GeneratingTests",
  "WritingOutput",
  "Completed",
];

type StageStatus = "done" | "active" | "pending" | "error";

function getStageStatus(
  stage: PipelineStage,
  current: PipelineStage | null,
  pipelineStatus: string,
): StageStatus {
  if (pipelineStatus === "error") {
    if (!current) return "error";
    const currentIdx = stageOrder.indexOf(current);
    const stageIdx = stageOrder.indexOf(stage);
    if (stageIdx <= currentIdx) return "error";
    return "pending";
  }
  if (!current) return "pending";
  const currentIdx = stageOrder.indexOf(current);
  const stageIdx = stageOrder.indexOf(stage);
  if (stageIdx < currentIdx) return "done";
  if (stageIdx === currentIdx) return "active";
  return "pending";
}

function getOverallProgress(
  current: PipelineStage | null,
  pipelineStatus: string,
): number {
  if (pipelineStatus === "completed") return 100;
  if (!current) return 0;
  const idx = stageOrder.indexOf(current);
  if (idx < 0) return 0;
  // Each of the 4 stages = 25%, active stage counts as half done
  return Math.min(idx * 25 + 12, 100);
}

function formatTime(date: Date): string {
  return date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", second: "2-digit" });
}

function ProgressLog({ messages, label }: { messages: { stage: string; message: string; timestamp: Date; progressPct?: number }[]; label: string }) {
  const logRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (logRef.current) {
      logRef.current.scrollTop = logRef.current.scrollHeight;
    }
  }, [messages.length]);

  return (
    <details className="group">
      <summary className="text-subtext-0 hover:text-text flex cursor-pointer items-center gap-2 text-xs transition-colors">
        <span className="group-open:rotate-90 transition-transform duration-200">
          &#9656;
        </span>
        {label} ({messages.length})
      </summary>
      <div
        ref={logRef}
        data-testid="progress-log"
        className="bg-crust font-mono mt-2 max-h-52 overflow-y-auto rounded-lg p-3 space-y-0.5"
      >
        {messages.map((msg, i) => (
          <p key={i} className="text-xs leading-relaxed">
            <span className="text-overlay-1">{formatTime(msg.timestamp)}</span>{" "}
            <span className="text-blue font-medium">[{msg.stage}]</span>{" "}
            <span className="text-subtext-0">{msg.message}</span>
            {msg.progressPct !== undefined && (
              <span className="text-overlay-0 ml-1">({Math.round(msg.progressPct)}%)</span>
            )}
          </p>
        ))}
      </div>
    </details>
  );
}

export function ProgressPanel() {
  const { t } = useTranslation();
  const status = usePipelineStore((s) => s.status);
  const currentStage = usePipelineStore((s) => s.currentStage);
  const progressMessages = usePipelineStore((s) => s.progressMessages);
  const error = usePipelineStore((s) => s.error);

  if (status === "idle") return null;

  const overallProgress = getOverallProgress(currentStage, status);
  const isCompleted = status === "completed";

  return (
    <div
      data-testid="progress-panel"
      className={cn(
        "animate-in fade-in duration-200 overflow-hidden rounded-xl border transition-all",
        isCompleted
          ? "border-green/30 bg-mantle"
          : status === "error"
            ? "border-red/30 bg-mantle"
            : "border-blue/20 bg-mantle",
      )}
    >
      {/* Overall progress bar */}
      <div className="bg-crust h-1.5 w-full">
        <div
          className={cn(
            "h-full rounded-r-full transition-all duration-700 ease-out",
            isCompleted
              ? "bg-green"
              : status === "error"
                ? "bg-red"
                : "bg-gradient-to-r from-blue via-mauve to-teal animate-pulse",
          )}
          style={{ width: `${overallProgress}%` }}
        />
      </div>

      <div className="p-5">
        <div className="mb-5 flex items-center justify-between">
          <h3 className="text-text text-sm font-semibold">
            {t("pipeline.progress")}
          </h3>
          {isCompleted && (
            <span className="bg-green/15 text-green animate-in fade-in zoom-in-95 duration-200 rounded-full px-3 py-1 text-xs font-medium">
              {t("pipeline.completed")}
            </span>
          )}
          {status === "running" && (
            <span className="text-blue text-base font-bold tabular-nums">
              {overallProgress}%
            </span>
          )}
        </div>

        {/* Pipeline stages - vertical stepper */}
        <div className="relative mb-5">
          <div className="relative space-y-1">
            {stages.map((stage) => {
              const stageStatus = getStageStatus(
                stage.key,
                currentStage,
                status,
              );
              const StageIcon = stage.icon;

              return (
                <div
                  key={stage.key}
                  data-testid={`progress-stage-${stage.key}`}
                  className={cn(
                    "flex items-center gap-4 rounded-lg px-2 py-2.5 transition-all duration-300",
                    stageStatus === "active" && "bg-surface-0/50",
                  )}
                >
                  {/* Stage indicator circle */}
                  <div
                    className={cn(
                      "relative z-10 flex h-10 w-10 shrink-0 items-center justify-center rounded-full transition-all duration-300",
                      stageStatus === "done" && "bg-green/20",
                      stageStatus === "active" &&
                        `${stage.bgColor}/15 shadow-lg ${stage.glowColor}`,
                      stageStatus === "pending" && "bg-surface-0",
                      stageStatus === "error" && "bg-red/20",
                    )}
                  >
                    {stageStatus === "done" && (
                      <CheckCircle2 className="text-green h-5 w-5 animate-in zoom-in-95 duration-200" />
                    )}
                    {stageStatus === "active" && (
                      <div className="relative">
                        <StageIcon className={cn("h-5 w-5", stage.color)} />
                        <div
                          className={cn(
                            "absolute -inset-1 animate-ping rounded-full opacity-20",
                            stage.bgColor,
                          )}
                        />
                      </div>
                    )}
                    {stageStatus === "pending" && (
                      <Circle className="text-surface-2 h-4 w-4" />
                    )}
                    {stageStatus === "error" && (
                      <AlertCircle className="text-red h-5 w-5" />
                    )}
                  </div>

                  {/* Stage info */}
                  <div className="min-w-0 flex-1">
                    <span
                      className={cn(
                        "block text-sm font-medium transition-colors duration-200",
                        stageStatus === "active" && stage.color,
                        stageStatus === "done" && "text-green",
                        stageStatus === "pending" && "text-subtext-0",
                        stageStatus === "error" && "text-red",
                      )}
                    >
                      {t(stage.labelKey)}
                    </span>
                    {stageStatus === "active" && (
                      <div className="mt-0.5 flex items-center gap-1.5">
                        <Loader2
                          className={cn("h-3 w-3 animate-spin", stage.color)}
                        />
                        <span className="text-subtext-0 text-xs">
                          {progressMessages.length > 0
                            ? progressMessages[progressMessages.length - 1]
                                ?.message
                            : t("pipeline.processing")}
                        </span>
                      </div>
                    )}
                    {stageStatus === "done" && (
                      <span className="text-subtext-0 text-xs">
                        {t("pipeline.stageComplete")}
                      </span>
                    )}
                  </div>

                  {/* Done check animation */}
                  {stageStatus === "done" && (
                    <div className="text-green/60 text-xs font-medium">
                      &#10003;
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>

        {/* Error display */}
        {error && (
          <div
            data-testid="progress-error"
            className="bg-red/10 border-red/30 animate-in fade-in duration-200 mb-4 rounded-lg border p-4"
          >
            <div className="flex items-start gap-2">
              <AlertCircle className="text-red mt-0.5 h-4 w-4 shrink-0" />
              <p className="text-red text-xs leading-relaxed">{error}</p>
            </div>
          </div>
        )}

        {/* Progress log */}
        {progressMessages.length > 0 && (
          <ProgressLog messages={progressMessages} label={t("pipeline.showLog")} />
        )}
      </div>
    </div>
  );
}
