//! Configuration de l'application
//!
//! Gestion de la configuration via fichier YAML et variables d'environnement.
//! Pattern porte depuis mcp-doc-rag.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::domain::specification::ComplianceProfile;
use crate::ports::llm_service::LlmConfig;

/// Configuration principale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Configuration du pipeline
    pub pipeline: PipelineConfig,

    /// Configuration LLM
    #[serde(default)]
    pub llm: LlmConfig,

    /// Configuration des templates
    pub templates: TemplatesConfig,

    /// Configuration des sorties
    pub output: OutputConfig,

    /// Configuration de la validation
    pub validation: ValidationConfig,

    /// Configuration de la conformite ISO
    #[serde(default)]
    pub compliance: ComplianceConfig,

    /// Configuration du logging
    pub logging: LoggingConfig,

    /// Configuration des chemins
    pub paths: PathsConfig,
}

/// Configuration du pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    /// Nombre max de retries LLM
    pub max_retries: usize,
    /// Langue par defaut
    pub default_language: String,
    /// Budget max de tokens pour le user prompt par batch LLM
    #[serde(default = "default_token_budget")]
    pub token_budget: usize,
}

fn default_token_budget() -> usize {
    2000
}

/// Configuration des templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatesConfig {
    /// Repertoire des templates
    pub directory: PathBuf,
}

/// Configuration des sorties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Format de la spec (markdown)
    pub spec_format: String,
    /// Inclure le rapport de tracabilite
    pub traceability: bool,
    /// Code langue Gherkin
    pub gherkin_language: String,
}

/// Configuration de la validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Seuil de couverture minimum (0-100)
    pub min_coverage_percent: u32,
    /// Valider la syntaxe Gherkin
    pub validate_gherkin_syntax: bool,
    /// Nombre max de clarifications
    pub max_clarifications: usize,
}

/// Configuration du logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Niveau de log
    pub level: String,
    /// Format (text, json)
    pub format: String,
    /// Couleurs
    pub colors: bool,
}

/// Configuration de la conformite ISO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    /// Profil de conformite (general, aviation, medical, automotive, railway, safety)
    #[serde(default = "default_profile")]
    pub profile: String,

    /// Niveau de surete specifique au profil (ex: "DAL-A", "SIL-3", "ASIL-D", "SSIL-2", "SW-B")
    pub safety_level: Option<String>,

    /// Inclure les metadonnees ISO dans les sorties
    #[serde(default = "default_true")]
    pub include_metadata: bool,

    /// Mode strict : les warnings deviennent des erreurs
    #[serde(default)]
    pub strict_validation: bool,

    /// Exiger un rationale sur chaque exigence fonctionnelle
    #[serde(default)]
    pub require_rationale: bool,

    /// Exiger un risk_level sur chaque exigence fonctionnelle
    #[serde(default)]
    pub require_risk_level: bool,

    /// Verifier les mots-cles normatifs (MUST/SHOULD/COULD) dans les exigences
    #[serde(default = "default_true")]
    pub normative_keywords: bool,

    /// Couverture minimale pour les exigences P1 (%)
    #[serde(default = "default_p1_coverage")]
    pub min_p1_coverage: u32,

    /// Couverture minimale pour les exigences P2 (%)
    #[serde(default = "default_p2_coverage")]
    pub min_p2_coverage: u32,

    /// Couverture minimale pour les exigences P3 (%)
    #[serde(default = "default_p3_coverage")]
    pub min_p3_coverage: u32,
}

fn default_profile() -> String {
    "general".to_string()
}

fn default_true() -> bool {
    true
}

fn default_p1_coverage() -> u32 {
    100
}

fn default_p2_coverage() -> u32 {
    80
}

fn default_p3_coverage() -> u32 {
    60
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            profile: default_profile(),
            safety_level: None,
            include_metadata: true,
            strict_validation: false,
            require_rationale: false,
            require_risk_level: false,
            normative_keywords: true,
            min_p1_coverage: default_p1_coverage(),
            min_p2_coverage: default_p2_coverage(),
            min_p3_coverage: default_p3_coverage(),
        }
    }
}

