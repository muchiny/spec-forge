//! Regles de validation du domaine
//!
//! Fonctions de validation pour les specifications et les tests generes.
//! Conformite : ISO/IEC/IEEE 29148:2018, ISO/IEC 25010:2023, ISO/IEC 25023:2016

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use super::specification::{
    ChecklistItem, FunctionalRequirement, RequirementCategory, RiskLevel, SpecValidation,
    Specification, VerificationMethod,
};
use super::test_case::TestSuite;
use super::user_story::Priority;

// ---------------------------------------------------------------------------
// ISO 29148 — Well-formedness check (9 criteres)
// ---------------------------------------------------------------------------

/// Avertissement de conformite sur une exigence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellFormednessWarning {
    pub requirement_id: String,
    pub criterion: WellFormednessCriterion,
    pub message: String,
}

/// Les 9 criteres de bien-formation d'une exigence (ISO 29148 section 5.2.5)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WellFormednessCriterion {
    /// L'exigence est necessaire (pas de doublon)
    Necessary,
    /// L'exigence n'est pas ambigue
    Unambiguous,
    /// L'exigence est complete (tous champs remplis)
    Complete,
    /// L'exigence est singuliere (une seule exigence par statement)
    Singular,
    /// L'exigence est realisable
    Feasible,
    /// L'exigence est verifiable
    Verifiable,
    /// L'exigence est correcte (syntaxe normative)
    Correct,
    /// L'exigence est conforme au format
    Conforming,
    /// L'exigence est tracable (a une source)
    Traceable,
}

/// Mots ambigus (ISO 29148 — interdit dans les exigences)
pub const AMBIGUOUS_WORDS: &[&str] = &[
    "environ",
    "quelques",
    "peut-etre",
    "certains",
    "parfois",
    "souvent",
    "approximativement",
    "approximately",
    "some",
    "maybe",
    "sometimes",
    "usually",
    "often",
    "few",
    "several",
    "many",
    "etc",
    "adequate",
    "as appropriate",
];

/// Verifie si un mot ambigu est present en tant que mot entier (word boundary)
/// Evite les faux positifs: "some" ne matche pas "something"
fn contains_ambiguous_word(text: &str, word: &str) -> bool {
    // "as appropriate" est multi-mots, utiliser contains directement
    if word.contains(' ') {
        return text.contains(word);
    }
    // Pour les mots simples, verifier les word boundaries
    for (start, _) in text.match_indices(word) {
        let before_ok = start == 0 || !text.as_bytes()[start - 1].is_ascii_alphanumeric();
        let end = start + word.len();
        let after_ok = end >= text.len() || !text.as_bytes()[end].is_ascii_alphanumeric();
        if before_ok && after_ok {
            return true;
        }
    }
    false
}

/// Mots normatifs (ISO 29148 section 5.2.4)
const NORMATIVE_KEYWORDS: &[&str] = &[
    "MUST", "SHALL", "SHOULD", "COULD", "WILL", "DOIT", "DEVRAIT", "POURRAIT",
];

/// Connecteurs indiquant des exigences multiples (violation Singular)
const COMPOUND_CONNECTORS: &[&str] = &[
    " and shall ",
    " et doit ",
    " and must ",
    " et le systeme ",
    " and the system ",
];

