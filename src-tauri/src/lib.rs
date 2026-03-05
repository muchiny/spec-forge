//! Spec Forge — Application Tauri 2

pub mod presentation;

use std::sync::Arc;

use tauri::Manager;

use presentation::commands::{
    config_commands, file_commands, llm_commands, model_commands, pipeline_commands,
};
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
            // Chercher config.yaml : d'abord dans le CWD, sinon dans le parent (cargo tauri dev)
            let config = if std::path::Path::new("config.yaml").exists() {
                Config::load().unwrap_or_default()
            } else if std::path::Path::new("../config.yaml").exists() {
                Config::load_from_file("../config").unwrap_or_default()
            } else {
                tracing::warn!("config.yaml non trouve, utilisation des valeurs par defaut");
                Config::default()
            };
            tracing::info!(
                max_tokens = config.llm.max_tokens,
                context_size = config.llm.context_size,
                timeout_secs = config.llm.timeout_secs,
                "Configuration LLM chargee"
            );

            // Resoudre le chemin des templates (relatif au repertoire de l'executable)
            let template_dir = if config.templates.directory.is_relative() {
                let exe_dir = std::env::current_exe()
                    .ok()
                    .and_then(|p| p.parent().map(|d| d.to_path_buf()));

                // Chercher d'abord a cote de l'exe, puis dans le cwd, puis remonter d'un niveau (dev)
                let candidates = [
                    exe_dir.as_ref().map(|d| d.join(&config.templates.directory)),
                    Some(config.templates.directory.clone()),
                    Some(std::path::PathBuf::from("..").join(&config.templates.directory)),
                ];

                candidates
                    .into_iter()
                    .flatten()
                    .find(|p| p.exists())
                    .unwrap_or_else(|| config.templates.directory.clone())
            } else {
                config.templates.directory.clone()
            };

            tracing::info!("Repertoire templates: {}", template_dir.display());

            // Templates restent fatales (erreur de packaging)
            let templates: Arc<dyn spec_forge::ports::template_engine::TemplateEngine> = Arc::new(
                FileTemplateEngine::new(&template_dir).map_err(|e| {
                    tracing::error!("Erreur chargement templates: {e}");
                    e.to_string()
                })?,
            );

            // LLM et Pipeline : non-fatals si Ollama indisponible
            let (llm, pipeline) = match OllamaAdapter::new(config.llm.clone()) {
                Ok(adapter) => {
                    let llm: Arc<dyn spec_forge::ports::llm_service::LlmService> =
                        Arc::new(adapter);
                    let pipeline =
                        Arc::new(Pipeline::new(llm.clone(), templates.clone(), config.clone()));
                    (Some(llm), Some(pipeline))
                }
                Err(e) => {
                    tracing::warn!(
                        "Ollama indisponible au demarrage ({e}), \
                         l'application demarre sans LLM"
                    );
                    (None, None)
                }
            };

            let app_state = AppState {
                pipeline: Arc::new(tokio::sync::RwLock::new(pipeline)),
                llm: Arc::new(tokio::sync::RwLock::new(llm)),
                config: Arc::new(tokio::sync::RwLock::new(config)),
                templates,
                pull_cancel: Arc::new(tokio::sync::RwLock::new(None)),
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
            model_commands::check_ollama_system,
            model_commands::pull_model,
            model_commands::cancel_pull,
            model_commands::initialize_llm,
        ])
        .run(tauri::generate_context!())
        .expect("Erreur lors du lancement de l'application Tauri");
}
