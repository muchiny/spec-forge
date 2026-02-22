use spec_forge::adapters::output::gherkin_writer::GherkinWriter;
use spec_forge::adapters::output::markdown_writer::MarkdownWriter;
use spec_forge::adapters::output::traceability_writer::TraceabilityWriter;
use spec_forge::domain::specification::*;
use spec_forge::domain::test_case::*;
use spec_forge::domain::user_story::{Language, Priority};
use uuid::Uuid;

fn make_test_spec() -> Specification {
    let mut spec = Specification::new("Test Spec".into());
    spec.user_scenarios.push(UserScenario {
        id: "US-001".into(),
        title: "Recherche ISBN".into(),
        priority: Priority::P1,
        description: "Recherche par ISBN".into(),
        why_priority: "Critique".into(),
        independent_test: "Saisir un ISBN".into(),
        acceptance_scenarios: vec![AcceptanceScenario {
            given: "catalogue".into(),
            when: "saisie ISBN".into(),
            then: "resultat".into(),
        }],
        source_story_id: Uuid::new_v4(),
    });
    spec.functional_requirements.push(FunctionalRequirement {
        id: "FR-001".into(),
        statement: "Le systeme DOIT accepter ISBN-10 et ISBN-13".into(),
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
    spec
}

fn make_test_feature() -> Feature {
    let mut feature = Feature::new("Recherche ISBN".into(), "Description".into());
    feature.tags = vec!["US-001".into(), "P1".into()];
    feature.covered_requirements = vec!["FR-001".into()];
    feature.source_scenario_ids = vec!["US-001".into()];
    feature.scenarios.push(Scenario {
        name: "Recherche par ISBN valide".into(),
        tags: vec!["happy_path".into()],
        scenario_type: ScenarioType::HappyPath,
        steps: vec![
            Step {
                keyword: StepKeyword::Given,
                text: "Un catalogue de livres".into(),
                doc_string: None,
                data_table: None,
            },
            Step {
                keyword: StepKeyword::When,
                text: "saisie ISBN valide".into(),
                doc_string: None,
                data_table: None,
            },
            Step {
                keyword: StepKeyword::Then,
                text: "le livre est affiche".into(),
                doc_string: None,
                data_table: None,
            },
        ],
        examples: None,
        test_data_suggestions: vec![],
        verification_of: Vec::new(),
        coverage_technique: None,
    });
    feature
}

fn make_test_suite() -> TestSuite {
    TestSuite {
        features: vec![make_test_feature()],
        source_spec_id: Uuid::new_v4(),
        total_scenarios: 1,
        coverage: TestCoverage {
            requirements_covered: vec!["FR-001".into()],
            requirements_total: 1,
            coverage_percentage: 100.0,
            scenarios_by_type: ScenarioCounts {
                happy_path: 1,
                edge_case: 0,
                error_scenario: 0,
                boundary: 0,
            },
        },
    }
}

#[tokio::test]
async fn test_gherkin_writer_to_disk() {
    let writer = GherkinWriter::new(Language::French);
    let feature = make_test_feature();
    let dir = tempfile::TempDir::new().unwrap();

    let path = writer.write_feature(&feature, dir.path()).await.unwrap();
    assert!(path.exists());

    let content = tokio::fs::read_to_string(&path).await.unwrap();
    assert!(content.contains("# language: fr"));
    assert!(content.contains("Fonctionnalité:"));
    assert!(content.contains("Soit"));
}

#[tokio::test]
async fn test_markdown_writer_to_disk() {
    let writer = MarkdownWriter::new();
    let spec = make_test_spec();
    let dir = tempfile::TempDir::new().unwrap();

    let path = writer.write(&spec, dir.path()).await.unwrap();
    assert!(path.exists());

    let content = tokio::fs::read_to_string(&path).await.unwrap();
    assert!(content.contains("Feature Specification"));
    assert!(content.contains("FR-001"));
}

#[tokio::test]
async fn test_traceability_writer_to_disk() {
    let writer = TraceabilityWriter::new();
    let spec = make_test_spec();
    let suite = make_test_suite();
    let dir = tempfile::TempDir::new().unwrap();

    let path = writer.write(&spec, &suite, dir.path()).await.unwrap();
    assert!(path.exists());

    let content = tokio::fs::read_to_string(&path).await.unwrap();
    assert!(content.contains("Matrice de Tracabilite"));
    assert!(content.contains("FR-001"));
    assert!(content.contains("Couvert"));
}
