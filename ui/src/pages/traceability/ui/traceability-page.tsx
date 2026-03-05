import { useTranslation } from "react-i18next";
import {
  GitCompare,
  Shield,
  CheckCircle2,
  AlertTriangle,
  XCircle,
} from "lucide-react";
import { cn } from "@/shared/lib/utils";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import { ExportButton } from "@/features/export/ui/export-button";
import { useExport } from "@/features/export/hooks/use-export";
import type { TraceabilityMatrix } from "@/shared/types/traceability";

const statusConfig: Record<
  string,
  { badge: string; icon: typeof CheckCircle2 }
> = {
  FullyCovered: {
    badge: "bg-green/15 text-green border border-green/20",
    icon: CheckCircle2,
  },
  PartiallyCovered: {
    badge: "bg-yellow/15 text-yellow border border-yellow/20",
    icon: AlertTriangle,
  },
  NotCovered: {
    badge: "bg-red/15 text-red border border-red/20",
    icon: XCircle,
  },
  VerifiedByAnalysis: {
    badge: "bg-blue/15 text-blue border border-blue/20",
    icon: CheckCircle2,
  },
  VerifiedByInspection: {
    badge: "bg-blue/15 text-blue border border-blue/20",
    icon: CheckCircle2,
  },
  VerifiedByDemo: {
    badge: "bg-teal/15 text-teal border border-teal/20",
    icon: CheckCircle2,
  },
};

