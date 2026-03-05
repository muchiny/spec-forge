//! Commandes LLM — verification du statut et configuration

use serde::Serialize;
use tauri::State;

use crate::presentation::state::AppState;

/// Reponse du statut LLM
#[derive(Serialize)]
pub struct LlmStatusResponse {
    pub ready: bool,
    pub model_name: String,
    pub provider: String,
    pub url: String,
}

/// Verifie si le LLM est accessible
#[tauri::command]
pub async fn check_llm_status(state: State<'_, AppState>) -> Result<LlmStatusResponse, String> {
    let llm_guard = state.llm.read().await;
    match llm_guard.as_ref() {
        Some(llm) => {
            let config = llm.config();
            let ready = llm.is_ready().await;
            Ok(LlmStatusResponse {
                ready,
                model_name: config.model_name.clone(),
                provider: config.provider.clone(),
                url: config.api_base_url.clone(),
            })
        }
        None => {
            let config = state.config.read().await;
            Ok(LlmStatusResponse {
                ready: false,
                model_name: config.llm.model_name.clone(),
                provider: config.llm.provider.clone(),
                url: config.llm.api_base_url.clone(),
            })
        }
    }
}

/// Retourne la configuration LLM actuelle
#[tauri::command]
pub async fn get_llm_config(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let config = state.config.read().await;
    serde_json::to_value(&config.llm).map_err(|e| e.to_string())
}
