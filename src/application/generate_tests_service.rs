//! Service de generation de tests Gherkin depuis les Specifications

use std::sync::Arc;
use tracing::{debug, info, warn};

use crate::domain::errors::GenerationError;
use crate::domain::specification::Specification;
use crate::domain::test_case::*;
use crate::domain::user_story::Language;
use crate::ports::llm_service::LlmService;
use crate::ports::template_engine::TemplateEngine;

/// Service de generation Spec -> Tests Gherkin
pub struct GenerateTestsService {
    llm: Arc<dyn LlmService>,
    templates: Arc<dyn TemplateEngine>,
    language: Language,
    max_retries: usize,
    token_budget: usize,
}

/// Schema JSON de sortie du LLM pour la generation de tests
#[derive(Debug, serde::Deserialize)]
struct LlmTestOutput {
    features: Vec<LlmFeature>,
}

#[derive(Debug, serde::Deserialize)]
struct LlmFeature {
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    background: Option<LlmBackground>,
    #[serde(default)]
    scenarios: Vec<LlmScenario>,
    #[serde(default)]
    source_scenario_ids: Vec<String>,
    #[serde(default)]
    covered_requirements: Vec<String>,
    // ISO 29119 enrichment (optional from LLM)
    #[serde(default)]
    test_level: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct LlmBackground {
    steps: Vec<LlmStep>,
}

#[derive(Debug, serde::Deserialize)]
struct LlmScenario {
    name: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default = "default_scenario_type")]
    scenario_type: String,
    steps: Vec<LlmStep>,
    #[serde(default)]
    examples: Option<LlmExamples>,
    #[serde(default)]
    test_data_suggestions: Vec<String>,
    // ISO 29119 enrichment (optional from LLM)
    #[serde(default)]
    verification_of: Vec<String>,
    #[serde(default)]
    coverage_technique: Option<String>,
}

fn default_scenario_type() -> String {
    "HappyPath".to_string()
}

#[derive(Debug, serde::Deserialize)]
struct LlmStep {
    keyword: String,
    text: String,
    #[serde(default)]
    doc_string: Option<String>,
    #[serde(default)]
    data_table: Option<Vec<Vec<String>>>,
}

#[derive(Debug, serde::Deserialize)]
struct LlmExamples {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

/// Estime le nombre de tokens d'une spec pour le prompt de generation de tests
fn estimate_spec_tokens(spec: &Specification) -> usize {
    let mut len = 0usize;
    for us in &spec.user_scenarios {
        len += us.title.len() + us.description.len();
        for ac in &us.acceptance_scenarios {
            len += ac.given.len() + ac.when.len() + ac.then.len();
        }
    }
    for fr in &spec.functional_requirements {
        len += fr.id.len() + fr.statement.len();
    }
    for ec in &spec.edge_cases {
        len += ec.description.len();
    }
    len / 4
}

/// Construit des sous-specs pour le batching de generation de tests.
///
/// Chaque batch contient un sous-ensemble de scenarios utilisateur, les edge cases
/// associes, et TOUTES les exigences fonctionnelles (pour que le LLM ait le contexte
/// complet des exigences a couvrir dans chaque batch).
fn build_spec_batches(spec: &Specification, token_budget: usize) -> Vec<Specification> {
    let total_tokens = estimate_spec_tokens(spec);
    if total_tokens <= token_budget || spec.user_scenarios.len() <= 1 {
        return vec![spec.clone()];
    }

    // Split par scenario utilisateur, mais garder toutes les FRs dans chaque batch
    let mut batches = Vec::new();
    let mut current_scenarios = Vec::new();
    let mut current_edges = Vec::new();

    // Tokens de base pour les FRs (incluses dans chaque batch)
    let fr_base_tokens: usize = spec
        .functional_requirements
        .iter()
        .map(|fr| (fr.id.len() + fr.statement.len()) / 4)
        .sum();

    // Demarrer avec le cout des FRs (elles sont incluses dans chaque batch)
    let mut current_tokens = fr_base_tokens;

    // Edge cases non lies a un scenario specifique : inclure dans TOUS les batches
    let unrelated_edges: Vec<_> = spec
        .edge_cases
        .iter()
        .filter(|ec| ec.related_scenario.is_none())
        .cloned()
        .collect();

    for us in &spec.user_scenarios {
        let mut tokens = (us.title.len() + us.description.len()) / 4;
        for ac in &us.acceptance_scenarios {
            tokens += (ac.given.len() + ac.when.len() + ac.then.len()) / 4;
        }

        if !current_scenarios.is_empty() && current_tokens + tokens > token_budget {
            // Flush current batch
            let mut batch_spec = spec.clone();
            batch_spec.user_scenarios = std::mem::take(&mut current_scenarios);
            // Inclure les edge cases lies + les edge cases generiques
            let mut batch_edges = std::mem::take(&mut current_edges);
            batch_edges.extend(unrelated_edges.clone());
            batch_spec.edge_cases = batch_edges;
            batches.push(batch_spec);
            current_tokens = fr_base_tokens; // Reset avec le cout de base des FRs
        }

        current_scenarios.push(us.clone());
        current_edges.extend(
            spec.edge_cases
                .iter()
                .filter(|ec| ec.related_scenario.as_deref() == Some(&us.id))
                .cloned(),
        );
        current_tokens += tokens;
    }

    // Dernier batch
    if !current_scenarios.is_empty() {
        let mut batch_spec = spec.clone();
        batch_spec.user_scenarios = current_scenarios;
        current_edges.extend(unrelated_edges);
        batch_spec.edge_cases = current_edges;
        batches.push(batch_spec);
    }

    batches
}

impl GenerateTestsService {
    pub fn new(
        llm: Arc<dyn LlmService>,
        templates: Arc<dyn TemplateEngine>,
        language: Language,
        max_retries: usize,
    ) -> Self {
        Self {
            llm,
            templates,
            language,
            max_retries,
            token_budget: 6000,
        }
    }

