import { useTranslation } from "react-i18next";
import { FileText } from "lucide-react";
import { cn } from "@/shared/lib/utils";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import { usePreferencesStore } from "@/shared/stores/use-preferences-store";
import { ExportButton } from "@/features/export/ui/export-button";
import { useExport } from "@/features/export/hooks/use-export";
import type { Specification } from "@/shared/types/specification";

const tabs = ["scenarios", "requirements", "entities", "clarifications"] as const;

export function SpecificationPage() {
  const { t } = useTranslation();
  const rawSpec = usePipelineStore((s) => s.specification);
  const activeTab = usePreferencesStore((s) => s.specViewTab);
  const setTab = usePreferencesStore((s) => s.setSpecViewTab);
  const { exportSpec } = useExport();

  const spec = rawSpec as Specification | null;

  if (!spec) {
    return (
      <div className="flex min-h-[400px] flex-col items-center justify-center gap-4">
        <FileText className="text-surface-2 h-16 w-16" />
        <p className="text-subtext-0">{t("spec.noData")}</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-text text-2xl font-bold">{spec.title}</h1>
        <ExportButton onExport={(format) => exportSpec(spec, format)} />
      </div>

      <div className="flex gap-1">
        {tabs.map((tab, i) => (
          <button
            key={tab}
            onClick={() => setTab(i)}
            className={cn(
              "rounded-t-lg px-4 py-2 text-sm transition-colors",
              activeTab === i
                ? "bg-mantle text-blue font-medium"
                : "text-subtext-0 hover:text-text",
            )}
          >
            {t(`spec.tabs.${tab}`)}
          </button>
        ))}
      </div>

      <div className="bg-mantle rounded-xl border p-4">
        {activeTab === 0 && (
          <div className="space-y-4">
            {spec.user_scenarios.map((us) => (
              <div key={us.id} className="bg-surface-0 rounded-lg p-4">
                <div className="mb-2 flex items-center gap-2">
                  <span className="text-blue text-xs font-mono font-bold">{us.id}</span>
                  <span className="text-text font-medium">{us.title}</span>
                  <span className={cn(
                    "rounded px-1.5 py-0.5 text-xs font-medium",
                    us.priority === "P1" ? "bg-red/20 text-red" : us.priority === "P2" ? "bg-yellow/20 text-yellow" : "bg-green/20 text-green",
                  )}>
                    {us.priority}
                  </span>
                </div>
                <p className="text-subtext-0 text-sm">{us.description}</p>
              </div>
            ))}
          </div>
        )}

        {activeTab === 1 && (
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-surface-1 border-b">
                  <th className="text-subtext-0 px-3 py-2 text-left text-xs font-medium">ID</th>
                  <th className="text-subtext-0 px-3 py-2 text-left text-xs font-medium">{t("spec.statement")}</th>
                  <th className="text-subtext-0 px-3 py-2 text-left text-xs font-medium">{t("spec.priority")}</th>
                  <th className="text-subtext-0 px-3 py-2 text-left text-xs font-medium">{t("spec.testable")}</th>
                  <th className="text-subtext-0 px-3 py-2 text-left text-xs font-medium">{t("spec.risk")}</th>
                </tr>
              </thead>
              <tbody>
                {spec.functional_requirements.map((fr) => (
                  <tr key={fr.id} className="border-surface-1 border-b">
                    <td className="text-blue px-3 py-2 font-mono text-xs">{fr.id}</td>
                    <td className="text-text px-3 py-2">{fr.statement}</td>
                    <td className="px-3 py-2">
                      <span className={cn(
                        "rounded px-1.5 py-0.5 text-xs",
                        fr.priority === "P1" ? "bg-red/20 text-red" : fr.priority === "P2" ? "bg-yellow/20 text-yellow" : "bg-green/20 text-green",
                      )}>
                        {fr.priority}
                      </span>
                    </td>
                    <td className="px-3 py-2">{fr.testable ? "✓" : "✗"}</td>
                    <td className="text-subtext-0 px-3 py-2 text-xs">{fr.risk_level ?? "-"}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}

        {activeTab === 2 && (
          <div className="space-y-3">
            {spec.key_entities.map((entity) => (
              <div key={entity.name} className="bg-surface-0 rounded-lg p-4">
                <h4 className="text-text font-medium">{entity.name}</h4>
                <p className="text-subtext-0 text-sm">{entity.description}</p>
                {entity.attributes.length > 0 && (
                  <div className="mt-2 flex flex-wrap gap-1">
                    {entity.attributes.map((attr) => (
                      <span key={attr} className="bg-blue/10 text-blue rounded px-2 py-0.5 text-xs">
                        {attr}
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
              <div key={i} className={cn(
                "rounded-lg border p-4",
                cl.resolved ? "border-green/30 bg-green/5" : "border-yellow/30 bg-yellow/5",
              )}>
                <p className="text-text text-sm font-medium">{cl.question}</p>
                <p className="text-subtext-0 mt-1 text-xs">{cl.context}</p>
                {cl.resolved && cl.answer && (
                  <p className="text-green mt-2 text-xs">{cl.answer}</p>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
