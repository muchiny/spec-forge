//! Modele de domaine - Cas de test Gherkin/BDD
//!
//! Represente les fichiers .feature generes a partir des specifications.
//!
//! Conformite : ISO/IEC/IEEE 29119-3/4, ISO/IEC 25010:2023

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Enums ISO 29119 — niveaux de test et techniques de couverture
// ---------------------------------------------------------------------------

/// Niveau de test (ISO/IEC/IEEE 29119-1)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TestLevel {
    Unit,
    Integration,
    System,
    #[default]
    Acceptance,
}

impl std::fmt::Display for TestLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestLevel::Unit => write!(f, "Unit"),
            TestLevel::Integration => write!(f, "Integration"),
            TestLevel::System => write!(f, "System"),
            TestLevel::Acceptance => write!(f, "Acceptance"),
        }
    }
}

/// Technique de couverture de test (ISO/IEC/IEEE 29119-4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CoverageTechnique {
    EquivalencePartitioning,
    BoundaryValueAnalysis,
    DecisionTable,
    StateTransition,
    ErrorGuessing,
}

impl std::fmt::Display for CoverageTechnique {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoverageTechnique::EquivalencePartitioning => write!(f, "EP"),
            CoverageTechnique::BoundaryValueAnalysis => write!(f, "BVA"),
            CoverageTechnique::DecisionTable => write!(f, "DT"),
            CoverageTechnique::StateTransition => write!(f, "ST"),
            CoverageTechnique::ErrorGuessing => write!(f, "EG"),
        }
    }
}

/// Feature Gherkin (un fichier .feature)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    /// Identifiant unique
    pub id: Uuid,

    /// Nom de la feature
    pub name: String,

    /// Description
    pub description: String,

    /// Tags (@US-001, @P1, etc.)
    #[serde(default)]
    pub tags: Vec<String>,

    /// Background (contexte partage)
    #[serde(default)]
    pub background: Option<Background>,

    /// Scenarios de test
    pub scenarios: Vec<Scenario>,

    /// Tracabilite : IDs des scenarios spec source
    #[serde(default)]
    pub source_scenario_ids: Vec<String>,

    /// Tracabilite : IDs des FR couverts
    #[serde(default)]
    pub covered_requirements: Vec<String>,

    /// Niveau de test — ISO 29119-1
    #[serde(default)]
    pub test_level: TestLevel,
}

impl Feature {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            tags: Vec::new(),
            background: None,
            scenarios: Vec::new(),
            source_scenario_ids: Vec::new(),
            covered_requirements: Vec::new(),
            test_level: TestLevel::default(),
        }
    }
}

/// Section Background Gherkin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Background {
    pub steps: Vec<Step>,
}

/// Scenario Gherkin (ou Scenario Outline)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    /// Nom du scenario
    pub name: String,

    /// Tags (@happy_path, @edge_case, @US-001, etc.)
    #[serde(default)]
    pub tags: Vec<String>,

    /// Type de scenario
    pub scenario_type: ScenarioType,

    /// Etapes (Given/When/Then)
    pub steps: Vec<Step>,

    /// Table d'exemples (pour Scenario Outline)
    #[serde(default)]
    pub examples: Option<Examples>,

    /// Suggestions de donnees de test
    #[serde(default)]
    pub test_data_suggestions: Vec<String>,

    /// FR-IDs que ce scenario verifie (tracabilite inverse)
    #[serde(default)]
    pub verification_of: Vec<String>,

    /// Technique de couverture — ISO 29119-4
    #[serde(default)]
    pub coverage_technique: Option<CoverageTechnique>,
}

/// Type de scenario
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScenarioType {
    HappyPath,
    EdgeCase,
    ErrorScenario,
    BoundaryCondition,
}

impl std::fmt::Display for ScenarioType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScenarioType::HappyPath => write!(f, "happy_path"),
            ScenarioType::EdgeCase => write!(f, "edge_case"),
            ScenarioType::ErrorScenario => write!(f, "error"),
            ScenarioType::BoundaryCondition => write!(f, "boundary"),
        }
    }
}

/// Etape Gherkin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    /// Mot-cle (Given, When, Then, And, But)
    pub keyword: StepKeyword,

    /// Texte de l'etape
    pub text: String,

    /// Doc string optionnel
    #[serde(default)]
    pub doc_string: Option<String>,

    /// Table de donnees optionnelle
    #[serde(default)]
    pub data_table: Option<Vec<Vec<String>>>,
}

/// Mot-cle Gherkin
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepKeyword {
    Given,
    When,
    Then,
    And,
    But,
}

