import { useEffect } from "react";
import { useTranslation } from "react-i18next";
import { Loader2, Wifi, WifiOff, AlertTriangle, Download } from "lucide-react";
import { cn } from "@/shared/lib/utils";
import { useOllamaStore } from "@/shared/stores/use-ollama-store";
import { useOllamaSystemStatus } from "../api/queries";
import { usePullModel, useCancelPull } from "../api/mutations";
import { useModelPullProgress } from "../hooks/use-model-pull-progress";

export function LlmStatusBadge() {
  const { t } = useTranslation();
  const { data, isLoading } = useOllamaSystemStatus();
  const ollamaStatus = useOllamaStore((s) => s.status);
  const pullProgress = useOllamaStore((s) => s.pullProgress);
  const modelName = useOllamaStore((s) => s.modelName);
  const setNotRunning = useOllamaStore((s) => s.setNotRunning);
  const setModelMissing = useOllamaStore((s) => s.setModelMissing);
  const setReady = useOllamaStore((s) => s.setReady);

  const pullModel = usePullModel();
  const cancelPull = useCancelPull();
  useModelPullProgress();

  // Synchroniser le store avec la query
  useEffect(() => {
    if (isLoading || !data) return;

    // Ne pas ecraser l'etat "pulling"
    if (ollamaStatus === "pulling") return;

    if (!data.ollama_running) {
      setNotRunning();
    } else if (!data.model_installed) {
      setModelMissing(data.model_name, data.url);
    } else {
      setReady(data.model_name, data.url);
    }
  }, [data, isLoading, ollamaStatus, setNotRunning, setModelMissing, setReady]);

  if (isLoading || ollamaStatus === "checking") {
    return (
      <div
        data-testid="llm-status-loading"
        className="bg-surface-0 text-subtext-0 flex items-center gap-2 rounded-full px-3 py-1.5 text-xs"
      >
        <Loader2 className="h-3 w-3 animate-spin" />
        {t("llm.checking")}
      </div>
    );
  }

  // Ollama non demarre
  if (ollamaStatus === "not_running") {
    return (
      <div
        data-testid="llm-status-badge"
        className="bg-red/15 text-red border-red/20 flex items-center gap-2 rounded-full border px-3 py-1.5 text-xs font-medium"
      >
        <WifiOff className="h-3.5 w-3.5" />
        <span>{t("llm.notRunning")}</span>
      </div>
    );
  }

  // Modele manquant
  if (ollamaStatus === "model_missing") {
    return (
      <div
        data-testid="llm-status-badge"
        className="bg-yellow/15 text-yellow border-yellow/20 flex items-center gap-2 rounded-full border px-3 py-1.5 text-xs font-medium"
      >
        <AlertTriangle className="h-3.5 w-3.5" />
        <span>{t("llm.modelMissing")}</span>
        <button
          data-testid="llm-install-button"
          onClick={() => pullModel.mutate()}
          disabled={pullModel.isPending}
          className="bg-yellow/25 hover:bg-yellow/40 ml-1 rounded px-2 py-0.5 text-xs transition-colors"
        >
          {t("llm.install")}
        </button>
      </div>
    );
  }

  // Pull en cours
  if (ollamaStatus === "pulling") {
    return (
      <div
        data-testid="llm-status-badge"
        className="bg-blue/15 text-blue border-blue/20 flex items-center gap-2 rounded-full border px-3 py-1.5 text-xs font-medium"
      >
        <Download className="h-3.5 w-3.5 animate-bounce" />
        <div className="flex items-center gap-2">
          <div className="bg-blue/20 h-1.5 w-20 overflow-hidden rounded-full">
            <div
              className="bg-blue h-full rounded-full transition-all duration-300"
              style={{ width: `${pullProgress}%` }}
            />
          </div>
          <span>{pullProgress}%</span>
        </div>
        <button
          data-testid="llm-cancel-pull"
          onClick={() => cancelPull.mutate()}
          className="text-subtext-0 hover:text-red ml-1 text-xs transition-colors"
        >
          {t("llm.cancel")}
        </button>
      </div>
    );
  }

  // Erreur
  if (ollamaStatus === "error") {
    return (
      <div
        data-testid="llm-status-badge"
        className="bg-red/15 text-red border-red/20 flex items-center gap-2 rounded-full border px-3 py-1.5 text-xs font-medium"
      >
        <AlertTriangle className="h-3.5 w-3.5" />
        <span>{t("llm.error")}</span>
      </div>
    );
  }

  // Pret
  return (
    <div
      data-testid="llm-status-badge"
      className={cn(
        "flex items-center gap-2 rounded-full px-3 py-1.5 text-xs font-medium",
        "bg-green/15 text-green border border-green/20",
      )}
    >
      <div className="relative">
        <Wifi className="h-3.5 w-3.5" />
        <span className="bg-green absolute -top-0.5 -right-0.5 h-1.5 w-1.5 rounded-full shadow-[0_0_6px] shadow-green/50" />
      </div>
      <span>{data?.model_name ?? modelName}</span>
    </div>
  );
}
