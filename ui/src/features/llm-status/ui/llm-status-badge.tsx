import { useTranslation } from "react-i18next";
import { Loader2, Wifi, WifiOff } from "lucide-react";
import { cn } from "@/shared/lib/utils";
import { useLlmStatus } from "../api/queries";

export function LlmStatusBadge() {
  const { t } = useTranslation();
  const { data, isLoading } = useLlmStatus();

  if (isLoading) {
    return (
      <div className="text-subtext-0 flex items-center gap-2 text-xs">
        <Loader2 className="h-3 w-3 animate-spin" />
        {t("llm.checking")}
      </div>
    );
  }

  const ready = data?.ready ?? false;

  return (
    <div className={cn("flex items-center gap-2 text-xs", ready ? "text-green" : "text-red")}>
      {ready ? <Wifi className="h-3 w-3" /> : <WifiOff className="h-3 w-3" />}
      <span>
        {ready ? data?.model_name : t("llm.offline")}
      </span>
    </div>
  );
}
