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
export function exportAllXlsx(
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

function formatSteps(
  steps: { keyword: string; text: string; doc_string: string | null; data_table: string[][] | null }[],
  indent: string,
): string {
  return steps
    .map((st) => {
      let line = `${indent}${st.keyword} ${st.text}`;
      if (st.doc_string) line += `\n${indent}  """\n${indent}  ${st.doc_string}\n${indent}  """`;
      if (st.data_table)
        line += `\n${st.data_table.map((r) => `${indent}  | ${r.join(" | ")} |`).join("\n")}`;
      return line;
    })
    .join("\n");
}

/** Genere un document Markdown complet avec toutes les donnees du pipeline */
export function exportAllMarkdown(
  spec: Specification,
  suite: TestSuite,
  traceability: TraceabilityMatrix,
): Blob {
  const lines: string[] = [];
  const ln = (s = "") => lines.push(s);

  // --- Header ---
  ln(`# Rapport complet spec-forge : ${spec.title}`);
  ln();
  ln(`| Champ | Valeur |`);
  ln(`|-------|--------|`);
  ln(`| **ID** | ${spec.id} |`);
  ln(`| **Version** | ${spec.version} |`);
  ln(`| **Statut** | ${spec.status} |`);
  ln(`| **Auteur** | ${spec.author ?? "—"} |`);
  ln(`| **Date** | ${spec.created_at} |`);
  ln(`| **Baseline** | ${spec.baseline ?? "—"} |`);
  ln(`| **Outil** | ${spec.tool_version} |`);
  ln(`| **Profil conformite** | ${formatComplianceProfile(spec.compliance_profile) || "—"} |`);
  ln(`| **Stories source** | ${spec.source_stories.join(", ") || "—"} |`);
  ln();

  // --- Validation ---
  if (spec.validation) {
    ln(`## Validation`);
    ln();
    ln(`- **Completude** : ${spec.validation.completeness_score}%`);
    ln(`- **Clarte** : ${spec.validation.clarity_score}%`);
    ln(`- **Testabilite** : ${spec.validation.testability_score}%`);
    ln();
    if (spec.validation.checklist_items.length > 0) {
      ln(`| Categorie | Critere | Statut |`);
      ln(`|-----------|---------|--------|`);
      for (const item of spec.validation.checklist_items) {
        ln(`| ${item.category} | ${item.description} | ${item.passed ? "✅" : "❌"} |`);
      }
      ln();
    }
  }

  // --- Scenarios ---
  ln(`## Scenarios utilisateur (${spec.user_scenarios.length})`);
  ln();
  for (const us of spec.user_scenarios) {
    ln(`### ${us.id} — ${us.title} (Priorite: ${us.priority})`);
    ln();
    ln(us.description);
    ln();
    ln(`**Justification priorite** : ${us.why_priority}`);
    ln();
    ln(`**Test independant** : ${us.independent_test}`);
    ln();
    ln(`**Story source** : ${us.source_story_id}`);
    ln();
    if (us.acceptance_scenarios.length > 0) {
      ln(`**Scenarios d'acceptation** :`);
      ln();
      for (const [i, a] of us.acceptance_scenarios.entries()) {
        ln(`${i + 1}. **Given** ${a.given}, **When** ${a.when}, **Then** ${a.then}`);
      }
      ln();
    }
    ln(`---`);
    ln();
  }

  // --- Exigences ---
  ln(`## Exigences fonctionnelles (${spec.functional_requirements.length})`);
  ln();
  for (const fr of spec.functional_requirements) {
    ln(`### ${fr.id} — ${fr.category} (${fr.priority})`);
    ln();
    ln(`> ${fr.statement}`);
    ln();
    ln(`- **Testable** : ${fr.testable ? "Oui" : "Non"}`);
    ln(`- **Verification** : ${fr.verification_method}`);
    ln(`- **Risque** : ${fr.risk_level ?? "—"}`);
    ln(`- **Qualite** : ${fr.quality_characteristic ?? "—"}`);
    if (fr.rationale) ln(`- **Justification** : ${fr.rationale}`);
    if (fr.source) ln(`- **Source** : ${fr.source}`);
    if (fr.parent_requirement) ln(`- **Exigence parente** : ${fr.parent_requirement}`);
    if (fr.allocated_to.length > 0) ln(`- **Alloue a** : ${fr.allocated_to.join(", ")}`);
    ln();
  }

  // --- Entites ---
  if (spec.key_entities.length > 0) {
    ln(`## Entites cles (${spec.key_entities.length})`);
    ln();
    ln(`| Nom | Description | Attributs | Relations |`);
    ln(`|-----|-------------|-----------|-----------|`);
    for (const e of spec.key_entities) {
      ln(`| ${e.name} | ${e.description.replace(/\|/g, "\\|")} | ${e.attributes.join(", ")} | ${e.relationships.join(", ")} |`);
    }
    ln();
  }

  // --- Cas limites ---
  if (spec.edge_cases.length > 0) {
    ln(`## Cas limites (${spec.edge_cases.length})`);
    ln();
    ln(`| Description | Scenario lie | Severite |`);
    ln(`|-------------|-------------|----------|`);
    for (const ec of spec.edge_cases) {
      ln(`| ${ec.description.replace(/\|/g, "\\|")} | ${ec.related_scenario ?? "—"} | ${ec.severity} |`);
    }
    ln();
  }

  // --- Criteres de succes ---
  if (spec.success_criteria.length > 0) {
    ln(`## Criteres de succes (${spec.success_criteria.length})`);
    ln();
    ln(`| ID | Description | Metrique mesurable |`);
    ln(`|----|-------------|-------------------|`);
    for (const sc of spec.success_criteria) {
      ln(`| ${sc.id} | ${sc.description.replace(/\|/g, "\\|")} | ${sc.measurable_metric.replace(/\|/g, "\\|")} |`);
    }
    ln();
  }

  // --- Clarifications ---
  if (spec.clarifications_needed.length > 0) {
    ln(`## Clarifications (${spec.clarifications_needed.length})`);
    ln();
    for (const c of spec.clarifications_needed) {
      ln(`### ❓ ${c.question}`);
      ln();
      ln(`- **Contexte** : ${c.context}`);
      ln(`- **Impact** : ${c.impact}`);
      ln(`- **Options** : ${c.suggested_options.join(" / ") || "—"}`);
      ln(`- **Resolue** : ${c.resolved ? `Oui — ${c.answer}` : "Non"}`);
      ln();
    }
  }

  // --- Tests Gherkin ---
  ln(`---`);
  ln();
  ln(`## Tests Gherkin (${suite.total_scenarios} scenarios, ${suite.features.length} features)`);
  ln();
  for (const f of suite.features) {
    ln(`### Feature: ${f.name}`);
    ln();
    ln(`> ${f.description}`);
    ln();
    ln(`- **ID** : ${f.id}`);
    ln(`- **Niveau** : ${f.test_level}`);
    ln(`- **Tags** : ${f.tags.join(", ") || "—"}`);
    ln(`- **Exigences couvertes** : ${f.covered_requirements.join(", ") || "—"}`);
    ln(`- **Scenarios source** : ${f.source_scenario_ids.join(", ") || "—"}`);
    ln();
    if (f.background) {
      ln(`**Background:**`);
      ln();
      ln("```gherkin");
      ln(formatSteps(f.background.steps, "  "));
      ln("```");
      ln();
    }
    for (const s of f.scenarios) {
      const icon = s.scenario_type === "HappyPath" ? "✅" : s.scenario_type === "ErrorScenario" ? "❌" : s.scenario_type === "EdgeCase" ? "⚠️" : "🔲";
      ln(`#### ${icon} Scenario: ${s.name}`);
      ln();
      ln(`- **Type** : ${s.scenario_type}`);
      if (s.coverage_technique) ln(`- **Technique** : ${s.coverage_technique}`);
      if (s.tags.length > 0) ln(`- **Tags** : ${s.tags.join(", ")}`);
      if (s.verification_of.length > 0) ln(`- **Verifie** : ${s.verification_of.join(", ")}`);
      ln();
      ln("```gherkin");
      ln(formatSteps(s.steps, "  "));
      ln("```");
      ln();
      if (s.examples) {
        ln(`**Exemples :**`);
        ln();
        ln(`| ${s.examples.headers.join(" | ")} |`);
        ln(`|${s.examples.headers.map(() => "---").join("|")}|`);
        for (const row of s.examples.rows) {
          ln(`| ${row.join(" | ")} |`);
        }
        ln();
      }
      if (s.test_data_suggestions.length > 0) {
        ln(`**Suggestions donnees test** : ${s.test_data_suggestions.join(", ")}`);
        ln();
      }
    }
    ln(`---`);
    ln();
  }

  // --- Couverture ---
  ln(`## Couverture des tests`);
  ln();
  ln(`| Metrique | Valeur |`);
  ln(`|----------|--------|`);
  ln(`| **Couverture** | ${suite.coverage.coverage_percentage}% |`);
  ln(`| **Exigences couvertes** | ${suite.coverage.requirements_covered.length} / ${suite.coverage.requirements_total} |`);
  ln(`| **Exigences** | ${suite.coverage.requirements_covered.join(", ")} |`);
  ln(`| **Happy path** | ${suite.coverage.scenarios_by_type.happy_path} |`);
  ln(`| **Cas limites** | ${suite.coverage.scenarios_by_type.edge_case} |`);
  ln(`| **Scenarios erreur** | ${suite.coverage.scenarios_by_type.error_scenario} |`);
  ln(`| **Conditions limites** | ${suite.coverage.scenarios_by_type.boundary} |`);
  ln();

  // --- Tracabilite ---
  ln(`## Matrice de tracabilite`);
  ln();
  ln(`### Resume`);
  ln();
  ln(`| Total | Couvertes | Partielles | Non couvertes | Verifiees autrement | Couverture |`);
  ln(`|-------|-----------|------------|---------------|---------------------|------------|`);
  ln(`| ${traceability.summary.total_requirements} | ${traceability.summary.covered} | ${traceability.summary.partially_covered} | ${traceability.summary.not_covered} | ${traceability.summary.verified_other} | ${traceability.summary.forward_coverage_pct}% |`);
  ln();
  if (traceability.summary.orphan_tests.length > 0) {
    ln(`**Tests orphelins** : ${traceability.summary.orphan_tests.join(", ")}`);
    ln();
  }

  ln(`### Detail par exigence`);
  ln();
  for (const e of traceability.entries) {
    ln(`#### ${e.requirement_id} — ${e.status}`);
    ln();
    ln(`> ${e.statement}`);
    ln();
    ln(`- **Priorite** : ${e.priority}`);
    ln(`- **Risque** : ${e.risk_level ?? "—"}`);
    ln(`- **Verification** : ${e.verification_method}`);
    ln(`- **Stories source** : ${e.source_stories.join(", ") || "—"}`);
    ln(`- **Features** : ${e.covering_features.join(", ") || "—"}`);
    ln(`- **Scenarios** : ${e.covering_scenarios.join(", ") || "—"}`);
    ln(`- **Techniques** : ${e.coverage_techniques.join(", ") || "—"}`);
    ln();
  }

  // --- Conformite ---
  if (traceability.compliance_notes.length > 0) {
    ln(`## Notes de conformite`);
    ln();
    ln(`| Standard | Section | Statut | Details |`);
    ln(`|----------|---------|--------|---------|`);
    for (const n of traceability.compliance_notes) {
      ln(`| ${n.standard} | ${n.section} | ${n.status} | ${n.details.replace(/\|/g, "\\|")} |`);
    }
    ln();
  }

  ln(`---`);
  ln(`*Genere par spec-forge le ${new Date().toISOString().slice(0, 10)}*`);

  return new Blob([lines.join("\n")], { type: "text/markdown;charset=utf-8" });
}
