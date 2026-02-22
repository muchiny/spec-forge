//! Modele de domaine - Specification raffinee
//!
//! Represente une specification produite a partir des User Stories,
//! suivant le format spec-kit (SDD).
//!
//! Conformite : ISO/IEC/IEEE 29148:2018, ISO/IEC 25010:2023

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user_story::Priority;

// ---------------------------------------------------------------------------
// Enums ISO 29148 / 25010 / Safety standards
// ---------------------------------------------------------------------------

/// Methode de verification (IEEE 1012 / ISO 29148)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum VerificationMethod {
    /// Revue visuelle/documentaire
    Inspection,
    /// Analyse formelle/mathematique
    Analysis,
    /// Execution avec observation
    Demonstration,
    /// Execution avec verification automatisee
    #[default]
    Test,
}

impl std::fmt::Display for VerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationMethod::Inspection => write!(f, "Inspection"),
            VerificationMethod::Analysis => write!(f, "Analysis"),
            VerificationMethod::Demonstration => write!(f, "Demonstration"),
            VerificationMethod::Test => write!(f, "Test"),
        }
    }
}

/// Niveau de risque (ISO 29148 section 5.2.8)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Impact critique
    High,
    /// Impact significatif
    Medium,
    /// Impact mineur
    Low,
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::High => write!(f, "High"),
            RiskLevel::Medium => write!(f, "Medium"),
            RiskLevel::Low => write!(f, "Low"),
        }
    }
}

/// Caracteristique qualite produit (ISO/IEC 25010:2023 — 9 caracteristiques)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityCharacteristic {
    FunctionalSuitability,
    PerformanceEfficiency,
    Compatibility,
    InteractionCapability,
    Reliability,
    Security,
    Maintainability,
    Flexibility,
    Safety,
}

impl std::fmt::Display for QualityCharacteristic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QualityCharacteristic::FunctionalSuitability => write!(f, "Functional Suitability"),
            QualityCharacteristic::PerformanceEfficiency => write!(f, "Performance Efficiency"),
            QualityCharacteristic::Compatibility => write!(f, "Compatibility"),
            QualityCharacteristic::InteractionCapability => write!(f, "Interaction Capability"),
            QualityCharacteristic::Reliability => write!(f, "Reliability"),
            QualityCharacteristic::Security => write!(f, "Security"),
            QualityCharacteristic::Maintainability => write!(f, "Maintainability"),
            QualityCharacteristic::Flexibility => write!(f, "Flexibility"),
            QualityCharacteristic::Safety => write!(f, "Safety"),
        }
    }
}

/// Niveau DAL — DO-178C (aviation)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DalLevel {
    A,
    B,
    C,
    D,
    E,
}

/// Classe logicielle — IEC 62304 (medical)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwClass {
    A,
    B,
    C,
}

/// Niveau ASIL — ISO 26262 (automobile)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AsilLevel {
    A,
    B,
    C,
    D,
}

/// Niveau SSIL — EN 50716 (ferroviaire)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SsilLevel {
    #[serde(rename = "0")]
    Level0,
    #[serde(rename = "1")]
    Level1,
    #[serde(rename = "2")]
    Level2,
    #[serde(rename = "3")]
    Level3,
    #[serde(rename = "4")]
    Level4,
}

/// Niveau SIL — IEC 61508 (securite fonctionnelle)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SilLevel {
    #[serde(rename = "1")]
    Sil1,
    #[serde(rename = "2")]
    Sil2,
    #[serde(rename = "3")]
    Sil3,
    #[serde(rename = "4")]
    Sil4,
}

/// Profil de conformite (domaine reglementaire)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceProfile {
    /// ISO 29148 basique
    General,
    /// DO-178C (aviation)
    Aviation(DalLevel),
    /// IEC 62304 (medical)
    Medical(SwClass),
    /// ISO 26262 (automobile)
    Automotive(AsilLevel),
    /// EN 50716 (ferroviaire)
    Railway(SsilLevel),
    /// IEC 61508 (securite fonctionnelle)
    Safety(SilLevel),
}

