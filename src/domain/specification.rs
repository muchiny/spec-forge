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
    use pretty_assertions::assert_eq;

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
        assert_eq!(SpecStatus::Validated.to_string(), "Validated");
    }

    #[test]
    fn test_specification_defaults() {
        let spec = Specification::new("Defaults".into());
        assert_eq!(spec.version, "1.0.0");
        assert_eq!(spec.tool_version, env!("CARGO_PKG_VERSION"));
        assert!(spec.baseline.is_none());
        assert!(spec.author.is_none());
        assert!(spec.compliance_profile.is_none());
        assert!(spec.validation.is_none());
        assert!(spec.source_stories.is_empty());
    }

    #[test]
    fn test_resolved_clarifications() {
        let mut spec = Specification::new("Test".into());
        spec.clarifications_needed.push(Clarification {
            question: "Quel protocole ?".into(),
            context: "FR-001".into(),
            suggested_options: vec!["REST".into(), "gRPC".into()],
            impact: "Interface API".into(),
            resolved: true,
            answer: Some("REST".into()),
        });
        assert!(!spec.has_unresolved_clarifications());
    }

    #[test]
    fn test_total_requirements() {
        let mut spec = Specification::new("Test".into());
        assert_eq!(spec.total_requirements(), 0);

        spec.functional_requirements.push(FunctionalRequirement {
            id: "FR-001".into(),
            statement: "System MUST authenticate".into(),
            priority: Priority::P1,
            category: RequirementCategory::Functional,
            testable: true,
            rationale: None,
            source: None,
            verification_method: VerificationMethod::default(),
            risk_level: None,
            parent_requirement: None,
            allocated_to: Vec::new(),
            quality_characteristic: None,
        });
        assert_eq!(spec.total_requirements(), 1);
    }

    // --- Tests Display pour toutes les variantes d'enum ---

    #[test]
    fn test_verification_method_display() {
        assert_eq!(VerificationMethod::Inspection.to_string(), "Inspection");
        assert_eq!(VerificationMethod::Analysis.to_string(), "Analysis");
        assert_eq!(
            VerificationMethod::Demonstration.to_string(),
            "Demonstration"
        );
        assert_eq!(VerificationMethod::Test.to_string(), "Test");
    }

    #[test]
    fn test_verification_method_default() {
        assert_eq!(VerificationMethod::default(), VerificationMethod::Test);
    }

    #[test]
    fn test_risk_level_display() {
        assert_eq!(RiskLevel::High.to_string(), "High");
        assert_eq!(RiskLevel::Medium.to_string(), "Medium");
        assert_eq!(RiskLevel::Low.to_string(), "Low");
    }

    #[test]
    fn test_quality_characteristic_display_all_variants() {
        let expected = vec![
            (
                QualityCharacteristic::FunctionalSuitability,
                "Functional Suitability",
            ),
            (
                QualityCharacteristic::PerformanceEfficiency,
                "Performance Efficiency",
            ),
            (QualityCharacteristic::Compatibility, "Compatibility"),
            (
                QualityCharacteristic::InteractionCapability,
                "Interaction Capability",
            ),
            (QualityCharacteristic::Reliability, "Reliability"),
            (QualityCharacteristic::Security, "Security"),
            (QualityCharacteristic::Maintainability, "Maintainability"),
            (QualityCharacteristic::Flexibility, "Flexibility"),
            (QualityCharacteristic::Safety, "Safety"),
        ];
        for (variant, label) in expected {
            assert_eq!(variant.to_string(), label);
        }
    }

    #[test]
    fn test_requirement_category_display() {
        assert_eq!(RequirementCategory::Functional.to_string(), "Fonctionnelle");
        assert_eq!(
            RequirementCategory::NonFunctional.to_string(),
            "Non-fonctionnelle"
        );
        assert_eq!(RequirementCategory::Constraint.to_string(), "Contrainte");
    }

    // --- Tests serialisation round-trip profils de conformite ---

    #[test]
    fn test_compliance_profile_general_serde() {
        let profile = ComplianceProfile::General;
        let json = serde_json::to_string(&profile).unwrap();
        let deserialized: ComplianceProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(profile, deserialized);
    }

    #[test]
    fn test_compliance_profile_aviation_serde() {
        for level in [
            DalLevel::A,
            DalLevel::B,
            DalLevel::C,
            DalLevel::D,
            DalLevel::E,
        ] {
            let profile = ComplianceProfile::Aviation(level);
            let json = serde_json::to_string(&profile).unwrap();
            let deserialized: ComplianceProfile = serde_json::from_str(&json).unwrap();
            assert_eq!(
                profile, deserialized,
                "Round-trip echec pour Aviation({level:?})"
            );
        }
    }

    #[test]
    fn test_compliance_profile_medical_serde() {
        for class in [SwClass::A, SwClass::B, SwClass::C] {
            let profile = ComplianceProfile::Medical(class);
            let json = serde_json::to_string(&profile).unwrap();
            let deserialized: ComplianceProfile = serde_json::from_str(&json).unwrap();
            assert_eq!(
                profile, deserialized,
                "Round-trip echec pour Medical({class:?})"
            );
        }
    }

    #[test]
    fn test_compliance_profile_automotive_serde() {
        for level in [AsilLevel::A, AsilLevel::B, AsilLevel::C, AsilLevel::D] {
            let profile = ComplianceProfile::Automotive(level);
            let json = serde_json::to_string(&profile).unwrap();
            let deserialized: ComplianceProfile = serde_json::from_str(&json).unwrap();
            assert_eq!(
                profile, deserialized,
                "Round-trip echec pour Automotive({level:?})"
            );
        }
    }

    #[test]
    fn test_compliance_profile_railway_serde() {
        for level in [
            SsilLevel::Level0,
            SsilLevel::Level1,
            SsilLevel::Level2,
            SsilLevel::Level3,
            SsilLevel::Level4,
        ] {
            let profile = ComplianceProfile::Railway(level);
            let json = serde_json::to_string(&profile).unwrap();
            let deserialized: ComplianceProfile = serde_json::from_str(&json).unwrap();
            assert_eq!(
                profile, deserialized,
                "Round-trip echec pour Railway({level:?})"
            );
        }
    }

    #[test]
    fn test_compliance_profile_safety_serde() {
        for level in [
            SilLevel::Sil1,
            SilLevel::Sil2,
            SilLevel::Sil3,
            SilLevel::Sil4,
        ] {
            let profile = ComplianceProfile::Safety(level);
            let json = serde_json::to_string(&profile).unwrap();
            let deserialized: ComplianceProfile = serde_json::from_str(&json).unwrap();
            assert_eq!(
                profile, deserialized,
                "Round-trip echec pour Safety({level:?})"
            );
        }
    }

    // --- Tests serialisation round-trip types composites ---

    #[test]
    fn test_functional_requirement_serde_full() {
        let fr = FunctionalRequirement {
            id: "FR-001".into(),
            statement: "Le systeme DOIT authentifier les utilisateurs".into(),
            priority: Priority::P1,
            category: RequirementCategory::Functional,
            testable: true,
            rationale: Some("Securite critique".into()),
            source: Some("US-001".into()),
            verification_method: VerificationMethod::Test,
            risk_level: Some(RiskLevel::High),
            parent_requirement: Some("FR-000".into()),
            allocated_to: vec!["Module Auth".into(), "Module API".into()],
            quality_characteristic: Some(QualityCharacteristic::Security),
        };
        let json = serde_json::to_string(&fr).unwrap();
        let deserialized: FunctionalRequirement = serde_json::from_str(&json).unwrap();
        assert_eq!(fr.id, deserialized.id);
        assert_eq!(fr.statement, deserialized.statement);
        assert_eq!(fr.priority, deserialized.priority);
        assert_eq!(fr.rationale, deserialized.rationale);
        assert_eq!(fr.risk_level, deserialized.risk_level);
        assert_eq!(fr.allocated_to, deserialized.allocated_to);
        assert_eq!(
            fr.quality_characteristic,
            deserialized.quality_characteristic
        );
    }

    #[test]
    fn test_functional_requirement_serde_minimal() {
        let fr = FunctionalRequirement {
            id: "FR-001".into(),
            statement: "System MUST work".into(),
            priority: Priority::P3,
            category: RequirementCategory::Constraint,
            testable: false,
            rationale: None,
            source: None,
            verification_method: VerificationMethod::default(),
            risk_level: None,
            parent_requirement: None,
            allocated_to: Vec::new(),
            quality_characteristic: None,
        };
        let json = serde_json::to_string(&fr).unwrap();
        let deserialized: FunctionalRequirement = serde_json::from_str(&json).unwrap();
        assert_eq!(fr.id, deserialized.id);
        assert!(deserialized.rationale.is_none());
        assert!(deserialized.risk_level.is_none());
        assert!(deserialized.allocated_to.is_empty());
    }

    #[test]
    fn test_acceptance_scenario_serde() {
        let scenario = AcceptanceScenario {
            given: "un utilisateur connecte".into(),
            when: "il clique sur deconnexion".into(),
            then: "la session est terminee".into(),
        };
        let json = serde_json::to_string(&scenario).unwrap();
        let deserialized: AcceptanceScenario = serde_json::from_str(&json).unwrap();
        assert_eq!(scenario.given, deserialized.given);
        assert_eq!(scenario.when, deserialized.when);
        assert_eq!(scenario.then, deserialized.then);
    }

    #[test]
    fn test_key_entity_serde() {
        let entity = KeyEntity {
            name: "Utilisateur".into(),
            description: "Entite representant un utilisateur du systeme".into(),
            attributes: vec!["nom".into(), "email".into(), "role".into()],
            relationships: vec!["possede des Commandes".into()],
        };
        let json = serde_json::to_string(&entity).unwrap();
        let deserialized: KeyEntity = serde_json::from_str(&json).unwrap();
        assert_eq!(entity.name, deserialized.name);
        assert_eq!(entity.attributes, deserialized.attributes);
        assert_eq!(entity.relationships, deserialized.relationships);
    }

    #[test]
    fn test_edge_case_serde() {
        let edge = EdgeCase {
            description: "Utilisateur avec caracteres speciaux dans le nom".into(),
            related_scenario: Some("US-001".into()),
            severity: Priority::P2,
        };
        let json = serde_json::to_string(&edge).unwrap();
        let deserialized: EdgeCase = serde_json::from_str(&json).unwrap();
        assert_eq!(edge.description, deserialized.description);
        assert_eq!(edge.related_scenario, deserialized.related_scenario);
    }

    #[test]
    fn test_success_criterion_serde() {
        let criterion = SuccessCriterion {
            id: "SC-001".into(),
            description: "Temps de reponse acceptable".into(),
            measurable_metric: "< 200ms au P95".into(),
        };
        let json = serde_json::to_string(&criterion).unwrap();
        let deserialized: SuccessCriterion = serde_json::from_str(&json).unwrap();
        assert_eq!(criterion.id, deserialized.id);
        assert_eq!(criterion.measurable_metric, deserialized.measurable_metric);
    }

    #[test]
    fn test_clarification_serde_resolved() {
        let clarif = Clarification {
            question: "Quel format de token ?".into(),
            context: "FR-006".into(),
            suggested_options: vec!["JWT".into(), "opaque".into()],
            impact: "Securite et performance".into(),
            resolved: true,
            answer: Some("JWT".into()),
        };
        let json = serde_json::to_string(&clarif).unwrap();
        let deserialized: Clarification = serde_json::from_str(&json).unwrap();
        assert!(deserialized.resolved);
        assert_eq!(deserialized.answer, Some("JWT".into()));
    }

    #[test]
    fn test_clarification_serde_unresolved() {
        let clarif = Clarification {
            question: "Limite de taille ?".into(),
            context: "FR-003".into(),
            suggested_options: vec![],
            impact: "Stockage".into(),
            resolved: false,
            answer: None,
        };
        let json = serde_json::to_string(&clarif).unwrap();
        let deserialized: Clarification = serde_json::from_str(&json).unwrap();
        assert!(!deserialized.resolved);
        assert!(deserialized.answer.is_none());
    }

    #[test]
    fn test_spec_validation_serde() {
        let validation = SpecValidation {
            completeness_score: 0.85,
            clarity_score: 0.92,
            testability_score: 0.78,
            checklist_items: vec![
                ChecklistItem {
                    description: "Exigences testables".into(),
                    passed: true,
                    category: "Testabilite".into(),
                },
                ChecklistItem {
                    description: "Mots normatifs presents".into(),
                    passed: false,
                    category: "Clarte".into(),
                },
            ],
        };
        let json = serde_json::to_string(&validation).unwrap();
        let deserialized: SpecValidation = serde_json::from_str(&json).unwrap();
        assert!((deserialized.completeness_score - 0.85).abs() < f32::EPSILON);
        assert_eq!(deserialized.checklist_items.len(), 2);
        assert!(deserialized.checklist_items[0].passed);
        assert!(!deserialized.checklist_items[1].passed);
    }

    #[test]
    fn test_specification_full_serde_roundtrip() {
        let mut spec = Specification::new("Test complet".into());
        spec.compliance_profile = Some(ComplianceProfile::Aviation(DalLevel::B));
        spec.user_scenarios.push(UserScenario {
            id: "US-001".into(),
            title: "Connexion".into(),
            priority: Priority::P1,
            description: "Authentification utilisateur".into(),
            why_priority: "Securite fondamentale".into(),
            independent_test: "Testable en isolation".into(),
            acceptance_scenarios: vec![AcceptanceScenario {
                given: "un utilisateur enregistre".into(),
                when: "il saisit ses identifiants".into(),
                then: "il est connecte".into(),
            }],
            source_story_id: Uuid::new_v4(),
        });
        spec.functional_requirements.push(FunctionalRequirement {
            id: "FR-001".into(),
            statement: "Le systeme DOIT verifier les identifiants".into(),
            priority: Priority::P1,
            category: RequirementCategory::Functional,
            testable: true,
            rationale: Some("Securite".into()),
            source: Some("US-001".into()),
            verification_method: VerificationMethod::Test,
            risk_level: Some(RiskLevel::High),
            parent_requirement: None,
            allocated_to: vec!["Auth".into()],
            quality_characteristic: Some(QualityCharacteristic::Security),
        });
        spec.key_entities.push(KeyEntity {
            name: "Utilisateur".into(),
            description: "Entite principale".into(),
            attributes: vec!["email".into()],
            relationships: vec![],
        });
        spec.edge_cases.push(EdgeCase {
            description: "Mot de passe vide".into(),
            related_scenario: Some("US-001".into()),
            severity: Priority::P1,
        });
        spec.success_criteria.push(SuccessCriterion {
            id: "SC-001".into(),
            description: "Auth < 500ms".into(),
            measurable_metric: "P95 < 500ms".into(),
        });

        let json = serde_json::to_string(&spec).unwrap();
        let deserialized: Specification = serde_json::from_str(&json).unwrap();

        assert_eq!(spec.id, deserialized.id);
        assert_eq!(spec.title, deserialized.title);
        assert_eq!(spec.status, deserialized.status);
        assert_eq!(spec.compliance_profile, deserialized.compliance_profile);
        assert_eq!(spec.user_scenarios.len(), deserialized.user_scenarios.len());
        assert_eq!(
            spec.functional_requirements.len(),
            deserialized.functional_requirements.len()
        );
        assert_eq!(spec.key_entities.len(), deserialized.key_entities.len());
        assert_eq!(spec.edge_cases.len(), deserialized.edge_cases.len());
        assert_eq!(
            spec.success_criteria.len(),
            deserialized.success_criteria.len()
        );
    }

    // --- Tests edge cases ---

    #[test]
    fn test_specification_empty_is_valid() {
        let spec = Specification::new("Vide".into());
        assert_eq!(spec.total_requirements(), 0);
        assert!(!spec.has_unresolved_clarifications());
        assert!(spec.user_scenarios.is_empty());
        assert!(spec.key_entities.is_empty());
        assert!(spec.edge_cases.is_empty());
        assert!(spec.success_criteria.is_empty());
    }

    #[test]
    fn test_ssil_level_serde_with_rename() {
        // SsilLevel utilise #[serde(rename = "0")] etc.
        let level = SsilLevel::Level0;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"0\"");
        let deserialized: SsilLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(level, deserialized);

        let level4 = SsilLevel::Level4;
        let json4 = serde_json::to_string(&level4).unwrap();
        assert_eq!(json4, "\"4\"");
    }

    #[test]
    fn test_sil_level_serde_with_rename() {
        let level = SilLevel::Sil1;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"1\"");
        let deserialized: SilLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(level, deserialized);

        let level4 = SilLevel::Sil4;
        let json4 = serde_json::to_string(&level4).unwrap();
        assert_eq!(json4, "\"4\"");
    }
}