/// Verifie les 9 criteres de bien-formation d'une exigence (ISO 29148)
pub fn check_requirement_wellformedness(
    fr: &FunctionalRequirement,
    all_ids: &HashSet<&str>,
) -> Vec<WellFormednessWarning> {
    let mut warnings = Vec::new();

    // 1. Necessary — pas de doublon (detecte au niveau collection)
    // Gere au niveau de validate_specification_iso

    // 2. Unambiguous — pas de mots vagues (avec word boundary)
    let lower = fr.statement.to_lowercase();
    for word in AMBIGUOUS_WORDS {
        if contains_ambiguous_word(&lower, word) {
            warnings.push(WellFormednessWarning {
                requirement_id: fr.id.clone(),
                criterion: WellFormednessCriterion::Unambiguous,
                message: format!("Mot ambigu detecte: \"{}\"", word),
            });
            break; // Un seul avertissement par exigence
        }
    }

    // 3. Complete — statement non vide, ID present
    if fr.statement.trim().is_empty() {
        warnings.push(WellFormednessWarning {
            requirement_id: fr.id.clone(),
            criterion: WellFormednessCriterion::Complete,
            message: "Enonce vide".into(),
        });
    }

    // 4. Singular — une seule exigence par statement
    let lower_stmt = fr.statement.to_lowercase();
    for connector in COMPOUND_CONNECTORS {
        if lower_stmt.contains(connector) {
            warnings.push(WellFormednessWarning {
                requirement_id: fr.id.clone(),
                criterion: WellFormednessCriterion::Singular,
                message: format!(
                    "Exigence composee detectee (connecteur: \"{}\")",
                    connector.trim()
                ),
            });
            break;
        }
    }

    // 5. Feasible — toujours true par defaut (flag structurel)

    // 6. Verifiable — testable flag + verification_method
    if !fr.testable {
        warnings.push(WellFormednessWarning {
            requirement_id: fr.id.clone(),
            criterion: WellFormednessCriterion::Verifiable,
            message: "Exigence marquee non-testable".into(),
        });
    }
    if fr.verification_method == VerificationMethod::default() && !fr.testable {
        warnings.push(WellFormednessWarning {
            requirement_id: fr.id.clone(),
            criterion: WellFormednessCriterion::Verifiable,
            message: "Methode de verification absente pour exigence non-testable".into(),
        });
    }

    // 7. Correct — syntaxe normative (MUST/SHOULD/COULD)
    let upper = fr.statement.to_uppercase();
    if !NORMATIVE_KEYWORDS.iter().any(|kw| upper.contains(kw)) {
        warnings.push(WellFormednessWarning {
            requirement_id: fr.id.clone(),
            criterion: WellFormednessCriterion::Correct,
            message: "Enonce sans mot normatif (MUST/SHALL/SHOULD/COULD)".into(),
        });
    }

    // 8. Conforming — ID suit le pattern FR-NNN
    if !fr.id.starts_with("FR-") {
        warnings.push(WellFormednessWarning {
            requirement_id: fr.id.clone(),
            criterion: WellFormednessCriterion::Conforming,
            message: format!("ID \"{}\" ne suit pas le format FR-NNN", fr.id),
        });
    }

    // 9. Traceable — lien vers une source
    if let Some(ref parent) = fr.parent_requirement
        && !all_ids.contains(parent.as_str())
    {
        warnings.push(WellFormednessWarning {
            requirement_id: fr.id.clone(),
            criterion: WellFormednessCriterion::Traceable,
            message: format!("Parent \"{}\" n'existe pas", parent),
        });
    }

    warnings
}

// ---------------------------------------------------------------------------
// ISO 29148 — Bidirectional Traceability Report
// ---------------------------------------------------------------------------

/// Rapport de tracabilite bidirectionnelle (ISO 29148 + ASPICE BP6)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceabilityReport {
    /// Couverture Forward: FR → Tests (%)
    pub forward_coverage: f64,
    /// Couverture Backward: Tests → FR (%)
    pub backward_coverage: f64,
    /// Exigences non couvertes
    pub uncovered_requirements: Vec<String>,
    /// Tests orphelins (sans lien FR)
    pub orphan_tests: Vec<String>,
    /// Couverture par priorite
    pub coverage_by_priority: HashMap<String, f64>,
    /// Couverture par niveau de risque
    pub coverage_by_risk: HashMap<String, f64>,
}

