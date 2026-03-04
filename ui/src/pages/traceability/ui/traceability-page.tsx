import { useTranslation } from "react-i18next";
import { GitCompare } from "lucide-react";
import { cn } from "@/shared/lib/utils";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import { ExportButton } from "@/features/export/ui/export-button";
import { useExport } from "@/features/export/hooks/use-export";
import type { TraceabilityMatrix } from "@/shared/types/traceability";

const statusColors: Record<string, string> = {
  FullyCovered: "bg-green/20 text-green",
  PartiallyCovered: "bg-yellow/20 text-yellow",
  NotCovered: "bg-red/20 text-red",
  VerifiedByAnalysis: "bg-blue/20 text-blue",
  VerifiedByInspection: "bg-blue/20 text-blue",
  VerifiedByDemo: "bg-teal/20 text-teal",
};

export function TraceabilityPage() {
  const { t } = useTranslation();
  const rawTrace = usePipelineStore((s) => s.traceability);
  const { exportTraceability } = useExport();

  const matrix = rawTrace as TraceabilityMatrix | null;

  if (!matrix) {
    return (
      <div className="flex min-h-[400px] flex-col items-center justify-center gap-4">
        <GitCompare className="text-surface-2 h-16 w-16" />
        <p className="text-subtext-0">{t("trace.noData")}</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-text text-2xl font-bold">{t("trace.title")}</h1>
        <ExportButton onExport={(format) => exportTraceability(matrix, format)} />
      </div>

      {/* Summary cards */}
      <div className="grid grid-cols-4 gap-4">
        <div className="bg-mantle rounded-xl border p-4 text-center">
          <p className="text-green text-2xl font-bold">{matrix.summary.covered}</p>
          <p className="text-subtext-0 text-xs">{t("trace.covered")}</p>
        </div>
        <div className="bg-mantle rounded-xl border p-4 text-center">
          <p className="text-yellow text-2xl font-bold">{matrix.summary.partially_covered}</p>
          <p className="text-subtext-0 text-xs">{t("trace.partial")}</p>
        </div>
        <div className="bg-mantle rounded-xl border p-4 text-center">
          <p className="text-red text-2xl font-bold">{matrix.summary.not_covered}</p>
          <p className="text-subtext-0 text-xs">{t("trace.notCovered")}</p>
        </div>
        <div className="bg-mantle rounded-xl border p-4 text-center">
          <p className="text-blue text-2xl font-bold">{matrix.summary.forward_coverage_pct.toFixed(0)}%</p>
          <p className="text-subtext-0 text-xs">{t("trace.coverage")}</p>
        </div>
      </div>

      {/* Matrix table */}
      <div className="bg-mantle overflow-x-auto rounded-xl border">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-surface-1 border-b">
              <th className="text-subtext-0 px-3 py-2 text-left text-xs font-medium">ID</th>
              <th className="text-subtext-0 px-3 py-2 text-left text-xs font-medium">{t("trace.statement")}</th>
              <th className="text-subtext-0 px-3 py-2 text-left text-xs font-medium">{t("trace.priority")}</th>
              <th className="text-subtext-0 px-3 py-2 text-left text-xs font-medium">{t("trace.verification")}</th>
              <th className="text-subtext-0 px-3 py-2 text-left text-xs font-medium">{t("trace.scenarios")}</th>
              <th className="text-subtext-0 px-3 py-2 text-left text-xs font-medium">{t("trace.status")}</th>
            </tr>
          </thead>
          <tbody>
            {matrix.entries.map((entry) => (
              <tr key={entry.requirement_id} className="border-surface-1 border-b">
                <td className="text-blue px-3 py-2 font-mono text-xs">{entry.requirement_id}</td>
                <td className="text-text max-w-xs truncate px-3 py-2">{entry.statement}</td>
                <td className="px-3 py-2 text-xs">{entry.priority}</td>
                <td className="text-subtext-0 px-3 py-2 text-xs">{entry.verification_method}</td>
                <td className="text-subtext-0 px-3 py-2 text-xs">{entry.covering_scenarios.join(", ")}</td>
                <td className="px-3 py-2">
                  <span className={cn("rounded px-2 py-0.5 text-xs", statusColors[entry.status] ?? "bg-surface-0 text-text")}>
                    {entry.status}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
