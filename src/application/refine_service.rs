//! Service de raffinement des User Stories en Specifications
//!
//! Orchestre l'appel au LLM pour transformer les US brutes en specs completes.

use std::sync::Arc;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::domain::errors::RefinementError;
use crate::domain::specification::*;
use crate::domain::user_story::{Language, UserStorySet};
use crate::domain::validation::validate_specification;
use crate::ports::llm_service::LlmService;
use crate::ports::template_engine::TemplateEngine;

/// Service de raffinement US -> Specification
pub struct RefineService {
    llm: Arc<dyn LlmService>,
    templates: Arc<dyn TemplateEngine>,
    max_retries: usize,
    token_budget: usize,
}

/// Schema JSON de sortie du LLM pour le raffinement
#[derive(Debug, serde::Deserialize)]
struct LlmRefineOutput {
    #[serde(default)]
    user_scenarios: Vec<LlmUserScenario>,
    #[serde(default)]
    functional_requirements: Vec<LlmFunctionalRequirement>,
    #[serde(default)]
    key_entities: Vec<LlmKeyEntity>,
    #[serde(default)]
    edge_cases: Vec<LlmEdgeCase>,
    #[serde(default)]
    success_criteria: Vec<LlmSuccessCriterion>,
    #[serde(default)]
    clarifications_needed: Vec<LlmClarification>,
}

#[derive(Debug, serde::Deserialize)]
struct LlmUserScenario {
    #[serde(default)]
    id: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    priority: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    why_priority: String,
    #[serde(default)]
    independent_test: String,
    #[serde(default)]
    acceptance_scenarios: Vec<LlmAcceptanceScenario>,
}

#[derive(Debug, serde::Deserialize)]
struct LlmAcceptanceScenario {
    #[serde(default)]
    given: String,
    #[serde(default)]
    when: String,
    #[serde(default)]
    then: String,
}

#[derive(Debug, serde::Deserialize)]
struct LlmFunctionalRequirement {
    #[serde(default)]
    id: String,
    #[serde(default)]
    statement: String,
    #[serde(default)]
    priority: String,
    #[serde(default = "default_category")]
    category: String,
    #[serde(default = "default_true")]
    testable: bool,
    // ISO 29148 enrichment fields (optional from LLM)
    #[serde(default)]
    rationale: Option<String>,
    #[serde(default)]
    source: Option<String>,
    #[serde(default)]
    verification_method: Option<String>,
    #[serde(default)]
    risk_level: Option<String>,
    #[serde(default)]
    parent_requirement: Option<String>,
    #[serde(default)]
    allocated_to: Vec<String>,
    #[serde(default)]
    quality_characteristic: Option<String>,
}

fn default_category() -> String {
    "Functional".to_string()
}
fn default_true() -> bool {
    true
}

#[derive(Debug, serde::Deserialize)]
struct LlmKeyEntity {
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    attributes: Vec<String>,
    #[serde(default)]
    relationships: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
struct LlmEdgeCase {
    #[serde(default)]
    description: String,
    #[serde(default)]
    related_scenario: Option<String>,
    #[serde(default = "default_severity")]
    severity: String,
}

fn default_severity() -> String {
    "P2".to_string()
}

#[derive(Debug, serde::Deserialize)]
struct LlmSuccessCriterion {
    #[serde(default)]
    id: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    measurable_metric: String,
}

#[derive(Debug, serde::Deserialize)]
struct LlmClarification {
    #[serde(default)]
    question: String,
    #[serde(default)]
    context: String,
    #[serde(default)]
    suggested_options: Vec<String>,
    #[serde(default)]
    impact: String,
}

/// Estime le nombre de tokens d'une UserStory (heuristique: chars / 4)
fn estimate_story_tokens(story: &crate::domain::user_story::UserStory) -> usize {
    let mut len = story.title.len()
        + story.actor.len()
        + story.action.len()
        + story.benefit.len()
        + story.raw_text.len();
    for ac in &story.acceptance_criteria {
        len += ac.len();
    }
    len / 4
}

/// Construit des lots adaptatifs par budget de tokens
fn build_batches(
    stories: &[crate::domain::user_story::UserStory],
    token_budget: usize,
) -> Vec<Vec<usize>> {
    if stories.is_empty() {
        return vec![];
    }

    let mut batches: Vec<Vec<usize>> = Vec::new();
    let mut current_batch: Vec<usize> = Vec::new();
    let mut current_tokens: usize = 0;

    for (i, story) in stories.iter().enumerate() {
        let tokens = estimate_story_tokens(story);

        // Si le lot courant est non-vide et ajouterrait depasse le budget → nouveau lot
        if !current_batch.is_empty() && current_tokens + tokens > token_budget {
            batches.push(std::mem::take(&mut current_batch));
            current_tokens = 0;
        }

        current_batch.push(i);
        current_tokens += tokens;
    }

    // Dernier lot
    if !current_batch.is_empty() {
        batches.push(current_batch);
    }

    batches
}

impl RefineService {
    pub fn new(
        llm: Arc<dyn LlmService>,
        templates: Arc<dyn TemplateEngine>,
        max_retries: usize,
    ) -> Self {
        Self {
            llm,
            templates,
            max_retries,
            token_budget: 6000,
        }
    }

    pub fn with_token_budget(mut self, token_budget: usize) -> Self {
        self.token_budget = token_budget;
        self
    }