/// Genere un rapport de tracabilite bidirectionnelle
pub fn check_bidirectional_traceability(
    spec: &Specification,
    suite: &TestSuite,
) -> TraceabilityReport {
    let fr_ids: HashSet<&str> = spec
        .functional_requirements
        .iter()
        .map(|r| r.id.as_str())
        .collect();

    // Forward: FR → Tests
    let covered_by_features: HashSet<&str> = suite
        .features
        .iter()
        .flat_map(|f| f.covered_requirements.iter().map(|s| s.as_str()))
        .collect();

    let covered_by_verification: HashSet<&str> = suite
        .features
        .iter()
        .flat_map(|f| f.scenarios.iter())
        .flat_map(|s| s.verification_of.iter().map(|v| v.as_str()))
        .collect();

    let all_covered: HashSet<&str> = covered_by_features
        .union(&covered_by_verification)
        .copied()
        .collect();

    let uncovered: Vec<String> = fr_ids
        .difference(&all_covered)
        .map(|s| s.to_string())
        .collect();

    let forward_coverage = if fr_ids.is_empty() {
        1.0
    } else {
        (fr_ids.len() - uncovered.len()) as f64 / fr_ids.len() as f64
    };

    // Backward: Tests → FR (orphan detection)
    let mut orphan_tests = Vec::new();
    let total_scenarios: usize = suite.features.iter().map(|f| f.scenarios.len()).sum();
    let mut linked_scenarios = 0usize;

    for feature in &suite.features {
        for scenario in &feature.scenarios {
            let has_link = !scenario.verification_of.is_empty()
                || scenario.tags.iter().any(|t| {
                    let tag = t.strip_prefix('@').unwrap_or(t);
                    fr_ids.contains(tag)
                });
            if has_link {
                linked_scenarios += 1;
            } else {
                orphan_tests.push(scenario.name.clone());
            }
        }
    }

    let backward_coverage = if total_scenarios == 0 {
        1.0
    } else {
        linked_scenarios as f64 / total_scenarios as f64
    };

    // Couverture par priorite
    let mut coverage_by_priority = HashMap::new();
    for priority in &[Priority::P1, Priority::P2, Priority::P3] {
        let frs_at_prio: Vec<&str> = spec
            .functional_requirements
            .iter()
            .filter(|fr| fr.priority == *priority)
            .map(|fr| fr.id.as_str())
            .collect();
        if !frs_at_prio.is_empty() {
            let covered_at_prio = frs_at_prio
                .iter()
                .filter(|id| all_covered.contains(**id))
                .count();
            coverage_by_priority.insert(
                priority.to_string(),
                covered_at_prio as f64 / frs_at_prio.len() as f64,
            );
        }
    }

    // Couverture par niveau de risque
    let mut coverage_by_risk = HashMap::new();
    for risk in &[RiskLevel::High, RiskLevel::Medium, RiskLevel::Low] {
        let frs_at_risk: Vec<&str> = spec
            .functional_requirements
            .iter()
            .filter(|fr| fr.risk_level.as_ref() == Some(risk))
            .map(|fr| fr.id.as_str())
            .collect();
        if !frs_at_risk.is_empty() {
            let covered_at_risk = frs_at_risk
                .iter()
                .filter(|id| all_covered.contains(**id))
                .count();
            coverage_by_risk.insert(
                risk.to_string(),
                covered_at_risk as f64 / frs_at_risk.len() as f64,
            );
        }
    }

    TraceabilityReport {
        forward_coverage,
        backward_coverage,
        uncovered_requirements: uncovered,
        orphan_tests,
        coverage_by_priority,
        coverage_by_risk,
    }
}

// ---------------------------------------------------------------------------
// ISO 25023 — Metriques qualite quantitatives
// ---------------------------------------------------------------------------

/// Metriques qualite (ISO/IEC 25023:2016)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Completude fonctionnelle: FR couverts / FR total
    pub functional_completeness: f64,
    /// Stabilite des exigences: % de FR sans clarification pendante
    pub requirement_stability: f64,
    /// Adequation des tests: ratio scenarios / FR
    pub test_adequacy_ratio: f64,
    /// Couverture risque: P1 FR couverts / P1 FR total
    pub risk_coverage: f64,
    /// Couverture NFR: NFR identifies / 9 caracteristiques ISO 25010
    pub nfr_coverage: f64,
    /// Score global (moyenne ponderee)
    pub overall_score: f64,
}