impl ComplianceConfig {
    /// Convertit la configuration en ComplianceProfile du domaine
    pub fn to_compliance_profile(&self) -> Option<ComplianceProfile> {
        use crate::domain::specification::*;

        let safety = self.safety_level.as_deref().map(|s| s.to_uppercase());
        let s = safety.as_deref();

        match self.profile.to_lowercase().as_str() {
            "general" => Some(ComplianceProfile::General),
            "aviation" => {
                let dal = s
                    .and_then(|v| match v {
                        "DAL-A" | "A" => Some(DalLevel::A),
                        "DAL-B" | "B" => Some(DalLevel::B),
                        "DAL-C" | "C" => Some(DalLevel::C),
                        "DAL-D" | "D" => Some(DalLevel::D),
                        "DAL-E" | "E" => Some(DalLevel::E),
                        _ => None,
                    })
                    .unwrap_or(DalLevel::E);
                Some(ComplianceProfile::Aviation(dal))
            }
            "medical" => {
                let sw = s
                    .and_then(|v| match v {
                        "SW-A" | "A" => Some(SwClass::A),
                        "SW-B" | "B" => Some(SwClass::B),
                        "SW-C" | "C" => Some(SwClass::C),
                        _ => None,
                    })
                    .unwrap_or(SwClass::A);
                Some(ComplianceProfile::Medical(sw))
            }
            "automotive" => {
                let asil = s
                    .and_then(|v| match v {
                        "ASIL-A" | "A" => Some(AsilLevel::A),
                        "ASIL-B" | "B" => Some(AsilLevel::B),
                        "ASIL-C" | "C" => Some(AsilLevel::C),
                        "ASIL-D" | "D" => Some(AsilLevel::D),
                        _ => None,
                    })
                    .unwrap_or(AsilLevel::A);
                Some(ComplianceProfile::Automotive(asil))
            }
            "railway" => {
                let ssil = s
                    .and_then(|v| match v {
                        "SSIL-0" | "0" => Some(SsilLevel::Level0),
                        "SSIL-1" | "1" => Some(SsilLevel::Level1),
                        "SSIL-2" | "2" => Some(SsilLevel::Level2),
                        "SSIL-3" | "3" => Some(SsilLevel::Level3),
                        "SSIL-4" | "4" => Some(SsilLevel::Level4),
                        _ => None,
                    })
                    .unwrap_or(SsilLevel::Level0);
                Some(ComplianceProfile::Railway(ssil))
            }
            "safety" => {
                let sil = s
                    .and_then(|v| match v {
                        "SIL-1" | "1" => Some(SilLevel::Sil1),
                        "SIL-2" | "2" => Some(SilLevel::Sil2),
                        "SIL-3" | "3" => Some(SilLevel::Sil3),
                        "SIL-4" | "4" => Some(SilLevel::Sil4),
                        _ => None,
                    })
                    .unwrap_or(SilLevel::Sil1);
                Some(ComplianceProfile::Safety(sil))
            }
            _ => None,
        }
    }
}

/// Configuration des chemins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathsConfig {
    /// Repertoire d'entree
    pub input_dir: PathBuf,
    /// Repertoire de sortie
    pub output_dir: PathBuf,
    /// Repertoire des specs
    pub specs_dir: PathBuf,
    /// Repertoire des features
    pub features_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            pipeline: PipelineConfig {
                max_retries: 2,
                default_language: "fr".to_string(),
                token_budget: default_token_budget(),
            },
            llm: LlmConfig::default(),
            templates: TemplatesConfig {
                directory: PathBuf::from("templates"),
            },
            output: OutputConfig {
                spec_format: "markdown".to_string(),
                traceability: true,
                gherkin_language: "fr".to_string(),
            },
            validation: ValidationConfig {
                min_coverage_percent: 80,
                validate_gherkin_syntax: true,
                max_clarifications: 3,
            },
            compliance: ComplianceConfig::default(),
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "text".to_string(),
                colors: true,
            },
            paths: PathsConfig {
                input_dir: PathBuf::from("input"),
                output_dir: PathBuf::from("output"),
                specs_dir: PathBuf::from("output/specs"),
                features_dir: PathBuf::from("output/features"),
            },
        }
    }
}

