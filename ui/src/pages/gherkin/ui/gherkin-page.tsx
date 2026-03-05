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
  And: "text-subtext-1",
  Et: "text-subtext-1",
  But: "text-red",
  Mais: "text-red",
};

const scenarioTypeBadge = (type: string) => {
  switch (type) {
    case "HappyPath":
      return "bg-green/15 text-green border border-green/20";
    case "EdgeCase":
      return "bg-yellow/15 text-yellow border border-yellow/20";
    case "BoundaryCondition":
      return "bg-peach/15 text-peach border border-peach/20";
    default:
      return "bg-red/15 text-red border border-red/20";
  }
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
      <div
        data-testid="gherkin-no-data"
        className="flex min-h-[400px] flex-col items-center justify-center gap-4"
      >
        <div className="bg-surface-0/50 flex h-20 w-20 items-center justify-center rounded-2xl">
          <TestTube className="text-surface-2 h-10 w-10" />
        </div>
        <p className="text-subtext-0 text-sm">{t("gherkin.noData")}</p>
      </div>
    );
  }

  const feature: Feature = suite.features[selectedFeature]!;
  const scenario: Scenario | undefined = feature.scenarios[selectedScenario];

  return (
    <div data-testid="gherkin-page" className="min-w-0 space-y-5">
      {/* Page header */}
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div className="flex items-center gap-3">
          <div className="bg-mauve/25 flex h-9 w-9 items-center justify-center rounded-lg">
            <TestTube className="text-mauve h-5 w-5" />
          </div>
          <div>
            <h2 className="text-text text-xl font-bold">
              {t("gherkin.title")}
            </h2>
            <p className="text-subtext-0 text-sm">
              {suite.features.length} features &middot; {suite.total_scenarios}{" "}
              scenarios
            </p>
          </div>
        </div>
        <ExportButton onExport={(format) => exportTests(suite, format)} />
      </div>

      <div className="flex gap-4">
        {/* Feature list sidebar */}
        <div
          data-testid="gherkin-feature-list"
          className="bg-mantle w-64 shrink-0 rounded-xl border p-3"
        >
          <h3 className="text-subtext-0 mb-3 px-2 text-xs font-semibold uppercase tracking-wider">
            {t("gherkin.features")}
          </h3>
          <div className="space-y-1">
            {suite.features.map((f, i) => (
              <button
                data-testid={`gherkin-feature-${i}`}
                key={f.id}
                onClick={() => {
                  setSelectedFeature(i);
                  setSelectedScenario(0);
                }}
                className={cn(
                  "w-full rounded-lg px-3 py-2.5 text-left text-xs transition-all duration-200",
                  selectedFeature === i
                    ? "bg-blue text-crust font-medium shadow-md"
                    : "text-subtext-0 hover:bg-surface-0 hover:text-text",
                )}
              >
                {f.name}
              </button>
            ))}
          </div>
        </div>

        {/* Feature detail */}
        <div className="min-w-0 flex-1 space-y-4">
          {/* Feature info card */}
          <div className="bg-mantle rounded-xl border p-5 transition-all duration-200 hover:border-mauve/20">
            <h3 className="text-text text-lg font-semibold">{feature.name}</h3>
            <p className="text-subtext-0 mt-1 text-sm leading-relaxed">
              {feature.description}
            </p>
            {feature.tags.length > 0 && (
              <div className="mt-3 flex flex-wrap gap-1.5">
                {feature.tags.map((tag) => (
                  <span
                    key={tag}
                    className="bg-mauve/10 text-mauve border-mauve/20 rounded-full border px-2.5 py-0.5 text-xs"
                  >
                    {tag}
                  </span>
                ))}
              </div>
            )}
          </div>

          {/* Scenario tabs */}
          <div className="bg-crust inline-flex flex-wrap items-center gap-1 rounded-xl p-1">
            {feature.scenarios.map((s, i) => (
              <button
                key={i}
                onClick={() => setSelectedScenario(i)}
                className={cn(
                  "rounded-lg px-3 py-2 text-xs font-medium transition-all duration-200",
                  selectedScenario === i
                    ? "bg-blue text-crust shadow-md"
                    : "text-subtext-1 hover:bg-surface-0 hover:text-text",
                )}
              >
                {s.name}
              </button>
            ))}
          </div>

          {/* Scenario detail */}
          {scenario && (
            <div className="animate-in fade-in duration-150 bg-mantle rounded-xl border p-5 transition-all duration-200 hover:border-blue/20">
              <div className="mb-4 flex flex-wrap items-center gap-2">
                <span
                  className={cn(
                    "rounded-md px-2.5 py-1 text-xs font-medium",
                    scenarioTypeBadge(scenario.scenario_type),
                  )}
                >
                  {scenario.scenario_type}
                </span>
                {scenario.coverage_technique && (
                  <span className="bg-blue/10 text-blue border-blue/20 rounded-full border px-2.5 py-0.5 text-xs">
                    {scenario.coverage_technique}
                  </span>
                )}
              </div>

              {/* Steps */}
              <div className="bg-crust space-y-1.5 rounded-lg p-4 font-mono text-sm">
                {scenario.steps.map((step, i) => (
                  <p key={i} className="leading-relaxed">
                    <span
                      className={cn(
                        "font-bold",
                        stepColors[step.keyword] ?? "text-text",
                      )}
                    >
                      {step.keyword}
                    </span>{" "}
                    <span className="text-text">{step.text}</span>
                  </p>
                ))}
              </div>

              {/* Examples table */}
              {scenario.examples && (
                <div className="mt-4 overflow-hidden rounded-lg border">
                  <table className="w-full text-xs">
                    <thead>
                      <tr className="bg-crust/50">
                        {scenario.examples.headers.map((h) => (
                          <th
                            key={h}
                            className="text-subtext-0 px-3 py-2 text-left font-medium"
                          >
                            {h}
                          </th>
                        ))}
                      </tr>
                    </thead>
                    <tbody>
                      {scenario.examples.rows.map((row, i) => (
                        <tr key={i} className="border-surface-1 border-t">
                          {row.map((cell, j) => (
                            <td key={j} className="text-text px-3 py-1.5">
                              {cell}
                            </td>
                          ))}
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              )}

              {/* Test data suggestions */}
              {scenario.test_data_suggestions.length > 0 && (
                <div className="mt-3 flex flex-wrap gap-1.5">
                  {scenario.test_data_suggestions.map((s, i) => (
                    <span
                      key={i}
                      className="bg-surface-0 text-subtext-1 rounded-full px-2.5 py-0.5 text-xs"
                    >
                      {s}
                    </span>
                  ))}
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
