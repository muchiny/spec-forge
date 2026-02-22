//! Module de tracabilite bidirectionnelle
//!
//! Construction d'une matrice de tracabilite (RTM) reliant exigences,
//! scenarios de test, et notes de conformite.
//!
//! Conformite : ISO/IEC/IEEE 29148:2018 section 6.6, ASPICE BP6

use serde::{Deserialize, Serialize};

use super::specification::{ComplianceProfile, RiskLevel, Specification, VerificationMethod};
use super::test_case::{CoverageTechnique, TestSuite};
use super::user_story::Priority;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Matrice de tracabilite complete
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceabilityMatrix {
    pub entries: Vec<TraceabilityEntry>,
    pub summary: TraceabilitySummary,
    pub compliance_notes: Vec<ComplianceNote>,
}

/// Une ligne de la matrice de tracabilite (1 par FR)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceabilityEntry {
    pub requirement_id: String,
    pub statement: String,
    pub priority: Priority,
    pub risk_level: Option<RiskLevel>,
    pub source_stories: Vec<String>,
    pub verification_method: VerificationMethod,
    pub covering_features: Vec<String>,
    pub covering_scenarios: Vec<String>,
    pub coverage_techniques: Vec<CoverageTechnique>,
    pub status: TraceabilityStatus,
}

/// Statut de couverture d'une exigence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceabilityStatus {
    /// Couvert par au moins 1 test
    FullyCovered,
    /// Couvert mais incomplet (ex: P1 avec un seul scenario)
    PartiallyCovered,
    /// GAP — aucune couverture
    NotCovered,
    /// Verifie par analyse (pas de test necessaire)
    VerifiedByAnalysis,
    /// Verifie par inspection
    VerifiedByInspection,
    /// Verifie par demonstration
    VerifiedByDemo,
}

impl std::fmt::Display for TraceabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TraceabilityStatus::FullyCovered => write!(f, "Couvert"),
            TraceabilityStatus::PartiallyCovered => write!(f, "Partiel"),
            TraceabilityStatus::NotCovered => write!(f, "GAP"),
            TraceabilityStatus::VerifiedByAnalysis => write!(f, "Analyse"),
            TraceabilityStatus::VerifiedByInspection => write!(f, "Inspection"),
            TraceabilityStatus::VerifiedByDemo => write!(f, "Demo"),
        }
    }
}

/// Resume synthetique de la matrice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceabilitySummary {
    pub total_requirements: usize,
    pub covered: usize,
    pub partially_covered: usize,
    pub not_covered: usize,
    pub verified_other: usize,
    pub forward_coverage_pct: f64,
    pub orphan_tests: Vec<String>,
}

/// Note de conformite normative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceNote {
    pub standard: String,
    pub section: String,
    pub status: ComplianceStatus,
    pub details: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    PartiallyCompliant,
    NonCompliant,
}

impl std::fmt::Display for ComplianceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplianceStatus::Compliant => write!(f, "Conforme"),
            ComplianceStatus::PartiallyCompliant => write!(f, "Partiellement conforme"),
            ComplianceStatus::NonCompliant => write!(f, "Non conforme"),
        }
    }
}

// ---------------------------------------------------------------------------
// Construction de la matrice
// ---------------------------------------------------------------------------