    /// Raffine un ensemble de User Stories en une Specification
    pub async fn refine(
        &self,
        story_set: &UserStorySet,
        constitution: Option<&str>,
    ) -> Result<Specification, RefinementError> {
        let batches = build_batches(&story_set.stories, self.token_budget);
        let num_batches = batches.len();

        info!(
            count = story_set.stories.len(),
            batches = num_batches,
            token_budget = self.token_budget,
            "Debut du raffinement des User Stories"
        );

        // File d'attente avec re-decoupage adaptatif si troncature
        // Meme pour un seul batch, on passe par la queue pour gerer la troncature
        let mut queue: std::collections::VecDeque<Vec<usize>> = if num_batches <= 1 {
            std::collections::VecDeque::from(vec![(0..story_set.stories.len()).collect()])
        } else {
            batches.into()
        };
        let mut specs = Vec::new();
        let mut batch_counter = 0usize;

        while let Some(batch_indices) = queue.pop_front() {
            batch_counter += 1;
            info!(
                batch = batch_counter,
                stories = batch_indices.len(),
                "Batch de raffinement"
            );

            let batch_stories: Vec<_> = batch_indices
                .iter()
                .map(|&i| story_set.stories[i].clone())
                .collect();
            let batch_set = UserStorySet {
                stories: batch_stories,
                source_files: story_set.source_files.clone(),
                language: story_set.language,
            };

            match self.refine_single(&batch_set, constitution).await {
                Ok(spec) => specs.push(spec),
                Err(RefinementError::OutputTruncated { .. }) if batch_indices.len() > 1 => {
                    let mid = batch_indices.len() / 2;
                    let (left, right) = batch_indices.split_at(mid);
                    warn!(
                        original = batch_indices.len(),
                        left = left.len(),
                        right = right.len(),
                        "Troncature detectee — re-decoupage du batch"
                    );
                    queue.push_front(right.to_vec());
                    queue.push_front(left.to_vec());
                }
                Err(e) => return Err(e),
            }
        }

        // Merge all specifications
        let mut merged = Self::merge_specifications(specs);

        // Post-validation ISO 29148 on merged result
        let llm_warnings = validate_llm_spec_output(&merged);
        for w in &llm_warnings {
            warn!(rule = %w.rule, element = %w.element_id, "{}", w.message);
        }

        let validation = validate_specification(&merged);
        merged.validation = Some(validation);

        if merged.has_unresolved_clarifications() {
            merged.status = SpecStatus::NeedsClarification;
        }

        info!(
            scenarios = merged.user_scenarios.len(),
            requirements = merged.functional_requirements.len(),
            edge_cases = merged.edge_cases.len(),
            batches = num_batches,
            "Raffinement multi-batch termine"
        );

        Ok(merged)
    }

    /// Raffine un batch unique de User Stories
    async fn refine_single(
        &self,
        story_set: &UserStorySet,
        constitution: Option<&str>,
    ) -> Result<Specification, RefinementError> {
        let language_str = match story_set.language {
            Language::French => "francais",
            Language::English => "english",
        };

        // Construire le system prompt
        let system_context = serde_json::json!({ "language": language_str });
        let system_prompt = self
            .templates
            .render("refine_system", &system_context)
            .map_err(|e| RefinementError::LlmFailed {
                details: format!("Erreur template systeme: {}", e),
            })?;

        // Construire un prompt user consolide
        let mut user_prompt =
            String::from("Raffine les User Stories suivantes en une specification complete:\n\n");
        for us in &story_set.stories {
            user_prompt.push_str(&format!(
                "### {} - {}\n\nEn tant que {}, je veux {} afin de {}.\n",
                us.external_id.as_deref().unwrap_or(&us.title),
                us.title,
                us.actor,
                us.action,
                us.benefit,
            ));
            if !us.acceptance_criteria.is_empty() {
                user_prompt.push_str("\nCriteres d'acceptation:\n");
                for ac in &us.acceptance_criteria {
                    user_prompt.push_str(&format!("- {}\n", ac));
                }
            }
            user_prompt.push('\n');
        }

        if let Some(constitution_text) = constitution {
            user_prompt.push_str(&format!(
                "\n---\nPrincipes du projet (constitution):\n{}\n",
                constitution_text
            ));
        }

        debug!(
            system_prompt_len = system_prompt.len(),
            user_prompt_len = user_prompt.len(),
            stories = story_set.stories.len(),
            language = language_str,
            "Prompts construits pour le raffinement"
        );

        // Appel au LLM avec retry
        let llm_output = self
            .call_llm_with_retry(&system_prompt, &user_prompt)
            .await?;

        // Detecter une sortie incomplete (LLM tronque par max_tokens)
        if !story_set.stories.is_empty() && llm_output.functional_requirements.is_empty() {
            warn!(
                stories = story_set.stories.len(),
                scenarios = llm_output.user_scenarios.len(),
                "Sortie LLM incomplete: 0 exigences fonctionnelles generees. \
                 Augmentez max_tokens dans la config LLM (recommande: 16384+)"
            );
        }

        // Construire la Specification
        let mut spec = self.build_specification(&llm_output, story_set)?;

        // Post-validation ISO 29148
        let llm_warnings = validate_llm_spec_output(&spec);
        for w in &llm_warnings {
            warn!(rule = %w.rule, element = %w.element_id, "{}", w.message);
        }

        let validation = validate_specification(&spec);
        spec.validation = Some(validation);

        if spec.has_unresolved_clarifications() {
            spec.status = SpecStatus::NeedsClarification;
        }

        Ok(spec)
    }

    /// Fusionne plusieurs Specifications en une seule avec renumerotation des FR-IDs
    /// et mise a jour des references croisees.
    fn merge_specifications(specs: Vec<Specification>) -> Specification {
        if specs.is_empty() {
            return Specification::new("Empty".to_string());
        }
        if specs.len() == 1 {
            return specs.into_iter().next().unwrap();
        }

        let total_stories: usize = specs.iter().map(|s| s.source_stories.len()).sum();
        let title = format!("Specification ({} User Stories)", total_stories);
        let mut merged = Specification::new(title);

        let mut fr_counter = 1usize;
        let mut us_counter = 1usize;
        let mut sc_counter = 1usize;

        // Passe 1 : construire les mappings globaux (old ID -> new ID)
        // pour TOUS les batches avant de mettre a jour les references.
        let mut global_fr_map = std::collections::HashMap::new();
        let mut global_us_map = std::collections::HashMap::new();
        let mut temp_fr_counter = 1usize;
        let mut temp_us_counter = 1usize;
        for spec in &specs {
            for fr in &spec.functional_requirements {
                let new_id = format!("FR-{:03}", temp_fr_counter);
                global_fr_map.insert(fr.id.clone(), new_id);
                temp_fr_counter += 1;
            }
            for us in &spec.user_scenarios {
                let new_id = format!("US-{:03}", temp_us_counter);
                global_us_map.insert(us.id.clone(), new_id);
                temp_us_counter += 1;
            }
        }

        // Passe 2 : renumeroter et mettre a jour les references avec le mapping global
        for spec in specs {
            merged.source_stories.extend(spec.source_stories);
            merged.key_entities.extend(spec.key_entities);
            merged
                .clarifications_needed
                .extend(spec.clarifications_needed);

            for mut fr in spec.functional_requirements {
                let new_id = format!("FR-{:03}", fr_counter);
                // Mettre a jour parent_requirement avec le mapping global
                if let Some(ref parent) = fr.parent_requirement
                    && let Some(new_parent) = global_fr_map.get(parent)
                {
                    fr.parent_requirement = Some(new_parent.clone());
                }
                fr.id = new_id;
                fr_counter += 1;
                merged.functional_requirements.push(fr);
            }

            for mut us in spec.user_scenarios {
                let new_id = format!("US-{:03}", us_counter);
                us.id = new_id;
                us_counter += 1;
                merged.user_scenarios.push(us);
            }

            // Renumeroter les SC-IDs
            for mut sc in spec.success_criteria {
                sc.id = format!("SC-{:03}", sc_counter);
                sc_counter += 1;
                merged.success_criteria.push(sc);
            }

            // Mettre a jour les references dans les edge cases
            for mut ec in spec.edge_cases {
                if let Some(ref related) = ec.related_scenario
                    && let Some(new_id) = global_us_map.get(related)
                {
                    ec.related_scenario = Some(new_id.clone());
                }
                merged.edge_cases.push(ec);
            }
        }

        merged
    }

