//! Adapter GherkinWriter - Ecrit les fichiers .feature Gherkin

use std::fmt::Write;
use std::path::{Path, PathBuf};

use crate::domain::test_case::{Feature, Scenario, Step};
use crate::domain::user_story::Language;

/// Ecrit des fichiers .feature Gherkin
pub struct GherkinWriter {
    language: Language,
}

impl GherkinWriter {
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    /// Genere le contenu d'un fichier .feature
    pub fn render_feature(&self, feature: &Feature) -> String {
        let mut content = String::new();

        // Header de langue
        writeln!(content, "# language: {}", self.language.gherkin_code()).unwrap();
        writeln!(content).unwrap();

        // Tags de la feature
        if !feature.tags.is_empty() {
            writeln!(content, "{}", feature.tags.join(" ")).unwrap();
        }

        // Feature
        let feature_keyword = match self.language {
            Language::French => "Fonctionnalité",
            Language::English => "Feature",
        };
        writeln!(content, "{}: {}", feature_keyword, feature.name).unwrap();

        // Description
        if !feature.description.is_empty() {
            for line in feature.description.lines() {
                writeln!(content, "  {}", line).unwrap();
            }
        }
        writeln!(content).unwrap();

        // Background
        if let Some(ref bg) = feature.background {
            let bg_keyword = match self.language {
                Language::French => "Contexte",
                Language::English => "Background",
            };
            writeln!(content, "  {}:", bg_keyword).unwrap();
            for step in &bg.steps {
                writeln!(content, "    {}", self.render_step(step)).unwrap();
            }
            writeln!(content).unwrap();
        }

        // Scenarios
        for scenario in &feature.scenarios {
            self.render_scenario(&mut content, scenario);
            writeln!(content).unwrap();
        }

        content
    }

    fn render_scenario(&self, content: &mut String, scenario: &Scenario) {
        // Tags
        if !scenario.tags.is_empty() {
            writeln!(content, "  {}", scenario.tags.join(" ")).unwrap();
        }

        // Scenario keyword
        let keyword = if scenario.examples.is_some() {
            match self.language {
                Language::French => "Plan du Scénario",
                Language::English => "Scenario Outline",
            }
        } else {
            match self.language {
                Language::French => "Scénario",
                Language::English => "Scenario",
            }
        };

        writeln!(content, "  {}: {}", keyword, scenario.name).unwrap();

        // Steps
        for step in &scenario.steps {
            writeln!(content, "    {}", self.render_step(step)).unwrap();

            // Doc string
            if let Some(ref doc) = step.doc_string {
                writeln!(content, "      \"\"\"").unwrap();
                for line in doc.lines() {
                    writeln!(content, "      {}", line).unwrap();
                }
                writeln!(content, "      \"\"\"").unwrap();
            }

            // Data table
            if let Some(ref table) = step.data_table {
                for row in table {
                    let formatted = row.iter().map(|c| format!(" {} ", c)).collect::<Vec<_>>();
                    writeln!(content, "      |{}|", formatted.join("|")).unwrap();
                }
            }
        }

        // Examples
        if let Some(ref examples) = scenario.examples {
            writeln!(content).unwrap();
            let examples_keyword = match self.language {
                Language::French => "Exemples",
                Language::English => "Examples",
            };
            writeln!(content, "    {}:", examples_keyword).unwrap();

            // Headers
            let headers = examples
                .headers
                .iter()
                .map(|h| format!(" {} ", h))
                .collect::<Vec<_>>();
            writeln!(content, "      |{}|", headers.join("|")).unwrap();

            // Rows
            for row in &examples.rows {
                let cells = row.iter().map(|c| format!(" {} ", c)).collect::<Vec<_>>();
                writeln!(content, "      |{}|", cells.join("|")).unwrap();
            }
        }
    }

    fn render_step(&self, step: &Step) -> String {
        let keyword = match self.language {
            Language::French => step.keyword.to_french(),
            Language::English => step.keyword.to_english(),
        };
        format!("{} {}", keyword, step.text)
    }

    /// Ecrit un fichier .feature
    pub async fn write_feature(
        &self,
        feature: &Feature,
        output_dir: &Path,
    ) -> Result<PathBuf, anyhow::Error> {
        tokio::fs::create_dir_all(output_dir).await?;

        let filename = format!(
            "{}.feature",
            feature
                .name
                .to_lowercase()
                .replace(' ', "_")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '_')
                .collect::<String>()
        );
        let path = output_dir.join(&filename);

        let content = self.render_feature(feature);
        tokio::fs::write(&path, &content).await?;

        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::test_case::*;
    use uuid::Uuid;

