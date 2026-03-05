import { useTranslation } from "react-i18next";
import { Play, Loader2, RotateCw, Zap, FolderOpen } from "lucide-react";
import { open } from "@tauri-apps/plugin-dialog";
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
    <div data-testid="pipeline-page" className="min-w-0 space-y-6">
      {/* Page header */}
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div className="flex items-center gap-3">
          <div className="bg-green/25 flex h-9 w-9 items-center justify-center rounded-lg">
            <Zap className="text-green h-5 w-5" />
          </div>
          <div>
            <h2 className="text-text text-xl font-bold">
              {t("pipeline.title")}
            </h2>
            <p className="text-subtext-0 text-sm">{t("pipeline.subtitle")}</p>
          </div>
        </div>
        {status === "completed" && (
          <button
            data-testid="pipeline-reset-button"
            onClick={reset}
            className="text-subtext-0 hover:bg-surface-0 hover:text-text flex items-center gap-2 rounded-lg px-3 py-2 text-xs transition-colors"
          >
            <RotateCw className="h-3.5 w-3.5" />
            {t("pipeline.reset")}
          </button>
        )}
      </div>

      <FilePickerPanel />

      {/* Output dir */}
      <div className="bg-mantle rounded-xl border p-5 transition-all duration-200 hover:border-blue/20">
        <label className="text-text mb-2.5 block text-sm font-semibold">
          {t("pipeline.outputDir")}
        </label>
        <div className="flex gap-2">
          <input
            data-testid="pipeline-output-dir"
            type="text"
            value={outputDir}
            onChange={(e) => setOutputDir(e.target.value)}
            className="bg-surface-0 border-surface-1 text-text focus-ring min-w-0 flex-1 rounded-lg border px-3 py-2.5 text-sm outline-none transition-colors focus:border-blue/50"
          />
          <button
            data-testid="pipeline-output-dir-browse"
            type="button"
            onClick={async () => {
              const result = await open({ directory: true });
              if (result) {
                const path = Array.isArray(result) ? result[0] : result;
                if (path) setOutputDir(path);
              }
            }}
            className="text-subtext-0 hover:bg-surface-0 hover:text-text border-surface-1 flex shrink-0 items-center gap-2 rounded-lg border px-3 py-2.5 text-sm transition-colors"
          >
            <FolderOpen className="h-4 w-4" />
            <span className="hidden sm:inline">{t("pipeline.browse")}</span>
          </button>
        </div>
      </div>

      {/* Constitution */}
      <div className="bg-mantle rounded-xl border p-5 transition-all duration-200 hover:border-blue/20">
        <label className="text-text mb-2.5 block text-sm font-semibold">
          {t("pipeline.constitution")}
        </label>
        <textarea
          data-testid="pipeline-constitution"
          value={constitution ?? ""}
          onChange={(e) => setConstitution(e.target.value || null)}
          placeholder={t("pipeline.constitutionPlaceholder")}
          className="bg-surface-0 border-surface-1 text-text placeholder:text-overlay-0 focus-ring h-28 w-full resize-none rounded-lg border px-3 py-2.5 text-sm outline-none transition-colors focus:border-blue/50"
        />
      </div>

      {/* Run button */}
      <button
        data-testid="pipeline-run-button"
        onClick={handleRun}
        disabled={selectedFiles.length === 0 || status === "running"}
        className="bg-blue text-crust hover:bg-blue/90 disabled:bg-surface-1 disabled:text-overlay-0 flex w-full items-center justify-center gap-2 rounded-xl px-4 py-3.5 font-semibold shadow-lg shadow-blue/20 transition-all duration-200 hover:shadow-xl hover:shadow-blue/25 disabled:cursor-not-allowed disabled:shadow-none"
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