    async fn call_llm_with_retry(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<LlmRefineOutput, RefinementError> {
        use crate::application::llm_retry::{self, LlmRetryError};

        // Validation post-parsing : detecter sortie incomplete (scenarios OK mais 0 requirements)
        let validate =
            |output: &LlmRefineOutput, attempt: usize, max_retries: usize| -> Option<String> {
                if !output.user_scenarios.is_empty()
                    && output.functional_requirements.is_empty()
                    && attempt < max_retries
                {
                    Some(format!(
                        "Sortie LLM incomplete: {} scenarios mais 0 exigences",
                        output.user_scenarios.len(),
                    ))
                } else {
                    None
                }
            };

        let result = llm_retry::call_with_retry(
            self.llm.as_ref(),
            system_prompt,
            user_prompt,
            self.max_retries,
            Some(&validate),
        )
        .await;

        match result {
            Ok(output) => {
                info!(
                    user_scenarios = output.user_scenarios.len(),
                    functional_requirements = output.functional_requirements.len(),
                    key_entities = output.key_entities.len(),
                    edge_cases = output.edge_cases.len(),
                    success_criteria = output.success_criteria.len(),
                    "Parsing JSON reussi"
                );
                if !output.functional_requirements.is_empty() {
                    let fr_ids: Vec<&str> = output
                        .functional_requirements
                        .iter()
                        .map(|fr| fr.id.as_str())
                        .collect();
                    debug!(fr_ids = ?fr_ids, "Exigences fonctionnelles generees");
                }
                Ok(output)
            }
            Err(LlmRetryError::Truncated { details }) => {
                Err(RefinementError::OutputTruncated { details })
            }
            Err(LlmRetryError::Failed { details }) => {
                Err(RefinementError::OutputParseFailed { details })
            }
        }
    }

    fn build_specification(
        &self,
        output: &LlmRefineOutput,
        story_set: &UserStorySet,
    ) -> Result<Specification, RefinementError> {
        let title = if story_set.stories.len() == 1 {
            story_set.stories[0].title.clone()
        } else {
            format!("Specification ({} User Stories)", story_set.stories.len())
        };

        let mut spec = Specification::new(title);
        spec.source_stories = story_set.stories.iter().map(|s| s.id).collect();

        // User scenarios — correler par ID/titre plutot que par index
        // Construire un index des stories par external_id et titre pour matching
        let story_by_external_id: std::collections::HashMap<&str, Uuid> = story_set
            .stories
            .iter()
            .filter_map(|s| s.external_id.as_deref().map(|eid| (eid, s.id)))
            .collect();
        let story_by_title: std::collections::HashMap<&str, Uuid> = story_set
            .stories
            .iter()
            .map(|s| (s.title.as_str(), s.id))
            .collect();

        for (idx, ls) in output.user_scenarios.iter().enumerate() {
            // Warn si champs critiques vides (deserialization tolerante)
            if ls.id.is_empty() || ls.description.is_empty() {
                warn!(
                    index = idx,
                    id = %ls.id,
                    "Scenario utilisateur avec champs critiques vides (id ou description) — conserve avec valeurs par defaut"
                );
            }

            // Essayer de corréler par ID, puis par titre, puis par index en fallback
            let source_id = story_by_external_id
                .get(ls.id.as_str())
                .or_else(|| story_by_title.get(ls.title.as_str()))
                .copied()
                .or_else(|| story_set.stories.get(idx).map(|s| s.id))
                .unwrap_or_else(Uuid::new_v4);

            spec.user_scenarios.push(UserScenario {
                id: ls.id.clone(),
                title: ls.title.clone(),
                priority: parse_priority(&ls.priority),
                description: ls.description.clone(),
                why_priority: ls.why_priority.clone(),
                independent_test: ls.independent_test.clone(),
                acceptance_scenarios: ls
                    .acceptance_scenarios
                    .iter()
                    .map(|a| AcceptanceScenario {
                        given: a.given.clone(),
                        when: a.when.clone(),
                        then: a.then.clone(),
                    })
                    .collect(),
                source_story_id: source_id,
            });
        }

        // Functional requirements (filtrer les entrees avec id ou statement vide)
        for lfr in &output.functional_requirements {
            if lfr.id.trim().is_empty() || lfr.statement.trim().is_empty() {
                warn!(
                    id = %lfr.id,
                    "Exigence fonctionnelle ignoree: id ou statement vide"
                );
                continue;
            }
            spec.functional_requirements.push(FunctionalRequirement {
                id: lfr.id.clone(),
                statement: lfr.statement.clone(),
                priority: parse_priority(&lfr.priority),
                category: parse_category(&lfr.category),
                testable: lfr.testable,
                rationale: lfr.rationale.clone(),
                source: lfr.source.clone(),
                verification_method: lfr
                    .verification_method
                    .as_deref()
                    .map(parse_verification_method)
                    .unwrap_or_default(),
                risk_level: lfr.risk_level.as_deref().map(parse_risk_level),
                parent_requirement: lfr.parent_requirement.clone(),
                allocated_to: lfr.allocated_to.clone(),
                quality_characteristic: lfr
                    .quality_characteristic
                    .as_deref()
                    .map(parse_quality_characteristic),
            });
        }

        // Key entities (filtrer les entrees avec name vide)
        for le in &output.key_entities {
            if le.name.trim().is_empty() {
                warn!("Entite cle ignoree: name vide");
                continue;
            }
            spec.key_entities.push(KeyEntity {
                name: le.name.clone(),
                description: le.description.clone(),
                attributes: le.attributes.clone(),
                relationships: le.relationships.clone(),
            });
        }

        // Edge cases
        for lec in &output.edge_cases {
            spec.edge_cases.push(EdgeCase {
                description: lec.description.clone(),
                related_scenario: lec.related_scenario.clone(),
                severity: parse_priority(&lec.severity),
            });
        }

        // Success criteria (filtrer les entrees avec id vide)
        for lsc in &output.success_criteria {
            if lsc.id.trim().is_empty() {
                warn!("Critere de succes ignore: id vide");
                continue;
            }
            spec.success_criteria.push(SuccessCriterion {
                id: lsc.id.clone(),
                description: lsc.description.clone(),
                measurable_metric: lsc.measurable_metric.clone(),
            });
        }

        // Clarifications
        for lc in &output.clarifications_needed {
            spec.clarifications_needed.push(Clarification {
                question: lc.question.clone(),
                context: lc.context.clone(),
                suggested_options: lc.suggested_options.clone(),
                impact: lc.impact.clone(),
                resolved: false,
                answer: None,
            });
        }

        Ok(spec)
    }
}

/// Parse une priorite depuis une string LLM
fn parse_priority(s: &str) -> crate::domain::user_story::Priority {
    use crate::domain::user_story::Priority;
    match s.to_uppercase().as_str() {
        "P1" | "MUST" | "HAUTE" | "HIGH" => Priority::P1,
        "P2" | "SHOULD" | "MOYENNE" | "MEDIUM" => Priority::P2,
        _ => Priority::P3,
    }
}

/// Parse une categorie depuis une string LLM
fn parse_category(s: &str) -> RequirementCategory {
    match s.to_lowercase().as_str() {
        "functional" | "fonctionnelle" => RequirementCategory::Functional,
        "nonfunctional" | "non-functional" | "non-fonctionnelle" => {
            RequirementCategory::NonFunctional
        }
        "constraint" | "contrainte" => RequirementCategory::Constraint,
        _ => RequirementCategory::Functional,
    }
}

/// Parse une methode de verification depuis une string LLM (IEEE 1012)
fn parse_verification_method(s: &str) -> VerificationMethod {
    match s.to_lowercase().as_str() {
        "inspection" => VerificationMethod::Inspection,
        "analysis" | "analyse" => VerificationMethod::Analysis,
        "demonstration" | "demo" => VerificationMethod::Demonstration,
        "test" => VerificationMethod::Test,
        _ => VerificationMethod::Test,
    }
}

/// Parse un niveau de risque depuis une string LLM
fn parse_risk_level(s: &str) -> RiskLevel {
    match s.to_lowercase().as_str() {
        "high" | "haute" | "critique" => RiskLevel::High,
        "medium" | "moyenne" | "significatif" => RiskLevel::Medium,
        "low" | "basse" | "mineur" => RiskLevel::Low,
        _ => RiskLevel::Medium,
    }
}

/// Parse une caracteristique qualite ISO 25010 depuis une string LLM
fn parse_quality_characteristic(s: &str) -> QualityCharacteristic {
    match s.to_lowercase().replace([' ', '-', '_'], "").as_str() {
        "functionalsuitability" => QualityCharacteristic::FunctionalSuitability,
        "performanceefficiency" | "performance" => QualityCharacteristic::PerformanceEfficiency,
        "compatibility" => QualityCharacteristic::Compatibility,
        "interactioncapability" | "usability" => QualityCharacteristic::InteractionCapability,
        "reliability" | "fiabilite" => QualityCharacteristic::Reliability,
        "security" | "securite" => QualityCharacteristic::Security,
        "maintainability" | "maintenabilite" => QualityCharacteristic::Maintainability,
        "flexibility" | "portability" => QualityCharacteristic::Flexibility,
        "safety" | "surete" => QualityCharacteristic::Safety,
        _ => QualityCharacteristic::FunctionalSuitability,
    }
}

/// Nettoie la reponse JSON du LLM (retire les blocs markdown)
///
/// Delegue a `json_utils::clean_json_response` pour eviter la duplication.
pub fn clean_json_response(response: &str) -> String {
    crate::application::json_utils::clean_json_response(response)
}

// ---------------------------------------------------------------------------
// Post-validation ISO 29148 de la sortie LLM
// ---------------------------------------------------------------------------

/// Avertissement de validation post-LLM
#[derive(Debug, Clone)]
pub struct LlmValidationWarning {
    pub rule: String,
    pub element_id: String,
    pub severity: WarningSeverity,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningSeverity {
    Error,
    Warning,
    Info,
}

// Reutiliser la liste canonique depuis le module de validation
use crate::domain::validation::AMBIGUOUS_WORDS;

/// Mots normatifs attendus dans les enonces (ISO 29148)
const NORMATIVE_KEYWORDS: &[&str] = &[
    "MUST", "SHOULD", "COULD", "SHALL", "WILL", "DOIT", "DEVRAIT", "POURRAIT",
];

/// Verifie si un mot est present en tant que mot entier (word boundary)
fn contains_word(text: &str, word: &str) -> bool {
    if word.contains(' ') {
        return text.contains(word);
    }
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

/// Valide la sortie LLM selon les criteres ISO 29148
fn validate_llm_spec_output(spec: &Specification) -> Vec<LlmValidationWarning> {
    let mut warnings = Vec::new();

    // 1. Detection de doublons d'ID
    let mut seen_ids = std::collections::HashSet::new();
    for fr in &spec.functional_requirements {
        if !seen_ids.insert(&fr.id) {
            warnings.push(LlmValidationWarning {
                rule: "ISO-29148-ID-UNIQUE".into(),
                element_id: fr.id.clone(),
                severity: WarningSeverity::Error,
                message: format!("ID duplique: {}", fr.id),
            });
        }
    }

    for fr in &spec.functional_requirements {
        // 2. Verification syntaxe normative (MUST/SHOULD/COULD)
        let upper = fr.statement.to_uppercase();
        if !NORMATIVE_KEYWORDS.iter().any(|kw| upper.contains(kw)) {
            warnings.push(LlmValidationWarning {
                rule: "ISO-29148-NORMATIVE".into(),
                element_id: fr.id.clone(),
                severity: WarningSeverity::Warning,
                message: format!("{}: enonce sans mot normatif (MUST/SHOULD/COULD)", fr.id),
            });
        }

        // 3. Detection de mots ambigus (word boundary matching)
        let lower = fr.statement.to_lowercase();
        for word in AMBIGUOUS_WORDS {
            if contains_word(&lower, word) {
                warnings.push(LlmValidationWarning {
                    rule: "ISO-29148-UNAMBIGUOUS".into(),
                    element_id: fr.id.clone(),
                    severity: WarningSeverity::Warning,
                    message: format!("{}: mot ambigu detecte: \"{}\"", fr.id, word),
                });
            }
        }

        // 4. Verification format ID (FR-NNN)
        if !fr.id.starts_with("FR-") {
            warnings.push(LlmValidationWarning {
                rule: "ISO-29148-CONFORMING".into(),
                element_id: fr.id.clone(),
                severity: WarningSeverity::Warning,
                message: format!("{}: ID ne suit pas le format FR-NNN", fr.id),
            });
        }

        // 5. Enonce non vide
        if fr.statement.trim().is_empty() {
            warnings.push(LlmValidationWarning {
                rule: "ISO-29148-COMPLETE".into(),
                element_id: fr.id.clone(),
                severity: WarningSeverity::Error,
                message: format!("{}: enonce vide", fr.id),
            });
        }

        // 6. Exigence P1 sans risk_level
        if fr.priority == crate::domain::user_story::Priority::P1 && fr.risk_level.is_none() {
            warnings.push(LlmValidationWarning {
                rule: "ISO-29148-RISK".into(),
                element_id: fr.id.clone(),
                severity: WarningSeverity::Info,
                message: format!("{}: exigence P1 sans niveau de risque", fr.id),
            });
        }

        // 7. NFR sans quality_characteristic
        if fr.category == RequirementCategory::NonFunctional && fr.quality_characteristic.is_none()
        {
            warnings.push(LlmValidationWarning {
                rule: "ISO-25010-NFR".into(),
                element_id: fr.id.clone(),
                severity: WarningSeverity::Info,
                message: format!(
                    "{}: exigence non-fonctionnelle sans quality_characteristic ISO 25010",
                    fr.id
                ),
            });
        }
    }

    // 8. Verification acceptance scenarios complets (Given+When+Then)
    for us in &spec.user_scenarios {
        for (i, ac) in us.acceptance_scenarios.iter().enumerate() {
            if ac.given.trim().is_empty() || ac.when.trim().is_empty() || ac.then.trim().is_empty()
            {
                warnings.push(LlmValidationWarning {
                    rule: "ISO-29148-TESTABLE".into(),
                    element_id: us.id.clone(),
                    severity: WarningSeverity::Warning,
                    message: format!(
                        "{}: scenario d'acceptation {} incomplet (Given/When/Then vide)",
                        us.id,
                        i + 1
                    ),
                });
            }
        }
    }

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user_story::Priority;

    #[test]
    fn test_clean_json_response() {
        let input = r#"```json
{"key": "value"}
```"#;
        assert_eq!(clean_json_response(input), r#"{"key": "value"}"#);
    }

    #[test]
    fn test_clean_json_response_plain() {
        let input = r#"{"key": "value"}"#;
        assert_eq!(clean_json_response(input), r#"{"key": "value"}"#);
    }

    #[test]
    fn test_clean_json_response_with_text() {
        let input = r#"Here is the JSON:
{"key": "value"}
That's all."#;
        assert_eq!(clean_json_response(input), r#"{"key": "value"}"#);
    }

    #[test]
    fn test_parse_priority() {
        assert_eq!(
            parse_priority("P1"),
            crate::domain::user_story::Priority::P1
        );
        assert_eq!(
            parse_priority("MUST"),
            crate::domain::user_story::Priority::P1
        );
        assert_eq!(
            parse_priority("P2"),
            crate::domain::user_story::Priority::P2
        );
        assert_eq!(
            parse_priority("unknown"),
            crate::domain::user_story::Priority::P3
        );
    }

    #[test]
    fn test_parse_category() {
        assert_eq!(
            parse_category("Functional"),
            RequirementCategory::Functional
        );
        assert_eq!(
            parse_category("non-functional"),
            RequirementCategory::NonFunctional
        );
        assert_eq!(
            parse_category("Constraint"),
            RequirementCategory::Constraint
        );
    }

    #[tokio::test]
    async fn test_refine_with_mock() {
        use crate::adapters::llm::mock_adapter::MockLlmAdapter;
        use crate::adapters::templates::file_template_engine::FileTemplateEngine;
        use crate::domain::user_story::{Language, UserStory, UserStorySet};
        use std::fs;
        use tempfile::TempDir;

        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join("refine_system.md"),
            "System prompt {{language}}",
        )
        .unwrap();

        let mock_response = serde_json::json!({
            "user_scenarios": [{
                "id": "US-001",
                "title": "Recherche ISBN",
                "priority": "P1",
                "description": "Recherche par ISBN",
                "why_priority": "Critique",
                "independent_test": "Saisir ISBN",
                "acceptance_scenarios": [{"given": "catalogue", "when": "saisie ISBN", "then": "resultat"}]
            }],
            "functional_requirements": [{
                "id": "FR-001",
                "statement": "Le systeme DOIT permettre la recherche par ISBN",
                "priority": "P1",
                "category": "Functional",
                "testable": true
            }],
            "key_entities": [],
            "edge_cases": [{"description": "ISBN invalide", "severity": "P2"}],
            "success_criteria": [{"id": "SC-001", "description": "Rapide", "measurable_metric": "< 2s"}],
            "clarifications_needed": []
        });

        let llm = Arc::new(MockLlmAdapter::new(vec![mock_response.to_string()]));
        let templates = Arc::new(FileTemplateEngine::new(dir.path()).unwrap());
        let service = RefineService::new(llm, templates, 1);

        let story_set = UserStorySet {
            stories: vec![UserStory::new(
                "Recherche ISBN".into(),
                "bibliothecaire".into(),
                "rechercher par ISBN".into(),
                "trouver un ouvrage".into(),
            )],
            source_files: vec![],
            language: Language::French,
        };

        let result = service.refine(&story_set, None).await;
        assert!(result.is_ok());
        let spec = result.unwrap();
        assert_eq!(spec.user_scenarios.len(), 1);
        assert_eq!(spec.functional_requirements.len(), 1);
        assert_eq!(spec.edge_cases.len(), 1);
    }

    #[test]
    fn test_clean_json_response_bare_backticks() {
        let input = "```\n{\"key\": \"value\"}\n```";
        assert_eq!(clean_json_response(input), "{\"key\": \"value\"}");
    }

    #[test]
    fn test_clean_json_response_empty() {
        assert_eq!(clean_json_response(""), "");
        assert_eq!(clean_json_response("   "), "");
    }

    #[test]
    fn test_clean_json_response_no_json() {
        assert_eq!(clean_json_response("just text"), "just text");
    }

    #[test]
    fn test_clean_json_response_nested_braces() {
        let input = r#"Some text {"a": {"b": "c"}} more text"#;
        assert_eq!(clean_json_response(input), r#"{"a": {"b": "c"}}"#);
    }

    #[test]
    fn test_parse_priority_all_variants() {
        assert_eq!(parse_priority("P1"), Priority::P1);
        assert_eq!(parse_priority("MUST"), Priority::P1);
        assert_eq!(parse_priority("HAUTE"), Priority::P1);
        assert_eq!(parse_priority("HIGH"), Priority::P1);
        assert_eq!(parse_priority("P2"), Priority::P2);
        assert_eq!(parse_priority("SHOULD"), Priority::P2);
        assert_eq!(parse_priority("MOYENNE"), Priority::P2);
        assert_eq!(parse_priority("MEDIUM"), Priority::P2);
        assert_eq!(parse_priority("P3"), Priority::P3);
        assert_eq!(parse_priority("anything"), Priority::P3);
    }

    #[test]
    fn test_parse_category_all_variants() {
        assert_eq!(
            parse_category("Functional"),
            RequirementCategory::Functional
        );
        assert_eq!(
            parse_category("fonctionnelle"),
            RequirementCategory::Functional
        );
        assert_eq!(
            parse_category("non-functional"),
            RequirementCategory::NonFunctional
        );
        assert_eq!(
            parse_category("NonFunctional"),
            RequirementCategory::NonFunctional
        );
        assert_eq!(
            parse_category("non-fonctionnelle"),
            RequirementCategory::NonFunctional
        );
        assert_eq!(
            parse_category("Constraint"),
            RequirementCategory::Constraint
        );
        assert_eq!(
            parse_category("contrainte"),
            RequirementCategory::Constraint
        );
        assert_eq!(parse_category("unknown"), RequirementCategory::Functional);
    }

    #[test]
    fn test_clean_json_response_brace_before_open() {
        // Regression test from fuzzing: } before { should not panic
        assert_eq!(clean_json_response("}={R"), "}={R");
    }

    // -----------------------------------------------------------------------
    // Helpers pour les tests de merge / batching / validation
    // -----------------------------------------------------------------------

    use crate::domain::specification::{
        AcceptanceScenario, EdgeCase, FunctionalRequirement, Specification, UserScenario,
        VerificationMethod,
    };
    use crate::domain::user_story::UserStory;
    use uuid::Uuid;

    /// Cree une FunctionalRequirement minimale
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

    /// Cree un UserScenario minimal
    fn make_us(id: &str) -> UserScenario {
        UserScenario {
            id: id.to_string(),
            title: format!("Scenario {}", id),
            priority: Priority::P2,
            description: "Description".to_string(),
            why_priority: String::new(),
            independent_test: String::new(),
            acceptance_scenarios: vec![],
            source_story_id: Uuid::new_v4(),
        }
    }

    // -----------------------------------------------------------------------
    // Tests build_batches()
    // -----------------------------------------------------------------------

    #[test]
    fn test_build_batches_empty() {
        let stories: Vec<UserStory> = vec![];
        assert!(build_batches(&stories, 1000).is_empty());
    }

    #[test]
    fn test_build_batches_single_exceeds_budget() {
        // Une story de ~1000 tokens, budget de 100 → doit quand meme etre dans 1 batch
        let story = UserStory::new(
            "x".repeat(4000), // ~1000 tokens
            String::new(),
            String::new(),
            String::new(),
        );
        let batches = build_batches(&[story], 100);
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0], vec![0]);
    }

    #[test]
    fn test_build_batches_exact_boundary() {
        // 2 stories de 200 tokens chacune, budget 400 → 1 seul batch
        let s1 = UserStory::new("a".repeat(800), String::new(), String::new(), String::new());
        let s2 = UserStory::new("b".repeat(800), String::new(), String::new(), String::new());
        let batches = build_batches(&[s1, s2], 400);
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0], vec![0, 1]);
    }