impl Config {
    /// Charge la configuration depuis les sources
    pub fn load() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::Config::try_from(&Config::default())?)
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::Environment::with_prefix("SPEC_FORGE").separator("__"))
            .build()
            .context("Echec du chargement de la configuration")?;

        config
            .try_deserialize()
            .context("Echec de la deserialisation de la configuration")
    }

    /// Charge depuis un fichier specifique
    pub fn load_from_file(path: &str) -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::Config::try_from(&Config::default())?)
            .add_source(config::File::with_name(path))
            .build()
            .context("Echec du chargement depuis le fichier")?;

        config
            .try_deserialize()
            .context("Echec de la deserialisation de la configuration")
    }

    /// Valide la configuration
    pub fn validate(&self) -> Result<()> {
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.logging.level.as_str()) {
            anyhow::bail!(
                "Niveau de log invalide: {}. Acceptes: {:?}",
                self.logging.level,
                valid_levels
            );
        }

        if self.validation.min_coverage_percent > 100 {
            anyhow::bail!(
                "Seuil de couverture invalide: {}%",
                self.validation.min_coverage_percent
            );
        }

        // Validation du profil de conformite
        let valid_profiles = [
            "general",
            "aviation",
            "medical",
            "automotive",
            "railway",
            "safety",
        ];
        if !valid_profiles.contains(&self.compliance.profile.to_lowercase().as_str()) {
            anyhow::bail!(
                "Profil de conformite invalide: {}. Acceptes: {:?}",
                self.compliance.profile,
                valid_profiles
            );
        }

        for (label, pct) in [
            ("P1", self.compliance.min_p1_coverage),
            ("P2", self.compliance.min_p2_coverage),
            ("P3", self.compliance.min_p3_coverage),
        ] {
            if pct > 100 {
                anyhow::bail!("Seuil de couverture {} invalide: {}%", label, pct);
            }
        }

        // Token budget doit etre > 0
        if self.pipeline.token_budget == 0 {
            anyhow::bail!("pipeline.token_budget doit etre > 0");
        }

        // Timeout LLM doit etre > 0
        if self.llm.timeout_secs == 0 {
            anyhow::bail!("llm.timeout_secs doit etre > 0");
        }

        // max_retries raisonnable
        if self.pipeline.max_retries > 10 {
            anyhow::bail!(
                "pipeline.max_retries trop eleve: {} (max 10)",
                self.pipeline.max_retries
            );
        }

        // URL LLM doit commencer par http:// ou https://
        if !self.llm.api_base_url.starts_with("http://")
            && !self.llm.api_base_url.starts_with("https://")
        {
            anyhow::bail!(
                "llm.api_base_url invalide: '{}' (doit commencer par http:// ou https://)",
                self.llm.api_base_url
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.pipeline.default_language, "fr");
        assert_eq!(config.llm.model_name, "qwen3:8b");
        assert_eq!(config.output.gherkin_language, "fr");
        assert_eq!(config.validation.min_coverage_percent, 80);
    }

    #[test]
    fn test_config_validate_valid() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_invalid_level() {
        let mut config = Config::default();
        config.logging.level = "invalid".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validate_invalid_coverage() {
        let mut config = Config::default();
        config.validation.min_coverage_percent = 150;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_compliance_config_default() {
        let config = ComplianceConfig::default();
        assert_eq!(config.profile, "general");
        assert!(config.safety_level.is_none());
        assert!(config.include_metadata);
        assert!(!config.strict_validation);
        assert!(!config.require_rationale);
        assert!(!config.require_risk_level);
        assert!(config.normative_keywords);
        assert_eq!(config.min_p1_coverage, 100);
        assert_eq!(config.min_p2_coverage, 80);
        assert_eq!(config.min_p3_coverage, 60);
    }

    #[test]
    fn test_compliance_to_profile_general() {
        let config = ComplianceConfig::default();
        let profile = config.to_compliance_profile();
        assert!(matches!(profile, Some(ComplianceProfile::General)));
    }

    #[test]
    fn test_compliance_to_profile_aviation() {
        let config = ComplianceConfig {
            profile: "aviation".to_string(),
            safety_level: Some("DAL-B".to_string()),
            ..Default::default()
        };
        let profile = config.to_compliance_profile();
        assert!(matches!(
            profile,
            Some(ComplianceProfile::Aviation(
                crate::domain::specification::DalLevel::B
            ))
        ));
    }

    #[test]
    fn test_compliance_to_profile_medical() {
        let config = ComplianceConfig {
            profile: "medical".to_string(),
            safety_level: Some("SW-C".to_string()),
            ..Default::default()
        };
        let profile = config.to_compliance_profile();
        assert!(matches!(
            profile,
            Some(ComplianceProfile::Medical(
                crate::domain::specification::SwClass::C
            ))
        ));
    }

    #[test]
    fn test_compliance_to_profile_unknown() {
        let config = ComplianceConfig {
            profile: "unknown".to_string(),
            ..Default::default()
        };
        let profile = config.to_compliance_profile();
        assert!(profile.is_none());
    }

    #[test]
    fn test_config_validate_invalid_profile() {
        let mut config = Config::default();
        config.compliance.profile = "invalid".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validate_invalid_p1_coverage() {
        let mut config = Config::default();
        config.compliance.min_p1_coverage = 150;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validate_zero_token_budget() {
        let mut config = Config::default();
        config.pipeline.token_budget = 0;
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("token_budget"));
    }

    #[test]
    fn test_config_validate_zero_timeout() {
        let mut config = Config::default();
        config.llm.timeout_secs = 0;
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("timeout_secs"));
    }

    #[test]
    fn test_config_validate_invalid_url() {
        let mut config = Config::default();
        config.llm.api_base_url = "ftp://invalid".to_string();
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("api_base_url"));
    }

    #[test]
    fn test_config_validate_excessive_retries() {
        let mut config = Config::default();
        config.pipeline.max_retries = 99;
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("max_retries"));
    }

    #[test]
    fn test_compliance_to_profile_automotive() {
        let config = ComplianceConfig {
            profile: "automotive".to_string(),
            safety_level: Some("ASIL-D".to_string()),
            ..Default::default()
        };
        let profile = config.to_compliance_profile();
        assert!(matches!(
            profile,
            Some(ComplianceProfile::Automotive(
                crate::domain::specification::AsilLevel::D
            ))
        ));
    }

    #[test]
    fn test_compliance_to_profile_railway() {
        let config = ComplianceConfig {
            profile: "railway".to_string(),
            safety_level: Some("SSIL-3".to_string()),
            ..Default::default()
        };
        let profile = config.to_compliance_profile();
        assert!(matches!(
            profile,
            Some(ComplianceProfile::Railway(
                crate::domain::specification::SsilLevel::Level3
            ))
        ));
    }

    #[test]
    fn test_compliance_to_profile_safety() {
        let config = ComplianceConfig {
            profile: "safety".to_string(),
            safety_level: Some("SIL-4".to_string()),
            ..Default::default()
        };
        let profile = config.to_compliance_profile();
        assert!(matches!(
            profile,
            Some(ComplianceProfile::Safety(
                crate::domain::specification::SilLevel::Sil4
            ))
        ));
    }

    #[test]
    fn test_compliance_to_profile_default_levels() {
        // Sans safety_level, les profils utilisent un niveau par defaut
        let aviation = ComplianceConfig {
            profile: "aviation".to_string(),
            safety_level: None,
            ..Default::default()
        };
        assert!(matches!(
            aviation.to_compliance_profile(),
            Some(ComplianceProfile::Aviation(
                crate::domain::specification::DalLevel::E
            ))
        ));

        let safety = ComplianceConfig {
            profile: "safety".to_string(),
            safety_level: None,
            ..Default::default()
        };
        assert!(matches!(
            safety.to_compliance_profile(),
            Some(ComplianceProfile::Safety(
                crate::domain::specification::SilLevel::Sil1
            ))
        ));
    }

    #[test]
    fn test_compliance_profile_case_insensitive() {
        let config = ComplianceConfig {
            profile: "AVIATION".to_string(),
            safety_level: Some("a".to_string()),
            ..Default::default()
        };
        let profile = config.to_compliance_profile();
        assert!(matches!(
            profile,
            Some(ComplianceProfile::Aviation(
                crate::domain::specification::DalLevel::A
            ))
        ));
    }

    #[test]
    fn test_config_default_completeness() {
        // Verifie que la config par defaut est coherente et valide
        let config = Config::default();
        assert!(
            config.validate().is_ok(),
            "La config par defaut DOIT etre valide"
        );
        assert_eq!(config.pipeline.token_budget, 2000);
        assert_eq!(config.compliance.profile, "general");
        assert!(config.output.traceability);
        assert!(config.validation.validate_gherkin_syntax);
    }

    #[test]
    fn test_config_validate_invalid_p2_p3_coverage() {
        let mut config = Config::default();
        config.compliance.min_p2_coverage = 200;
        assert!(config.validate().is_err());

        let mut config2 = Config::default();
        config2.compliance.min_p3_coverage = 101;
        assert!(config2.validate().is_err());
    }
}