/// Specification raffinee produite depuis les user stories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specification {
    /// Identifiant unique
    pub id: Uuid,

    /// Titre de la specification
    pub title: String,

    /// Date de creation
    pub created_at: DateTime<Utc>,

    /// Statut
    pub status: SpecStatus,

    /// Version semantique (ISO 29148: gestion des versions)
    #[serde(default = "default_version")]
    pub version: String,

    /// Identifiant de baseline (ex: "BL-2025-001")
    #[serde(default)]
    pub baseline: Option<String>,

    /// Auteur ou generateur
    #[serde(default)]
    pub author: Option<String>,

    /// Version de l'outil spec-forge
    #[serde(default = "default_tool_version")]
    pub tool_version: String,

    /// Profil de conformite reglementaire
    #[serde(default)]
    pub compliance_profile: Option<ComplianceProfile>,

    /// Scenarios utilisateur raffines avec priorites
    pub user_scenarios: Vec<UserScenario>,

    /// Exigences fonctionnelles (FR-001, FR-002, ...)
    pub functional_requirements: Vec<FunctionalRequirement>,

    /// Entites cles identifiees
    pub key_entities: Vec<KeyEntity>,

    /// Cas limites identifies par le LLM
    pub edge_cases: Vec<EdgeCase>,

    /// Criteres de succes (mesurables)
    pub success_criteria: Vec<SuccessCriterion>,

    /// Clarifications necessaires (ambiguites detectees)
    #[serde(default)]
    pub clarifications_needed: Vec<Clarification>,

    /// Resultats de validation qualite
    #[serde(default)]
    pub validation: Option<SpecValidation>,

    /// Tracabilite : quelles US ont produit cette spec
    pub source_stories: Vec<Uuid>,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

fn default_tool_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

impl Specification {
    /// Cree une specification vide
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            created_at: Utc::now(),
            status: SpecStatus::Draft,
            version: default_version(),
            baseline: None,
            author: None,
            tool_version: default_tool_version(),
            compliance_profile: None,
            user_scenarios: Vec::new(),
            functional_requirements: Vec::new(),
            key_entities: Vec::new(),
            edge_cases: Vec::new(),
            success_criteria: Vec::new(),
            clarifications_needed: Vec::new(),
            validation: None,
            source_stories: Vec::new(),
        }
    }

    /// Verifie si la spec a des clarifications non resolues
    pub fn has_unresolved_clarifications(&self) -> bool {
        self.clarifications_needed.iter().any(|c| !c.resolved)
    }

    /// Nombre total d'exigences
    pub fn total_requirements(&self) -> usize {
        self.functional_requirements.len()
    }
}

/// Statut de la specification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecStatus {
    Draft,
    NeedsClarification,
    Validated,
}

impl std::fmt::Display for SpecStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecStatus::Draft => write!(f, "Draft"),
            SpecStatus::NeedsClarification => write!(f, "Needs Clarification"),
            SpecStatus::Validated => write!(f, "Validated"),
        }
    }
}

/// Scenario utilisateur raffine (mappe depuis UserStory + enrichissement LLM)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserScenario {
    /// Identifiant (ex: "US-001")
    pub id: String,

    /// Titre
    pub title: String,

    /// Priorite
    pub priority: Priority,

    /// Description detaillee
    pub description: String,

    /// Justification de la priorite (spec-kit: "Why this priority")
    pub why_priority: String,

    /// Test independant (spec-kit: "Independent Test")
    pub independent_test: String,

    /// Scenarios d'acceptation (Given/When/Then)
    pub acceptance_scenarios: Vec<AcceptanceScenario>,

    /// ID de la user story source
    pub source_story_id: Uuid,
}

/// Scenario d'acceptation en format BDD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptanceScenario {
    pub given: String,
    pub when: String,
    pub then: String,
}

