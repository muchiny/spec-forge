//! Adapter TraceabilityWriter - Ecrit le rapport de tracabilite
//!
//! Genere une matrice de tracabilite enrichie (ISO 29148 section 6.6)

use std::fmt::Write;
use std::path::{Path, PathBuf};

use crate::domain::specification::Specification;
use crate::domain::test_case::TestSuite;
use crate::domain::traceability::{TraceabilityStatus, build_traceability_matrix};

/// Ecrit un rapport de tracabilite Markdown
pub struct TraceabilityWriter;

impl TraceabilityWriter {
    pub fn new() -> Self {
        Self
    }

    /// Genere le rapport de tracabilite en Markdown
    pub fn render(&self, spec: &Specification, suite: &TestSuite) -> String {
        let matrix = build_traceability_matrix(spec, suite);
        let mut md = String::new();

        writeln!(md, "# Matrice de Tracabilite").unwrap();
        writeln!(md).unwrap();
        writeln!(md, "**Specification**: {}", spec.title).unwrap();
        writeln!(md, "**Date**: {}", spec.created_at.format("%Y-%m-%d")).unwrap();
        writeln!(md, "**Version**: {}", spec.version).unwrap();
        writeln!(md, "**Conformite**: ISO/IEC/IEEE 29148:2018 section 6.6").unwrap();
        writeln!(md).unwrap();

        // Enriched traceability table
        writeln!(md, "## Matrice de tracabilite exigences-tests").unwrap();
        writeln!(md).unwrap();
        writeln!(md, "| FR-ID | Priorite | Risque | Verification | Feature | Scenarios | Technique | Statut |").unwrap();
        writeln!(
            md,
            "|-------|----------|--------|-------------|---------|-----------|-----------|--------|"
        )
        .unwrap();

        for entry in &matrix.entries {
            let risk = entry
                .risk_level
                .as_ref()
                .map(|r| r.to_string())
                .unwrap_or_else(|| "-".into());
            let features = if entry.covering_features.is_empty() {
                "-".into()
            } else {
                entry.covering_features.join(", ")
            };
            let scenario_count = entry.covering_scenarios.len();
            let techniques: String = if entry.coverage_techniques.is_empty() {
                "-".into()
            } else {
                entry
                    .coverage_techniques
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join("+")
            };
            let status = match entry.status {
                TraceabilityStatus::NotCovered => format!("**{}**", entry.status),
                _ => entry.status.to_string(),
            };

            writeln!(
                md,
                "| {} | {} | {} | {} | {} | {} | {} | {} |",
                entry.requirement_id,
                entry.priority,
                risk,
                entry.verification_method,
                features,
                scenario_count,
                techniques,
                status,
            )
            .unwrap();
        }

        writeln!(md).unwrap();

        // Resume de couverture
        writeln!(md, "## Resume de couverture").unwrap();
        writeln!(md).unwrap();
        writeln!(
            md,
            "- Exigences totales: {}",
            matrix.summary.total_requirements,
        )
        .unwrap();
        writeln!(
            md,
            "- Couvertes: {} | Partielles: {} | GAP: {} | Autre verification: {}",
            matrix.summary.covered,
            matrix.summary.partially_covered,
            matrix.summary.not_covered,
            matrix.summary.verified_other,
        )
        .unwrap();
        writeln!(
            md,
            "- Couverture forward: **{:.0}%**",
            matrix.summary.forward_coverage_pct,
        )
        .unwrap();
        writeln!(
            md,
            "- Scenarios happy path: {}",
            suite.coverage.scenarios_by_type.happy_path
        )
        .unwrap();
        writeln!(
            md,
            "- Scenarios edge case: {}",
            suite.coverage.scenarios_by_type.edge_case
        )
        .unwrap();
        writeln!(
            md,
            "- Scenarios erreur: {}",
            suite.coverage.scenarios_by_type.error_scenario
        )
        .unwrap();
        writeln!(md, "- Total scenarios: {}", suite.total_scenarios).unwrap();
        writeln!(md).unwrap();

        // Orphan tests
        if !matrix.summary.orphan_tests.is_empty() {
            writeln!(md, "### Tests orphelins (sans exigence associee)").unwrap();
            writeln!(md).unwrap();
            for name in &matrix.summary.orphan_tests {
                writeln!(md, "- {}", name).unwrap();
            }
            writeln!(md).unwrap();
        }

        // Compliance notes
        if !matrix.compliance_notes.is_empty() {
            writeln!(md, "## Notes de conformite").unwrap();
            writeln!(md).unwrap();
            writeln!(md, "| Norme | Section | Statut | Details |").unwrap();
            writeln!(md, "|-------|---------|--------|---------|").unwrap();
            for note in &matrix.compliance_notes {
                writeln!(
                    md,
                    "| {} | {} | {} | {} |",
                    note.standard, note.section, note.status, note.details,
                )
                .unwrap();
            }
            writeln!(md).unwrap();
        }

        md
    }

    /// Ecrit le rapport dans un fichier
    pub async fn write(
        &self,
        spec: &Specification,
        suite: &TestSuite,
        output_dir: &Path,
    ) -> Result<PathBuf, anyhow::Error> {
        tokio::fs::create_dir_all(output_dir).await?;
        let path = output_dir.join("traceability.md");
        let content = self.render(spec, suite);
        tokio::fs::write(&path, &content).await?;
        Ok(path)
    }
}