/// Calcule les metriques qualite ISO 25023
pub fn compute_quality_metrics(spec: &Specification, suite: &TestSuite) -> QualityMetrics {
    let total_fr = spec.functional_requirements.len();

    // Functional completeness
    let covered: HashSet<String> = suite
        .features
        .iter()
        .flat_map(|f| f.covered_requirements.iter().cloned())
        .chain(
            suite
                .features
                .iter()
                .flat_map(|f| f.scenarios.iter())
                .flat_map(|s| s.verification_of.iter().cloned()),
        )
        .collect();

    let functional_completeness = if total_fr == 0 {
        1.0
    } else {
        let covered_count = spec
            .functional_requirements
            .iter()
            .filter(|fr| covered.contains(&fr.id))
            .count();
        covered_count as f64 / total_fr as f64
    };

    // Requirement stability
    let unresolved_clarifications = spec
        .clarifications_needed
        .iter()
        .filter(|c| !c.resolved)
        .count();
    let requirement_stability = if total_fr == 0 {
        1.0
    } else {
        1.0 - (unresolved_clarifications as f64 / total_fr.max(1) as f64).min(1.0)
    };

    // Test adequacy ratio
    let total_scenarios: usize = suite.features.iter().map(|f| f.scenarios.len()).sum();
    let test_adequacy_ratio = if total_fr == 0 {
        0.0
    } else {
        total_scenarios as f64 / total_fr as f64
    };

    // Risk coverage (P1 focus)
    let p1_frs: Vec<&str> = spec
        .functional_requirements
        .iter()
        .filter(|fr| fr.priority == Priority::P1)
        .map(|fr| fr.id.as_str())
        .collect();
    let risk_coverage = if p1_frs.is_empty() {
        1.0
    } else {
        let p1_covered = p1_frs.iter().filter(|id| covered.contains(**id)).count();
        p1_covered as f64 / p1_frs.len() as f64
    };

    // NFR coverage (ISO 25010: 9 characteristics)
    let nfr_characteristics: HashSet<String> = spec
        .functional_requirements
        .iter()
        .filter(|fr| fr.category == RequirementCategory::NonFunctional)
        .filter_map(|fr| fr.quality_characteristic.as_ref())
        .map(|qc| qc.to_string())
        .collect();
    let nfr_coverage = nfr_characteristics.len() as f64 / 9.0;

    // Overall score (weighted average)
    let overall_score = functional_completeness * 0.30
        + requirement_stability * 0.15
        + (test_adequacy_ratio / 3.0).min(1.0) * 0.20 // normalize: 3 scenarios/FR = 100%
        + risk_coverage * 0.25
        + nfr_coverage * 0.10;

    QualityMetrics {
        functional_completeness,
        requirement_stability,
        test_adequacy_ratio,
        risk_coverage,
        nfr_coverage,
        overall_score,
    }
}

// ---------------------------------------------------------------------------
// Validation existante enrichie
// ---------------------------------------------------------------------------

