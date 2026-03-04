import { useQuery } from "@tanstack/react-query";
import { tauriInvoke } from "@/shared/api/tauri-client";
import type { LlmStatusResponse } from "@/shared/types/pipeline";

export function useLlmStatus() {
  return useQuery({
    queryKey: ["llm-status"],
    queryFn: () => tauriInvoke<LlmStatusResponse>("check_llm_status"),
    refetchInterval: 30_000,
  });
}