export function TraceabilityPage() {
  const { t } = useTranslation();
  const rawTrace = usePipelineStore((s) => s.traceability);
  const { exportTraceability } = useExport();

  const matrix = rawTrace as TraceabilityMatrix | null;

  if (!matrix) {
    return (
      <div
        data-testid="trace-no-data"
        className="flex min-h-[400px] flex-col items-center justify-center gap-4"
      >
        <div className="bg-surface-0/50 flex h-20 w-20 items-center justify-center rounded-2xl">
          <GitCompare className="text-surface-2 h-10 w-10" />
        </div>
        <p className="text-subtext-0 text-sm">{t("trace.noData")}</p>
      </div>
    );
  }

  const summaryCards = [
    {
      value: matrix.summary.covered,
      label: "trace.covered",
      color: "text-green",
      bg: "bg-green/10",
      border: "border-green/20",
      icon: CheckCircle2,
    },
    {
      value: matrix.summary.partially_covered,
      label: "trace.partial",
      color: "text-yellow",
      bg: "bg-yellow/10",
      border: "border-yellow/20",
      icon: AlertTriangle,
    },
    {
      value: matrix.summary.not_covered,
      label: "trace.notCovered",
      color: "text-red",
      bg: "bg-red/10",
      border: "border-red/20",
      icon: XCircle,
    },
    {
      value: `${matrix.summary.forward_coverage_pct.toFixed(0)}%`,
      label: "trace.coverage",
      color: "text-blue",
      bg: "bg-blue/10",
      border: "border-blue/20",
      icon: Shield,
    },
  ];

  return (
    <div data-testid="trace-page" className="min-w-0 space-y-5">
      {/* Page header */}
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div className="flex items-center gap-3">
          <div className="bg-peach/25 flex h-9 w-9 items-center justify-center rounded-lg">
            <GitCompare className="text-peach h-5 w-5" />
          </div>
          <div>
            <h2 className="text-text text-xl font-bold">{t("trace.title")}</h2>
            <p className="text-subtext-0 text-sm">
              {matrix.entries.length} {t("trace.requirements")}
            </p>
          </div>
        </div>
        <ExportButton
          onExport={(format) => exportTraceability(matrix, format)}
        />
      </div>

      {/* Summary cards */}
      <div
        data-testid="trace-summary"
        className="grid grid-cols-2 gap-4 lg:grid-cols-4"
      >
        {summaryCards.map((card) => {
          const Icon = card.icon;
          return (
            <div
              key={card.label}
              className={cn(
                "rounded-xl border p-5 transition-all duration-200 hover:shadow-lg",
                card.bg,
                card.border,
              )}
            >
              <div className="mb-2 flex items-center justify-between">
                <Icon className={cn("h-5 w-5", card.color)} />
              </div>
              <p className={cn("text-3xl font-bold", card.color)}>
                {card.value}
              </p>
              <p className="text-subtext-0 mt-1 text-xs">{t(card.label)}</p>
            </div>
          );
        })}
      </div>

      {/* Matrix table */}
      <div
        data-testid="trace-matrix"
        className="bg-mantle overflow-hidden rounded-xl border"
      >
        <table className="w-full text-sm">
          <thead>
            <tr className="border-surface-1 bg-crust/50 border-b">
              <th className="text-subtext-0 px-4 py-3 text-left text-xs font-medium">
                ID
              </th>
              <th className="text-subtext-0 px-4 py-3 text-left text-xs font-medium">
                {t("trace.statement")}
              </th>
              <th className="text-subtext-0 px-4 py-3 text-left text-xs font-medium">
                {t("trace.priority")}
              </th>
              <th className="text-subtext-0 px-4 py-3 text-left text-xs font-medium">
                {t("trace.verification")}
              </th>
              <th className="text-subtext-0 px-4 py-3 text-left text-xs font-medium">
                {t("trace.scenarios")}
              </th>
              <th className="text-subtext-0 px-4 py-3 text-left text-xs font-medium">
                {t("trace.status")}
              </th>
            </tr>
          </thead>
          <tbody>
            {matrix.entries.map((entry) => {
              const config = statusConfig[entry.status];
              return (
                <tr
                  key={entry.requirement_id}
                  className="border-surface-1 border-b transition-colors last:border-b-0 hover:bg-surface-0/50"
                >
                  <td className="text-blue px-4 py-3 font-mono text-xs font-medium">
                    {entry.requirement_id}
                  </td>
                  <td className="text-text max-w-xs px-4 py-3">
                    {entry.statement}
                  </td>
                  <td className="px-4 py-3">
                    <span
                      className={cn(
                        "rounded-md px-2 py-0.5 text-xs font-medium",
                        entry.priority === "P1"
                          ? "bg-red/15 text-red border border-red/20"
                          : entry.priority === "P2"
                            ? "bg-yellow/15 text-yellow border border-yellow/20"
                            : "bg-green/15 text-green border border-green/20",
                      )}
                    >
                      {entry.priority}
                    </span>
                  </td>
                  <td className="text-subtext-0 px-4 py-3 text-xs">
                    {entry.verification_method}
                  </td>
                  <td className="px-4 py-3">
                    <div className="flex flex-wrap gap-1">
                      {entry.covering_scenarios.map((sc) => (
                        <span
                          key={sc}
                          className="bg-surface-0 text-subtext-1 rounded px-1.5 py-0.5 text-xs"
                        >
                          {sc}
                        </span>
                      ))}
                    </div>
                  </td>
                  <td className="px-4 py-3">
                    <span
                      className={cn(
                        "rounded-md px-2.5 py-1 text-xs font-medium",
                        config?.badge ?? "bg-surface-0 text-text",
                      )}
                    >
                      {entry.status}
                    </span>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>

      {/* Compliance notes */}
      {matrix.compliance_notes.length > 0 && (
        <div
          data-testid="trace-compliance"
          className="bg-mantle rounded-xl border p-5"
        >
          <h3 className="text-text mb-3 font-semibold">
            {t("trace.complianceNotes")}
          </h3>
          <div className="space-y-2">
            {matrix.compliance_notes.map((note, i) => (
              <div
                key={i}
                className={cn(
                  "rounded-lg border p-3 text-sm",
                  note.status === "Compliant"
                    ? "border-green/20 bg-green/5"
                    : note.status === "PartiallyCompliant"
                      ? "border-yellow/20 bg-yellow/5"
                      : "border-red/20 bg-red/5",
                )}
              >
                <div className="flex items-center gap-2">
                  <span className="text-text font-medium">{note.standard}</span>
                  <span
                    className={cn(
                      "rounded px-2 py-0.5 text-xs",
                      note.status === "Compliant"
                        ? "bg-green/15 text-green"
                        : note.status === "PartiallyCompliant"
                          ? "bg-yellow/15 text-yellow"
                          : "bg-red/15 text-red",
                    )}
                  >
                    {note.status}
                  </span>
                </div>
                <p className="text-subtext-0 mt-1 text-xs">{note.details}</p>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