/// Construit la matrice de tracabilite complete a partir d'une spec et d'un test suite
pub fn build_traceability_matrix(spec: &Specification, suite: &TestSuite) -> TraceabilityMatrix {
    let mut entries = Vec::new();

    for fr in &spec.functional_requirements {
        // Trouver les features/scenarios qui couvrent cette exigence
        let mut covering_features_set = std::collections::HashSet::new();
        let mut covering_scenarios = Vec::new();
        let mut coverage_techniques_set = std::collections::HashSet::new();

        for feature in &suite.features {
            let feature_covers = feature.covered_requirements.contains(&fr.id);

            for scenario in &feature.scenarios {
                let scenario_covers = scenario.verification_of.contains(&fr.id)
                    || scenario.tags.iter().any(|t| {
                        let tag = t.strip_prefix('@').unwrap_or(t);
                        tag == fr.id
                    });

                // Un scenario ne couvre un FR que s'il a un lien direct
                // (verification_of ou tag). Le feature_covers sert uniquement
                // a enregistrer la feature comme couvrante, pas a gonfler le count.
                if scenario_covers {
                    covering_features_set.insert(feature.name.clone());
                    covering_scenarios.push(scenario.name.clone());
                    if let Some(ct) = scenario.coverage_technique {
                        coverage_techniques_set.insert(ct);
                    }
                }
            }

            // Si la feature couvre le FR mais aucun scenario individuel ne le reference,
            // enregistrer au moins la feature comme couvrante
            if feature_covers {
                covering_features_set.insert(feature.name.clone());
            }
        }

        let covering_features: Vec<String> = covering_features_set.into_iter().collect();
        let coverage_techniques: Vec<CoverageTechnique> =
            coverage_techniques_set.into_iter().collect();

        // Determiner le statut
        let status = determine_status(fr, &covering_scenarios);

        // Trouver les US sources pour cette FR
        // Correler via fr.source (qui reference les IDs de scenarios)
        // ou par defaut via les scenarios dont les acceptance_scenarios
        // pourraient etre a l'origine de cette FR
        let source_stories: Vec<String> = if let Some(ref source) = fr.source {
            // Le LLM a fourni une source explicite — la matcher aux US
            spec.user_scenarios
                .iter()
                .filter(|us| source.contains(&us.id))
                .map(|us| us.id.clone())
                .collect()
        } else {
            // Pas de source explicite — laisser vide plutot que tout inclure
            Vec::new()
        };

        entries.push(TraceabilityEntry {
            requirement_id: fr.id.clone(),
            statement: fr.statement.clone(),
            priority: fr.priority,
            risk_level: fr.risk_level,
            source_stories,
            verification_method: fr.verification_method,
            covering_features,
            covering_scenarios,
            coverage_techniques,
            status,
        });
    }

    // Detect orphan tests
    let fr_ids: std::collections::HashSet<&str> = spec
        .functional_requirements
        .iter()
        .map(|fr| fr.id.as_str())
        .collect();

    let orphan_tests: Vec<String> = suite
        .features
        .iter()
        .flat_map(|f| &f.scenarios)
        .filter(|s| {
            let has_link = !s.verification_of.is_empty()
                || s.tags.iter().any(|t| {
                    let tag = t.strip_prefix('@').unwrap_or(t);
                    fr_ids.contains(tag)
                });
            !has_link
        })
        .map(|s| s.name.clone())
        .collect();

    // Summary
    let covered = entries
        .iter()
        .filter(|e| e.status == TraceabilityStatus::FullyCovered)
        .count();
    let partially = entries
        .iter()
        .filter(|e| e.status == TraceabilityStatus::PartiallyCovered)
        .count();
    let not_covered = entries
        .iter()
        .filter(|e| e.status == TraceabilityStatus::NotCovered)
        .count();
    let verified_other = entries
        .iter()
        .filter(|e| {
            matches!(
                e.status,
                TraceabilityStatus::VerifiedByAnalysis
                    | TraceabilityStatus::VerifiedByInspection
                    | TraceabilityStatus::VerifiedByDemo
            )
        })
        .count();

    let total = entries.len();
    let forward_coverage_pct = if total == 0 {
        100.0
    } else {
        ((total - not_covered) as f64 / total as f64) * 100.0
    };

    let summary = TraceabilitySummary {
        total_requirements: total,
        covered,
        partially_covered: partially,
        not_covered,
        verified_other,
        forward_coverage_pct,
        orphan_tests,
    };

    // Compliance notes
    let compliance_notes = build_compliance_notes(spec, &entries, forward_coverage_pct);

    TraceabilityMatrix {
        entries,
        summary,
        compliance_notes,
    }
}

/// Determine le statut de couverture d'une exigence
fn determine_status(
    fr: &super::specification::FunctionalRequirement,
    covering_scenarios: &[String],
) -> TraceabilityStatus {
    // Si la methode de verification n'est pas Test, c'est un autre statut
    match fr.verification_method {
        VerificationMethod::Analysis if covering_scenarios.is_empty() => {
            TraceabilityStatus::VerifiedByAnalysis
        }
        VerificationMethod::Inspection if covering_scenarios.is_empty() => {
            TraceabilityStatus::VerifiedByInspection
        }
        VerificationMethod::Demonstration if covering_scenarios.is_empty() => {
            TraceabilityStatus::VerifiedByDemo
        }
        _ => {
            if covering_scenarios.is_empty() {
                TraceabilityStatus::NotCovered
            } else if fr.priority == Priority::P1 && covering_scenarios.len() < 2 {
                TraceabilityStatus::PartiallyCovered
            } else {
                TraceabilityStatus::FullyCovered
            }
        }
    }
}