/// Exigence fonctionnelle (format spec-kit + ISO 29148 section 5.2.8)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionalRequirement {
    /// Identifiant (ex: "FR-001")
    pub id: String,

    /// Enonce ("System MUST ...")
    pub statement: String,

    /// Priorite
    pub priority: Priority,

    /// Categorie
    pub category: RequirementCategory,

    /// L'exigence est-elle testable ?
    pub testable: bool,

    /// Justification — ISO 29148: pourquoi cette exigence existe
    #[serde(default)]
    pub rationale: Option<String>,

    /// Origine (stakeholder, norme, etc.)
    #[serde(default)]
    pub source: Option<String>,

    /// Methode de verification — IEEE 1012 / ISO 29148
    #[serde(default)]
    pub verification_method: VerificationMethod,

    /// Niveau de risque — criticite
    #[serde(default)]
    pub risk_level: Option<RiskLevel>,

    /// Lien hierarchique — derivation d'exigence parente
    #[serde(default)]
    pub parent_requirement: Option<String>,

    /// Allocation architecture/composant
    #[serde(default)]
    pub allocated_to: Vec<String>,

    /// Caracteristique qualite ISO 25010 (pour les NFR)
    #[serde(default)]
    pub quality_characteristic: Option<QualityCharacteristic>,
}

/// Categorie d'exigence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequirementCategory {
    Functional,
    NonFunctional,
    Constraint,
}

impl std::fmt::Display for RequirementCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequirementCategory::Functional => write!(f, "Fonctionnelle"),
            RequirementCategory::NonFunctional => write!(f, "Non-fonctionnelle"),
            RequirementCategory::Constraint => write!(f, "Contrainte"),
        }
    }
}

/// Entite cle du domaine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyEntity {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub attributes: Vec<String>,
    #[serde(default)]
    pub relationships: Vec<String>,
}

/// Cas limite identifie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeCase {
    pub description: String,
    #[serde(default)]
    pub related_scenario: Option<String>,
    pub severity: Priority,
}

/// Critere de succes mesurable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessCriterion {
    /// Identifiant (ex: "SC-001")
    pub id: String,
    pub description: String,
    pub measurable_metric: String,
}

/// Clarification necessaire (ambiguite detectee)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clarification {
    pub question: String,
    pub context: String,
    #[serde(default)]
    pub suggested_options: Vec<String>,
    pub impact: String,
    #[serde(default)]
    pub resolved: bool,
    #[serde(default)]
    pub answer: Option<String>,
}

/// Validation qualite de la specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecValidation {
    /// Score de completude (0.0 - 1.0)
    pub completeness_score: f32,
    /// Score de clarte (0.0 - 1.0)
    pub clarity_score: f32,
    /// Score de testabilite (0.0 - 1.0)
    pub testability_score: f32,
    /// Elements de la checklist
    pub checklist_items: Vec<ChecklistItem>,
}

/// Element de checklist de validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub description: String,
    pub passed: bool,
    pub category: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specification_creation() {
        let spec = Specification::new("Test Feature".into());
        assert!(!spec.id.is_nil());
        assert_eq!(spec.status, SpecStatus::Draft);
        assert_eq!(spec.total_requirements(), 0);
    }

    #[test]
    fn test_unresolved_clarifications() {
        let mut spec = Specification::new("Test".into());
        assert!(!spec.has_unresolved_clarifications());

        spec.clarifications_needed.push(Clarification {
            question: "Quelle auth ?".into(),
            context: "FR-006".into(),
            suggested_options: vec!["OAuth".into(), "JWT".into()],
            impact: "Architecture auth".into(),
            resolved: false,
            answer: None,
        });
        assert!(spec.has_unresolved_clarifications());
    }

    #[test]
    fn test_spec_status_display() {
        assert_eq!(SpecStatus::Draft.to_string(), "Draft");
        assert_eq!(
            SpecStatus::NeedsClarification.to_string(),
            "Needs Clarification"
        );
    }
}
