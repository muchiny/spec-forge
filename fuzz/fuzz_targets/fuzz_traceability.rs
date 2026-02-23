#![no_main]
use libfuzzer_sys::{fuzz_target, Corpus};
use spec_forge::domain::specification::*;
use spec_forge::domain::test_case::*;
use spec_forge::domain::traceability::build_traceability_matrix;
use spec_forge::domain::user_story::Priority;

/// Fuzz la construction de la matrice de tracabilite
fuzz_target!(|data: &[u8]| -> Corpus {
    let Ok(input) = std::str::from_utf8(data) else {
        return Corpus::Reject;
    };

    // Construire une spec avec des FR generes depuis l'input fuzz
    let mut spec = Specification::new("Fuzz Spec".into());
    let lines: Vec<&str> = input.lines().take(20).collect();

    for (i, line) in lines.iter().enumerate() {
        if line.len() < 3 {
            continue;
        }
        spec.functional_requirements.push(FunctionalRequirement {
            id: format!("FR-{:03}", i + 1),
            statement: line.to_string(),
            priority: match i % 3 {
                0 => Priority::P1,
                1 => Priority::P2,
                _ => Priority::P3,
            },
            category: RequirementCategory::Functional,
            testable: true,
            rationale: None,
            source: None,
            verification_method: VerificationMethod::Test,
            risk_level: None,
            parent_requirement: None,
            allocated_to: Vec::new(),
            quality_characteristic: None,
        });
    }

    // Construire un TestSuite minimal avec couverture partielle
    let covered: Vec<String> = spec
        .functional_requirements
        .iter()
        .take(spec.functional_requirements.len() / 2)
        .map(|fr| fr.id.clone())
        .collect();

    let mut feature = Feature::new("Fuzz Feature".into(), "".into());
    feature.covered_requirements = covered.clone();
    for cov in &covered {
        feature.scenarios.push(Scenario {
            name: format!("Test {cov}"),
            tags: vec![],
            scenario_type: ScenarioType::HappyPath,
            steps: vec![],
            examples: None,
            test_data_suggestions: vec![],
            verification_of: vec![cov.clone()],
            coverage_technique: None,
        });
    }

    let suite = TestSuite {
        features: vec![feature],
        source_spec_id: spec.id,
        total_scenarios: covered.len(),
        coverage: TestCoverage {
            requirements_covered: covered,
            requirements_total: spec.functional_requirements.len(),
            coverage_percentage: 0.0,
            scenarios_by_type: ScenarioCounts::default(),
        },
    };

    // Ne doit JAMAIS panic
    let matrix = build_traceability_matrix(&spec, &suite);

    // Invariants basiques
    assert!(
        matrix.entries.len() == spec.functional_requirements.len(),
        "La matrice doit avoir autant d'entrees que de FR"
    );

    Corpus::Keep
});