/// Valide la completude d'une specification (checklist + well-formedness ISO 29148)
pub fn validate_specification(spec: &Specification) -> SpecValidation {
    let mut checklist = Vec::new();

    // --- Checks existants ---

    checklist.push(ChecklistItem {
        description: "Au moins un scenario utilisateur defini".into(),
        passed: !spec.user_scenarios.is_empty(),
        category: "Completude".into(),
    });

    checklist.push(ChecklistItem {
        description: "Exigences fonctionnelles definies".into(),
        passed: !spec.functional_requirements.is_empty(),
        category: "Completude".into(),
    });

    checklist.push(ChecklistItem {
        description: "Criteres de succes mesurables definis".into(),
        passed: !spec.success_criteria.is_empty(),
        category: "Completude".into(),
    });

    let scenarios_with_criteria = spec
        .user_scenarios
        .iter()
        .all(|s| !s.acceptance_scenarios.is_empty());
    checklist.push(ChecklistItem {
        description: "Chaque scenario a des criteres d'acceptation".into(),
        passed: scenarios_with_criteria,
        category: "Completude".into(),
    });

    let unresolved_count = spec
        .clarifications_needed
        .iter()
        .filter(|c| !c.resolved)
        .count();
    checklist.push(ChecklistItem {
        description: "Moins de 3 clarifications non resolues".into(),
        passed: unresolved_count < 3,
        category: "Clarte".into(),
    });

    let all_testable = spec.functional_requirements.iter().all(|r| r.testable);
    checklist.push(ChecklistItem {
        description: "Toutes les exigences sont testables".into(),
        passed: all_testable,
        category: "Testabilite".into(),
    });

    checklist.push(ChecklistItem {
        description: "Cas limites identifies".into(),
        passed: !spec.edge_cases.is_empty(),
        category: "Completude".into(),
    });

    // --- Checks ISO 29148 enrichis ---

    // IDs uniques
    let mut seen_ids = HashSet::new();
    let all_unique = spec
        .functional_requirements
        .iter()
        .all(|fr| seen_ids.insert(fr.id.as_str()));
    checklist.push(ChecklistItem {
        description: "ISO-29148: IDs d'exigences uniques".into(),
        passed: all_unique,
        category: "Conformite".into(),
    });

    // Tous les enonces ont un mot normatif
    let all_normative = spec.functional_requirements.iter().all(|fr| {
        let upper = fr.statement.to_uppercase();
        NORMATIVE_KEYWORDS.iter().any(|kw| upper.contains(kw))
    });
    checklist.push(ChecklistItem {
        description: "ISO-29148: Syntaxe normative (MUST/SHALL/SHOULD)".into(),
        passed: all_normative || spec.functional_requirements.is_empty(),
        category: "Conformite".into(),
    });

    // Pas de mots ambigus (avec word boundary matching)
    let no_ambiguous = spec.functional_requirements.iter().all(|fr| {
        let lower = fr.statement.to_lowercase();
        !AMBIGUOUS_WORDS
            .iter()
            .any(|w| contains_ambiguous_word(&lower, w))
    });
    checklist.push(ChecklistItem {
        description: "ISO-29148: Aucun mot ambigu dans les exigences".into(),
        passed: no_ambiguous || spec.functional_requirements.is_empty(),
        category: "Clarte".into(),
    });

    // Exigences P1 ont un risk_level
    let p1_have_risk = spec
        .functional_requirements
        .iter()
        .filter(|fr| fr.priority == Priority::P1)
        .all(|fr| fr.risk_level.is_some());
    checklist.push(ChecklistItem {
        description: "ISO-29148: Exigences P1 ont un niveau de risque".into(),
        passed: p1_have_risk,
        category: "Conformite".into(),
    });

    // NFR ont une quality_characteristic
    let nfr_have_qc = spec
        .functional_requirements
        .iter()
        .filter(|fr| fr.category == RequirementCategory::NonFunctional)
        .all(|fr| fr.quality_characteristic.is_some());
    checklist.push(ChecklistItem {
        description: "ISO-25010: NFR ont une quality_characteristic".into(),
        passed: nfr_have_qc,
        category: "Conformite".into(),
    });

    // --- Calcul des scores ---
    let score_for = |cat: &str| -> f32 {
        let items: Vec<&ChecklistItem> = checklist.iter().filter(|c| c.category == cat).collect();
        if items.is_empty() {
            return 1.0;
        }
        items.iter().filter(|c| c.passed).count() as f32 / items.len() as f32
    };

    let completeness_score = score_for("Completude");
    let clarity_score = score_for("Clarte");
    let testability_score = score_for("Testabilite");
    SpecValidation {
        completeness_score,
        clarity_score,
        testability_score,
        checklist_items: checklist,
    }
}