/// Genere les notes de conformite
fn build_compliance_notes(
    spec: &Specification,
    entries: &[TraceabilityEntry],
    coverage_pct: f64,
) -> Vec<ComplianceNote> {
    let mut notes = Vec::new();

    // ISO 29148 — Traceability
    let trace_status = if coverage_pct >= 100.0 {
        ComplianceStatus::Compliant
    } else if coverage_pct >= 80.0 {
        ComplianceStatus::PartiallyCompliant
    } else {
        ComplianceStatus::NonCompliant
    };
    notes.push(ComplianceNote {
        standard: "ISO/IEC/IEEE 29148:2018".into(),
        section: "6.6 — Traceability".into(),
        status: trace_status,
        details: format!("Couverture forward: {:.0}%", coverage_pct),
    });

    // ISO 29148 — Well-formedness (verifier tous les mots ambigus)
    let all_normative = entries.iter().all(|e| {
        let lower = e.statement.to_lowercase();
        !super::validation::AMBIGUOUS_WORDS
            .iter()
            .any(|w| lower.contains(w))
    });
    notes.push(ComplianceNote {
        standard: "ISO/IEC/IEEE 29148:2018".into(),
        section: "5.2.5 — Well-formed requirements".into(),
        status: if all_normative {
            ComplianceStatus::Compliant
        } else {
            ComplianceStatus::PartiallyCompliant
        },
        details: format!("{} exigences dans la matrice", entries.len()),
    });

    // Domain-specific compliance
    if let Some(ref profile) = spec.compliance_profile {
        let note = match profile {
            ComplianceProfile::Aviation(dal) => ComplianceNote {
                standard: "DO-178C".into(),
                section: "Objectives".into(),
                status: if coverage_pct >= 100.0 {
                    ComplianceStatus::Compliant
                } else {
                    ComplianceStatus::NonCompliant
                },
                details: format!("DAL {:?} — couverture {:.0}%", dal, coverage_pct),
            },
            ComplianceProfile::Medical(sw) => ComplianceNote {
                standard: "IEC 62304".into(),
                section: "Software safety classification".into(),
                status: if coverage_pct >= 90.0 {
                    ComplianceStatus::Compliant
                } else {
                    ComplianceStatus::PartiallyCompliant
                },
                details: format!("SW Class {:?} — couverture {:.0}%", sw, coverage_pct),
            },
            ComplianceProfile::Automotive(asil) => ComplianceNote {
                standard: "ISO 26262".into(),
                section: "Part 6 — Software development".into(),
                status: if coverage_pct >= 95.0 {
                    ComplianceStatus::Compliant
                } else {
                    ComplianceStatus::PartiallyCompliant
                },
                details: format!("ASIL {:?} — couverture {:.0}%", asil, coverage_pct),
            },
            ComplianceProfile::Railway(ssil) => ComplianceNote {
                standard: "EN 50716".into(),
                section: "Software requirements".into(),
                status: if coverage_pct >= 100.0 {
                    ComplianceStatus::Compliant
                } else {
                    ComplianceStatus::NonCompliant
                },
                details: format!("SSIL {:?} — couverture {:.0}%", ssil, coverage_pct),
            },
            ComplianceProfile::Safety(sil) => ComplianceNote {
                standard: "IEC 61508".into(),
                section: "Part 3 — Software requirements".into(),
                status: if coverage_pct >= 95.0 {
                    ComplianceStatus::Compliant
                } else {
                    ComplianceStatus::PartiallyCompliant
                },
                details: format!("SIL {:?} — couverture {:.0}%", sil, coverage_pct),
            },
            ComplianceProfile::General => ComplianceNote {
                standard: "ISO/IEC/IEEE 29148:2018".into(),
                section: "General compliance".into(),
                status: if coverage_pct >= 80.0 {
                    ComplianceStatus::Compliant
                } else {
                    ComplianceStatus::PartiallyCompliant
                },
                details: format!("Couverture generale: {:.0}%", coverage_pct),
            },
        };
        notes.push(note);
    }

    notes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::specification::*;
    use crate::domain::test_case::*;

    fn make_fr(id: &str, priority: Priority) -> FunctionalRequirement {
        FunctionalRequirement {
            id: id.into(),
            statement: format!("System MUST do {}", id),
            priority,
            category: RequirementCategory::Functional,
            testable: true,
            rationale: None,
            source: None,
            verification_method: VerificationMethod::Test,
            risk_level: Some(RiskLevel::High),
            parent_requirement: None,
            allocated_to: Vec::new(),
            quality_characteristic: None,
        }
    }

    fn make_scenario_with_verif(name: &str, verifies: Vec<String>) -> Scenario {
        Scenario {
            name: name.into(),
            tags: vec![],
            scenario_type: ScenarioType::HappyPath,
            steps: vec![],
            examples: None,
            test_data_suggestions: vec![],
            verification_of: verifies,
            coverage_technique: Some(CoverageTechnique::EquivalencePartitioning),
        }
    }

    #[test]
    fn test_build_matrix_full_coverage() {
        let mut spec = Specification::new("Test".into());
        spec.functional_requirements
            .push(make_fr("FR-001", Priority::P1));

        let mut feature = Feature::new("Test".into(), "".into());
        feature.covered_requirements = vec!["FR-001".into()];
        feature
            .scenarios
            .push(make_scenario_with_verif("Happy", vec!["FR-001".into()]));
        feature
            .scenarios
            .push(make_scenario_with_verif("Error", vec!["FR-001".into()]));

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

        let matrix = build_traceability_matrix(&spec, &suite);
        assert_eq!(matrix.entries.len(), 1);
        assert_eq!(matrix.entries[0].status, TraceabilityStatus::FullyCovered);
        assert_eq!(matrix.summary.covered, 1);
        assert_eq!(matrix.summary.not_covered, 0);
        assert!(matrix.summary.orphan_tests.is_empty());
    }

    #[test]
    fn test_build_matrix_with_gap() {
        let mut spec = Specification::new("Test".into());
        spec.functional_requirements
            .push(make_fr("FR-001", Priority::P1));
        spec.functional_requirements
            .push(make_fr("FR-002", Priority::P2));

        let mut feature = Feature::new("Test".into(), "".into());
        feature.covered_requirements = vec!["FR-001".into()];
        feature
            .scenarios
            .push(make_scenario_with_verif("S1", vec!["FR-001".into()]));
        feature
            .scenarios
            .push(make_scenario_with_verif("S2", vec!["FR-001".into()]));

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

        let matrix = build_traceability_matrix(&spec, &suite);
        assert_eq!(matrix.summary.covered, 1);
        assert_eq!(matrix.summary.not_covered, 1);
        assert!(
            matrix.summary.forward_coverage_pct > 49.0
                && matrix.summary.forward_coverage_pct < 51.0
        );
    }

    #[test]
    fn test_build_matrix_analysis_verification() {
        let mut spec = Specification::new("Test".into());
        let mut fr = make_fr("FR-001", Priority::P1);
        fr.verification_method = VerificationMethod::Analysis;
        spec.functional_requirements.push(fr);

        let suite = TestSuite {
            features: vec![],
            source_spec_id: spec.id,
            total_scenarios: 0,
            coverage: TestCoverage {
                requirements_covered: vec![],
                requirements_total: 1,
                coverage_percentage: 0.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        };

        let matrix = build_traceability_matrix(&spec, &suite);
        assert_eq!(
            matrix.entries[0].status,
            TraceabilityStatus::VerifiedByAnalysis
        );
        assert_eq!(matrix.summary.verified_other, 1);
    }

    #[test]
    fn test_orphan_detection() {
        let mut spec = Specification::new("Test".into());
        spec.functional_requirements
            .push(make_fr("FR-001", Priority::P1));

        let mut feature = Feature::new("Test".into(), "".into());
        feature
            .scenarios
            .push(make_scenario_with_verif("Linked", vec!["FR-001".into()]));
        // Orphan scenario — no verification_of
        feature.scenarios.push(Scenario {
            name: "Orphan".into(),
            tags: vec![],
            scenario_type: ScenarioType::HappyPath,
            steps: vec![],
            examples: None,
            test_data_suggestions: vec![],
            verification_of: vec![],
            coverage_technique: None,
        });

        let suite = TestSuite {
            features: vec![feature],
            source_spec_id: spec.id,
            total_scenarios: 2,
            coverage: TestCoverage {
                requirements_covered: vec![],
                requirements_total: 1,
                coverage_percentage: 0.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        };

        let matrix = build_traceability_matrix(&spec, &suite);
        assert_eq!(matrix.summary.orphan_tests, vec!["Orphan"]);
    }

    #[test]
    fn test_compliance_notes_generated() {
        let mut spec = Specification::new("Test".into());
        spec.compliance_profile = Some(ComplianceProfile::General);
        spec.functional_requirements
            .push(make_fr("FR-001", Priority::P1));

        let suite = TestSuite {
            features: vec![],
            source_spec_id: spec.id,
            total_scenarios: 0,
            coverage: TestCoverage {
                requirements_covered: vec![],
                requirements_total: 1,
                coverage_percentage: 0.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        };

        let matrix = build_traceability_matrix(&spec, &suite);
        assert!(matrix.compliance_notes.len() >= 2);
        assert!(
            matrix
                .compliance_notes
                .iter()
                .any(|n| n.standard.contains("29148"))
        );
    }
}