    #[test]
    fn test_build_batches_multiple() {
        // 4 stories de ~200 tokens, budget 300 → 2+ batches
        let stories: Vec<UserStory> = (0..4)
            .map(|_| UserStory::new("x".repeat(800), String::new(), String::new(), String::new()))
            .collect();
        let batches = build_batches(&stories, 300);
        assert!(
            batches.len() >= 2,
            "Attendu 2+ batches, obtenu {}",
            batches.len()
        );
        // Verifier que tous les indices sont presents
        let all_indices: Vec<usize> = batches.iter().flatten().copied().collect();
        assert_eq!(all_indices, vec![0, 1, 2, 3]);
    }

    // -----------------------------------------------------------------------
    // Tests merge_specifications()
    // -----------------------------------------------------------------------

    #[test]
    fn test_merge_specs_empty() {
        let merged = RefineService::merge_specifications(vec![]);
        assert_eq!(merged.functional_requirements.len(), 0);
        assert!(merged.title.contains("Empty"));
    }

    #[test]
    fn test_merge_specs_single() {
        let mut spec = Specification::new("Test".into());
        spec.functional_requirements.push(make_fr(
            "FR-001",
            "Le systeme DOIT faire X",
            Priority::P1,
        ));
        let merged = RefineService::merge_specifications(vec![spec.clone()]);
        assert_eq!(merged.functional_requirements.len(), 1);
        assert_eq!(merged.functional_requirements[0].id, "FR-001");
    }

