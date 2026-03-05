import { useTranslation } from "react-i18next";
import { FileText, CheckCircle2, XCircle, AlertTriangle } from "lucide-react";
import { cn } from "@/shared/lib/utils";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import { usePreferencesStore } from "@/shared/stores/use-preferences-store";
import { ExportButton } from "@/features/export/ui/export-button";
import { useExport } from "@/features/export/hooks/use-export";
import type { Specification } from "@/shared/types/specification";

const tabs = [
  "scenarios",
  "requirements",
  "entities",
  "clarifications",
] as const;

const priorityBadge = (p: string) =>
  p === "P1"
    ? "bg-red/15 text-red border border-red/20"
    : p === "P2"
      ? "bg-yellow/15 text-yellow border border-yellow/20"
      : "bg-green/15 text-green border border-green/20";

export function SpecificationPage() {
  const { t } = useTranslation();
  const rawSpec = usePipelineStore((s) => s.specification);
  const activeTab = usePreferencesStore((s) => s.specViewTab);
  const setTab = usePreferencesStore((s) => s.setSpecViewTab);
  const { exportSpec } = useExport();

  const spec = rawSpec as Specification | null;

  if (!spec) {
    return (
      <div
        data-testid="spec-no-data"
        className="flex min-h-[400px] flex-col items-center justify-center gap-4"
      >
        <div className="bg-surface-0/50 flex h-20 w-20 items-center justify-center rounded-2xl">
          <FileText className="text-surface-2 h-10 w-10" />
        </div>
        <p className="text-subtext-0 text-sm">{t("spec.noData")}</p>
      </div>
    );
  }

  return (
    <div data-testid="spec-page" className="min-w-0 space-y-5">
      {/* Page header */}
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div className="flex items-center gap-3">
          <div className="bg-green/25 flex h-9 w-9 items-center justify-center rounded-lg">
            <FileText className="text-green h-5 w-5" />
          </div>
          <div>
            <h2 className="text-text text-xl font-bold">{spec.title}</h2>
            <p className="text-subtext-0 text-sm">
              v{spec.version} &middot; {spec.status}
            </p>
          </div>
        </div>
        <ExportButton onExport={(format) => exportSpec(spec, format)} />
      </div>

      {/* Tabs */}
      <div
        data-testid="spec-tabs"
        className="bg-crust inline-flex items-center gap-1 rounded-xl p-1"
      >
        {tabs.map((tab, i) => (
          <button
            key={tab}
            data-testid={`spec-tab-${tab}`}
            onClick={() => setTab(i)}
            className={cn(
              "rounded-lg px-4 py-2 text-sm font-medium transition-all duration-200",
              activeTab === i
                ? "bg-blue text-crust shadow-md"
                : "text-subtext-1 hover:bg-surface-0 hover:text-text",
            )}
          >
            {t(`spec.tabs.${tab}`)}
          </button>
        ))}
      </div>

      {/* Tab content */}
      <div
        data-testid="spec-tab-content"
        className="animate-in fade-in duration-150"
      >
        {activeTab === 0 && (
          <div className="space-y-3">
            {spec.user_scenarios.map((us) => (
              <div
                key={us.id}
                className="bg-mantle rounded-xl border p-5 transition-all duration-200 hover:border-blue/30 hover:shadow-lg hover:shadow-blue/5"
              >
                <div className="mb-3 flex flex-wrap items-center gap-2">
                  <span className="bg-blue/15 text-blue rounded-md px-2 py-0.5 font-mono text-xs font-bold">
                    {us.id}
                  </span>
                  <span className="text-text font-medium">{us.title}</span>
                  <span
                    className={cn(
                      "rounded-md px-2 py-0.5 text-xs font-medium",
                      priorityBadge(us.priority),
                    )}
                  >
                    {us.priority}
                  </span>
                </div>
                <p className="text-subtext-0 text-sm leading-relaxed">
                  {us.description}
                </p>
                {us.acceptance_scenarios.length > 0 && (
                  <div className="bg-crust mt-3 rounded-lg p-3">
                    {us.acceptance_scenarios.map((sc, i) => (
                      <div key={i} className="text-xs leading-relaxed">
                        <span className="text-blue font-semibold">Given</span>{" "}
                        <span className="text-text">{sc.given}</span>
                        <br />
                        <span className="text-yellow font-semibold">
                          When
                        </span>{" "}
                        <span className="text-text">{sc.when}</span>
                        <br />
                        <span className="text-green font-semibold">
                          Then
                        </span>{" "}
                        <span className="text-text">{sc.then}</span>
                        {i < us.acceptance_scenarios.length - 1 && (
                          <div className="border-surface-1 my-2 border-t" />
                        )}
                      </div>
                    ))}
                  </div>
                )}
              </div>
            ))}
          </div>
        )}

        {activeTab === 1 && (
          <div className="bg-mantle overflow-hidden rounded-xl border">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-surface-1 bg-crust/50 border-b">
                  <th className="text-subtext-0 px-4 py-3 text-left text-xs font-medium">
                    ID
                  </th>
                  <th className="text-subtext-0 px-4 py-3 text-left text-xs font-medium">
                    {t("spec.statement")}
                  </th>
                  <th className="text-subtext-0 px-4 py-3 text-left text-xs font-medium">
                    {t("spec.priority")}
                  </th>
                  <th className="text-subtext-0 px-4 py-3 text-center text-xs font-medium">
                    {t("spec.testable")}
                  </th>
                  <th className="text-subtext-0 px-4 py-3 text-left text-xs font-medium">
                    {t("spec.risk")}
                  </th>
                </tr>
              </thead>
              <tbody>
                {spec.functional_requirements.map((fr) => (
                  <tr
                    key={fr.id}
                    className="border-surface-1 border-b transition-colors last:border-b-0 hover:bg-surface-0/50"
                  >
                    <td className="text-blue px-4 py-3 font-mono text-xs font-medium">
                      {fr.id}
                    </td>
                    <td className="text-text max-w-md px-4 py-3">
                      {fr.statement}
                    </td>
                    <td className="px-4 py-3">
                      <span
                        className={cn(
                          "rounded-md px-2 py-0.5 text-xs font-medium",
                          priorityBadge(fr.priority),
                        )}
                      >
                        {fr.priority}
                      </span>
                    </td>
                    <td className="px-4 py-3 text-center">
                      {fr.testable ? (
                        <CheckCircle2 className="text-green mx-auto h-4 w-4" />
                      ) : (
                        <XCircle className="text-red mx-auto h-4 w-4" />
                      )}
                    </td>
                    <td className="px-4 py-3">
                      {fr.risk_level && (
                        <span
                          className={cn(
                            "rounded-md px-2 py-0.5 text-xs font-medium",
                            fr.risk_level === "High"
                              ? "bg-red/15 text-red"
                              : fr.risk_level === "Medium"
                                ? "bg-yellow/15 text-yellow"
                                : "bg-green/15 text-green",
                          )}
                        >
                          {fr.risk_level}
                        </span>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}

        {activeTab === 2 && (
          <div className="grid gap-3 sm:grid-cols-2">
            {spec.key_entities.map((entity) => (
              <div
                key={entity.name}
                className="bg-mantle rounded-xl border p-5 transition-all duration-200 hover:border-teal/30 hover:shadow-lg hover:shadow-teal/5"
              >
                <h4 className="text-text mb-1 font-semibold">{entity.name}</h4>
                <p className="text-subtext-0 mb-3 text-sm leading-relaxed">
                  {entity.description}
                </p>
                {entity.attributes.length > 0 && (
                  <div className="flex flex-wrap gap-1.5">
                    {entity.attributes.map((attr) => (
                      <span
                        key={attr}
                        className="bg-blue/10 text-blue border-blue/20 rounded-full border px-2.5 py-0.5 text-xs"
                      >
                        {attr}
                      </span>
                    ))}
                  </div>
                )}
                {entity.relationships.length > 0 && (
                  <div className="mt-2 flex flex-wrap gap-1.5">
                    {entity.relationships.map((rel) => (
                      <span
                        key={rel}
                        className="bg-teal/10 text-teal border-teal/20 rounded-full border px-2.5 py-0.5 text-xs"
                      >
                        {rel}
                      </span>
                    ))}
                  </div>
                )}
              </div>
            ))}
          </div>
        )}

        {activeTab === 3 && (
          <div className="space-y-3">
            {spec.clarifications_needed.map((cl, i) => (
              <div
                key={i}
                className={cn(
                  "rounded-xl border p-5 transition-all duration-200",
                  cl.resolved
                    ? "border-green/30 bg-green/5 hover:border-green/50"
                    : "border-yellow/30 bg-yellow/5 hover:border-yellow/50",
                )}
              >
                <div className="mb-2 flex items-start gap-2">
                  {cl.resolved ? (
                    <CheckCircle2 className="text-green mt-0.5 h-4 w-4 shrink-0" />
                  ) : (
                    <AlertTriangle className="text-yellow mt-0.5 h-4 w-4 shrink-0" />
                  )}
                  <p className="text-text text-sm font-medium">{cl.question}</p>
                </div>
                <p className="text-subtext-0 ml-6 text-xs leading-relaxed">
                  {cl.context}
                </p>
                {cl.impact && (
                  <p className="text-subtext-0 ml-6 mt-1 text-xs italic">
                    {cl.impact}
                  </p>
                )}
                {cl.suggested_options.length > 0 && (
                  <div className="ml-6 mt-2 flex flex-wrap gap-1.5">
                    {cl.suggested_options.map((opt, j) => (
                      <span
                        key={j}
                        className="bg-surface-0 text-subtext-1 rounded-full px-2.5 py-0.5 text-xs"
                      >
                        {opt}
                      </span>
                    ))}
                  </div>
                )}
                {cl.resolved && cl.answer && (
                  <div className="bg-green/10 ml-6 mt-3 rounded-lg p-2.5">
                    <p className="text-green text-xs font-medium">
                      {cl.answer}
                    </p>
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