    pub fn with_token_budget(mut self, token_budget: usize) -> Self {
        self.token_budget = token_budget;
        self
    }

    /// Genere les tests Gherkin depuis une specification
    pub async fn generate(&self, spec: &Specification) -> Result<TestSuite, GenerationError> {
        let batches = build_spec_batches(spec, self.token_budget);
        let num_batches = batches.len();

        info!(
            scenarios = spec.user_scenarios.len(),
            requirements = spec.functional_requirements.len(),
            batches = num_batches,
            "Debut de la generation de tests Gherkin"
        );

        // File d'attente avec re-decoupage adaptatif si troncature
        let mut queue: std::collections::VecDeque<Specification> = if num_batches <= 1 {
            // Single batch → mettre le spec original dans la queue
            std::collections::VecDeque::from(vec![spec.clone()])
        } else {
            batches.into()
        };
        let mut suites = Vec::new();
        let mut batch_counter = 0usize;

        while let Some(batch_spec) = queue.pop_front() {
            batch_counter += 1;
            info!(
                batch = batch_counter,
                scenarios = batch_spec.user_scenarios.len(),
                "Batch de generation"
            );

            match self.generate_single(&batch_spec).await {
                Ok(suite) => suites.push(suite),
                Err(GenerationError::OutputTruncated { .. })
                    if batch_spec.user_scenarios.len() > 1 =>
                {
                    let mid = batch_spec.user_scenarios.len() / 2;
                    warn!(
                        original = batch_spec.user_scenarios.len(),
                        "Troncature generation — re-decoupage"
                    );
                    // Construire les deux sous-specs
                    let mut left = batch_spec.clone();
                    let right_scenarios = left.user_scenarios.split_off(mid);
                    let mut right = spec.clone();
                    right.user_scenarios = right_scenarios;
                    // Re-attribuer les edge_cases
                    left.edge_cases = spec
                        .edge_cases
                        .iter()
                        .filter(|ec| {
                            ec.related_scenario.is_none()
                                || left
                                    .user_scenarios
                                    .iter()
                                    .any(|us| ec.related_scenario.as_deref() == Some(&us.id))
                        })
                        .cloned()
                        .collect();
                    right.edge_cases = spec
                        .edge_cases
                        .iter()
                        .filter(|ec| {
                            ec.related_scenario.is_none()
                                || right
                                    .user_scenarios
                                    .iter()
                                    .any(|us| ec.related_scenario.as_deref() == Some(&us.id))
                        })
                        .cloned()
                        .collect();
                    queue.push_front(right);
                    queue.push_front(left);
                }
                Err(e) => return Err(e),
            }
        }

        let mut merged = Self::merge_test_suites(suites);
        merged.source_spec_id = spec.id;
        merged.compute_coverage(spec.functional_requirements.len());

        // --- Passe supplementaire : combler les gaps par chunks de FR ---
        const MAX_GAP_FRS_PER_CHUNK: usize = 20;
        let max_gap_passes = 2;

        for pass in 0..max_gap_passes {
            // Identifier les FR non couverts
            let covered_set: std::collections::HashSet<&str> = merged
                .coverage
                .requirements_covered
                .iter()
                .map(String::as_str)
                .collect();
            let gap_frs: Vec<_> = spec
                .functional_requirements
                .iter()
                .filter(|fr| !covered_set.contains(fr.id.as_str()))
                .cloned()
                .collect();

            if gap_frs.is_empty() {
                info!("Couverture complete — aucun gap restant");
                break;
            }

            let num_chunks = gap_frs.len().div_ceil(MAX_GAP_FRS_PER_CHUNK);
            info!(
                pass = pass + 1,
                gaps = gap_frs.len(),
                chunks = num_chunks,
                total = spec.functional_requirements.len(),
                "Passe supplementaire pour combler les gaps"
            );

            // Traiter les FR manquants par chunks
            for (chunk_idx, chunk) in gap_frs.chunks(MAX_GAP_FRS_PER_CHUNK).enumerate() {
                info!(
                    chunk = chunk_idx + 1,
                    chunk_frs = chunk.len(),
                    "Gap-fill chunk"
                );

                let mut gap_spec = spec.clone();
                gap_spec.functional_requirements = chunk.to_vec();

                match self.generate_single(&gap_spec).await {
                    Ok(gap_suite) => {
                        merged.features.extend(gap_suite.features);
                    }
                    Err(e) => {
                        warn!(
                            error = %e,
                            chunk = chunk_idx + 1,
                            "Chunk gap-fill echoue — on continue avec les suivants"
                        );
                    }
                }
            }

            // Recalculer la couverture apres tous les chunks
            merged.compute_coverage(spec.functional_requirements.len());
            info!(
                coverage = format!("{:.0}%", merged.coverage.coverage_percentage),
                "Couverture apres passe supplementaire {}", pass + 1
            );
        }

        // Post-validation on merged result (apres gap-filling)
        let llm_warnings = validate_llm_test_output(&merged, spec);
        for w in &llm_warnings {
            warn!(rule = %w.rule, element = %w.element_id, "{}", w.message);
        }

        // Warning explicite si couverture faible
        let gap_count = llm_warnings
            .iter()
            .filter(|w| w.rule == "ISO-29119-COVERAGE")
            .count();
        let total_frs = spec.functional_requirements.len();
        if total_frs > 0 {
            let gap_pct = (gap_count as f32 / total_frs as f32) * 100.0;
            if gap_pct > 50.0 {
                warn!(
                    gaps = gap_count,
                    total = total_frs,
                    gap_pct = format!("{gap_pct:.0}%"),
                    "Couverture faible apres gap-filling"
                );
            }
        }

        info!(
            features = merged.features.len(),
            scenarios = merged.total_scenarios,
            coverage = format!("{:.0}%", merged.coverage.coverage_percentage),
            batches = num_batches,
            "Generation de tests multi-batch terminee"
        );

        Ok(merged)
    }

