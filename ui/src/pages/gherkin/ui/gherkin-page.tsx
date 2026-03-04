import { useState } from "react";
import { useTranslation } from "react-i18next";
import { TestTube } from "lucide-react";
import { cn } from "@/shared/lib/utils";
import { usePipelineStore } from "@/shared/stores/use-pipeline-store";
import { ExportButton } from "@/features/export/ui/export-button";
import { useExport } from "@/features/export/hooks/use-export";
import type { TestSuite, Feature, Scenario } from "@/shared/types/test-suite";

const stepColors: Record<string, string> = {
  Given: "text-blue",
  Soit: "text-blue",
  When: "text-yellow",
  Quand: "text-yellow",
  Then: "text-green",
  Alors: "text-green",
  And: "text-subtext-0",
  Et: "text-subtext-0",
  But: "text-red",
  Mais: "text-red",
};

export function GherkinPage() {
  const { t } = useTranslation();
  const rawSuite = usePipelineStore((s) => s.testSuite);
  const { exportTests } = useExport();
  const [selectedFeature, setSelectedFeature] = useState(0);
  const [selectedScenario, setSelectedScenario] = useState(0);

  const suite = rawSuite as TestSuite | null;

  if (!suite || suite.features.length === 0) {
    return (
      <div className="flex min-h-[400px] flex-col items-center justify-center gap-4">
        <TestTube className="text-surface-2 h-16 w-16" />
        <p className="text-subtext-0">{t("gherkin.noData")}</p>
      </div>
    );
  }

  const feature: Feature = suite.features[selectedFeature]!;
  const scenario: Scenario | undefined = feature.scenarios[selectedScenario];

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h1 className="text-text text-2xl font-bold">{t("gherkin.title")}</h1>
        <ExportButton onExport={(format) => exportTests(suite, format)} />
      </div>

      <div className="flex gap-4">
        {/* Feature list */}
        <div className="bg-mantle w-64 shrink-0 rounded-xl border p-3">
          <h3 className="text-subtext-0 mb-2 text-xs font-semibold uppercase">{t("gherkin.features")}</h3>
          {suite.features.map((f, i) => (
            <button
              key={f.id}
              onClick={() => { setSelectedFeature(i); setSelectedScenario(0); }}
              className={cn(
                "mb-1 w-full rounded-lg px-3 py-2 text-left text-xs transition-colors",
                selectedFeature === i ? "bg-surface-0 text-blue font-medium" : "text-subtext-0 hover:bg-surface-0",
              )}
            >
              {f.name}
            </button>
          ))}
        </div>

        {/* Feature detail */}
        <div className="min-w-0 flex-1 space-y-4">
          <div className="bg-mantle rounded-xl border p-4">
            <h2 className="text-text font-semibold">{feature.name}</h2>
            <p className="text-subtext-0 mt-1 text-sm">{feature.description}</p>
            <div className="mt-2 flex flex-wrap gap-1">
              {feature.tags.map((tag) => (
                <span key={tag} className="bg-mauve/10 text-mauve rounded px-2 py-0.5 text-xs">
                  {tag}
                </span>
              ))}
            </div>
          </div>

          {/* Scenario tabs */}
          <div className="flex flex-wrap gap-1">
            {feature.scenarios.map((s, i) => (
              <button
                key={i}
                onClick={() => setSelectedScenario(i)}
                className={cn(
                  "rounded-lg px-3 py-1.5 text-xs transition-colors",
                  selectedScenario === i ? "bg-surface-0 text-text font-medium" : "text-subtext-0 hover:bg-surface-0",
                )}
              >
                {s.name}
              </button>
            ))}
          </div>

          {/* Scenario detail */}
          {scenario && (
            <div className="bg-mantle rounded-xl border p-4">
              <div className="mb-3 flex items-center gap-2">
                <span className={cn(
                  "rounded px-2 py-0.5 text-xs font-medium",
                  scenario.scenario_type === "HappyPath" ? "bg-green/20 text-green" :
                  scenario.scenario_type === "EdgeCase" ? "bg-yellow/20 text-yellow" : "bg-red/20 text-red",
                )}>
                  {scenario.scenario_type}
                </span>
                {scenario.coverage_technique && (
                  <span className="bg-blue/10 text-blue rounded px-2 py-0.5 text-xs">{scenario.coverage_technique}</span>
                )}
              </div>

              <div className="bg-crust space-y-1 rounded-lg p-4 font-mono text-sm">
                {scenario.steps.map((step, i) => (
                  <p key={i}>
                    <span className={cn("font-bold", stepColors[step.keyword] ?? "text-text")}>
                      {step.keyword}
                    </span>{" "}
                    <span className="text-text">{step.text}</span>
                  </p>
                ))}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
