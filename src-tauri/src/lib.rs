//! Spec Forge — Application Tauri 2

pub mod presentation;

use std::sync::Arc;

use tauri::Manager;

use presentation::commands::{config_commands, file_commands, llm_commands, pipeline_commands};
use presentation::state::AppState;

use spec_forge::adapters::llm::ollama_adapter::OllamaAdapter;
use spec_forge::adapters::templates::file_template_engine::FileTemplateEngine;
use spec_forge::application::pipeline::Pipeline;
use spec_forge::infrastructure::config::Config;

/// Lancement de l'application Tauri
pub fn run() {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_ansi(true)
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(move |app| {
            let config = Config::load().unwrap_or_else(|_| {
                tracing::warn!("config.yaml non trouve, utilisation des valeurs par defaut");
                Config::default()
            });

            let llm: Arc<dyn spec_forge::ports::llm_service::LlmService> =
                Arc::new(OllamaAdapter::new(config.llm.clone()).map_err(|e| {
                    tracing::error!("Erreur initialisation OllamaAdapter: {e}");
                    e.to_string()
                })?);

            let templates = Arc::new(
                FileTemplateEngine::new(&config.templates.directory).map_err(|e| {
                    tracing::error!("Erreur chargement templates: {e}");
                    e.to_string()
                })?,
            );

            let pipeline = Arc::new(Pipeline::new(
                llm.clone(),
                templates,
                config.clone(),
            ));

            let app_state = AppState {
                pipeline,
                llm,
                config: Arc::new(tokio::sync::RwLock::new(config)),
            };

            app.manage(app_state);

            tracing::info!("Spec Forge demarre avec succes");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            llm_commands::check_llm_status,
            llm_commands::get_llm_config,
            config_commands::get_config,
            file_commands::read_stories,
            file_commands::expand_input_paths,
            pipeline_commands::run_full_pipeline,
            pipeline_commands::build_traceability,
        ])
        .run(tauri::generate_context!())
        .expect("Erreur lors du lancement de l'application Tauri");
}
