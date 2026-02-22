//! Adapter MarkdownWriter - Ecrit les specifications en Markdown (format spec-kit)

use std::fmt::Write;
use std::path::{Path, PathBuf};

use crate::domain::specification::Specification;

/// Ecrit une specification raffinee au format Markdown spec-kit
pub struct MarkdownWriter;

impl MarkdownWriter {
    pub fn new() -> Self {
        Self
    }

    /// Genere le contenu Markdown de la specification
    pub fn render_specification(&self, spec: &Specification) -> String {
        let mut md = String::new();

        // Header
        writeln!(md, "# Feature Specification: {}", spec.title).unwrap();
        writeln!(md).unwrap();
        writeln!(md, "**Created**: {}", spec.created_at.format("%Y-%m-%d")).unwrap();
        writeln!(md, "**Status**: {}", spec.status).unwrap();
        writeln!(md, "**Version**: {}", spec.version).unwrap();
        writeln!(md, "**Tool**: spec-forge v{}", spec.tool_version).unwrap();
        if let Some(ref compliance) = spec.compliance_profile {
            writeln!(md, "**Compliance**: {:?}", compliance).unwrap();
        }
        writeln!(md).unwrap();

        // User Scenarios
        writeln!(md, "## User Scenarios & Testing").unwrap();
        writeln!(md).unwrap();

        for scenario in &spec.user_scenarios {
            writeln!(
                md,
                "### {} - {} (Priority: {})",
                scenario.id, scenario.title, scenario.priority
            )
            .unwrap();
            writeln!(md).unwrap();
            writeln!(md, "{}", scenario.description).unwrap();
            writeln!(md).unwrap();
            writeln!(md, "**Why this priority**: {}", scenario.why_priority).unwrap();
            writeln!(md).unwrap();
            writeln!(md, "**Independent Test**: {}", scenario.independent_test).unwrap();
            writeln!(md).unwrap();
            writeln!(md, "**Acceptance Scenarios**:").unwrap();
            writeln!(md).unwrap();

            for (i, ac) in scenario.acceptance_scenarios.iter().enumerate() {
                writeln!(
                    md,
                    "{}. **Given** {}, **When** {}, **Then** {}",
                    i + 1,
                    ac.given,
                    ac.when,
                    ac.then
                )
                .unwrap();
            }
            writeln!(md).unwrap();
            writeln!(md, "---").unwrap();
            writeln!(md).unwrap();
        }

        // Edge Cases
        if !spec.edge_cases.is_empty() {
            writeln!(md, "### Edge Cases").unwrap();
            writeln!(md).unwrap();
            for ec in &spec.edge_cases {
                write!(md, "- {}", ec.description).unwrap();
                if let Some(ref related) = ec.related_scenario {
                    write!(md, " (lie a: {})", related).unwrap();
                }
                writeln!(md).unwrap();
            }
            writeln!(md).unwrap();
        }

        // Functional Requirements
        writeln!(md, "## Requirements").unwrap();
        writeln!(md).unwrap();
        writeln!(md, "### Functional Requirements").unwrap();
        writeln!(md).unwrap();

        writeln!(
            md,
            "| ID | Enonce | Priorite | Categorie | Verification | Risque |"
        )
        .unwrap();
        writeln!(
            md,
            "|---|--------|----------|-----------|-------------|--------|"
        )
        .unwrap();
        for fr in &spec.functional_requirements {
            writeln!(
                md,
                "| {} | {} | {} | {} | {} | {} |",
                fr.id,
                fr.statement,
                fr.priority,
                fr.category,
                fr.verification_method,
                fr.risk_level
                    .as_ref()
                    .map(|r| r.to_string())
                    .unwrap_or_else(|| "-".into()),
            )
            .unwrap();
        }
        writeln!(md).unwrap();

        // FR details (rationale, source) if present
        let has_details = spec.functional_requirements.iter().any(|fr| {
            fr.rationale.is_some() || fr.source.is_some() || fr.quality_characteristic.is_some()
        });
        if has_details {
            writeln!(md, "#### Details des exigences").unwrap();
            writeln!(md).unwrap();
            for fr in &spec.functional_requirements {
                let mut details = Vec::new();
                if let Some(ref r) = fr.rationale {
                    details.push(format!("Justification: {}", r));
                }
                if let Some(ref s) = fr.source {
                    details.push(format!("Source: {}", s));
                }
                if let Some(ref qc) = fr.quality_characteristic {
                    details.push(format!("Qualite ISO 25010: {}", qc));
                }
                if !details.is_empty() {
                    writeln!(md, "- **{}**: {}", fr.id, details.join(" | ")).unwrap();
                }
            }
            writeln!(md).unwrap();
        }

        // Key Entities
        if !spec.key_entities.is_empty() {
            writeln!(md, "### Key Entities").unwrap();
            writeln!(md).unwrap();
            for entity in &spec.key_entities {
                writeln!(md, "- **{}**: {}", entity.name, entity.description).unwrap();
                for attr in &entity.attributes {
                    writeln!(md, "  - {}", attr).unwrap();
                }
            }
            writeln!(md).unwrap();
        }

        // Success Criteria
        writeln!(md, "## Success Criteria").unwrap();
        writeln!(md).unwrap();
        writeln!(md, "### Measurable Outcomes").unwrap();
        writeln!(md).unwrap();

        for sc in &spec.success_criteria {
            writeln!(
                md,
                "- **{}**: {} (Metrique: {})",
                sc.id, sc.description, sc.measurable_metric
            )
            .unwrap();
        }
        writeln!(md).unwrap();

        // Clarifications
        if !spec.clarifications_needed.is_empty() {
            writeln!(md, "## Clarifications").unwrap();
            writeln!(md).unwrap();
            for cl in &spec.clarifications_needed {
                if cl.resolved {
                    writeln!(
                        md,
                        "- **Q**: {} -> **A**: {}",
                        cl.question,
                        cl.answer.as_deref().unwrap_or("N/A")
                    )
                    .unwrap();
                } else {
                    writeln!(md, "- **[NEEDS CLARIFICATION]**: {}", cl.question).unwrap();
                    writeln!(md, "  - Contexte: {}", cl.context).unwrap();
                    writeln!(md, "  - Impact: {}", cl.impact).unwrap();
                    if !cl.suggested_options.is_empty() {
                        writeln!(md, "  - Options: {}", cl.suggested_options.join(", ")).unwrap();
                    }
                }
            }
            writeln!(md).unwrap();
        }

        // Validation
        if let Some(ref validation) = spec.validation {
            writeln!(md, "## Validation").unwrap();
            writeln!(md).unwrap();
            writeln!(
                md,
                "- Completude: {:.0}%",
                validation.completeness_score * 100.0
            )
            .unwrap();
            writeln!(md, "- Clarte: {:.0}%", validation.clarity_score * 100.0).unwrap();
            writeln!(
                md,
                "- Testabilite: {:.0}%",
                validation.testability_score * 100.0
            )
            .unwrap();
            writeln!(md).unwrap();
        }

        md
    }