    #[test]
    fn test_merge_specs_id_renumbering() {
        let mut spec1 = Specification::new("Spec 1".into());
        spec1
            .functional_requirements
            .push(make_fr("FR-001", "DOIT A", Priority::P1));
        spec1
            .functional_requirements
            .push(make_fr("FR-002", "DOIT B", Priority::P2));

        let mut spec2 = Specification::new("Spec 2".into());
        spec2
            .functional_requirements
            .push(make_fr("FR-001", "DOIT C", Priority::P1));
        spec2
            .functional_requirements
            .push(make_fr("FR-002", "DOIT D", Priority::P2));

        let merged = RefineService::merge_specifications(vec![spec1, spec2]);
        assert_eq!(merged.functional_requirements.len(), 4);
        // IDs monotones FR-001..FR-004
        let ids: Vec<&str> = merged
            .functional_requirements
            .iter()
            .map(|f| f.id.as_str())
            .collect();
        assert_eq!(ids, vec!["FR-001", "FR-002", "FR-003", "FR-004"]);
    }

    #[test]
    fn test_merge_specs_parent_cross_refs() {
        // Batch 1: FR-001 (racine)
        let mut spec1 = Specification::new("Spec 1".into());
        spec1
            .functional_requirements
            .push(make_fr("FR-001", "DOIT racine", Priority::P1));

        // Batch 2: FR-001 (parent = FR-001 du batch 2), FR-002 (parent = FR-001 du batch 1 — forward ref)
        let mut spec2 = Specification::new("Spec 2".into());
        let mut fr_a = make_fr("FR-001", "DOIT enfant A", Priority::P2);
        fr_a.parent_requirement = Some("FR-002".to_string()); // reference forward dans le batch
        spec2.functional_requirements.push(fr_a);
        let mut fr_b = make_fr("FR-002", "DOIT enfant B", Priority::P2);
        fr_b.parent_requirement = None;
        spec2.functional_requirements.push(fr_b);

        let merged = RefineService::merge_specifications(vec![spec1, spec2]);
        // spec1 FR-001 → FR-001, spec2 FR-001 → FR-002, spec2 FR-002 → FR-003
        assert_eq!(merged.functional_requirements[0].id, "FR-001");
        assert_eq!(merged.functional_requirements[1].id, "FR-002");
        assert_eq!(merged.functional_requirements[2].id, "FR-003");
        // FR-002 avait parent "FR-002" (old) → doit etre remappe vers "FR-003" (new)
        assert_eq!(
            merged.functional_requirements[1]
                .parent_requirement
                .as_deref(),
            Some("FR-003"),
            "Le parent_requirement doit etre remappe via le mapping global"
        );
    }