impl Default for TraceabilityWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::specification::*;
    use crate::domain::test_case::*;
    use crate::domain::user_story::Priority;
    use uuid::Uuid;

    fn make_spec_with_requirements(req_ids: &[&str]) -> Specification {
        let mut spec = Specification::new("Test Spec".into());
        for id in req_ids {
            spec.functional_requirements.push(FunctionalRequirement {
                id: id.to_string(),
                statement: format!("Requirement {id}"),
                priority: Priority::P1,
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
        }
        spec
    }

    fn make_suite_covering(covered: &[&str], total: usize) -> TestSuite {
        let mut feature = Feature::new("Test Feature".into(), "Description".into());
        feature.covered_requirements = covered.iter().map(|s| s.to_string()).collect();
        feature.source_scenario_ids = vec!["US-001".into()];
        // Each scenario directly references the covered FRs via verification_of.
        // Two scenarios are needed so P1 FRs reach FullyCovered (requires >=2).
        feature.scenarios.push(Scenario {
            name: "Test Scenario".into(),
            tags: vec!["@happy_path".into()],
            scenario_type: ScenarioType::HappyPath,
            steps: vec![],
            examples: None,
            test_data_suggestions: vec![],
            verification_of: covered.iter().map(|s| s.to_string()).collect(),
            coverage_technique: None,
        });
        feature.scenarios.push(Scenario {
            name: "Test Scenario 2".into(),
            tags: vec!["@edge_case".into()],
            scenario_type: ScenarioType::EdgeCase,
            steps: vec![],
            examples: None,
            test_data_suggestions: vec![],
            verification_of: covered.iter().map(|s| s.to_string()).collect(),
            coverage_technique: None,
        });

        TestSuite {
            features: vec![feature],
            source_spec_id: Uuid::new_v4(),
            total_scenarios: 2,
            coverage: TestCoverage {
                requirements_covered: covered.iter().map(|s| s.to_string()).collect(),
                requirements_total: total,
                coverage_percentage: (covered.len() as f32 / total as f32) * 100.0,
                scenarios_by_type: ScenarioCounts {
                    happy_path: 1,
                    edge_case: 1,
                    error_scenario: 0,
                    boundary: 0,
                },
            },
        }
    }

    #[test]
    fn test_render_with_covered_requirements() {
        let writer = TraceabilityWriter::new();
        let spec = make_spec_with_requirements(&["FR-001", "FR-002"]);
        let suite = make_suite_covering(&["FR-001", "FR-002"], 2);

        let output = writer.render(&spec, &suite);
        assert!(output.contains("# Matrice de Tracabilite"));
        assert!(output.contains("FR-001"));
        assert!(output.contains("FR-002"));
        assert!(output.contains("Couvert"));
        assert!(output.contains("100%"));
        assert!(!output.contains("**GAP**"));
    }

    #[test]
    fn test_render_with_gap() {
        let writer = TraceabilityWriter::new();
        let spec = make_spec_with_requirements(&["FR-001", "FR-002", "FR-003"]);
        let suite = make_suite_covering(&["FR-001"], 3);

        let output = writer.render(&spec, &suite);
        assert!(output.contains("FR-001"));
        assert!(output.contains("Couvert"));
        assert!(output.contains("FR-002"));
        assert!(output.contains("**GAP**"));
        assert!(output.contains("FR-003"));
    }

    #[test]
    fn test_render_all_gaps() {
        let writer = TraceabilityWriter::new();
        let spec = make_spec_with_requirements(&["FR-001", "FR-002"]);
        let suite = TestSuite {
            features: vec![],
            source_spec_id: Uuid::new_v4(),
            total_scenarios: 0,
            coverage: TestCoverage {
                requirements_covered: vec![],
                requirements_total: 2,
                coverage_percentage: 0.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        };

        let output = writer.render(&spec, &suite);
        assert!(output.contains("**GAP**"));
        assert!(output.contains("0%"));
        assert!(output.contains("Total scenarios: 0"));
    }

    #[tokio::test]
    async fn test_write_to_disk() {
        let writer = TraceabilityWriter::new();
        let spec = make_spec_with_requirements(&["FR-001"]);
        let suite = make_suite_covering(&["FR-001"], 1);

        let dir = tempfile::TempDir::new().unwrap();
        let path = writer.write(&spec, &suite, dir.path()).await.unwrap();

        assert!(path.exists());
        let content = tokio::fs::read_to_string(&path).await.unwrap();
        assert!(content.contains("FR-001"));
        assert!(content.contains("Couvert"));
    }

    #[test]
    fn test_render_full_coverage_snapshot() {
        let writer = TraceabilityWriter::new();
        let mut spec = make_spec_with_requirements(&["FR-001", "FR-002"]);
        spec.created_at = chrono::NaiveDate::from_ymd_opt(2025, 1, 15)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap()
            .and_utc();
        let suite = make_suite_covering(&["FR-001", "FR-002"], 2);
        let output = writer.render(&spec, &suite);
        insta::assert_snapshot!(output);
    }

    #[test]
    fn test_render_with_gaps_snapshot() {
        let writer = TraceabilityWriter::new();
        let mut spec = make_spec_with_requirements(&["FR-001", "FR-002", "FR-003"]);
        spec.created_at = chrono::NaiveDate::from_ymd_opt(2025, 1, 15)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap()
            .and_utc();
        let suite = make_suite_covering(&["FR-001"], 3);
        let output = writer.render(&spec, &suite);
        insta::assert_snapshot!(output);
    }
}