impl StepKeyword {
    /// Mot-cle en francais pour Gherkin
    pub fn to_french(self) -> &'static str {
        match self {
            StepKeyword::Given => "Soit",
            StepKeyword::When => "Quand",
            StepKeyword::Then => "Alors",
            StepKeyword::And => "Et",
            StepKeyword::But => "Mais",
        }
    }

    /// Mot-cle en anglais pour Gherkin
    pub fn to_english(self) -> &'static str {
        match self {
            StepKeyword::Given => "Given",
            StepKeyword::When => "When",
            StepKeyword::Then => "Then",
            StepKeyword::And => "And",
            StepKeyword::But => "But",
        }
    }
}

/// Table d'exemples pour Scenario Outline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Examples {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

/// Collection de features pour une specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    /// Features generees
    pub features: Vec<Feature>,

    /// ID de la specification source
    pub source_spec_id: Uuid,

    /// Nombre total de scenarios
    pub total_scenarios: usize,

    /// Couverture de test
    pub coverage: TestCoverage,
}

/// Metriques de couverture de test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCoverage {
    /// IDs des requirements couverts
    pub requirements_covered: Vec<String>,

    /// Nombre total de requirements
    pub requirements_total: usize,

    /// Pourcentage de couverture
    pub coverage_percentage: f32,

    /// Compteurs par type de scenario
    pub scenarios_by_type: ScenarioCounts,
}

/// Compteurs de scenarios par type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScenarioCounts {
    pub happy_path: usize,
    pub edge_case: usize,
    pub error_scenario: usize,
    pub boundary: usize,
}

impl TestSuite {
    /// Calcule les metriques de couverture
    pub fn compute_coverage(&mut self, total_requirements: usize) {
        let mut covered = Vec::new();
        let mut counts = ScenarioCounts::default();

        for feature in &self.features {
            covered.extend(feature.covered_requirements.clone());
            for scenario in &feature.scenarios {
                // Inclure les FR references par verification_of au niveau scenario
                covered.extend(scenario.verification_of.clone());
                match scenario.scenario_type {
                    ScenarioType::HappyPath => counts.happy_path += 1,
                    ScenarioType::EdgeCase => counts.edge_case += 1,
                    ScenarioType::ErrorScenario => counts.error_scenario += 1,
                    ScenarioType::BoundaryCondition => counts.boundary += 1,
                }
            }
        }

        covered.sort();
        covered.dedup();

        let percentage = if total_requirements > 0 {
            (covered.len() as f32 / total_requirements as f32) * 100.0
        } else {
            0.0
        };

        self.total_scenarios = self.features.iter().map(|f| f.scenarios.len()).sum();
        self.coverage = TestCoverage {
            requirements_covered: covered,
            requirements_total: total_requirements,
            coverage_percentage: percentage,
            scenarios_by_type: counts,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_creation() {
        let feature = Feature::new("Recherche".into(), "Recherche par ISBN".into());
        assert!(!feature.id.is_nil());
        assert_eq!(feature.name, "Recherche");
        assert!(feature.scenarios.is_empty());
    }

    #[test]
    fn test_step_keyword_french() {
        assert_eq!(StepKeyword::Given.to_french(), "Soit");
        assert_eq!(StepKeyword::When.to_french(), "Quand");
        assert_eq!(StepKeyword::Then.to_french(), "Alors");
        assert_eq!(StepKeyword::And.to_french(), "Et");
        assert_eq!(StepKeyword::But.to_french(), "Mais");
    }

    #[test]
    fn test_scenario_type_display() {
        assert_eq!(ScenarioType::HappyPath.to_string(), "happy_path");
        assert_eq!(ScenarioType::ErrorScenario.to_string(), "error");
    }

    #[test]
    fn test_coverage_computation() {
        let mut suite = TestSuite {
            features: vec![Feature {
                id: Uuid::new_v4(),
                name: "Test".into(),
                description: "".into(),
                tags: vec![],
                background: None,
                scenarios: vec![
                    Scenario {
                        name: "Happy".into(),
                        tags: vec![],
                        scenario_type: ScenarioType::HappyPath,
                        steps: vec![],
                        examples: None,
                        test_data_suggestions: vec![],
                        verification_of: Vec::new(),
                        coverage_technique: None,
                    },
                    Scenario {
                        name: "Error".into(),
                        tags: vec![],
                        scenario_type: ScenarioType::ErrorScenario,
                        steps: vec![],
                        examples: None,
                        test_data_suggestions: vec![],
                        verification_of: Vec::new(),
                        coverage_technique: None,
                    },
                ],
                source_scenario_ids: vec![],
                covered_requirements: vec!["FR-001".into(), "FR-002".into()],
                test_level: TestLevel::default(),
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

        suite.compute_coverage(4);
        assert_eq!(suite.total_scenarios, 2);
        assert_eq!(suite.coverage.requirements_covered.len(), 2);
        assert!((suite.coverage.coverage_percentage - 50.0).abs() < 0.1);
        assert_eq!(suite.coverage.scenarios_by_type.happy_path, 1);
        assert_eq!(suite.coverage.scenarios_by_type.error_scenario, 1);
    }
}
