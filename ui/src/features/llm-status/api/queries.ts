import { useQuery } from "@tanstack/react-query";
import { tauriInvoke } from "@/shared/api/tauri-client";
import type {
  LlmStatusResponse,
  OllamaSystemStatus,
} from "@/shared/types/pipeline";

export function useLlmStatus() {
  return useQuery({
    queryKey: ["llm-status"],
    queryFn: () => tauriInvoke<LlmStatusResponse>("check_llm_status"),
    refetchInterval: 30_000,
  });
}

export function useOllamaSystemStatus() {
  return useQuery({
    queryKey: ["ollama-system"],
    queryFn: () => tauriInvoke<OllamaSystemStatus>("check_ollama_system"),
    refetchInterval: 10_000,
  });
}
