//! Etat partage de l'application Tauri

use std::sync::Arc;

use spec_forge::application::pipeline::Pipeline;
use spec_forge::infrastructure::config::Config;
use spec_forge::ports::llm_service::LlmService;
use spec_forge::ports::template_engine::TemplateEngine;

/// Etat global de l'application, injecte via `app.manage()`
pub struct AppState {
    /// Pipeline — `None` si Ollama indisponible au demarrage
    pub pipeline: Arc<tokio::sync::RwLock<Option<Arc<Pipeline>>>>,
    /// Service LLM — `None` si Ollama indisponible au demarrage
    pub llm: Arc<tokio::sync::RwLock<Option<Arc<dyn LlmService>>>>,
    pub config: Arc<tokio::sync::RwLock<Config>>,
    /// Moteur de templates charge au demarrage
    pub templates: Arc<dyn TemplateEngine>,
    /// Canal d'annulation du pull de modele
    pub pull_cancel: Arc<tokio::sync::RwLock<Option<tokio::sync::watch::Sender<bool>>>>,
}