    /// Ecrit la specification dans un fichier
    pub async fn write(
        &self,
        spec: &Specification,
        output_dir: &Path,
    ) -> Result<PathBuf, anyhow::Error> {
        tokio::fs::create_dir_all(output_dir).await?;

        let filename = format!(
            "spec-{}.md",
            spec.title
                .to_lowercase()
                .replace(' ', "-")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect::<String>()
        );
        let path = output_dir.join(&filename);

        let content = self.render_specification(spec);
        tokio::fs::write(&path, &content).await?;

        Ok(path)
    }
}

impl Default for MarkdownWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::specification::*;
    use crate::domain::user_story::Priority;
    use uuid::Uuid;

    #[test]
    fn test_render_specification() {
        let mut spec = Specification::new("Recherche Bibliotheque".into());
        spec.user_scenarios.push(UserScenario {
            id: "US-001".into(),
            title: "Recherche ISBN".into(),
            priority: Priority::P1,
            description: "Rechercher un livre par ISBN".into(),
            why_priority: "Fonctionnalite critique".into(),
            independent_test: "Saisir un ISBN et verifier le resultat".into(),
            acceptance_scenarios: vec![AcceptanceScenario {
                given: "un catalogue contenant des livres".into(),
                when: "je saisis un ISBN valide".into(),
                then: "le livre correspondant s'affiche".into(),
            }],
            source_story_id: Uuid::new_v4(),
        });
        spec.functional_requirements.push(FunctionalRequirement {
            id: "FR-001".into(),
            statement: "Le systeme DOIT permettre la recherche par ISBN".into(),
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
        spec.success_criteria.push(SuccessCriterion {
            id: "SC-001".into(),
            description: "Recherche rapide".into(),
            measurable_metric: "Temps de reponse < 2s".into(),
        });

        let writer = MarkdownWriter::new();
        let md = writer.render_specification(&spec);

        assert!(md.contains("# Feature Specification: Recherche Bibliotheque"));
        assert!(md.contains("US-001"));
        assert!(md.contains("FR-001"));
        assert!(md.contains("SC-001"));
        assert!(md.contains("Given"));
    }

    #[test]
    fn test_render_specification_snapshot() {
        let mut spec = Specification::new("Recherche Bibliotheque".into());
        // Override created_at for deterministic snapshot
        spec.created_at = chrono::NaiveDate::from_ymd_opt(2025, 1, 15)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap()
            .and_utc();

        spec.user_scenarios.push(UserScenario {
            id: "US-001".into(),
            title: "Recherche ISBN".into(),
            priority: Priority::P1,
            description: "Rechercher un livre par ISBN".into(),
            why_priority: "Fonctionnalite critique".into(),
            independent_test: "Saisir un ISBN et verifier le resultat".into(),
            acceptance_scenarios: vec![AcceptanceScenario {
                given: "un catalogue contenant des livres".into(),
                when: "je saisis un ISBN valide".into(),
                then: "le livre correspondant s'affiche".into(),
            }],
            source_story_id: Uuid::nil(),
        });
        spec.functional_requirements.push(FunctionalRequirement {
            id: "FR-001".into(),
            statement: "Le systeme DOIT permettre la recherche par ISBN".into(),
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
        spec.success_criteria.push(SuccessCriterion {
            id: "SC-001".into(),
            description: "Recherche rapide".into(),
            measurable_metric: "Temps de reponse < 2s".into(),
        });

        let writer = MarkdownWriter::new();
        let md = writer.render_specification(&spec);
        insta::assert_snapshot!(md);
    }

    #[test]
    fn test_render_with_clarifications_snapshot() {
        let mut spec = Specification::new("Spec avec clarifications".into());
        spec.created_at = chrono::NaiveDate::from_ymd_opt(2025, 1, 15)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap()
            .and_utc();

        spec.clarifications_needed.push(Clarification {
            question: "Quel format d'ISBN est prioritaire ?".into(),
            context: "ISBN-10 est obsolete depuis 2007".into(),
            impact: "Affecte la validation en entree".into(),
            suggested_options: vec!["ISBN-13 uniquement".into(), "Les deux formats".into()],
            resolved: false,
            answer: None,
        });
        spec.clarifications_needed.push(Clarification {
            question: "Faut-il supporter les tirets ?".into(),
            context: "Format avec ou sans tirets".into(),
            impact: "Parsing de l'input".into(),
            suggested_options: vec![],
            resolved: true,
            answer: Some("Oui, les deux formats".into()),
        });

        let writer = MarkdownWriter::new();
        let md = writer.render_specification(&spec);
        insta::assert_snapshot!(md);
    }
}
