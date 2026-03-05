import { useTranslation } from "react-i18next";
import { Link } from "@tanstack/react-router";
import {
  Play,
  FileText,
  TestTube,
  GitCompare,
  Sparkles,
  CheckCircle2,
  AlertCircle,
  Clock,
} from "lucide-react";
import { cn } from "@/shared/lib/utils";
import { LlmStatusBadge } from "@/features/llm-status/ui/llm-status-badge";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import { ExportButton } from "@/features/export/ui/export-button";
import { useExport } from "@/features/export/hooks/use-export";

const quickActions = [
  {
    path: "/pipeline",
    icon: Play,
    labelKey: "dashboard.newPipeline",
    color: "blue",
  },
  {
    path: "/specification",
    icon: FileText,
    labelKey: "dashboard.viewSpec",
    color: "green",
  },
  {
    path: "/gherkin",
    icon: TestTube,
    labelKey: "dashboard.viewTests",
    color: "mauve",
  },
  {
    path: "/traceability",
    icon: GitCompare,
    labelKey: "dashboard.viewTrace",
    color: "peach",
  },
] as const;

const colorMap: Record<
  string,
  { bg: string; text: string; hoverBorder: string; hoverShadow: string }
> = {
  blue: {
    bg: "bg-blue/15",
    text: "text-blue",
    hoverBorder: "hover:border-blue/40",
    hoverShadow: "hover:shadow-blue/5",
  },
  green: {
    bg: "bg-green/15",
    text: "text-green",
    hoverBorder: "hover:border-green/40",
    hoverShadow: "hover:shadow-green/5",
  },
  mauve: {
    bg: "bg-mauve/15",
    text: "text-mauve",
    hoverBorder: "hover:border-mauve/40",
    hoverShadow: "hover:shadow-mauve/5",
  },
  peach: {
    bg: "bg-peach/15",
    text: "text-peach",
    hoverBorder: "hover:border-peach/40",
    hoverShadow: "hover:shadow-peach/5",
  },
};

export function DashboardPage() {
  const { t } = useTranslation();
  const spec = usePipelineStore((s) => s.specification);
  const suite = usePipelineStore((s) => s.testSuite);
  const traceability = usePipelineStore((s) => s.traceability);
  const status = usePipelineStore((s) => s.status);
  const { exportAllData } = useExport();

  return (
    <div data-testid="dashboard-page" className="min-w-0 space-y-6">
      {/* Page header */}
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div className="flex items-center gap-3">
          <div className="bg-blue/25 flex h-9 w-9 items-center justify-center rounded-lg">
            <Sparkles className="text-blue h-5 w-5" />
          </div>
          <div>
            <h2 className="text-text text-xl font-bold">
              {t("dashboard.title")}
            </h2>
            <p className="text-subtext-0 text-sm">{t("dashboard.subtitle")}</p>
          </div>
        </div>
        <LlmStatusBadge />
      </div>

      {/* Quick actions */}
      <div
        data-testid="dashboard-quick-actions"
        className="grid grid-cols-2 gap-4 lg:grid-cols-4"
      >
        {quickActions.map((action) => {
          const colors = colorMap[action.color]!;
          const Icon = action.icon;
          return (
            <Link
              key={action.path}
              to={action.path}
              className={cn(
                "bg-mantle group flex flex-col items-center gap-3 rounded-xl border p-6 transition-all duration-200",
                colors.hoverBorder,
                colors.hoverShadow,
                "hover:shadow-lg",
              )}
            >
              <div
                className={cn(
                  "flex h-12 w-12 items-center justify-center rounded-xl transition-colors",
                  colors.bg,
                )}
              >
                <Icon className={cn("h-6 w-6", colors.text)} />
              </div>
              <span className="text-text text-sm font-medium">
                {t(action.labelKey)}
              </span>
            </Link>
          );
        })}
      </div>

      {/* Pipeline status */}
      <div className="bg-mantle rounded-xl border p-6 transition-all duration-200 hover:border-blue/20">
        <div className="mb-4 flex items-center gap-2">
          {status === "completed" ? (
            <CheckCircle2 className="text-green h-5 w-5" />
          ) : status === "error" ? (
            <AlertCircle className="text-red h-5 w-5" />
          ) : (
            <Clock className="text-subtext-0 h-5 w-5" />
          )}
          <h3 className="text-text font-semibold">{t("dashboard.status")}</h3>
        </div>

        {status === "idle" && !spec && (
          <div
            data-testid="dashboard-no-data"
            className="flex flex-col items-center py-6 text-center"
          >
            <FileText className="text-surface-2 mb-3 h-10 w-10" />
            <p className="text-subtext-0 text-sm">{t("dashboard.noData")}</p>
            <Link
              to="/pipeline"
              className="bg-blue/15 text-blue hover:bg-blue/25 mt-4 inline-flex items-center gap-2 rounded-lg px-4 py-2 text-sm font-medium transition-colors"
            >
              <Play className="h-4 w-4" />
              {t("dashboard.newPipeline")}
            </Link>
          </div>
        )}

        {status === "completed" && spec != null && suite != null && (
          <div className="space-y-4">
            <div data-testid="dashboard-stats" className="grid grid-cols-3 gap-4">
              {[
                {
                  value: spec.user_scenarios?.length ?? 0,
                  label: "dashboard.scenarios",
                  color: "text-blue",
                  bg: "bg-blue/10",
                  borderColor: "border-blue/20",
                },
                {
                  value: spec.functional_requirements?.length ?? 0,
                  label: "dashboard.requirements",
                  color: "text-green",
                  bg: "bg-green/10",
                  borderColor: "border-green/20",
                },
                {
                  value: suite.total_scenarios ?? 0,
                  label: "dashboard.tests",
                  color: "text-mauve",
                  bg: "bg-mauve/10",
                  borderColor: "border-mauve/20",
                },
              ].map((stat) => (
                <div
                  key={stat.label}
                  className={cn(
                    "rounded-xl border p-4 text-center transition-all duration-200",
                    stat.bg,
                    stat.borderColor,
                  )}
                >
                  <p className={cn("text-3xl font-bold", stat.color)}>
                    {stat.value}
                  </p>
                  <p className="text-subtext-0 mt-1 text-xs">{t(stat.label)}</p>
                </div>
              ))}
            </div>
            <div className="flex justify-end">
              <ExportButton
                onExport={() => {}}
                onExportAll={
                  traceability
                    ? () => exportAllData(spec, suite, traceability)
                    : undefined
                }
              />
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
