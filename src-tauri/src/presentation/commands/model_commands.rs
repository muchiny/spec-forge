//! Commandes de gestion des modeles Ollama — detection, telechargement, initialisation

use std::sync::Arc;

use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

use spec_forge::adapters::llm::ollama_adapter::OllamaAdapter;
use spec_forge::application::pipeline::Pipeline;

use crate::presentation::events::{EVENT_MODEL_PULL_PROGRESS, ModelPullProgressPayload};
use crate::presentation::state::AppState;

/// Statut du systeme Ollama
#[derive(Serialize)]
pub struct OllamaSystemStatus {
    pub ollama_running: bool,
    pub model_name: String,
    pub model_installed: bool,
    pub url: String,
}

/// Verifie l'etat complet du systeme Ollama (serveur + modele)
#[tauri::command]
pub async fn check_ollama_system(state: State<'_, AppState>) -> Result<OllamaSystemStatus, String> {
    let config = state.config.read().await;
    let model_name = config.llm.model_name.clone();
    let url = config.llm.api_base_url.clone();

    // Essayer de creer un adapter temporaire pour verifier
    let adapter = match OllamaAdapter::new(config.llm.clone()) {
        Ok(a) => a,
        Err(_) => {
            return Ok(OllamaSystemStatus {
                ollama_running: false,
                model_name,
                model_installed: false,
                url,
            });
        }
    };

    let server_ok = adapter.check_server().await;
    if !server_ok {
        return Ok(OllamaSystemStatus {
            ollama_running: false,
            model_name,
            model_installed: false,
            url,
        });
    }

    let model_installed = adapter.check_model().await.unwrap_or(false);

    Ok(OllamaSystemStatus {
        ollama_running: true,
        model_name,
        model_installed,
        url,
    })
}

/// Lance le telechargement d'un modele Ollama en arriere-plan
#[tauri::command]
pub async fn pull_model(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    // Verifier qu'un pull n'est pas deja en cours
    {
        let guard = state.pull_cancel.read().await;
        if guard.is_some() {
            return Err("Un telechargement est deja en cours".to_string());
        }
    }

    let config = state.config.read().await;
    let model_name = config.llm.model_name.clone();

    let adapter = OllamaAdapter::new(config.llm.clone()).map_err(|e| e.to_string())?;

    // Creer le canal d'annulation
    let (cancel_tx, cancel_rx) = tokio::sync::watch::channel(false);
    {
        let mut guard = state.pull_cancel.write().await;
        *guard = Some(cancel_tx);
    }

    let pull_cancel = state.pull_cancel.clone();
    let app_handle = app.clone();

    tokio::spawn(async move {
        let result = adapter
            .pull_model(&model_name, cancel_rx, move |progress| {
                let _ = app_handle.emit(
                    EVENT_MODEL_PULL_PROGRESS,
                    ModelPullProgressPayload {
                        status: progress.status.clone(),
                        completed: progress.completed,
                        total: progress.total,
                        digest: progress.digest.clone(),
                    },
                );
            })
            .await;

        // Nettoyer le cancel sender
        {
            let mut guard = pull_cancel.write().await;
            *guard = None;
        }

        match result {
            Ok(()) => {
                let _ = app.emit(
                    EVENT_MODEL_PULL_PROGRESS,
                    ModelPullProgressPayload {
                        status: "success".to_string(),
                        completed: None,
                        total: None,
                        digest: None,
                    },
                );
            }
            Err(e) => {
                let _ = app.emit(
                    EVENT_MODEL_PULL_PROGRESS,
                    ModelPullProgressPayload {
                        status: format!("error: {e}"),
                        completed: None,
                        total: None,
                        digest: None,
                    },
                );
            }
        }
    });

    Ok(())
}

/// Annule un pull de modele en cours
#[tauri::command]
pub async fn cancel_pull(state: State<'_, AppState>) -> Result<(), String> {
    let guard = state.pull_cancel.read().await;
    if let Some(tx) = guard.as_ref() {
        let _ = tx.send(true);
        Ok(())
    } else {
        Err("Aucun telechargement en cours".to_string())
    }
}

/// Re-initialise le LLM et le Pipeline apres un pull reussi
#[tauri::command]
pub async fn initialize_llm(state: State<'_, AppState>) -> Result<(), String> {
    let config = state.config.read().await;

    let adapter = OllamaAdapter::new(config.llm.clone()).map_err(|e| e.to_string())?;
    let llm: Arc<dyn spec_forge::ports::llm_service::LlmService> = Arc::new(adapter);

    let templates = state.templates.clone();
    let pipeline = Arc::new(Pipeline::new(llm.clone(), templates, config.clone()));

    // Mettre a jour l'etat partage
    {
        let mut llm_guard = state.llm.write().await;
        *llm_guard = Some(llm);
    }
    {
        let mut pipeline_guard = state.pipeline.write().await;
        *pipeline_guard = Some(pipeline);
    }

    tracing::info!("LLM et Pipeline re-initialises avec succes");
    Ok(())
}