    #[test]
    fn test_merge_specs_edge_case_us_mapping() {
        let mut spec1 = Specification::new("Spec 1".into());
        spec1.user_scenarios.push(make_us("US-001"));

        let mut spec2 = Specification::new("Spec 2".into());
        spec2.user_scenarios.push(make_us("US-001"));
        spec2.edge_cases.push(EdgeCase {
            description: "Cas limite".into(),
            related_scenario: Some("US-001".into()),
            severity: Priority::P2,
        });

        let merged = RefineService::merge_specifications(vec![spec1, spec2]);
        // spec2.US-001 → US-002
        assert_eq!(merged.user_scenarios[1].id, "US-002");
        // Edge case related_scenario doit pointer vers US-002
        assert_eq!(
            merged.edge_cases[0].related_scenario.as_deref(),
            Some("US-002"),
            "Le related_scenario doit etre remappe"
        );
    }

    // -----------------------------------------------------------------------
    // Tests validate_llm_spec_output()
    // -----------------------------------------------------------------------

    #[test]
    fn test_validate_duplicate_ids() {
        let mut spec = Specification::new("Test".into());
        spec.functional_requirements
            .push(make_fr("FR-001", "Le systeme DOIT A", Priority::P1));
        spec.functional_requirements
            .push(make_fr("FR-001", "Le systeme DOIT B", Priority::P2));
        let warnings = validate_llm_spec_output(&spec);
        assert!(warnings.iter().any(|w| w.rule == "ISO-29148-ID-UNIQUE"));
    }

