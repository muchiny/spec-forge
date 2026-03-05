import { tauriInvoke } from "@/shared/api/tauri-client";
import { useTauriMutation } from "@/shared/api/use-tauri-mutation";

export function usePullModel() {
  return useTauriMutation({
    mutationFn: () => tauriInvoke("pull_model"),
  });
}

export function useCancelPull() {
  return useTauriMutation({
    mutationFn: () => tauriInvoke("cancel_pull"),
  });
}

export function useInitializeLlm() {
  return useTauriMutation({
    mutationFn: () => tauriInvoke("initialize_llm"),
    invalidateKeys: [["llm-status"], ["ollama-system"]],
  });
}