    /// Genere les tests pour un batch unique
    async fn generate_single(&self, spec: &Specification) -> Result<TestSuite, GenerationError> {
        let is_french = self.language == Language::French;

        // System prompt
        let system_context = serde_json::json!({
            "gherkin_language": self.language.gherkin_code(),
            "french": is_french,
        });
        let system_prompt = self
            .templates
            .render("generate_tests_system", &system_context)
            .map_err(|e| GenerationError::GherkinFailed {
                details: format!("Erreur template: {}", e),
            })?;

        let user_prompt = self.build_user_prompt(spec);

        debug!(
            system_prompt_len = system_prompt.len(),
            user_prompt_len = user_prompt.len(),
            scenarios = spec.user_scenarios.len(),
            requirements = spec.functional_requirements.len(),
            edge_cases = spec.edge_cases.len(),
            "Prompts construits pour la generation de tests"
        );

        let llm_output = self
            .call_llm_with_retry(&system_prompt, &user_prompt)
            .await?;

        let mut suite = self.build_test_suite(&llm_output, spec)?;
        suite.compute_coverage(spec.functional_requirements.len());

        // Post-validation
        let llm_warnings = validate_llm_test_output(&suite, spec);
        for w in &llm_warnings {
            warn!(rule = %w.rule, element = %w.element_id, "{}", w.message);
        }

        Ok(suite)
    }

