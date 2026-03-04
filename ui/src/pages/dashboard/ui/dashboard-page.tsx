import { useTranslation } from "react-i18next";
import { Link } from "@tanstack/react-router";
import { Play, FileText, TestTube, GitCompare } from "lucide-react";
import { LlmStatusBadge } from "@/features/llm-status/ui/llm-status-badge";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";

export function DashboardPage() {
  const { t } = useTranslation();
  const spec = usePipelineStore((s) => s.specification);
  const suite = usePipelineStore((s) => s.testSuite);
  const status = usePipelineStore((s) => s.status);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-text text-2xl font-bold">{t("dashboard.title")}</h1>
        <LlmStatusBadge />
      </div>

      {/* Quick actions */}
      <div className="grid grid-cols-2 gap-4 lg:grid-cols-4">
        <Link
          to="/pipeline"
          className="bg-mantle hover:bg-surface-0 flex flex-col items-center gap-3 rounded-xl border p-6 transition-colors"
        >
          <Play className="text-blue h-8 w-8" />
          <span className="text-text text-sm font-medium">{t("dashboard.newPipeline")}</span>
        </Link>
        <Link
          to="/specification"
          className="bg-mantle hover:bg-surface-0 flex flex-col items-center gap-3 rounded-xl border p-6 transition-colors"
        >
          <FileText className="text-green h-8 w-8" />
          <span className="text-text text-sm font-medium">{t("dashboard.viewSpec")}</span>
        </Link>
        <Link
          to="/gherkin"
          className="bg-mantle hover:bg-surface-0 flex flex-col items-center gap-3 rounded-xl border p-6 transition-colors"
        >
          <TestTube className="text-mauve h-8 w-8" />
          <span className="text-text text-sm font-medium">{t("dashboard.viewTests")}</span>
        </Link>
        <Link
          to="/traceability"
          className="bg-mantle hover:bg-surface-0 flex flex-col items-center gap-3 rounded-xl border p-6 transition-colors"
        >
          <GitCompare className="text-peach h-8 w-8" />
          <span className="text-text text-sm font-medium">{t("dashboard.viewTrace")}</span>
        </Link>
      </div>

      {/* Status */}
      <div className="bg-mantle rounded-xl border p-6">
        <h2 className="text-text mb-4 font-semibold">{t("dashboard.status")}</h2>
        {status === "idle" && !spec && (
          <p className="text-subtext-0 text-sm">{t("dashboard.noData")}</p>
        )}
        {status === "completed" && spec != null && suite != null && (
          <div className="grid grid-cols-3 gap-4">
            <div className="bg-surface-0 rounded-lg p-4 text-center">
              <p className="text-blue text-2xl font-bold">
                {spec.user_scenarios?.length ?? 0}
              </p>
              <p className="text-subtext-0 text-xs">{t("dashboard.scenarios")}</p>
            </div>
            <div className="bg-surface-0 rounded-lg p-4 text-center">
              <p className="text-green text-2xl font-bold">
                {spec.functional_requirements?.length ?? 0}
              </p>
              <p className="text-subtext-0 text-xs">{t("dashboard.requirements")}</p>
            </div>
            <div className="bg-surface-0 rounded-lg p-4 text-center">
              <p className="text-mauve text-2xl font-bold">
                {suite.total_scenarios ?? 0}
              </p>
              <p className="text-subtext-0 text-xs">{t("dashboard.tests")}</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
