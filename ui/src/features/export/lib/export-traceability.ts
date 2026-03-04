import * as XLSX from "xlsx";
import type { TraceabilityMatrix } from "@/shared/types/traceability";

export function exportTraceability(matrix: TraceabilityMatrix, format: "csv" | "xlsx"): Blob {
  const workbook = XLSX.utils.book_new();

  // Sheet 1: Matrice
  const rows = matrix.entries.map((e) => ({
    "ID Exigence": e.requirement_id,
    Enonce: e.statement,
    Priorite: e.priority,
    Risque: e.risk_level ?? "",
    Verification: e.verification_method,
    Features: e.covering_features.join(", "),
    Scenarios: e.covering_scenarios.join(", "),
    Techniques: e.coverage_techniques.join(", "),
    Statut: e.status,
  }));
  XLSX.utils.book_append_sheet(workbook, XLSX.utils.json_to_sheet(rows), "Matrice");

  // Sheet 2: Resume
  const summary = [
    {
      "Total exigences": matrix.summary.total_requirements,
      Couvertes: matrix.summary.covered,
      "Partiellement couvertes": matrix.summary.partially_covered,
      "Non couvertes": matrix.summary.not_covered,
      "Couverture (%)": matrix.summary.forward_coverage_pct,
      "Tests orphelins": matrix.summary.orphan_tests.join(", "),
    },
  ];
  XLSX.utils.book_append_sheet(workbook, XLSX.utils.json_to_sheet(summary), "Resume");

  if (format === "csv") {
    return new Blob([XLSX.utils.sheet_to_csv(workbook.Sheets["Matrice"]!)], {
      type: "text/csv;charset=utf-8",
    });
  }

  const buffer = XLSX.write(workbook, { type: "array", bookType: "xlsx" });
  return new Blob([buffer], {
    type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
  });
}