    #[test]
    fn test_validate_missing_normative() {
        let mut spec = Specification::new("Test".into());
        spec.functional_requirements.push(make_fr(
            "FR-001",
            "Le systeme fait quelque chose",
            Priority::P1,
        ));
        let warnings = validate_llm_spec_output(&spec);
        assert!(warnings.iter().any(|w| w.rule == "ISO-29148-NORMATIVE"));
    }

    #[test]
    fn test_validate_ambiguous_word() {
        let mut spec = Specification::new("Test".into());
        spec.functional_requirements.push(make_fr(
            "FR-001",
            "Le systeme DOIT parfois fonctionner",
            Priority::P1,
        ));
        let warnings = validate_llm_spec_output(&spec);
        assert!(warnings.iter().any(|w| w.rule == "ISO-29148-UNAMBIGUOUS"));
    }

    #[test]
    fn test_validate_invalid_id_format() {
        let mut spec = Specification::new("Test".into());
        spec.functional_requirements.push(make_fr(
            "REQ-001",
            "Le systeme DOIT fonctionner",
            Priority::P1,
        ));
        let warnings = validate_llm_spec_output(&spec);
        assert!(warnings.iter().any(|w| w.rule == "ISO-29148-CONFORMING"));
    }

    #[test]
    fn test_validate_empty_statement() {
        let mut spec = Specification::new("Test".into());
        spec.functional_requirements
            .push(make_fr("FR-001", "   ", Priority::P1));
        let warnings = validate_llm_spec_output(&spec);
        assert!(warnings.iter().any(|w| w.rule == "ISO-29148-COMPLETE"));
    }

