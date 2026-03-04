//! Commandes de configuration

use tauri::State;

use crate::presentation::state::AppState;

/// Retourne la configuration complete
#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let config = state.config.read().await;
    serde_json::to_value(&*config).map_err(|e| e.to_string())
}
