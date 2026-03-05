import { useTauriEvent } from "@/shared/hooks/use-tauri-event";
import { useOllamaStore } from "@/shared/stores/use-ollama-store";
import { useInitializeLlm } from "../api/mutations";
import type { ModelPullProgressPayload } from "@/shared/types/pipeline";

export function useModelPullProgress() {
  const { setPulling, setError } = useOllamaStore();
  const initializeLlm = useInitializeLlm();

  useTauriEvent<ModelPullProgressPayload>("model-pull-progress", (payload) => {
    if (payload.status === "success") {
      initializeLlm.mutate();
      return;
    }

    if (payload.status.startsWith("error:")) {
      setError(payload.status.replace("error: ", ""));
      return;
    }

    // Calculer la progression en pourcentage
    let progress = 0;
    if (payload.total && payload.completed) {
      progress = Math.round((payload.completed / payload.total) * 100);
    }

    setPulling(payload.status, progress);
  });
}