/// Verifie la couverture de tracabilite entre spec et tests (legacy)
pub fn check_traceability(spec: &Specification, suite: &TestSuite) -> Vec<String> {
    let requirement_ids: Vec<String> = spec
        .functional_requirements
        .iter()
        .map(|r| r.id.clone())
        .collect();

    let covered: Vec<String> = suite
        .features
        .iter()
        .flat_map(|f| f.covered_requirements.clone())
        .collect();

    requirement_ids
        .into_iter()
        .filter(|id| !covered.contains(id))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::specification::*;
    use crate::domain::test_case::*;
    use crate::domain::user_story::Priority;
    use uuid::Uuid;

    fn make_fr(id: &str, statement: &str, priority: Priority) -> FunctionalRequirement {
        FunctionalRequirement {
            id: id.into(),
            statement: statement.into(),
            priority,
            category: RequirementCategory::Functional,
            testable: true,
            rationale: None,
            source: None,
            verification_method: Default::default(),
            risk_level: None,
            parent_requirement: None,
            allocated_to: Vec::new(),
            quality_characteristic: None,
        }
    }

    fn make_complete_spec() -> Specification {
        let mut spec = Specification::new("Test".into());
        spec.user_scenarios.push(UserScenario {
            id: "US-001".into(),
            title: "Test".into(),
            priority: Priority::P1,
            description: "Test desc".into(),
            why_priority: "Critical".into(),
            independent_test: "Can be tested".into(),
            acceptance_scenarios: vec![AcceptanceScenario {
                given: "state".into(),
                when: "action".into(),
                then: "result".into(),
            }],
            source_story_id: Uuid::new_v4(),
        });
        spec.functional_requirements
            .push(make_fr("FR-001", "System MUST do X", Priority::P1));
        spec.success_criteria.push(SuccessCriterion {
            id: "SC-001".into(),
            description: "Success".into(),
            measurable_metric: "100%".into(),
        });
        spec.edge_cases.push(EdgeCase {
            description: "Edge".into(),
            related_scenario: Some("US-001".into()),
            severity: Priority::P2,
        });
        spec
    }

    fn make_scenario(name: &str, verification_of: Vec<String>) -> Scenario {
        Scenario {
            name: name.into(),
            tags: vec![],
            scenario_type: ScenarioType::HappyPath,
            steps: vec![],
            examples: None,
            test_data_suggestions: vec![],
            verification_of,
            coverage_technique: None,
        }
    }

    #[test]
    fn test_validate_complete_spec() {
        let spec = make_complete_spec();
        let validation = validate_specification(&spec);
        assert!(validation.completeness_score > 0.9);
        assert!(validation.testability_score > 0.9);
    }

    #[test]
    fn test_validate_empty_spec() {
        let spec = Specification::new("Empty".into());
        let validation = validate_specification(&spec);
        assert!(validation.completeness_score < 0.5);
    }

    #[test]
    fn test_traceability_check() {
        let mut spec = Specification::new("Test".into());
        spec.functional_requirements
            .push(make_fr("FR-001", "Test", Priority::P1));
        spec.functional_requirements
            .push(make_fr("FR-002", "Test2", Priority::P2));

        let suite = TestSuite {
            features: vec![Feature {
                id: Uuid::new_v4(),
                name: "Test".into(),
                description: "".into(),
                tags: vec![],
                background: None,
                scenarios: vec![],
                source_scenario_ids: vec![],
                covered_requirements: vec!["FR-001".into()],
                test_level: Default::default(),
            }],
            source_spec_id: Uuid::new_v4(),
            total_scenarios: 0,
            coverage: TestCoverage {
                requirements_covered: vec![],
                requirements_total: 0,
                coverage_percentage: 0.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        };

        let gaps = check_traceability(&spec, &suite);
        assert_eq!(gaps, vec!["FR-002"]);
    }

    #[test]
    fn test_wellformedness_normative() {
        let fr = make_fr("FR-001", "The system does something", Priority::P1);
        let ids = HashSet::from(["FR-001"]);
        let warnings = check_requirement_wellformedness(&fr, &ids);
        assert!(
            warnings
                .iter()
                .any(|w| w.criterion == WellFormednessCriterion::Correct)
        );
    }

    #[test]
    fn test_wellformedness_ambiguous() {
        let fr = make_fr(
            "FR-001",
            "System MUST handle approximately 100 requests",
            Priority::P1,
        );
        let ids = HashSet::from(["FR-001"]);
        let warnings = check_requirement_wellformedness(&fr, &ids);
        assert!(
            warnings
                .iter()
                .any(|w| w.criterion == WellFormednessCriterion::Unambiguous)
        );
    }

    #[test]
    fn test_wellformedness_clean() {
        let fr = make_fr(
            "FR-001",
            "Le systeme DOIT traiter les requetes en 2 secondes",
            Priority::P1,
        );
        let ids = HashSet::from(["FR-001"]);
        let warnings = check_requirement_wellformedness(&fr, &ids);
        // Should only have the Traceable warning (no source)
        assert!(
            warnings
                .iter()
                .all(|w| w.criterion != WellFormednessCriterion::Correct
                    && w.criterion != WellFormednessCriterion::Unambiguous)
        );
    }

    #[test]
    fn test_bidirectional_traceability_full_coverage() {
        let mut spec = Specification::new("Test".into());
        spec.functional_requirements
            .push(make_fr("FR-001", "MUST do X", Priority::P1));

        let mut feature = Feature::new("Test".into(), "".into());
        feature.covered_requirements = vec!["FR-001".into()];
        feature
            .scenarios
            .push(make_scenario("Test scenario", vec!["FR-001".into()]));

        let suite = TestSuite {
            features: vec![feature],
            source_spec_id: spec.id,
            total_scenarios: 1,
            coverage: TestCoverage {
                requirements_covered: vec!["FR-001".into()],
                requirements_total: 1,
                coverage_percentage: 100.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        };

        let report = check_bidirectional_traceability(&spec, &suite);
        assert!((report.forward_coverage - 1.0).abs() < f64::EPSILON);
        assert!((report.backward_coverage - 1.0).abs() < f64::EPSILON);
        assert!(report.uncovered_requirements.is_empty());
        assert!(report.orphan_tests.is_empty());
    }

    #[test]
    fn test_bidirectional_traceability_with_gaps() {
        let mut spec = Specification::new("Test".into());
        spec.functional_requirements
            .push(make_fr("FR-001", "MUST do X", Priority::P1));
        spec.functional_requirements
            .push(make_fr("FR-002", "MUST do Y", Priority::P2));

        let mut feature = Feature::new("Test".into(), "".into());
        feature.covered_requirements = vec!["FR-001".into()];
        feature
            .scenarios
            .push(make_scenario("Linked", vec!["FR-001".into()]));
        feature.scenarios.push(make_scenario("Orphan", vec![]));

        let suite = TestSuite {
            features: vec![feature],
            source_spec_id: spec.id,
            total_scenarios: 2,
            coverage: TestCoverage {
                requirements_covered: vec!["FR-001".into()],
                requirements_total: 2,
                coverage_percentage: 50.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        };

        let report = check_bidirectional_traceability(&spec, &suite);
        assert!((report.forward_coverage - 0.5).abs() < f64::EPSILON);
        assert!((report.backward_coverage - 0.5).abs() < f64::EPSILON);
        assert_eq!(report.uncovered_requirements, vec!["FR-002"]);
        assert_eq!(report.orphan_tests, vec!["Orphan"]);
    }

    #[test]
    fn test_quality_metrics_complete() {
        let mut spec = Specification::new("Test".into());
        spec.functional_requirements
            .push(make_fr("FR-001", "MUST X", Priority::P1));

        let mut feature = Feature::new("Test".into(), "".into());
        feature.covered_requirements = vec!["FR-001".into()];
        feature
            .scenarios
            .push(make_scenario("S1", vec!["FR-001".into()]));
        feature
            .scenarios
            .push(make_scenario("S2", vec!["FR-001".into()]));

        let suite = TestSuite {
            features: vec![feature],
            source_spec_id: spec.id,
            total_scenarios: 2,
            coverage: TestCoverage {
                requirements_covered: vec!["FR-001".into()],
                requirements_total: 1,
                coverage_percentage: 100.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        };

        let metrics = compute_quality_metrics(&spec, &suite);
        assert!((metrics.functional_completeness - 1.0).abs() < f64::EPSILON);
        assert!((metrics.risk_coverage - 1.0).abs() < f64::EPSILON);
        assert!(metrics.test_adequacy_ratio >= 2.0);
        assert!(metrics.overall_score > 0.5);
    }

    #[test]
    fn test_iso_checks_in_validate_specification() {
        let mut spec = make_complete_spec();
        // Add a requirement with ambiguous word
        spec.functional_requirements.push(FunctionalRequirement {
            id: "FR-002".into(),
            statement: "System should handle some requests".into(),
            priority: Priority::P2,
            category: RequirementCategory::Functional,
            testable: true,
            rationale: None,
            source: None,
            verification_method: Default::default(),
            risk_level: None,
            parent_requirement: None,
            allocated_to: Vec::new(),
            quality_characteristic: None,
        });

        let validation = validate_specification(&spec);
        // Should have the ambiguity check failing
        let ambiguity_check = validation
            .checklist_items
            .iter()
            .find(|c| c.description.contains("ambigu"));
        assert!(ambiguity_check.is_some());
        assert!(!ambiguity_check.unwrap().passed);
    }
}
