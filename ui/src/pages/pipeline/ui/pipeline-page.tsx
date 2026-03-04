import { useTranslation } from "react-i18next";
import { Play, Loader2 } from "lucide-react";
import { FilePickerPanel } from "@/features/file-picker/ui/file-picker-panel";
import { ProgressPanel } from "@/features/pipeline-runner/ui/progress-panel";
import { usePipelineProgress } from "@/features/pipeline-runner/hooks/use-pipeline-progress";
import { useRunPipeline } from "@/features/pipeline-runner/api/mutations";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";

export function PipelinePage() {
  const { t } = useTranslation();
  usePipelineProgress();

  const selectedFiles = usePipelineStore((s) => s.selectedFiles);
  const outputDir = usePipelineStore((s) => s.outputDir);
  const constitution = usePipelineStore((s) => s.constitution);
  const status = usePipelineStore((s) => s.status);
  const setOutputDir = usePipelineStore((s) => s.setOutputDir);
  const setConstitution = usePipelineStore((s) => s.setConstitution);
  const reset = usePipelineStore((s) => s.reset);

  const runPipeline = useRunPipeline();

  const handleRun = () => {
    runPipeline.mutate({
      paths: selectedFiles,
      outputDir,
      constitution,
    });
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-text text-2xl font-bold">{t("pipeline.title")}</h1>
        {status === "completed" && (
          <button
            onClick={reset}
            className="text-subtext-0 hover:text-text text-xs transition-colors"
          >
            {t("pipeline.reset")}
          </button>
        )}
      </div>

      <FilePickerPanel />

      {/* Output dir */}
      <div className="bg-mantle rounded-xl border p-4">
        <label className="text-text mb-2 block text-sm font-semibold">
          {t("pipeline.outputDir")}
        </label>
        <input
          type="text"
          value={outputDir}
          onChange={(e) => setOutputDir(e.target.value)}
          className="bg-surface-0 text-text w-full rounded-lg px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-blue"
        />
      </div>

      {/* Constitution */}
      <div className="bg-mantle rounded-xl border p-4">
        <label className="text-text mb-2 block text-sm font-semibold">
          {t("pipeline.constitution")}
        </label>
        <textarea
          value={constitution ?? ""}
          onChange={(e) => setConstitution(e.target.value || null)}
          placeholder={t("pipeline.constitutionPlaceholder")}
          className="bg-surface-0 text-text placeholder:text-overlay-0 h-24 w-full resize-none rounded-lg px-3 py-2 text-sm outline-none focus:ring-2 focus:ring-blue"
        />
      </div>

      {/* Run button */}
      <button
        onClick={handleRun}
        disabled={selectedFiles.length === 0 || status === "running"}
        className="bg-blue text-crust hover:bg-blue/90 disabled:bg-surface-1 disabled:text-overlay-0 flex w-full items-center justify-center gap-2 rounded-xl px-4 py-3 font-semibold transition-colors disabled:cursor-not-allowed"
      >
        {status === "running" ? (
          <>
            <Loader2 className="h-4 w-4 animate-spin" />
            {t("pipeline.running")}
          </>
        ) : (
          <>
            <Play className="h-4 w-4" />
            {t("pipeline.run")}
          </>
        )}
      </button>

      <ProgressPanel />
    </div>
  );
}
