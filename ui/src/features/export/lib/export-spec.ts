import * as XLSX from "xlsx";
import type { Specification } from "@/shared/types/specification";

export function exportSpecification(
  spec: Specification,
  format: "csv" | "xlsx",
): Blob {
  const workbook = XLSX.utils.book_new();

  // Sheet 1: Scenarios
  const scenarioRows = spec.user_scenarios.map((us) => ({
    ID: us.id,
    Titre: us.title,
    Priorite: us.priority,
    Description: us.description,
    "Justification priorite": us.why_priority,
    "Test independant": us.independent_test,
  }));
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(scenarioRows),
    "Scenarios",
  );

  // Sheet 2: Exigences
  const reqRows = spec.functional_requirements.map((fr) => ({
    ID: fr.id,
    Enonce: fr.statement,
    Priorite: fr.priority,
    Categorie: fr.category,
    Testable: fr.testable ? "Oui" : "Non",
    Verification: fr.verification_method,
    Risque: fr.risk_level ?? "",
    Justification: fr.rationale ?? "",
    Qualite: fr.quality_characteristic ?? "",
  }));
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(reqRows),
    "Exigences",
  );

  // Sheet 3: Entites
  const entityRows = spec.key_entities.map((e) => ({
    Nom: e.name,
    Description: e.description,
    Attributs: e.attributes.join(", "),
    Relations: e.relationships.join(", "),
  }));
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(entityRows),
    "Entites",
  );

  // Sheet 4: Cas limites
  const edgeRows = spec.edge_cases.map((ec) => ({
    Description: ec.description,
    "Scenario lie": ec.related_scenario ?? "",
    Severite: ec.severity,
  }));
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(edgeRows),
    "Cas limites",
  );

  if (format === "csv") {
    return new Blob([XLSX.utils.sheet_to_csv(workbook.Sheets["Exigences"]!)], {
      type: "text/csv;charset=utf-8",
    });
  }

  const buffer = XLSX.write(workbook, { type: "array", bookType: "xlsx" });
  return new Blob([buffer], {
    type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
  });
}
