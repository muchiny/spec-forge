//! Port LLM Service - Interface pour les services de generation de texte
//!
//! Port depuis mcp-doc-rag avec ajout de generate_json.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Configuration du service LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// Activer le service LLM
    pub enabled: bool,

    /// Provider: "ollama", "openai"
    pub provider: String,

    /// Nom du modele (ex: "qwen2.5:7b")
    pub model_name: String,

    /// URL de base de l'API
    pub api_base_url: String,

    /// Cle API (optionnel pour Ollama local)
    #[serde(default)]
    pub api_key: Option<String>,

    /// Nombre maximum de tokens en sortie
    pub max_tokens: usize,

    /// Temperature (0.0 = deterministe, 1.0 = creatif)
    pub temperature: f32,

    /// Timeout en secondes
    pub timeout_secs: u64,

    /// Taille du contexte (num_ctx) envoyee a Ollama.
    /// Reduire economise de la VRAM (KV cache). 8192 suffit pour spec-forge.
    #[serde(default = "default_context_size")]
    pub context_size: usize,
}

fn default_context_size() -> usize {
    8192
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            provider: "ollama".to_string(),
            model_name: "qwen3:8b".to_string(),
            api_base_url: "http://localhost:11434".to_string(),
            api_key: None,
            max_tokens: 4096,
            temperature: 0.1,
            timeout_secs: 300,
            context_size: default_context_size(),
        }
    }
}

/// Reponse du LLM
#[derive(Debug, Clone)]
pub struct LlmResponse {
    /// Contenu genere
    pub content: String,
    /// Nombre de tokens utilises
    pub tokens_used: usize,
    /// Raison de fin de generation
    pub finish_reason: FinishReason,
}

/// Raison de fin de generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinishReason {
    Stop,
    Length,
    Error,
}

/// Erreurs du service LLM
#[derive(Error, Debug)]
pub enum LlmError {
    #[error("Echec de connexion: {0}")]
    ConnectionError(String),

    #[error("Erreur API ({status_code}): {message}")]
    ApiError { status_code: u16, message: String },

    #[error("Erreur de parsing JSON: {0}")]
    ParseError(String),

    #[error("Timeout apres {0}s")]
    Timeout(u64),

    #[error("Modele non trouve: {0}")]
    ModelNotFound(String),

    #[error("Service desactive")]
    Disabled,

    #[error("Erreur de configuration: {0}")]
    ConfigError(String),
}

/// Informations sur le modele LLM
#[derive(Debug, Clone)]
pub struct LlmModelInfo {
    pub name: String,
    pub provider: String,
    pub context_size: Option<usize>,
}

/// Trait definissant le service LLM
#[async_trait]
pub trait LlmService: Send + Sync {
    /// Genere du texte a partir d'un prompt
    async fn generate(&self, prompt: &str) -> Result<LlmResponse, LlmError>;

    /// Genere du texte avec un prompt systeme
    async fn generate_with_system(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<LlmResponse, LlmError>;

    /// Genere du JSON structure (utilise format:"json" avec Ollama)
    async fn generate_json(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<LlmResponse, LlmError>;

    /// Verifie si le service est pret
    async fn is_ready(&self) -> bool;

    /// Retourne la configuration
    fn config(&self) -> &LlmConfig;

    /// Retourne les informations sur le modele
    fn model_info(&self) -> LlmModelInfo;

    /// Estime le nombre de tokens d'un texte
    fn estimate_tokens(&self, text: &str) -> usize {
        text.len() / 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_llm_config_default() {
        let config = LlmConfig::default();
        assert!(config.enabled);
        assert_eq!(config.provider, "ollama");
        assert_eq!(config.model_name, "qwen3:8b");
        assert!((config.temperature - 0.1).abs() < 0.01);
    }
}
