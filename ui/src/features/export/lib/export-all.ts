import * as XLSX from "xlsx";
import type { Specification } from "@/shared/types/specification";
import type { TestSuite } from "@/shared/types/test-suite";
import type { TraceabilityMatrix } from "@/shared/types/traceability";

function formatComplianceProfile(
  profile: Specification["compliance_profile"],
): string {
  if (!profile) return "";
  if (typeof profile === "string") return profile;
  const [key, value] = Object.entries(profile)[0] ?? [];
  return key ? `${key} (${value})` : "";
}

/** Construit un workbook XLSX complet avec toutes les donnees du pipeline */
export function buildFullExportWorkbook(
  spec: Specification,
  suite: TestSuite,
  traceability: TraceabilityMatrix,
): XLSX.WorkBook {
  const workbook = XLSX.utils.book_new();

  // 1. Metadonnees
  const metaRows = [
    {
      Champ: "ID",
      Valeur: spec.id,
    },
    { Champ: "Titre", Valeur: spec.title },
    { Champ: "Version", Valeur: spec.version },
    { Champ: "Statut", Valeur: spec.status },
    { Champ: "Auteur", Valeur: spec.author ?? "" },
    { Champ: "Date de creation", Valeur: spec.created_at },
    { Champ: "Baseline", Valeur: spec.baseline ?? "" },
    { Champ: "Version outil", Valeur: spec.tool_version },
    {
      Champ: "Profil conformite",
      Valeur: formatComplianceProfile(spec.compliance_profile),
    },
    {
      Champ: "Stories source",
      Valeur: spec.source_stories.join(", "),
    },
  ];
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(metaRows),
    "Metadonnees",
  );

  // 2. Scenarios utilisateur
  const scenarioRows = spec.user_scenarios.map((us) => ({
    ID: us.id,
    Titre: us.title,
    Priorite: us.priority,
    Description: us.description,
    "Justification priorite": us.why_priority,
    "Test independant": us.independent_test,
    "Story source": us.source_story_id,
    "Scenarios acceptation": us.acceptance_scenarios
      .map((a) => `Etant donne: ${a.given} | Quand: ${a.when} | Alors: ${a.then}`)
      .join("\n"),
  }));
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(scenarioRows),
    "Scenarios",
  );

  // 3. Exigences fonctionnelles
  const reqRows = spec.functional_requirements.map((fr) => ({
    ID: fr.id,
    Enonce: fr.statement,
    Priorite: fr.priority,
    Categorie: fr.category,
    Testable: fr.testable ? "Oui" : "Non",
    "Methode verification": fr.verification_method,
    Risque: fr.risk_level ?? "",
    Justification: fr.rationale ?? "",
    Source: fr.source ?? "",
    "Exigence parente": fr.parent_requirement ?? "",
    "Alloue a": fr.allocated_to.join(", "),
    "Caracteristique qualite": fr.quality_characteristic ?? "",
  }));
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(reqRows),
    "Exigences",
  );

  // 4. Entites
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

  // 5. Cas limites
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

  // 6. Criteres de succes
  const criteriaRows = spec.success_criteria.map((sc) => ({
    ID: sc.id,
    Description: sc.description,
    "Metrique mesurable": sc.measurable_metric,
  }));
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(criteriaRows),
    "Criteres succes",
  );

  // 7. Clarifications
  const clarRows = spec.clarifications_needed.map((c) => ({
    Question: c.question,
    Contexte: c.context,
    "Options suggerees": c.suggested_options.join(", "),
    Impact: c.impact,
    Resolue: c.resolved ? "Oui" : "Non",
    Reponse: c.answer ?? "",
  }));
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(clarRows),
    "Clarifications",
  );

  // 8. Validation
  const validationRows: Record<string, string | number>[] = [];
  if (spec.validation) {
    validationRows.push({
      Critere: "Score completude",
      Valeur: spec.validation.completeness_score,
    });
    validationRows.push({
      Critere: "Score clarte",
      Valeur: spec.validation.clarity_score,
    });
    validationRows.push({
      Critere: "Score testabilite",
      Valeur: spec.validation.testability_score,
    });
    for (const item of spec.validation.checklist_items) {
      validationRows.push({
        Critere: `[${item.category}] ${item.description}`,
        Valeur: item.passed ? "OK" : "NOK",
      });
    }
  }
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(
      validationRows.length > 0
        ? validationRows
        : [{ Critere: "Aucune validation", Valeur: "" }],
    ),
    "Validation",
  );

  // 9. Features & Scenarios de test
  const testRows = suite.features.flatMap((f) =>
    f.scenarios.map((s) => ({
      Feature: f.name,
      "Description feature": f.description,
      "Niveau test": f.test_level,
      "Tags feature": f.tags.join(", "),
      "Exigences couvertes": f.covered_requirements.join(", "),
      "Scenarios source": f.source_scenario_ids.join(", "),
      Scenario: s.name,
      Type: s.scenario_type,
      "Tags scenario": s.tags.join(", "),
      "Technique couverture": s.coverage_technique ?? "",
      "Exigences verifiees": s.verification_of.join(", "),
      Etapes: s.steps
        .map((st) => {
          let line = `${st.keyword} ${st.text}`;
          if (st.doc_string) line += `\n  """\n  ${st.doc_string}\n  """`;
          if (st.data_table)
            line += `\n  | ${st.data_table.map((r) => r.join(" | ")).join(" |\n  | ")} |`;
          return line;
        })
        .join("\n"),
      Exemples: s.examples
        ? `| ${s.examples.headers.join(" | ")} |\n${s.examples.rows.map((r) => `| ${r.join(" | ")} |`).join("\n")}`
        : "",
      "Suggestions donnees test": s.test_data_suggestions.join(", "),
    })),
  );
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(testRows),
    "Tests",
  );

  // 10. Couverture des tests (resume)
  const coverageRows = [
    {
      "Exigences couvertes": suite.coverage.requirements_covered.join(", "),
      "Total exigences": suite.coverage.requirements_total,
      "Couverture (%)": suite.coverage.coverage_percentage,
      "Happy path": suite.coverage.scenarios_by_type.happy_path,
      "Cas limites": suite.coverage.scenarios_by_type.edge_case,
      "Scenarios erreur": suite.coverage.scenarios_by_type.error_scenario,
      "Conditions limites": suite.coverage.scenarios_by_type.boundary,
    },
  ];
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(coverageRows),
    "Couverture tests",
  );

  // 11. Tracabilite
  const traceRows = traceability.entries.map((e) => ({
    "ID Exigence": e.requirement_id,
    Enonce: e.statement,
    Priorite: e.priority,
    Risque: e.risk_level ?? "",
    "Stories source": e.source_stories.join(", "),
    Verification: e.verification_method,
    Features: e.covering_features.join(", "),
    Scenarios: e.covering_scenarios.join(", "),
    Techniques: e.coverage_techniques.join(", "),
    Statut: e.status,
  }));
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(traceRows),
    "Tracabilite",
  );

  // 12. Resume tracabilite
  const summaryRows = [
    {
      "Total exigences": traceability.summary.total_requirements,
      Couvertes: traceability.summary.covered,
      "Partiellement couvertes": traceability.summary.partially_covered,
      "Non couvertes": traceability.summary.not_covered,
      "Verifiees autrement": traceability.summary.verified_other,
      "Couverture (%)": traceability.summary.forward_coverage_pct,
      "Tests orphelins": traceability.summary.orphan_tests.join(", "),
    },
  ];
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(summaryRows),
    "Resume tracabilite",
  );

  // 13. Notes de conformite
  const complianceRows =
    traceability.compliance_notes.length > 0
      ? traceability.compliance_notes.map((n) => ({
          Standard: n.standard,
          Section: n.section,
          Statut: n.status,
          Details: n.details,
        }))
      : [{ Standard: "Aucune note", Section: "", Statut: "", Details: "" }];
  XLSX.utils.book_append_sheet(
    workbook,
    XLSX.utils.json_to_sheet(complianceRows),
    "Conformite",
  );

  return workbook;
}

/** Exporte toutes les donnees du pipeline en un seul fichier XLSX */
export function exportAll(
  spec: Specification,
  suite: TestSuite,
  traceability: TraceabilityMatrix,
): Blob {
  const workbook = buildFullExportWorkbook(spec, suite, traceability);
  const buffer = XLSX.write(workbook, { type: "array", bookType: "xlsx" });
  return new Blob([buffer], {
    type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
  });
}
