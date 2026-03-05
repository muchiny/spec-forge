import * as XLSX from "xlsx";
import type { TestSuite } from "@/shared/types/test-suite";

export function exportTestSuite(
  suite: TestSuite,
  format: "csv" | "xlsx",
): Blob {
  const workbook = XLSX.utils.book_new();

  // Sheet 1: Features
  const featureRows = suite.features.map((f) => ({
    Nom: f.name,
    Description: f.description,
    Tags: f.tags.join(", "),
    "Niveau de test": f.test_level,
    "Exigences couvertes": f.covered_requirements.join(", "),
    "Nombre scenarios": f.scenarios.length,
  }));
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(featureRows),
    "Features",
  );

  // Sheet 2: Scenarios
  const scenarioRows = suite.features.flatMap((f) =>
    f.scenarios.map((s) => ({
      Feature: f.name,
      Scenario: s.name,
      Type: s.scenario_type,
      Tags: s.tags.join(", "),
      "Technique couverture": s.coverage_technique ?? "",
      "Exigences verifiees": s.verification_of.join(", "),
      Etapes: s.steps.map((st) => `${st.keyword} ${st.text}`).join(" | "),
    })),
  );
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(scenarioRows),
    "Scenarios",
  );

  if (format === "csv") {
    return new Blob([XLSX.utils.sheet_to_csv(workbook.Sheets["Scenarios"]!)], {
      type: "text/csv;charset=utf-8",
    });
  }

  const buffer = XLSX.write(workbook, { type: "array", bookType: "xlsx" });
  return new Blob([buffer], {
    type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
  });
}