    /// Fusionne plusieurs TestSuites en une seule
    fn merge_test_suites(suites: Vec<TestSuite>) -> TestSuite {
        let mut features = Vec::new();
        let mut total_scenarios = 0usize;

        for suite in suites {
            total_scenarios += suite.total_scenarios;
            features.extend(suite.features);
        }

        TestSuite {
            features,
            source_spec_id: uuid::Uuid::nil(),
            total_scenarios,
            coverage: TestCoverage {
                requirements_covered: vec![],
                requirements_total: 0,
                coverage_percentage: 0.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        }
    }

    fn build_user_prompt(&self, spec: &Specification) -> String {
        let mut prompt = String::from(
            "Genere des scenarios de test Gherkin pour la specification suivante:\n\n",
        );

        prompt.push_str("## Scenarios utilisateur\n\n");
        for us in &spec.user_scenarios {
            prompt.push_str(&format!(
                "### {} - {} (Priorite: {})\n\n{}\n\n",
                us.id, us.title, us.priority, us.description
            ));
            prompt.push_str("Scenarios d'acceptation:\n");
            for ac in &us.acceptance_scenarios {
                prompt.push_str(&format!(
                    "- Soit {}, Quand {}, Alors {}\n",
                    ac.given, ac.when, ac.then
                ));
            }
            prompt.push('\n');
        }

        prompt.push_str("## Exigences fonctionnelles\n\n");
        for fr in &spec.functional_requirements {
            prompt.push_str(&format!(
                "- {}: {} ({})\n",
                fr.id, fr.statement, fr.priority
            ));
        }
        prompt.push('\n');

        if !spec.edge_cases.is_empty() {
            prompt.push_str("## Cas limites\n\n");
            for ec in &spec.edge_cases {
                prompt.push_str(&format!("- {}\n", ec.description));
            }
            prompt.push('\n');
        }

        // Checklist de couverture FR obligatoire
        if !spec.functional_requirements.is_empty() {
            prompt.push_str("## Checklist de couverture (OBLIGATOIRE)\n\n");
            prompt.push_str("Chaque FR-ID ci-dessous DOIT apparaitre dans `verification_of` d'au moins un scenario:\n");
            for fr in &spec.functional_requirements {
                prompt.push_str(&format!("- [ ] {}\n", fr.id));
            }
            prompt.push('\n');
        }

        prompt
    }

    async fn call_llm_with_retry(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<LlmTestOutput, GenerationError> {
        use crate::application::llm_retry::{self, LlmRetryError};

        let result: Result<LlmTestOutput, LlmRetryError> = llm_retry::call_with_retry(
            self.llm.as_ref(),
            system_prompt,
            user_prompt,
            self.max_retries,
            None,
        )
        .await;

        match result {
            Ok(output) => {
                info!(
                    features = output.features.len(),
                    total_scenarios = output
                        .features
                        .iter()
                        .map(|f| f.scenarios.len())
                        .sum::<usize>(),
                    "Parsing JSON generation reussi"
                );
                Ok(output)
            }
            Err(LlmRetryError::Truncated { details }) => {
                Err(GenerationError::OutputTruncated { details })
            }
            Err(LlmRetryError::Failed { details }) => {
                Err(GenerationError::GherkinFailed { details })
            }
        }
    }

    fn build_test_suite(
        &self,
        output: &LlmTestOutput,
        spec: &Specification,
    ) -> Result<TestSuite, GenerationError> {
        let mut features = Vec::new();

        for lf in &output.features {
            let mut feature = Feature::new(lf.name.clone(), lf.description.clone());
            feature.tags = lf.tags.clone();
            feature.source_scenario_ids = lf.source_scenario_ids.clone();
            feature.covered_requirements = lf.covered_requirements.clone();
            if let Some(ref tl) = lf.test_level {
                feature.test_level = parse_test_level(tl);
            }

            // Background
            if let Some(ref bg) = lf.background {
                feature.background = Some(Background {
                    steps: bg.steps.iter().map(parse_step).collect(),
                });
            }

            // Scenarios
            for ls in &lf.scenarios {
                feature.scenarios.push(Scenario {
                    name: ls.name.clone(),
                    tags: ls.tags.clone(),
                    scenario_type: parse_scenario_type(&ls.scenario_type),
                    steps: ls.steps.iter().map(parse_step).collect(),
                    examples: ls.examples.as_ref().map(|e| Examples {
                        headers: e.headers.clone(),
                        rows: e.rows.clone(),
                    }),
                    test_data_suggestions: ls.test_data_suggestions.clone(),
                    verification_of: ls.verification_of.clone(),
                    coverage_technique: ls
                        .coverage_technique
                        .as_deref()
                        .map(parse_coverage_technique),
                });
            }

            features.push(feature);
        }

        Ok(TestSuite {
            features,
            source_spec_id: spec.id,
            total_scenarios: 0,
            coverage: TestCoverage {
                requirements_covered: vec![],
                requirements_total: 0,
                coverage_percentage: 0.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        })
    }
}

fn parse_step(ls: &LlmStep) -> Step {
    Step {
        keyword: parse_keyword(&ls.keyword),
        text: ls.text.clone(),
        doc_string: ls.doc_string.clone(),
        data_table: ls.data_table.clone(),
    }
}

fn parse_keyword(s: &str) -> StepKeyword {
    match s.to_lowercase().as_str() {
        "given" | "soit" => StepKeyword::Given,
        "when" | "quand" => StepKeyword::When,
        "then" | "alors" => StepKeyword::Then,
        "and" | "et" => StepKeyword::And,
        "but" | "mais" => StepKeyword::But,
        _ => StepKeyword::And,
    }
}

fn parse_scenario_type(s: &str) -> ScenarioType {
    match s.to_lowercase().as_str() {
        "happypath" | "happy_path" | "happy path" => ScenarioType::HappyPath,
        "edgecase" | "edge_case" | "edge case" => ScenarioType::EdgeCase,
        "errorscenario" | "error_scenario" | "error" => ScenarioType::ErrorScenario,
        "boundarycondition" | "boundary_condition" | "boundary" => ScenarioType::BoundaryCondition,
        _ => ScenarioType::HappyPath,
    }
}

/// Parse un niveau de test depuis une string LLM (ISO 29119-1)
fn parse_test_level(s: &str) -> TestLevel {
    match s.to_lowercase().as_str() {
        "unit" | "unitaire" => TestLevel::Unit,
        "integration" => TestLevel::Integration,
        "system" | "systeme" => TestLevel::System,
        "acceptance" | "acceptation" => TestLevel::Acceptance,
        _ => TestLevel::Acceptance,
    }
}

/// Parse une technique de couverture depuis une string LLM (ISO 29119-4)
fn parse_coverage_technique(s: &str) -> CoverageTechnique {
    match s.to_lowercase().replace([' ', '-', '_'], "").as_str() {
        "equivalencepartitioning" | "ep" => CoverageTechnique::EquivalencePartitioning,
        "boundaryvalueanalysis" | "bva" => CoverageTechnique::BoundaryValueAnalysis,
        "decisiontable" | "dt" => CoverageTechnique::DecisionTable,
        "statetransition" | "st" => CoverageTechnique::StateTransition,
        "errorguessing" | "eg" => CoverageTechnique::ErrorGuessing,
        _ => CoverageTechnique::EquivalencePartitioning,
    }
}

fn clean_json(response: &str) -> String {
    crate::application::json_utils::clean_json_response(response)
}

// ---------------------------------------------------------------------------
// Post-validation ISO 29119 de la sortie test LLM
// ---------------------------------------------------------------------------

use crate::application::refine_service::{LlmValidationWarning, WarningSeverity};

/// Valide la sortie test LLM selon les criteres ISO 29119
fn validate_llm_test_output(suite: &TestSuite, spec: &Specification) -> Vec<LlmValidationWarning> {
    let mut warnings = Vec::new();

    // 1. Chaque FR doit etre couvert par au moins 1 scenario
    let fr_ids: Vec<&str> = spec
        .functional_requirements
        .iter()
        .map(|fr| fr.id.as_str())
        .collect();

    let covered: std::collections::HashSet<&str> = suite
        .features
        .iter()
        .flat_map(|f| f.covered_requirements.iter().map(String::as_str))
        .collect();

    for fr_id in &fr_ids {
        if !covered.contains(fr_id) {
            warnings.push(LlmValidationWarning {
                rule: "ISO-29119-COVERAGE".into(),
                element_id: fr_id.to_string(),
                severity: WarningSeverity::Warning,
                message: format!("{}: exigence non couverte par aucun test", fr_id),
            });
        }
    }

    // 2. Chaque P1 FR doit avoir >= 2 scenarios (happy + error)
    for fr in &spec.functional_requirements {
        if fr.priority == crate::domain::user_story::Priority::P1 {
            let scenario_count: usize = suite
                .features
                .iter()
                .flat_map(|f| &f.scenarios)
                .filter(|s| {
                    s.verification_of.contains(&fr.id) || s.tags.iter().any(|t| t.contains(&fr.id))
                })
                .count();
            if scenario_count < 2 {
                warnings.push(LlmValidationWarning {
                    rule: "ISO-29119-P1-DEPTH".into(),
                    element_id: fr.id.clone(),
                    severity: WarningSeverity::Info,
                    message: format!(
                        "{}: exigence P1 avec seulement {} scenario(s) (recommande >= 2)",
                        fr.id, scenario_count
                    ),
                });
            }
        }
    }

    // 3. Detection scenarios orphelins (pas de verification_of ni de tag FR)
    for feature in &suite.features {
        for scenario in &feature.scenarios {
            let has_fr_ref = !scenario.verification_of.is_empty()
                || scenario.tags.iter().any(|t| t.starts_with("@FR-"));
            if !has_fr_ref {
                warnings.push(LlmValidationWarning {
                    rule: "ISO-29119-TRACEABILITY".into(),
                    element_id: scenario.name.clone(),
                    severity: WarningSeverity::Info,
                    message: format!(
                        "Scenario \"{}\" sans reference a une exigence (verification_of vide)",
                        scenario.name
                    ),
                });
            }
        }
    }

    // 4. Verification tags coherents (FR-NNN dans tags doit exister dans spec)
    let fr_id_set: std::collections::HashSet<&str> = fr_ids.iter().copied().collect();
    for feature in &suite.features {
        for scenario in &feature.scenarios {
            for v_id in &scenario.verification_of {
                if !fr_id_set.contains(v_id.as_str()) {
                    warnings.push(LlmValidationWarning {
                        rule: "ISO-29119-REF-VALID".into(),
                        element_id: scenario.name.clone(),
                        severity: WarningSeverity::Warning,
                        message: format!(
                            "Scenario \"{}\" reference {} qui n'existe pas dans la spec",
                            scenario.name, v_id
                        ),
                    });
                }
            }
        }
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_keyword() {
        assert_eq!(parse_keyword("Given"), StepKeyword::Given);
        assert_eq!(parse_keyword("Soit"), StepKeyword::Given);
        assert_eq!(parse_keyword("When"), StepKeyword::When);
        assert_eq!(parse_keyword("Quand"), StepKeyword::When);
        assert_eq!(parse_keyword("Then"), StepKeyword::Then);
        assert_eq!(parse_keyword("Alors"), StepKeyword::Then);
    }

    #[test]
    fn test_parse_keyword_and_but() {
        assert_eq!(parse_keyword("And"), StepKeyword::And);
        assert_eq!(parse_keyword("Et"), StepKeyword::And);
        assert_eq!(parse_keyword("But"), StepKeyword::But);
        assert_eq!(parse_keyword("Mais"), StepKeyword::But);
    }

    #[test]
    fn test_parse_keyword_unknown_defaults_to_and() {
        assert_eq!(parse_keyword("unknown"), StepKeyword::And);
        assert_eq!(parse_keyword(""), StepKeyword::And);
    }

    #[test]
    fn test_parse_scenario_type() {
        assert_eq!(parse_scenario_type("HappyPath"), ScenarioType::HappyPath);
        assert_eq!(parse_scenario_type("edge_case"), ScenarioType::EdgeCase);
        assert_eq!(parse_scenario_type("error"), ScenarioType::ErrorScenario);
    }

    #[test]
    fn test_parse_scenario_type_boundary() {
        assert_eq!(
            parse_scenario_type("BoundaryCondition"),
            ScenarioType::BoundaryCondition
        );
        assert_eq!(
            parse_scenario_type("boundary"),
            ScenarioType::BoundaryCondition
        );
    }

    #[test]
    fn test_parse_scenario_type_unknown_defaults_to_happy() {
        assert_eq!(parse_scenario_type("unknown"), ScenarioType::HappyPath);
        assert_eq!(parse_scenario_type(""), ScenarioType::HappyPath);
    }

    #[test]
    fn test_clean_json_response_markdown_block() {
        let input = "```json\n{\"key\": \"value\"}\n```";
        assert_eq!(clean_json(input), "{\"key\": \"value\"}");
    }

    #[test]
    fn test_clean_json_response_bare_block() {
        let input = "```\n{\"key\": \"value\"}\n```";
        assert_eq!(clean_json(input), "{\"key\": \"value\"}");
    }

    #[test]
    fn test_clean_json_response_plain() {
        let input = "{\"key\": \"value\"}";
        assert_eq!(clean_json(input), "{\"key\": \"value\"}");
    }

    #[test]
    fn test_clean_json_response_with_surrounding_text() {
        let input = "Here is the JSON:\n{\"key\": \"value\"}\nDone.";
        assert_eq!(clean_json(input), "{\"key\": \"value\"}");
    }

    #[test]
    fn test_clean_json_response_empty() {
        assert_eq!(clean_json(""), "");
        assert_eq!(clean_json("   "), "");
    }

    // -----------------------------------------------------------------------
    // Helpers pour les tests de batching / merge / validation
    // -----------------------------------------------------------------------

    #[allow(unused_imports)]
    use crate::application::refine_service::{LlmValidationWarning, WarningSeverity};
    use crate::domain::specification::{
        EdgeCase, FunctionalRequirement, RequirementCategory, Specification, UserScenario,
        VerificationMethod,
    };
    use crate::domain::user_story::Priority;
    use uuid::Uuid;

    fn make_fr(id: &str, statement: &str, priority: Priority) -> FunctionalRequirement {
        FunctionalRequirement {
            id: id.to_string(),
            statement: statement.to_string(),
            priority,
            category: RequirementCategory::Functional,
            testable: true,
            rationale: None,
            source: None,
            verification_method: VerificationMethod::Test,
            risk_level: None,
            parent_requirement: None,
            allocated_to: vec![],
            quality_characteristic: None,
        }
    }

    fn make_us(id: &str, description_len: usize) -> UserScenario {
        UserScenario {
            id: id.to_string(),
            title: format!("Scenario {}", id),
            priority: Priority::P2,
            description: "d".repeat(description_len),
            why_priority: String::new(),
            independent_test: String::new(),
            acceptance_scenarios: vec![],
            source_story_id: Uuid::new_v4(),
        }
    }

    fn make_edge(description: &str, related: Option<&str>) -> EdgeCase {
        EdgeCase {
            description: description.to_string(),
            related_scenario: related.map(String::from),
            severity: Priority::P2,
        }
    }

    // -----------------------------------------------------------------------
    // Tests build_spec_batches()
    // -----------------------------------------------------------------------

    #[test]
    fn test_batches_single_batch() {
        let mut spec = Specification::new("Test".into());
        spec.user_scenarios.push(make_us("US-001", 100));
        spec.functional_requirements
            .push(make_fr("FR-001", "DOIT X", Priority::P1));
        let batches = build_spec_batches(&spec, 10000);
        assert_eq!(batches.len(), 1);
    }

    #[test]
    fn test_batches_fr_in_all() {
        // 2 scenarios avec des descriptions assez longues pour forcer 2 batches
        let mut spec = Specification::new("Test".into());
        spec.functional_requirements
            .push(make_fr("FR-001", "DOIT A", Priority::P1));
        spec.functional_requirements
            .push(make_fr("FR-002", "DOIT B", Priority::P2));
        // Chaque US ~500 tokens (2000 chars), budget 600 (FR tokens de base ~5)
        spec.user_scenarios.push(make_us("US-001", 2000));
        spec.user_scenarios.push(make_us("US-002", 2000));
        let batches = build_spec_batches(&spec, 600);
        assert!(
            batches.len() >= 2,
            "Attendu 2+ batches, obtenu {}",
            batches.len()
        );
        // Les FRs doivent etre dans CHAQUE batch
        for (i, batch) in batches.iter().enumerate() {
            assert_eq!(
                batch.functional_requirements.len(),
                2,
                "Batch {} devrait contenir les 2 FRs",
                i
            );
        }
    }

    #[test]
    fn test_batches_unrelated_edges_in_all() {
        let mut spec = Specification::new("Test".into());
        spec.user_scenarios.push(make_us("US-001", 2000));
        spec.user_scenarios.push(make_us("US-002", 2000));
        spec.edge_cases.push(make_edge("Cas generique", None)); // pas de related_scenario
        spec.edge_cases
            .push(make_edge("Cas lie a US-001", Some("US-001")));
        let batches = build_spec_batches(&spec, 600);
        assert!(batches.len() >= 2);
        // Le cas generique doit etre dans CHAQUE batch
        for (i, batch) in batches.iter().enumerate() {
            assert!(
                batch
                    .edge_cases
                    .iter()
                    .any(|ec| ec.description == "Cas generique"),
                "Batch {} devrait contenir le cas generique",
                i
            );
        }
    }

    #[test]
    fn test_batches_related_edges_correct_batch() {
        let mut spec = Specification::new("Test".into());
        spec.user_scenarios.push(make_us("US-001", 2000));
        spec.user_scenarios.push(make_us("US-002", 2000));
        spec.edge_cases
            .push(make_edge("Cas lie a US-002", Some("US-002")));
        let batches = build_spec_batches(&spec, 600);
        assert!(batches.len() >= 2);
        // Le cas lie a US-002 ne doit PAS etre dans le premier batch
        let first_has_it = batches[0]
            .edge_cases
            .iter()
            .any(|ec| ec.description == "Cas lie a US-002");
        let last_has_it = batches
            .last()
            .unwrap()
            .edge_cases
            .iter()
            .any(|ec| ec.description == "Cas lie a US-002");
        assert!(
            !first_has_it,
            "Le premier batch ne devrait pas contenir le cas lie a US-002"
        );
        assert!(
            last_has_it,
            "Le dernier batch devrait contenir le cas lie a US-002"
        );
    }

    #[test]
    fn test_batches_fr_base_cost() {
        // Si les FRs representent beaucoup de tokens, le premier batch devrait splitter plus tot
        let mut spec = Specification::new("Test".into());
        // 10 FRs avec de longs statements (~250 tokens de FRs)
        for i in 0..10 {
            spec.functional_requirements.push(make_fr(
                &format!("FR-{:03}", i + 1),
                &"x".repeat(100), // 100 chars = ~25 tokens par FR
                Priority::P1,
            ));
        }
        spec.user_scenarios.push(make_us("US-001", 400)); // ~100 tokens
        spec.user_scenarios.push(make_us("US-002", 400));
        // Budget 350 : FR base (~250) + 1 US (~100) = 350 → chaque batch ne peut contenir qu'1 US
        let batches = build_spec_batches(&spec, 350);
        assert_eq!(
            batches.len(),
            2,
            "Le cout FR doit forcer le split en 2 batches"
        );
    }

    // -----------------------------------------------------------------------
    // Tests merge_test_suites()
    // -----------------------------------------------------------------------

    #[test]
    fn test_merge_suites_two() {
        let suite1 = TestSuite {
            features: vec![Feature::new("Feature A".into(), "Desc A".into())],
            source_spec_id: Uuid::new_v4(),
            total_scenarios: 3,
            coverage: TestCoverage {
                requirements_covered: vec!["FR-001".into()],
                requirements_total: 2,
                coverage_percentage: 50.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        };
        let suite2 = TestSuite {
            features: vec![Feature::new("Feature B".into(), "Desc B".into())],
            source_spec_id: Uuid::new_v4(),
            total_scenarios: 2,
            coverage: TestCoverage {
                requirements_covered: vec!["FR-002".into()],
                requirements_total: 2,
                coverage_percentage: 50.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        };
        let merged = GenerateTestsService::merge_test_suites(vec![suite1, suite2]);
        assert_eq!(merged.features.len(), 2);
        assert_eq!(merged.total_scenarios, 5);
    }

    #[test]
    fn test_merge_suites_empty() {
        let merged = GenerateTestsService::merge_test_suites(vec![]);
        assert_eq!(merged.features.len(), 0);
        assert_eq!(merged.total_scenarios, 0);
    }

    // -----------------------------------------------------------------------
    // Tests validate_llm_test_output()
    // -----------------------------------------------------------------------

    #[test]
    fn test_validate_uncovered_fr() {
        let spec = {
            let mut s = Specification::new("Test".into());
            s.functional_requirements
                .push(make_fr("FR-001", "DOIT A", Priority::P1));
            s.functional_requirements
                .push(make_fr("FR-002", "DOIT B", Priority::P2));
            s
        };
        // Suite qui ne couvre que FR-001
        let mut feature = Feature::new("Test".into(), "".into());
        feature.covered_requirements = vec!["FR-001".into()];
        let suite = TestSuite {
            features: vec![feature],
            source_spec_id: spec.id,
            total_scenarios: 1,
            coverage: TestCoverage {
                requirements_covered: vec![],
                requirements_total: 0,
                coverage_percentage: 0.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        };
        let warnings = validate_llm_test_output(&suite, &spec);
        assert!(
            warnings
                .iter()
                .any(|w| w.rule == "ISO-29119-COVERAGE" && w.element_id == "FR-002")
        );
    }

    #[test]
    fn test_validate_p1_insufficient() {
        let spec = {
            let mut s = Specification::new("Test".into());
            s.functional_requirements
                .push(make_fr("FR-001", "DOIT critique", Priority::P1));
            s
        };
        // 1 seul scenario pour FR-001 P1 (< 2 requis)
        let mut feature = Feature::new("Test".into(), "".into());
        feature.covered_requirements = vec!["FR-001".into()];
        feature.scenarios.push(Scenario {
            name: "Seul scenario".into(),
            tags: vec![],
            scenario_type: ScenarioType::HappyPath,
            steps: vec![],
            examples: None,
            test_data_suggestions: vec![],
            verification_of: vec!["FR-001".into()],
            coverage_technique: None,
        });
        let suite = TestSuite {
            features: vec![feature],
            source_spec_id: spec.id,
            total_scenarios: 1,
            coverage: TestCoverage {
                requirements_covered: vec![],
                requirements_total: 0,
                coverage_percentage: 0.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        };
        let warnings = validate_llm_test_output(&suite, &spec);
        assert!(warnings.iter().any(|w| w.rule == "ISO-29119-P1-DEPTH"));
    }

    #[test]
    fn test_validate_orphan_scenario() {
        let spec = Specification::new("Test".into());
        let mut feature = Feature::new("Test".into(), "".into());
        feature.scenarios.push(Scenario {
            name: "Scenario orphelin".into(),
            tags: vec![],
            scenario_type: ScenarioType::HappyPath,
            steps: vec![],
            examples: None,
            test_data_suggestions: vec![],
            verification_of: vec![], // pas de reference FR
            coverage_technique: None,
        });
        let suite = TestSuite {
            features: vec![feature],
            source_spec_id: spec.id,
            total_scenarios: 1,
            coverage: TestCoverage {
                requirements_covered: vec![],
                requirements_total: 0,
                coverage_percentage: 0.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        };
        let warnings = validate_llm_test_output(&suite, &spec);
        assert!(warnings.iter().any(|w| w.rule == "ISO-29119-TRACEABILITY"));
    }

    #[test]
    fn test_validate_invalid_fr_ref() {
        let spec = {
            let mut s = Specification::new("Test".into());
            s.functional_requirements
                .push(make_fr("FR-001", "DOIT A", Priority::P1));
            s
        };
        let mut feature = Feature::new("Test".into(), "".into());
        feature.scenarios.push(Scenario {
            name: "Ref invalide".into(),
            tags: vec![],
            scenario_type: ScenarioType::HappyPath,
            steps: vec![],
            examples: None,
            test_data_suggestions: vec![],
            verification_of: vec!["FR-999".into()], // n'existe pas dans la spec
            coverage_technique: None,
        });
        let suite = TestSuite {
            features: vec![feature],
            source_spec_id: spec.id,
            total_scenarios: 1,
            coverage: TestCoverage {
                requirements_covered: vec![],
                requirements_total: 0,
                coverage_percentage: 0.0,
                scenarios_by_type: ScenarioCounts::default(),
            },
        };
        let warnings = validate_llm_test_output(&suite, &spec);
        assert!(warnings.iter().any(|w| w.rule == "ISO-29119-REF-VALID"));
    }

    // -----------------------------------------------------------------------
    // Tests parse helpers
    // -----------------------------------------------------------------------

    #[test]
    fn test_parse_test_level_all_variants() {
        assert_eq!(parse_test_level("unit"), TestLevel::Unit);
        assert_eq!(parse_test_level("unitaire"), TestLevel::Unit);
        assert_eq!(parse_test_level("integration"), TestLevel::Integration);
        assert_eq!(parse_test_level("system"), TestLevel::System);
        assert_eq!(parse_test_level("systeme"), TestLevel::System);
        assert_eq!(parse_test_level("acceptance"), TestLevel::Acceptance);
        assert_eq!(parse_test_level("acceptation"), TestLevel::Acceptance);
        assert_eq!(parse_test_level("unknown"), TestLevel::Acceptance);
    }

    #[test]
    fn test_parse_coverage_technique_all_variants() {
        assert_eq!(
            parse_coverage_technique("equivalence partitioning"),
            CoverageTechnique::EquivalencePartitioning
        );
        assert_eq!(
            parse_coverage_technique("ep"),
            CoverageTechnique::EquivalencePartitioning
        );
        assert_eq!(
            parse_coverage_technique("boundary value analysis"),
            CoverageTechnique::BoundaryValueAnalysis
        );
        assert_eq!(
            parse_coverage_technique("bva"),
            CoverageTechnique::BoundaryValueAnalysis
        );
        assert_eq!(
            parse_coverage_technique("decision table"),
            CoverageTechnique::DecisionTable
        );
        assert_eq!(
            parse_coverage_technique("state transition"),
            CoverageTechnique::StateTransition
        );
        assert_eq!(
            parse_coverage_technique("error guessing"),
            CoverageTechnique::ErrorGuessing
        );
        assert_eq!(
            parse_coverage_technique("unknown"),
            CoverageTechnique::EquivalencePartitioning
        );
    }

    mod proptest_suite {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn parse_test_level_never_panics(input in "\\PC*") {
                let _ = parse_test_level(&input);
            }

            #[test]
            fn parse_coverage_technique_never_panics(input in "\\PC*") {
                let _ = parse_coverage_technique(&input);
            }
        }
    }
}
