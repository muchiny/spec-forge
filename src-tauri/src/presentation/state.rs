//! Etat partage de l'application Tauri

use std::sync::Arc;

use spec_forge::application::pipeline::Pipeline;
use spec_forge::infrastructure::config::Config;
use spec_forge::ports::llm_service::LlmService;

/// Etat global de l'application, injecte via `app.manage()`
pub struct AppState {
    pub pipeline: Arc<Pipeline>,
    pub llm: Arc<dyn LlmService>,
    pub config: Arc<tokio::sync::RwLock<Config>>,
}