    fn make_test_feature() -> Feature {
        Feature {
            id: Uuid::new_v4(),
            name: "Recherche par ISBN".into(),
            description: "En tant que bibliothecaire\nJe veux rechercher par ISBN".into(),
            tags: vec!["@US-001".into(), "@P1".into()],
            background: Some(Background {
                steps: vec![Step {
                    keyword: StepKeyword::Given,
                    text: "un catalogue contenant 1000 ouvrages".into(),
                    doc_string: None,
                    data_table: None,
                }],
            }),
            scenarios: vec![
                Scenario {
                    name: "Recherche avec ISBN-13 valide".into(),
                    tags: vec!["@happy_path".into(), "@FR-001".into()],
                    scenario_type: ScenarioType::HappyPath,
                    steps: vec![
                        Step {
                            keyword: StepKeyword::Given,
                            text: "un ouvrage avec l'ISBN \"978-2-07-036024-1\" dans le catalogue"
                                .into(),
                            doc_string: None,
                            data_table: None,
                        },
                        Step {
                            keyword: StepKeyword::When,
                            text: "je saisis \"978-2-07-036024-1\" dans le champ de recherche"
                                .into(),
                            doc_string: None,
                            data_table: None,
                        },
                        Step {
                            keyword: StepKeyword::Then,
                            text: "l'ouvrage \"Le Petit Prince\" est affiche".into(),
                            doc_string: None,
                            data_table: None,
                        },
                    ],
                    examples: None,
                    test_data_suggestions: vec![],
                    verification_of: Vec::new(),
                    coverage_technique: None,
                },
                Scenario {
                    name: "Recherche avec formats ISBN varies".into(),
                    tags: vec!["@edge_case".into()],
                    scenario_type: ScenarioType::EdgeCase,
                    steps: vec![
                        Step {
                            keyword: StepKeyword::Given,
                            text: "un ouvrage avec l'ISBN \"<isbn>\" dans le catalogue".into(),
                            doc_string: None,
                            data_table: None,
                        },
                        Step {
                            keyword: StepKeyword::When,
                            text: "je saisis \"<isbn>\" dans le champ de recherche".into(),
                            doc_string: None,
                            data_table: None,
                        },
                        Step {
                            keyword: StepKeyword::Then,
                            text: "l'ouvrage est trouve".into(),
                            doc_string: None,
                            data_table: None,
                        },
                    ],
                    examples: Some(Examples {
                        headers: vec!["isbn".into(), "format".into()],
                        rows: vec![
                            vec!["978-2-07-036024-1".into(), "ISBN-13".into()],
                            vec!["2-07-036024-8".into(), "ISBN-10".into()],
                        ],
                    }),
                    test_data_suggestions: vec![],
                    verification_of: Vec::new(),
                    coverage_technique: None,
                },
            ],
            source_scenario_ids: vec!["US-001".into()],
            covered_requirements: vec!["FR-001".into()],
            test_level: Default::default(),
        }
    }

    #[test]
    fn test_render_feature_french() {
        let writer = GherkinWriter::new(Language::French);
        let feature = make_test_feature();
        let content = writer.render_feature(&feature);

        assert!(content.contains("# language: fr"));
        assert!(content.contains("Fonctionnalité: Recherche par ISBN"));
        assert!(content.contains("@US-001"));
        assert!(content.contains("Contexte:"));
        assert!(content.contains("Soit un catalogue"));
        assert!(content.contains("Scénario: Recherche avec ISBN-13 valide"));
        assert!(content.contains("Plan du Scénario: Recherche avec formats ISBN varies"));
        assert!(content.contains("Exemples:"));
        assert!(content.contains("isbn"));
    }

    #[test]
    fn test_render_feature_english() {
        let writer = GherkinWriter::new(Language::English);
        let feature = make_test_feature();
        let content = writer.render_feature(&feature);

        assert!(content.contains("# language: en"));
        assert!(content.contains("Feature: Recherche par ISBN"));
        assert!(content.contains("Background:"));
        assert!(content.contains("Given"));
        assert!(content.contains("Scenario Outline:"));
        assert!(content.contains("Examples:"));
    }

    #[test]
    fn test_render_feature_french_snapshot() {
        let writer = GherkinWriter::new(Language::French);
        let feature = make_test_feature();
        let content = writer.render_feature(&feature);
        insta::assert_snapshot!(content);
    }

    #[test]
    fn test_render_feature_english_snapshot() {
        let writer = GherkinWriter::new(Language::English);
        let feature = make_test_feature();
        let content = writer.render_feature(&feature);
        insta::assert_snapshot!(content);
    }

    #[test]
    fn test_render_simple_scenario_snapshot() {
        let writer = GherkinWriter::new(Language::French);
        let mut feature = Feature::new("Login utilisateur".into(), "Authentification".into());
        feature.tags = vec!["@auth".into()];
        feature.scenarios.push(Scenario {
            name: "Connexion reussie".into(),
            tags: vec!["@happy_path".into()],
            scenario_type: ScenarioType::HappyPath,
            steps: vec![
                Step {
                    keyword: StepKeyword::Given,
                    text: "un utilisateur enregistre".into(),
                    doc_string: None,
                    data_table: None,
                },
                Step {
                    keyword: StepKeyword::When,
                    text: "il saisit ses identifiants".into(),
                    doc_string: None,
                    data_table: None,
                },
                Step {
                    keyword: StepKeyword::Then,
                    text: "il est connecte".into(),
                    doc_string: None,
                    data_table: None,
                },
            ],
            examples: None,
            test_data_suggestions: vec![],
            verification_of: Vec::new(),
            coverage_technique: None,
        });
        let content = writer.render_feature(&feature);
        insta::assert_snapshot!(content);
    }
}