    #[test]
    fn test_validate_p1_no_risk() {
        let mut spec = Specification::new("Test".into());
        // P1 sans risk_level → warning info
        spec.functional_requirements.push(make_fr(
            "FR-001",
            "Le systeme DOIT fonctionner",
            Priority::P1,
        ));
        let warnings = validate_llm_spec_output(&spec);
        assert!(warnings.iter().any(|w| w.rule == "ISO-29148-RISK"));
    }

    #[test]
    fn test_validate_incomplete_acceptance() {
        let mut spec = Specification::new("Test".into());
        let mut us = make_us("US-001");
        us.acceptance_scenarios.push(AcceptanceScenario {
            given: "contexte".into(),
            when: "".into(), // incomplet
            then: "resultat".into(),
        });
        spec.user_scenarios.push(us);
        let warnings = validate_llm_spec_output(&spec);
        assert!(warnings.iter().any(|w| w.rule == "ISO-29148-TESTABLE"));
    }

    // -----------------------------------------------------------------------
    // Tests parse helpers
    // -----------------------------------------------------------------------

    #[test]
    fn test_parse_verification_method_all_variants() {
        assert_eq!(
            parse_verification_method("inspection"),
            VerificationMethod::Inspection
        );
        assert_eq!(
            parse_verification_method("analysis"),
            VerificationMethod::Analysis
        );
        assert_eq!(
            parse_verification_method("analyse"),
            VerificationMethod::Analysis
        );
        assert_eq!(
            parse_verification_method("demonstration"),
            VerificationMethod::Demonstration
        );
        assert_eq!(
            parse_verification_method("demo"),
            VerificationMethod::Demonstration
        );
        assert_eq!(parse_verification_method("test"), VerificationMethod::Test);
        assert_eq!(
            parse_verification_method("unknown"),
            VerificationMethod::Test
        );
    }

    #[test]
    fn test_parse_risk_level_all_variants() {
        use crate::domain::specification::RiskLevel;
        assert_eq!(parse_risk_level("high"), RiskLevel::High);
        assert_eq!(parse_risk_level("haute"), RiskLevel::High);
        assert_eq!(parse_risk_level("critique"), RiskLevel::High);
        assert_eq!(parse_risk_level("medium"), RiskLevel::Medium);
        assert_eq!(parse_risk_level("moyenne"), RiskLevel::Medium);
        assert_eq!(parse_risk_level("low"), RiskLevel::Low);
        assert_eq!(parse_risk_level("basse"), RiskLevel::Low);
        assert_eq!(parse_risk_level("mineur"), RiskLevel::Low);
        assert_eq!(parse_risk_level("unknown"), RiskLevel::Medium);
    }

    #[test]
    fn test_parse_quality_characteristic_all_variants() {
        use crate::domain::specification::QualityCharacteristic;
        assert_eq!(
            parse_quality_characteristic("FunctionalSuitability"),
            QualityCharacteristic::FunctionalSuitability
        );
        assert_eq!(
            parse_quality_characteristic("performance"),
            QualityCharacteristic::PerformanceEfficiency
        );
        assert_eq!(
            parse_quality_characteristic("compatibility"),
            QualityCharacteristic::Compatibility
        );
        assert_eq!(
            parse_quality_characteristic("usability"),
            QualityCharacteristic::InteractionCapability
        );
        assert_eq!(
            parse_quality_characteristic("reliability"),
            QualityCharacteristic::Reliability
        );
        assert_eq!(
            parse_quality_characteristic("fiabilite"),
            QualityCharacteristic::Reliability
        );
        assert_eq!(
            parse_quality_characteristic("security"),
            QualityCharacteristic::Security
        );
        assert_eq!(
            parse_quality_characteristic("securite"),
            QualityCharacteristic::Security
        );
        assert_eq!(
            parse_quality_characteristic("maintainability"),
            QualityCharacteristic::Maintainability
        );
        assert_eq!(
            parse_quality_characteristic("flexibility"),
            QualityCharacteristic::Flexibility
        );
        assert_eq!(
            parse_quality_characteristic("portability"),
            QualityCharacteristic::Flexibility
        );
        assert_eq!(
            parse_quality_characteristic("safety"),
            QualityCharacteristic::Safety
        );
        assert_eq!(
            parse_quality_characteristic("unknown"),
            QualityCharacteristic::FunctionalSuitability
        );
    }

    // -----------------------------------------------------------------------
    // Test tolerance aux champs manquants (deserialization LLM)
    // -----------------------------------------------------------------------

    #[test]
    fn test_deserialize_missing_description() {
        let json = r#"{
            "user_scenarios": [{"id": "US-001", "title": "Test", "priority": "P1"}],
            "functional_requirements": [],
            "key_entities": [{"name": "Entity"}],
            "edge_cases": [{"severity": "P2"}],
            "success_criteria": [{"id": "SC-001"}],
            "clarifications_needed": []
        }"#;
        let result: Result<LlmRefineOutput, _> = serde_json::from_str(json);
        assert!(
            result.is_ok(),
            "Parsing DOIT reussir meme sans description: {:?}",
            result.err()
        );
        let output = result.unwrap();
        assert_eq!(output.user_scenarios.len(), 1);
        assert_eq!(output.user_scenarios[0].description, "");
        assert_eq!(output.key_entities[0].description, "");
        assert_eq!(output.edge_cases[0].description, "");
        assert_eq!(output.success_criteria[0].description, "");
    }

    mod proptest_suite {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn clean_json_never_panics(input in "\\PC*") {
                let _ = clean_json_response(&input);
            }

            #[test]
            fn parse_priority_never_panics(input in "\\PC*") {
                let _ = parse_priority(&input);
            }

            #[test]
            fn parse_category_never_panics(input in "\\PC*") {
                let _ = parse_category(&input);
            }

            #[test]
            fn parse_verification_method_never_panics(input in "\\PC*") {
                let _ = parse_verification_method(&input);
            }

            #[test]
            fn parse_risk_level_never_panics(input in "\\PC*") {
                let _ = parse_risk_level(&input);
            }

            #[test]
            fn parse_quality_characteristic_never_panics(input in "\\PC*") {
                let _ = parse_quality_characteristic(&input);
            }
        }
    }
}
